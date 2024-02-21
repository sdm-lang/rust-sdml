/*!
Provide a generator for "concept" diagrams via GraphViz.

# Example

```rust,no_run
use sdml_core::cache::ModuleCache;
use sdml_core::model::modules::Module;
use sdml_generate::GenerateToWriter;
use sdml_generate::draw::concepts::ConceptDiagramGenerator;
use std::io::stdout;
# use sdml_core::model::identifiers::Identifier;
# fn load_module() -> (Module, ModuleCache) { (Module::empty(Identifier::new_unchecked("example")), ModuleCache::default()) }

let (module, cache) = load_module();

let mut generator = ConceptDiagramGenerator::default();
generator.write(&module, &cache,  &mut stdout()).expect("write to stdout failed");
```

*/

use crate::draw::OutputFormat;
use crate::exec::exec_with_temp_input;
use crate::GenerateToWriter;
use sdml_core::cache::ModuleCache;
use sdml_core::error::Error;
use sdml_core::model::definitions::Definition;
use sdml_core::model::definitions::HasMembers;
use sdml_core::model::members::HasCardinality;
use sdml_core::model::members::HasType;
use sdml_core::model::members::{Cardinality, TypeReference, DEFAULT_CARDINALITY};
use sdml_core::model::modules::Module;
use sdml_core::model::{HasBody, HasName, HasOptionalBody};
use std::collections::HashSet;
use std::io::Write;

use super::DOT_PROGRAM;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct ConceptDiagramGenerator {}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl GenerateToWriter<OutputFormat> for ConceptDiagramGenerator {
    fn write_in_format<W>(
        &mut self,
        module: &Module,
        _cache: &ModuleCache,
        writer: &mut W,
        format: OutputFormat,
    ) -> Result<(), Error>
    where
        W: Write + Sized,
    {
        let mut buffer = Vec::new();
        write_module(module, &mut buffer)?;

        if format == OutputFormat::Source {
            writer.write_all(&buffer)?;
        } else {
            let source = String::from_utf8(buffer).unwrap();
            match exec_with_temp_input(DOT_PROGRAM, vec![format.into()], source) {
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

fn write_module(me: &Module, writer: &mut dyn Write) -> Result<(), Error> {
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
        let current = entity.name().to_string();
        entities.insert(current.clone());

        if let Some(body) = entity.body() {
            for member in body.members() {
                let type_ref = if let Some(property_name) = member.as_property_reference() {
                    let property = me
                        .body()
                        .property_definitions()
                        .find(|p| p.name() == property_name.member())
                        .unwrap();
                    let role = property
                        .body()
                        .map(|b| b.roles().find(|r| r.name() == member.name()).unwrap())
                        .unwrap();
                    role.target_type()
                } else if let Some(definition) = member.as_definition() {
                    definition.target_type()
                } else {
                    unreachable!()
                };
                if let TypeReference::Type(type_name) = type_ref {
                    if let Some(Definition::Entity(entity)) = me.resolve_local(type_name.member()) {
                        entities.insert(entity.name().to_string());
                        if let Some(property_name) = member.as_property_reference() {
                            relations.push(format!(
                                "  {current} -> {} [label=\"{}\";dir=\"both\";arrowtail=\"teetee\";arrowhead=\"teetee\"];\n",
                                property_name,
                                member.name(),
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
                            let from_str = if let Some(name) = definition.inverse_name() {
                                name.to_string()
                            } else {
                                String::new()
                            };
                            let target_cardinality = definition.target_cardinality();
                            let to_str = if *target_cardinality == DEFAULT_CARDINALITY {
                                String::new()
                            } else {
                                to_uml_string(target_cardinality)
                            };
                            relations.push(format!(
                                "  {current} -> {target_type} [label=\"{}\"; taillabel=\"{from_str}\"; headlabel=\"{to_str}\"];\n",
                                member.name()
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

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
