/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::draw::OutputFormat;
use crate::exec::{exec_with_input, CommandArg};
use sdml_core::error::Error;
use sdml_core::model::walk::{walk_module, ModuleWalker};
use sdml_core::model::{
    ByReferenceMemberInner, ByValueMemberInner, Cardinality, Identifier, IdentifierReference,
    IdentityMemberInner, Module, Span, TypeReference, Value,
};
use std::io::Write;
use std::path::Path;
use tracing::debug;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub const UML_PROGRAM: &str = "plantuml";

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn write_uml_diagram<W: Write>(
    module: &Module,
    w: &mut W,
    format: OutputFormat,
) -> Result<(), Error> {
    let mut state = DiagramState::new_with_imports(make_imports(module));
    walk_module(module, &mut state)?;

    if format == OutputFormat::Source {
        w.write_all(state.buffer.as_bytes())?;
    } else {
        match exec_with_input(
            UML_PROGRAM,
            vec![
                CommandArg::new(format!("-o{}", default_output_path())),
                format_to_arg(format),
            ],
            state.buffer,
        ) {
            Ok(result) => {
                w.write_all(result.as_bytes())?;
            }
            Err(e) => {
                panic!("exec_with_input failed: {:?}", e);
            }
        }
    }

    Ok(())
}

pub fn uml_diagram_to_file<P>(module: &Module, path: P, format: OutputFormat) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    let mut state = DiagramState {
        output: Some(path_to_output(&path, module.name())),
        ..Default::default()
    };

    walk_module(module, &mut state)?;

    if format == OutputFormat::Source {
        std::fs::write(path.as_ref(), state.buffer)?;
    } else {
        match exec_with_input(
            UML_PROGRAM,
            vec![
                CommandArg::new(format!("-o{}", state.output.unwrap().output_dir)),
                format.into(),
            ],
            state.buffer,
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

write_to_string!(uml_diagram_to_string, write_uml_diagram, OutputFormat);

print_to_stdout!(print_uml_diagram, write_uml_diagram, OutputFormat);

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Default)]
struct DiagramState {
    buffer: String,
    imports: String,
    output: Option<DiagramOutput>,
    //entity: Option<String>,
    //has_unknown: bool,
}

#[derive(Debug, Default)]
struct DiagramOutput {
    file_name: String,
    output_dir: String,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl DiagramState {
    fn new_with_imports(imports: String) -> Self {
        Self {
            buffer: Default::default(),
            imports,
            output: None,
        }
    }
}

impl ModuleWalker for DiagramState {
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
show << event >> circle
show << union >> circle

title "Module {name}"

{}
package "{name}" as s_{name} {{
"#,
            self.output
                .as_ref()
                .map(|o| o.file_name.to_string())
                .unwrap_or_else(|| name.to_string()),
            self.imports
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
        self.buffer.push_str(&start_type("entity", name));
        Ok(())
    }

    fn start_identity_member(
        &mut self,
        name: &Identifier,
        inner: &IdentityMemberInner,
        _span: Option<&Span>,
    ) -> Result<(), Error> {
        self.buffer.push_str("    __identity__\n");

        self.buffer.push_str(&format!("    {name}"));
        match inner {
            IdentityMemberInner::PropertyRole(role) => {
                self.buffer.push_str(&format!("{{role = {role}}}"));
            }
            IdentityMemberInner::Defined(def) => {
                if let TypeReference::Reference(target_type) = def.target_type() {
                    self.buffer.push_str(&format!(": {target_type}\n"));
                } else {
                    self.buffer.push('\n');
                }
            }
        }

        self.buffer.push_str("    ..\n");

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
                self.buffer.push_str(&format!("{{role = {role}}}"));
            }
            ByValueMemberInner::Defined(def) => {
                if let TypeReference::Reference(target_type) = def.target_type() {
                    self.buffer.push_str(&format!(": {target_type}\n"));
                } else {
                    self.buffer.push('\n');
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
        self.buffer.push_str(&format!("    {name}"));

        match inner {
            ByReferenceMemberInner::PropertyRole(role) => {
                self.buffer.push_str(&format!("{{role = {role}}}"));
            }
            ByReferenceMemberInner::Defined(def) => {
                if let TypeReference::Reference(target_type) = def.target_type() {
                    self.buffer.push_str(&format!(": {target_type}\n"));
                } else {
                    self.buffer.push('\n');
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

    fn start_enum_variant(
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
        self.buffer
            .push_str(&format!("  s_{name} o--> s_{source}\n"));
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
        self.buffer.push_str("}\n");
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
        "  {} \"{}\" as s_{} << (D, orchid) {} >> {{\n",
        type_class, type_name, type_name, stereo_name
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
        imports.push_str(&format!("package \"{}\" as s_{} {{\n", other, other));
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
        imports.push_str(&format!("s_{} ..> s_{}\n\n", module.name(), other));
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
