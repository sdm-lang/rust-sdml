/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use sdml_core::generate::GenerateToWriter;
use crate::draw::OutputFormat;
use crate::exec::exec_with_temp_input;
use sdml_core::error::Error;
use sdml_core::model::walk::{walk_module, ModuleWalker};
use sdml_core::model::{
    ByReferenceMemberInner, Cardinality, Identifier, Module, Span, TypeReference,
};
use std::io::Write;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct ConceptDiagramGenerator {
    buffer: String,
    entity: Option<String>,
    has_unknown: bool,
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

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

pub const DOT_PROGRAM: &str = "dot";

impl GenerateToWriter<OutputFormat> for ConceptDiagramGenerator {
    fn write_in_format(&mut self, module: &Module, writer: &mut dyn Write, format: OutputFormat) -> Result<(), Error> {
        walk_module(module, self)?;

        if format == OutputFormat::Source {
            writer.write_all(self.buffer.as_bytes())?;
        } else {
            match exec_with_temp_input(DOT_PROGRAM, vec![format.into()], &self.buffer) {
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

impl ModuleWalker for ConceptDiagramGenerator {
    fn start_module(&mut self, _name: &Identifier, _: Option<&Span>) -> Result<(), Error> {
        self.buffer.push_str(r#"digraph G {
  bgcolor="transparent";
  rankdir="TB";
  fontname="Helvetica,Arial,sans-serif";
  node [fontname="Helvetica,Arial,sans-serif"; fontsize=10];
  edge [fontname="Helvetica,Arial,sans-serif"; fontsize=9; fontcolor="dimgrey";
        labelfontcolor="blue"; labeldistance=2.0];

"#);
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
