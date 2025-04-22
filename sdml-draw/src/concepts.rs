/*!
Provide a generator for "concept" diagrams via GraphViz.

# Example

```rust,no_run
use sdml_core::store::InMemoryModuleCache;
use sdml_core::model::modules::Module;
use sdml_generate::Generator;
use sdml_generate::draw::concepts::{ConceptDiagramGenerator, ConceptDiagramOptions};
use std::io::stdout;
# use sdml_core::model::identifiers::Identifier;
# fn load_module() -> (Module, InMemoryModuleCache) { (Module::empty(Identifier::new_unchecked("example")), InMemoryModuleCache::default()) }

let (module, cache) = load_module();

let mut generator = ConceptDiagramGenerator::default();
generator.generate(&module, &cache, None, &mut stdout()).expect("write to stdout failed");
```

*/

use crate::draw::{
    filter::{DefinitionKind, DiagramContentFilter},
    OutputFormat, DOT_PROGRAM,
};
use crate::exec::exec_with_temp_input;
use crate::Generator;
use sdml_core::error::Error;
use sdml_core::model::definitions::Definition;
use sdml_core::model::identifiers::IdentifierReference;
use sdml_core::model::members::MemberKind;
use sdml_core::model::members::{Cardinality, TypeReference, DEFAULT_CARDINALITY};
use sdml_core::model::modules::Module;
use sdml_core::model::{HasBody, HasName, HasOptionalBody};
use sdml_core::store::ModuleStore;
use std::collections::HashSet;
use std::io::Write;
use std::path::PathBuf;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct ConceptDiagramGenerator {
    options: ConceptDiagramOptions,
}

#[derive(Debug, Default)]
pub struct ConceptDiagramOptions {
    content_filter: DiagramContentFilter,
    output_format: OutputFormat,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl ConceptDiagramOptions {
    pub fn with_content_filter(self, content_filter: DiagramContentFilter) -> Self {
        Self {
            content_filter,
            ..self
        }
    }

    pub fn with_output_format(self, output_format: OutputFormat) -> Self {
        Self {
            output_format,
            ..self
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Generator for ConceptDiagramGenerator {
    type Options = ConceptDiagramOptions;

    fn generate_with_options<W>(
        &mut self,
        module: &Module,
        cache: &impl ModuleStore,
        options: Self::Options,
        _: Option<PathBuf>,
        writer: &mut W,
    ) -> Result<(), Error>
    where
        W: Write + Sized,
    {
        self.options = options;

        let mut buffer = Vec::new();
        write_module(module, cache, &self.options.content_filter, &mut buffer)?;

        if self.options.output_format == OutputFormat::Source {
            writer.write_all(&buffer)?;
        } else {
            let source = String::from_utf8(buffer).unwrap();
            match exec_with_temp_input(DOT_PROGRAM, vec![self.options.output_format.into()], source)
            {
                Ok(result) => {
                    writer.write_all(result.as_bytes())?;
                }
                Err(e) => {
                    panic!("exec_with_input failed: {:?}", e);
                }
            }
        }

        Ok(())
    }
}

fn write_module(
    me: &Module,
    cache: &impl ModuleStore,
    content_filter: &DiagramContentFilter,
    writer: &mut dyn Write,
) -> Result<(), Error> {
    writer.write_all(
        r#"digraph G {
  bgcolor="transparent";
  rankdir="TB";
  fontname="Helvetica,Arial,sans-serif";
  node [fontname="Helvetica,Arial,sans-serif"; fontsize=10];
  edge [fontname="Helvetica,Arial,sans-serif"; fontsize=9; fontcolor="dimgrey";
        labelfontcolor="blue"; labeldistance=2.0];

"#
        .as_bytes(),
    )?;

    let mut entities: HashSet<String> = Default::default();
    let mut relations: Vec<String> = Default::default();
    for entity in me.body().entity_definitions() {
        if content_filter.draw_definition_named(DefinitionKind::Entity, entity.name()) {
            let current = entity.name().to_string();
            entities.insert(current.clone());

            if let Some(body) = entity.body() {
                for member in body.members() {
                    let (member_name, member_type) = match member.kind() {
                        MemberKind::Reference(v) => {
                            if let Some(Definition::Property(property)) = match &v {
                                IdentifierReference::Identifier(v) => me.resolve_local(v),
                                IdentifierReference::QualifiedIdentifier(v) => cache.resolve(v),
                            } {
                                (
                                    property.member_def().name(),
                                    property.member_def().target_type(),
                                )
                            } else {
                                panic!()
                            }
                        }
                        MemberKind::Definition(v) => (v.name(), v.target_type()),
                    };
                    let definition = match member_type {
                        TypeReference::Type(IdentifierReference::Identifier(v)) => {
                            me.resolve_local(v)
                        }
                        TypeReference::Type(IdentifierReference::QualifiedIdentifier(v)) => {
                            cache.resolve(v)
                        }
                        _ => panic!(),
                    };
                    if let Some(Definition::Entity(entity)) = definition {
                        entities.insert(entity.name().to_string());
                        if let Some(property_name) = member.as_property_reference() {
                            relations.push(format!(
                        "  {current} -> {} [label=\"{}\";dir=\"both\";arrowtail=\"teetee\";arrowhead=\"teetee\"];\n",
                        property_name,
                        member_name,
                    ));
                        } else if let Some(definition) = member.as_definition() {
                            if matches!(definition.target_type(), TypeReference::Unknown) {
                                entities.insert("unknown".to_string());
                            }
                            let target_type = if let TypeReference::Type(target_type) =
                                definition.target_type()
                            {
                                target_type.to_string().to_lowercase()
                            } else {
                                "unknown".to_string()
                            };
                            let target_cardinality = definition.target_cardinality();
                            let head_str = if *target_cardinality == DEFAULT_CARDINALITY {
                                String::new()
                            } else {
                                to_uml_string(target_cardinality)
                            };
                            relations.push(format!(
                        "  {current} -> {target_type} [label=\"{}\"; headlabel=\"{head_str}\"];\n",
                        member_name
                    ));
                        }
                    }
                }
            }
        }
    }

    writer.write_all(
        entities
            .iter()
            .map(|name| format!("  {name} [label=\"{name}\"];"))
            .collect::<Vec<String>>()
            .join("\n")
            .as_bytes(),
    )?;

    writer.write_all(relations.join("\n").as_bytes())?;

    writer.write_all(b"}\n")?;

    Ok(())
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn to_uml_string(card: &Cardinality) -> String {
    card.range().to_string()
}
