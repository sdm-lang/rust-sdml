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
    ByReferenceMemberInner, Cardinality, Identifier, Module, Span, TypeReference,
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

pub const DOT_PROGRAM: &str = "dot";

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn write_concept_diagram<W: Write>(
    module: &Module,
    w: &mut W,
    format: OutputFormat,
) -> Result<(), Error> {
    let mut state = DiagramState::default();
    walk_module(module, &mut state)?;

    if format == OutputFormat::Source {
        w.write_all(state.buffer.as_bytes())?;
    } else {
        match exec_with_input(DOT_PROGRAM, vec![format.into()], state.buffer) {
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

pub fn concept_diagram_to_file<P>(
    module: &Module,
    path: P,
    format: OutputFormat,
) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    let mut state = DiagramState::default();
    walk_module(module, &mut state)?;

    if format == OutputFormat::Source {
        std::fs::write(path.as_ref(), state.buffer)?;
    } else {
        match exec_with_input(
            DOT_PROGRAM,
            vec![CommandArg::from_path_option("-o", path), format.into()],
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

write_to_string!(
    concept_diagram_to_string,
    write_concept_diagram,
    OutputFormat
);

print_to_stdout!(print_concept_diagram, write_concept_diagram, OutputFormat);

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Default)]
struct DiagramState {
    buffer: String,
    entity: Option<String>,
    has_unknown: bool,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl ModuleWalker for DiagramState {
    fn start_module(&mut self, name: &Identifier, _: Option<&Span>) -> Result<(), Error> {
        self.buffer.push_str(&format!(
            r#"digraph G {{
  bgcolor="transparent";
  rankdir="TB";
  fontname="Helvetica,Arial,sans-serif";
  node [fontname="Helvetica,Arial,sans-serif"; fontsize=10];
  edge [fontname="Helvetica,Arial,sans-serif"; fontsize=9; fontcolor="dimgrey";
        labelfontcolor="blue"; labeldistance=2.0];
  label="module {}";

"#,
            name
        ));
        Ok(())
    }

    fn start_entity(&mut self, name: &Identifier, _: bool, _: Option<&Span>) -> Result<(), Error> {
        let name = name.as_ref();
        self.buffer.push_str(&format!(
            "  {} [label=\"{}\"];\n",
            name.to_lowercase(),
            name
        ));
        self.entity = Some(name.to_string());
        Ok(())
    }

    fn start_by_reference_member(
        &mut self,
        name: &Identifier,
        inner: &ByReferenceMemberInner,
        _: Option<&Span>,
    ) -> Result<(), Error> {
        match inner {
            ByReferenceMemberInner::PropertyRole(role) => {
                self.buffer.push_str(&format!(
                    "  {} -> {} [label=\"{}\";dir=\"both\";arrowtail=\"teetee\";arrowhead=\"teetee\"];\n",
                    self.entity
                        .as_deref()
                        .unwrap_or_default()
                        .to_lowercase(),
                    name,
                    role
                ));
            }
            ByReferenceMemberInner::Defined(def) => {
                if matches!(def.target_type(), TypeReference::Unknown) && !self.has_unknown {
                    self.buffer.push_str(
                        "  unknown [shape=rect; label=\"Unknown\"; color=\"grey\"; fontcolor=\"grey\"];\n",
                    );
                    self.has_unknown = true;
                }
                let target_type = if let TypeReference::Reference(target_type) = def.target_type() {
                    target_type.to_string().to_lowercase()
                } else {
                    "unknown".to_string()
                };
                let from_str = if let Some(source_cardinality) = def.source_cardinality() {
                    if source_cardinality != &Cardinality::ref_source_default() {
                        source_cardinality.to_uml_string()
                    } else {
                        String::new()
                    }
                } else {
                    String::new()
                };
                let to_str = if let Some(target_cardinality) = def.target_cardinality() {
                    if target_cardinality != &Cardinality::ref_target_default() {
                        target_cardinality.to_uml_string()
                    } else {
                        String::new()
                    }
                } else {
                    String::new()
                };
                self.buffer.push_str(&format!(
                    "  {} -> {} [label=\"{}\"; taillabel=\"{}\"; headlabel=\"{}\"];\n",
                    self.entity.as_deref().unwrap_or_default().to_lowercase(),
                    target_type,
                    name,
                    from_str,
                    to_str
                ));
            }
        }

        Ok(())
    }

    fn end_module(&mut self, _: &Identifier) -> Result<(), Error> {
        self.buffer.push_str("}\n");
        self.entity = None;
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
