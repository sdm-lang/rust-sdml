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
use sdml_core::model::walk::{walk_module, ModuleWalker};
use sdml_core::model::{
    ByReferenceMemberInner, ByValueMemberInner, Cardinality, Identifier, IdentifierReference,
    IdentityMemberInner, Module, Span, TypeReference, Value,
};
use std::path::Path;
use tracing::debug;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct UmlDiagramGenerator {
    buffer: String,
    imports: String,
    output: Option<DiagramOutput>,
    assoc_src: Option<String>,
    refs: Option<String>,
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
        path: &Path,
        format: OutputFormat,
    ) -> Result<(), Error> {
        self.imports = make_imports(module);
        self.output = Some(path_to_output(path, module.name()));

        walk_module(module, self)?;

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

impl ModuleWalker for UmlDiagramGenerator {
    fn start_module(
        &mut self,
        name: &Identifier,
        _span: Option<&sdml_core::model::Span>,
    ) -> Result<(), Error> {
        self.buffer.push_str(&format!(
            r#"@startuml {}
skin rose
hide methods
hide circle

show << datatype >> circle
show << entity >> circle
show enum circle
show << event >> circle
show << union >> circle

package "{name}" as s_{name} <<module>> {{
"#,
            self.output
                .as_ref()
                .map(|o| o.file_name.to_string())
                .unwrap_or_else(|| name.to_string()),
        ));

        Ok(())
    }

    fn annotation_property(
        &mut self,
        _name: &IdentifierReference,
        _value: &Value,
        _span: Option<&Span>,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn informal_constraint(
        &mut self,
        _name: Option<&Identifier>,
        _value: &str,
        _span: Option<&Span>,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn start_datatype(
        &mut self,
        name: &Identifier,
        base_type: &IdentifierReference,
        has_body: bool,
        _span: Option<&Span>,
    ) -> Result<(), Error> {
        self.buffer
            .push_str(&start_type_with_sterotype("class", name, "datatype"));
        self.buffer.push_str(&end_type(name, has_body));
        self.buffer
            .push_str(&format!("  s_{} ..|> s_{}\n", name, base_type));
        Ok(())
    }

    fn end_datatype(&mut self, name: &Identifier, had_body: bool) -> Result<(), Error> {
        self.buffer.push_str(&end_type(name, had_body));
        Ok(())
    }

    fn start_entity(
        &mut self,
        name: &Identifier,
        _has_body: bool,
        _span: Option<&Span>,
    ) -> Result<(), Error> {
        self.buffer
            .push_str(&start_type_with_sterotype("class", name, "entity"));
        self.assoc_src = Some(name.to_string());
        Ok(())
    }

    fn start_identity_member(
        &mut self,
        name: &Identifier,
        inner: &IdentityMemberInner,
        _span: Option<&Span>,
    ) -> Result<(), Error> {
        self.buffer.push_str(&format!("    <<identity>> {name}"));
        match inner {
            IdentityMemberInner::PropertyRole(role) => {
                self.buffer.push_str(&format!(" {{role = {role}}}\n"));
            }
            IdentityMemberInner::Defined(def) => {
                if let TypeReference::Reference(target_type) = def.target_type() {
                    self.buffer.push_str(&format!(": {target_type}\n"));
                } else {
                    self.buffer.push_str(": ?\n");
                }
            }
        }
        self.buffer.push_str("    --\n");

        Ok(())
    }

    fn start_by_value_member(
        &mut self,
        name: &Identifier,
        inner: &ByValueMemberInner,
        _span: Option<&Span>,
    ) -> Result<(), Error> {
        self.buffer.push_str(&format!("    {name}"));
        match inner {
            ByValueMemberInner::PropertyRole(role) => {
                self.buffer.push_str(&format!(" {{role = {role}}}\n"));
            }
            ByValueMemberInner::Defined(def) => {
                let card_string = def
                    .target_cardinality()
                    .map(|c| format!("{{{}}} ", c.to_uml_string()))
                    .unwrap_or_default();
                if let TypeReference::Reference(target_type) = def.target_type() {
                    self.buffer
                        .push_str(&format!(": {card_string}{target_type}\n"));
                } else {
                    self.buffer.push_str(": ?\n");
                }
            }
        }

        Ok(())
    }

    fn start_by_reference_member(
        &mut self,
        name: &Identifier,
        inner: &ByReferenceMemberInner,
        _span: Option<&Span>,
    ) -> Result<(), Error> {
        match inner {
            ByReferenceMemberInner::PropertyRole(role) => {
                self.buffer
                    .push_str(&format!("    {name} {{role = {role}}}\n"));
            }
            ByReferenceMemberInner::Defined(def) => {
                if let TypeReference::Reference(target_type) = def.target_type() {
                    let from_card = def
                        .source_cardinality()
                        .map(|c| format!("\"{{{}}}\" ", c.to_uml_string()))
                        .unwrap_or_default();
                    let to_card = def
                        .target_cardinality()
                        .map(|c| format!("{{{}}}\\n", c.to_uml_string()))
                        .unwrap_or_default();
                    let reference = format!(
                        "  s_{} {from_card}o--> \"{to_card}{name}\" s_{target_type}\n",
                        self.assoc_src.as_ref().unwrap(),
                    );
                    self.refs = Some(
                        self.refs
                            .clone()
                            .map(|r| format!("{r}{reference}"))
                            .unwrap_or(reference),
                    );
                } else {
                    self.buffer.push_str(&format!("    {name}: ?\n"));
                }
            }
        }

        Ok(())
    }

    fn end_member(&mut self, _name: &Identifier) -> Result<(), Error> {
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
        self.buffer.push_str(&start_type("enum", name));
        Ok(())
    }

    fn start_value_variant(
        &mut self,
        name: &Identifier,
        value: u32,
        _has_body: bool,
        _span: Option<&Span>,
    ) -> Result<(), Error> {
        self.buffer.push_str(&format!("    {name} = {value}\n"));
        Ok(())
    }

    fn end_enum(&mut self, name: &Identifier, had_body: bool) -> Result<(), Error> {
        self.buffer.push_str(&end_type(name, had_body));
        Ok(())
    }

    fn start_event(
        &mut self,
        name: &Identifier,
        source: &IdentifierReference,
        _has_body: bool,
        _span: Option<&Span>,
    ) -> Result<(), Error> {
        let reference = format!("  s_{name} o--> s_{source}\n");
        self.refs = Some(
            self.refs
                .clone()
                .map(|r| format!("{r}{reference}"))
                .unwrap_or(reference),
        );
        self.buffer
            .push_str(&start_type_with_sterotype("class", name, "event"));
        Ok(())
    }

    fn start_group(&mut self, _span: Option<&Span>) -> Result<(), Error> {
        self.buffer.push_str("    --\n");
        Ok(())
    }

    fn end_event(&mut self, name: &Identifier, had_body: bool) -> Result<(), Error> {
        self.buffer.push_str(&end_type(name, had_body));
        Ok(())
    }

    fn start_structure(
        &mut self,
        name: &Identifier,
        _has_body: bool,
        _span: Option<&Span>,
    ) -> Result<(), Error> {
        self.buffer.push_str(&start_type("class", name));
        Ok(())
    }

    fn end_structure(&mut self, name: &Identifier, had_body: bool) -> Result<(), Error> {
        self.buffer.push_str(&end_type(name, had_body));
        Ok(())
    }

    fn start_union(
        &mut self,
        name: &Identifier,
        _has_body: bool,
        _span: Option<&Span>,
    ) -> Result<(), Error> {
        self.buffer
            .push_str(&start_type_with_sterotype("enum", name, "union"));
        Ok(())
    }

    fn start_type_variant(
        &mut self,
        name: &IdentifierReference,
        rename: Option<&Identifier>,
        _has_body: bool,
        _span: Option<&Span>,
    ) -> Result<(), Error> {
        if let Some(rename) = rename {
            self.buffer.push_str(&format!("    {rename} ({name})\n"));
        } else {
            self.buffer.push_str(&format!("    {name}\n"));
        }

        Ok(())
    }

    fn end_type_variant(
        &mut self,
        _name: &IdentifierReference,
        _had_body: bool,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn end_union(&mut self, name: &Identifier, had_body: bool) -> Result<(), Error> {
        self.buffer.push_str(&end_type(name, had_body));
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

    fn start_property_role(
        &mut self,
        name: &Identifier,
        _source_cardinality: Option<&Option<Cardinality>>,
        _target_cardinality: Option<&Cardinality>,
        target_type: &TypeReference,
        _has_body: bool,
        _span: Option<&Span>,
    ) -> Result<(), Error> {
        self.buffer.push_str(&format!("    <<role>> {name}"));
        if let TypeReference::Reference(target_type) = target_type {
            self.buffer.push_str(&format!(": {target_type}\n"));
        } else {
            self.buffer.push('\n');
        }
        Ok(())
    }

    fn end_property_role(&mut self, _name: &Identifier, _had_body: bool) -> Result<(), Error> {
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
        self.buffer.push_str("}\n");
        self.buffer.push_str(&self.imports.to_string());
        self.buffer.push_str("@enduml\n");
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn start_type(type_class: &str, type_name: &Identifier) -> String {
    format!("  {} \"{}\" as s_{} {{\n", type_class, type_name, type_name)
}

fn start_type_with_sterotype(
    type_class: &str,
    type_name: &Identifier,
    stereo_name: &str,
) -> String {
    format!(
        "  {} \"{}\" as s_{} << ({}, orchid) {} >> {{\n",
        type_class,
        type_name,
        type_name,
        stereo_name.chars().next().unwrap().to_uppercase(),
        stereo_name
    )
}

fn end_type(type_name: &Identifier, has_body: bool) -> String {
    if !has_body {
        format!("  }}\n  hide s_{} members\n\n", type_name)
    } else {
        "  }\n\n".to_string()
    }
}

fn make_imports(module: &Module) -> String {
    let mut imports = String::new();
    for other in module.imported_modules() {
        imports.push_str(&format!(
            "package \"{}\" as s_{} <<module>> #white {{\n",
            other, other
        ));
        for imported in module
            .imported_types()
            .iter()
            .filter(|qi| qi.module() == other)
        {
            imports.push_str(&format!(
                "  class \"{}\" as s_{}\n",
                imported.member(),
                imported
            ));
        }
        imports.push_str("}\n");
        imports.push_str(&format!(
            "s_{} ..> s_{}: <<import>>\n\n",
            module.name(),
            other
        ));
    }
    imports
}

fn format_to_arg(value: OutputFormat) -> CommandArg {
    CommandArg::new(match value {
        OutputFormat::ImageJpeg => "-tjpg",
        OutputFormat::ImagePng => "-tpng",
        OutputFormat::ImageSvg => "-tsvg",
        _ => unreachable!(),
    })
}

fn path_to_output<P>(path: P, module_name: &Identifier) -> DiagramOutput
where
    P: AsRef<Path>,
{
    DiagramOutput {
        file_name: path
            .as_ref()
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| module_name.to_string()),
        output_dir: path
            .as_ref()
            .parent()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(default_output_path),
    }
}

fn default_output_path() -> String {
    std::env::current_dir()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|_| ".".to_string())
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
