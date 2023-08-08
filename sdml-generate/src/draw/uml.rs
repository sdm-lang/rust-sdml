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
use sdml_core::model::ModelElement;
use sdml_core::model::{
    ByReferenceMemberInner, ByValueMemberInner, Cardinality, ControlledLanguageTag, Identifier,
    IdentifierReference, IdentityMemberInner, Module, Span, TypeReference, Value,
    DEFAULT_BY_REFERENCE_CARDINALITY, DEFAULT_BY_VALUE_CARDINALITY,
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
    imports: (String, String),
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
        _name: &IdentifierReference,
        _value: &Value,
        _span: Option<&Span>,
    ) -> Result<(), Error> {
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
        base_type: &IdentifierReference,
        _has_body: bool,
        _span: Option<&Span>,
    ) -> Result<(), Error> {
        self.buffer
            .push_str(&start_type_with_sterotype("class", name, "datatype"));
        self.buffer.push_str("  }\n");
        self.buffer
            .push_str(&format!("  hide {} members\n", make_id(name)));
        self.buffer
            .push_str(&format!("  {} ..|> s_{base_type}\n\n", make_id(name)));
        Ok(())
    }

    fn end_datatype(&mut self, _name: &Identifier, _had_body: bool) -> Result<(), Error> {
        self.assoc_src = None;
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
        match inner {
            ByValueMemberInner::PropertyRole(role) => {
                self.buffer
                    .push_str(&format!("    {name} {{role = {role}}}\n"));
            }
            ByValueMemberInner::Defined(def) => {
                match make_member(
                    self.assoc_src.as_ref().unwrap(),
                    name,
                    def.target_type(),
                    def.target_cardinality(),
                    false,
                ) {
                    (v, false) => {
                        self.refs = Some(self.refs.clone().map(|r| format!("{r}{v}")).unwrap_or(v))
                    }
                    (v, true) => self.buffer.push_str(v.as_str()),
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
                match make_member(
                    self.assoc_src.as_ref().unwrap(),
                    name,
                    def.target_type(),
                    def.target_cardinality(),
                    true,
                ) {
                    (v, false) => {
                        self.refs = Some(self.refs.clone().map(|r| format!("{r}{v}")).unwrap_or(v))
                    }
                    (v, true) => self.buffer.push_str(v.as_str()),
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
        let reference = format!("  {} o--> {}\n", make_id(name), make_id(source));
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
        self.buffer.push_str(&start_type("class", name));
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
            .push_str(&start_type_with_sterotype("enum", name, "union"));
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

    fn end_type_variant(
        &mut self,
        _name: &IdentifierReference,
        _had_body: bool,
    ) -> Result<(), Error> {
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

    fn start_property_role(
        &mut self,
        name: &Identifier,
        _inverse_name: Option<&Option<Identifier>>,
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
    format!("s_{}", id.into().replace(":", "__"))
}

#[inline(always)]
fn start_type(type_class: &str, type_name: &Identifier) -> String {
    format!(
        "  {} \"{}\" as {} {{\n",
        type_class,
        type_name,
        make_id(type_name)
    )
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
    if let TypeReference::Reference(target_type) = type_ref {
        if *card
            == if by_ref {
                DEFAULT_BY_REFERENCE_CARDINALITY
            } else {
                DEFAULT_BY_VALUE_CARDINALITY
            }
        {
            (format!("    {name}: {target_type}\n"), true)
        } else {
            (
                format!(
                    "  s_{source} {}--> \"{}\\n{name}\" {}\n",
                    if by_ref { "o" } else { "*" },
                    card.to_uml_string(),
                    make_id(target_type),
                ),
                false,
            )
        }
    } else {
        (format!("    {name}: ?\n"), true)
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
        for imported in module
            .referenced_types()
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
fn path_to_output<P>(path: P, module_name: &Identifier) -> DiagramOutput
where
    P: AsRef<Path>,
{
    // Note:
    //  PlantUML does not take output file names, it derives the names from the input file names.
    //  However, it will take the path of the directory to put output files in, which needs to be
    //  specified else it is derived from the input path (a temp file name).
    let output_dir = path
        .as_ref()
        .parent()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default();
    DiagramOutput {
        file_name: path
            .as_ref()
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| module_name.to_string()),
        output_dir: if output_dir.is_empty() {
            let default_output = std::env::current_dir()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_default();
            if default_output.is_empty() {
                String::from(".")
            } else {
                default_output
            }
        } else {
            output_dir
        },
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
