/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::draw::OutputFormat;
use crate::exec::{exec_with_temp_input, CommandArg};
use sdml_core::error::Error;
use sdml_core::generate::GenerateToFile;
use sdml_core::model::constraints::ControlledLanguageTag;
use sdml_core::model::identifiers::{Identifier, IdentifierReference};
use sdml_core::model::members::{
    Cardinality, 
    TypeReference, DEFAULT_CARDINALITY, Ordering, Uniqueness,
};
use sdml_core::model::modules::Module;
use sdml_core::model::values::Value;
use sdml_core::model::walk::{walk_module_simple, SimpleModuleWalker};
use sdml_core::model::{HasName, References, Span};
use sdml_core::syntax::{KW_ORDERING_ORDERED, KW_UNIQUENESS_UNIQUE};
use std::collections::HashSet;
use std::path::Path;
use tracing::debug;
use sdml_core::load::ModuleLoader;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct UmlDiagramGenerator {
    buffer: String,
    imports: (String, String),
    output: Option<DiagramOutput>,
    assoc_src: Option<String>,
    refs: Option<String>,
    emit_annotations: bool,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Default)]
struct DiagramOutput {
    file_name: String,
    output_dir: String,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

pub const UML_PROGRAM: &str = "plantuml";

impl GenerateToFile<OutputFormat> for UmlDiagramGenerator {
    fn write_to_file_in_format(
        &mut self,
        module: &Module,
        _loader: Option<&mut dyn ModuleLoader>,
        path: &Path,
        format: OutputFormat,
    ) -> Result<(), Error> {
        self.imports = make_imports(module);
        self.output = Some(path_to_output(path, module.name())?);

        walk_module_simple(module, self)?;

        if format == OutputFormat::Source {
            std::fs::write(path, &self.buffer)?;
        } else {
            match exec_with_temp_input(
                UML_PROGRAM,
                vec![
                    CommandArg::new(format!(
                        "-o{}",
                        self.output.as_ref().map(|o| &o.output_dir).unwrap()
                    )),
                    format_to_arg(format),
                ],
                &self.buffer,
            ) {
                Ok(result) => {
                    debug!("Response from command: {:?}", result);
                }
                Err(e) => {
                    panic!("exec_with_input failed: {:?}", e);
                }
            }
        }

        Ok(())
    }
}

impl SimpleModuleWalker for UmlDiagramGenerator {
    fn start_module(&mut self, name: &Identifier, _span: Option<&Span>) -> Result<(), Error> {
        self.buffer.push_str(&format!(
            r#"@startuml {}
skinparam backgroundColor transparent
skinparam style strictuml
skinparam linetype polyline
skinparam nodesep 50

hide methods
hide circle

show << datatype >> circle
show << entity >> circle
show enum circle
show << event >> circle
show << union >> circle

{}
package "{name}" as {} <<module>> {{
"#,
            self.output
                .as_ref()
                .map(|o| o.file_name.to_string())
                .unwrap_or_else(|| name.to_string()),
            self.imports.0,
            make_id(name),
        ));

        Ok(())
    }

    fn annotation_property(
        &mut self,
        name: &IdentifierReference,
        value: &Value,
        _span: Option<&Span>,
    ) -> Result<(), Error> {
        if self.emit_annotations {
            self.buffer.push_str(&format!("{{{name} = {value}}}\n"));
        }
        Ok(())
    }

    fn informal_constraint(
        &mut self,
        _name: &Identifier,
        _value: &str,
        _language: Option<&ControlledLanguageTag>,
        _span: Option<&Span>,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn start_datatype(
        &mut self,
        name: &Identifier,
        _is_opaque: bool,
        base_type: &IdentifierReference,
        _has_body: bool,
        _span: Option<&Span>,
    ) -> Result<(), Error> {
        self.buffer
            .push_str(&start_type_with_sterotype("class", name, "datatype"));

        // TODO: add opaque as stereotype on restriction
        let restriction = format!("  {} --|> {}\n", make_id(name), make_id(base_type));
        self.refs = Some(self.refs.clone().map(|r| format!("{r}{restriction}")).unwrap_or(restriction));
        self.emit_annotations = true;
        Ok(())
    }

    fn end_datatype(&mut self, name: &Identifier, had_body: bool) -> Result<(), Error> {
        self.buffer.push_str("  }\n");
        self.buffer
            .push_str(&format!("  hide {} {}\n", make_id(name), if had_body { "methods" } else { "members" }));

        self.assoc_src = None;
        self.emit_annotations = false;

        Ok(())
    }

    fn start_entity(
        &mut self,
        name: &Identifier,
        has_body: bool,
        _span: Option<&Span>,
    ) -> Result<(), Error> {
        self.buffer
            .push_str(&start_type_with_sterotype(if has_body { "class" } else { "abstract" }, name, "entity"));
        self.assoc_src = Some(name.to_string());
        Ok(())
    }

    fn start_entity_identity(
        &mut self,
        name: &Identifier,
        target_type: &TypeReference,
        _: bool,
        _: Option<&Span>,
    ) -> Result<(), Error> {
        if let TypeReference::Type(target_type) = target_type {
            self.buffer.push_str(&format!("    +{name}: {target_type}\n"));
        } else {
            self.buffer.push_str("    +{role_name}: ?\n");
        }
        self.buffer.push_str("    --\n");
        Ok(())
    }

    fn start_entity_identity_role_ref(
        &mut self,
        role_name: &Identifier,
        in_property: &IdentifierReference,
        _: Option<&Span>,
    ) -> Result<(), Error> {
        self.buffer.push_str(&format!("    +{role_name} <<identity>> {{property = {in_property}}}\n"));
        self.buffer.push_str("    --\n");
        Ok(())
    }

    fn start_member(
        &mut self,
        name: &Identifier,
        _inverse_name: Option<&Identifier>,
        target_cardinality: &Cardinality,
        target_type: &TypeReference,
        _: bool,
        _: Option<&Span>,
    ) -> Result<(), Error> {
        match make_member(
            self.assoc_src.as_ref().unwrap(),
            name,
            target_type,
            target_cardinality,
            false,
        ) {
            (v, false) => {
                self.refs = Some(self.refs.clone().map(|r| format!("{r}{v}")).unwrap_or(v))
            }
            (v, true) => self.buffer.push_str(v.as_str()),
        }
        Ok(())
    }

    fn start_member_role_ref(
        &mut self,
        role_name: &Identifier,
        in_property: &IdentifierReference,
        _: Option<&Span>,
    ) -> Result<(), Error> {
        self.buffer
            .push_str(&format!("    +{role_name} {{property = {in_property}}}\n"));
        Ok(())
    }

    fn end_entity(&mut self, name: &Identifier, had_body: bool) -> Result<(), Error> {
        self.buffer.push_str(&end_type(name, had_body));
        self.assoc_src = None;
        Ok(())
    }

    fn start_enum(
        &mut self,
        name: &Identifier,
        _has_body: bool,
        _span: Option<&Span>,
    ) -> Result<(), Error> {
        self.buffer.push_str(&start_type_with_sterotype("class", name, "enum"));
        Ok(())
    }

    fn start_value_variant(
        &mut self,
        name: &Identifier,
        _has_body: bool,
        _span: Option<&Span>,
    ) -> Result<(), Error> {
        self.buffer.push_str(&format!("    +{name}\n"));
        Ok(())
    }

    fn end_enum(&mut self, name: &Identifier, had_body: bool) -> Result<(), Error> {
        self.buffer.push_str(&end_type(name, had_body));
        self.assoc_src = None;
        Ok(())
    }

    fn start_event(
        &mut self,
        name: &Identifier,
        source: &IdentifierReference,
        _has_body: bool,
        _span: Option<&Span>,
    ) -> Result<(), Error> {
        self.buffer
            .push_str(&start_type_with_sterotype("class", name, "event"));
        self.assoc_src = Some(name.to_string());
        let reference = format!("  {} ..> {}: <<source>>\n", make_id(name), make_id(source));
        self.refs = Some(
            self.refs
                .clone()
                .map(|r| format!("{r}{reference}"))
                .unwrap_or(reference),
        );
        Ok(())
    }

    fn start_group(&mut self, _span: Option<&Span>) -> Result<(), Error> {
        self.buffer.push_str("    --\n");
        Ok(())
    }

    fn end_event(&mut self, name: &Identifier, had_body: bool) -> Result<(), Error> {
        self.buffer.push_str(&end_type(name, had_body));
        self.assoc_src = None;
        Ok(())
    }

    fn start_structure(
        &mut self,
        name: &Identifier,
        _has_body: bool,
        _span: Option<&Span>,
    ) -> Result<(), Error> {
        self.buffer.push_str(&&start_type_with_sterotype("class", name, "structure"));
        self.assoc_src = Some(name.to_string());
        Ok(())
    }

    fn end_structure(&mut self, name: &Identifier, had_body: bool) -> Result<(), Error> {
        self.buffer.push_str(&end_type(name, had_body));
        self.assoc_src = None;
        Ok(())
    }

    fn start_union(
        &mut self,
        name: &Identifier,
        _has_body: bool,
        _span: Option<&Span>,
    ) -> Result<(), Error> {
        self.buffer
            .push_str(&start_type_with_sterotype("class", name, "union"));
        self.assoc_src = Some(name.to_string());
        Ok(())
    }

    fn start_type_variant(
        &mut self,
        name: &IdentifierReference,
        rename: Option<&Identifier>,
        _has_body: bool,
        _span: Option<&Span>,
    ) -> Result<(), Error> {
        let reference = if let Some(rename) = rename {
            format!(
                "  {} *--> \"{rename}\" {}\n",
                make_id(self.assoc_src.as_ref().unwrap()),
                make_id(name),
            )
        } else {
            format!(
                "  {} *--> {}\n",
                make_id(self.assoc_src.as_ref().unwrap()),
                make_id(name),
            )
        };
        self.refs = Some(
            self.refs
                .clone()
                .map(|r| format!("{r}{reference}"))
                .unwrap_or(reference),
        );

        Ok(())
    }

    fn end_union(&mut self, name: &Identifier, had_body: bool) -> Result<(), Error> {
        self.buffer.push_str(&end_type(name, had_body));
        self.assoc_src = None;
        Ok(())
    }

    fn start_property(
        &mut self,
        name: &Identifier,
        _has_body: bool,
        _span: Option<&Span>,
    ) -> Result<(), Error> {
        self.buffer
            .push_str(&start_type_with_sterotype("class", name, "property"));
        Ok(())
    }

    fn start_identity_role(
        &mut self,
        name: &Identifier,
        target_type: &TypeReference,
        _has_body: bool,
        _span: Option<&Span>,
    ) -> Result<(), Error> {
        self.buffer
            .push_str(&format!("    <<role, identity>> {name}"));
        if let TypeReference::Type(target_type) = target_type {
            self.buffer.push_str(&format!(": {target_type}\n"));
            // TODO: Cardinality
            // TODO: Mapping Types
        } else {
            self.buffer.push('\n');
        }
        Ok(())
    }

    fn start_member_role(
        &mut self,
        name: &Identifier,
        _inverse_name: Option<&Identifier>,
        _target_cardinality: &Cardinality,
        target_type: &TypeReference,
        _has_body: bool,
        _span: Option<&Span>,
     ) -> Result<(), Error> {
        self.buffer.push_str(&format!("    <<role, ref>> {name}"));
        if let TypeReference::Type(target_type) = target_type {
            self.buffer.push_str(&format!(": {target_type}\n"));
            // TODO: Cardinality
            // TODO: Mapping Types
        } else {
            self.buffer.push('\n');
        }
        Ok(())
    }

    fn end_property(&mut self, name: &Identifier, had_body: bool) -> Result<(), Error> {
        self.buffer.push_str(&end_type(name, had_body));
        Ok(())
    }

    fn end_module(&mut self, _name: &Identifier) -> Result<(), Error> {
        if let Some(refs) = &self.refs {
            self.buffer.push_str(refs);
        }
        self.buffer.push_str(&format!(
            r#"}}

{}

@enduml
"#,
            &self.imports.1
        ));

        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

#[inline(always)]
fn make_id<S>(id: S) -> String
where
    S: Into<String>,
{
    format!("s_{}", id.into().replace(':', "__"))
}

fn start_type_with_sterotype(
    type_class: &str,
    type_name: &Identifier,
    stereo_name: &str,
) -> String {
    format!(
        "  {} \"{}\" as {} << ({}, orchid) {} >> {{\n",
        type_class,
        type_name,
        make_id(type_name),
        stereo_name.chars().next().unwrap().to_uppercase(),
        stereo_name
    )
}

fn end_type(type_name: &Identifier, has_body: bool) -> String {
    if !has_body {
        format!("  }}\n  hide {} members\n\n", make_id(type_name))
    } else {
        "  }\n\n".to_string()
    }
}

fn make_member(
    source: &String,
    name: &Identifier,
    type_ref: &TypeReference,
    card: &Cardinality,
    by_ref: bool,
) -> (String, bool) {
    match type_ref {
        TypeReference::Type(target_type) => {
            if *card ==  DEFAULT_CARDINALITY {
                (format!("    +{name}: {}\n", make_type_reference(type_ref)), true)
            } else {
                (
                    format!(
                        "  s_{source} {}--> \"+{name}\\n{}\" {}\n",
                        if by_ref { "o" } else { "*" },
                        to_uml_string(card, true),
                        make_id(target_type),
                    ),
                    false,
                )
            }
        }
        TypeReference::FeatureSet(target_type) => {
            if *card ==  DEFAULT_CARDINALITY {
                (format!("    <<features>> +{name}: {}\n", make_type_reference(type_ref)), true)
            } else {
                (
                    format!(
                        "  s_{source} {}--> \"+{name}\\n{}\" {}: <<features>>\n",
                        if by_ref { "o" } else { "*" },
                        to_uml_string(card, true),
                        make_id(target_type),
                    ),
                    false,
                )
            }
        }
        TypeReference::MappingType(_) => (
            format!(
                "    +{name}: {} {}\n",
                to_uml_string(card, false),
                make_type_reference(type_ref)
            ),
            true,
        ),
        TypeReference::Unknown => (format!("    +{name}: unknown\n"), true),
    }
}

fn make_type_reference(type_ref: &TypeReference) -> String {
    match type_ref {
        TypeReference::Unknown => "unknown".to_string(),
        TypeReference::Type(v) => v.to_string(),
        TypeReference::FeatureSet(v) => v.to_string(),
        TypeReference::MappingType(v) => format!(
            "Mapping<{}, {}>",
            make_type_reference(v.domain()),
            make_type_reference(v.range()),
        ),
    }
}

fn make_imports(module: &Module) -> (String, String) {
    let mut imports_top = String::new();
    let mut imports_tail = String::new();
    for other in module.imported_modules() {
        imports_top.push_str(&format!(
            "package \"{}\" as {} <<module>> #white {{\n",
            other,
            make_id(other)
        ));
        for imported in module
            .imported_types()
            .iter()
            .filter(|qi| qi.module() == other)
        {
            imports_top.push_str(&format!(
                "  class \"{}\" as {}\n",
                imported.member(),
                make_id(*imported),
            ));
        }
        let mut names = HashSet::default();
        module.referenced_types(&mut names);
        for imported in names
            .iter()
            .filter_map(|rt| rt.as_qualified_identifier())
            .filter(|qi| qi.module() == other)
        {
            imports_top.push_str(&format!(
                "  class \"{}\" as {}\n",
                imported.member(),
                make_id(imported),
            ));
        }
        imports_top.push_str("}\n\n");

        imports_tail.push_str(&format!(
            "{} ..> {}: <<import>>\n",
            make_id(module.name()),
            make_id(other)
        ));
    }
    (imports_top, imports_tail)
}

#[inline(always)]
fn format_to_arg(value: OutputFormat) -> CommandArg {
    CommandArg::new(match value {
        OutputFormat::ImageJpeg => "-tjpg",
        OutputFormat::ImagePng => "-tpng",
        OutputFormat::ImageSvg => "-tsvg",
        _ => unreachable!(),
    })
}

#[inline(always)]
fn path_to_output<P>(path: P, module_name: &Identifier) -> Result<DiagramOutput, Error>
where
    P: AsRef<Path>,
{
    ::tracing::trace!("path_to_output {:?} {:?}", path.as_ref(), module_name);
    // Note:
    //  PlantUML does not take output file names, it derives the names from the input file names.
    //  However, it will take the path of the directory to put output files in, which needs to be
    //  specified else it is derived from the input path (a temp file name).
    let current_dir = std::env::current_dir()?;
    let output_dir = path
        .as_ref()
        .parent()
        .unwrap_or_else(|| &current_dir)
        .canonicalize()?;
    ::tracing::trace!("path_to_output output_dir = {:?}", output_dir);

    Ok(DiagramOutput {
        file_name: path
            .as_ref()
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| module_name.to_string()),
        output_dir: output_dir.to_string_lossy().to_string(),
    })
}

fn to_uml_string(card: &Cardinality, as_association: bool) -> String {
    let mut constraints: Vec<&str> = Vec::new();
    if let Some(ordering) = card.ordering() {
        if ordering == Ordering::Ordered {
            constraints.push(KW_ORDERING_ORDERED);
        }
    }
    if let Some(uniqueness) = card.uniqueness() {
        if uniqueness == Uniqueness::Unique {
            constraints.push(KW_UNIQUENESS_UNIQUE);
        }
    }
    let constraints = if !constraints.is_empty() {
        format!("{{{}}}", constraints.join(", "))
    } else {
        String::new()
    };

    let range_str = if card.range().is_range() {
        format!(
            "{}..{}",
            card.range().min_occurs(),
            card.range().max_occurs().map(|i| i.to_string()).unwrap_or_else(|| String::from("*"))
        )
    } else {
        card.range().min_occurs().to_string()
    };

    if constraints.is_empty() {
        range_str
    } else if as_association {
        format!("{constraints}\\n{range_str}")
    } else {
        format!("[{range_str}] {constraints}")
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
