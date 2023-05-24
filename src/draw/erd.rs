/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::draw::OutputFormat;
use crate::api::ParseTree;
use crate::error::Error;
use crate::walk::{walk_tree, Cardinality, TreeWalker};
use std::io::Write;
use std::cell::RefCell;
use graphviz_rust::{cmd::CommandArg, cmd::Format, exec_dot};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn write_erd_diagram<W: Write>(tree: &ParseTree<'_>, w: &mut W, format: OutputFormat) -> Result<(), Error> {
    let state = DiagramState::default();
    walk_tree(tree, &state)?;

    let source = state.buffer.into_inner();

    if format == OutputFormat::Source {
        w.write(source.as_bytes())?;
    } else {
        match exec_dot(source, vec![CommandArg::Format(mkformat(format))]) {
            Ok(result) => {
                w.write(result.as_bytes())?;
            }
            Err(e) => {
                panic!("exec_dot failed: {:?}", e);
            }
        }
    }

    Ok(())
}

write_to_string!(erd_diagram_to_string, write_erd_diagram, OutputFormat);

write_to_file!(erd_diagram_to_file, write_erd_diagram, OutputFormat);

print_to_stdout!(print_erd_diagram, write_erd_diagram, OutputFormat);

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Default)]
struct DiagramState{
    buffer: RefCell<String>,
    entity: RefCell<Option<String>>,
    seen: RefCell<Vec<String>>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl TreeWalker for DiagramState {
    fn start_module(&self, name: &str) -> Result<(), Error> {
        let mut buffer = self.buffer.borrow_mut();
        buffer.push_str(&format!(r#"digraph G {{
  bgcolor="transparent";
  rankdir="TB";
  fontname="Helvetica,Arial,sans-serif";
  node [fontname="Helvetica,Arial,sans-serif"; fontsize=10];
  edge [fontname="Helvetica,Arial,sans-serif"; fontsize=9; fontcolor="dimgrey";
        labelfontcolor="blue"; labeldistance=2.0];
  label="module {}";

"#, name));
        Ok(())
    }

    fn start_entity(&self, name: &str) -> Result<(), Error> {
        let mut buffer = self.buffer.borrow_mut();
        buffer.push_str(&format!("  {} [label=\"{}\"];\n", name.to_lowercase(), name));
        *self.entity.borrow_mut() = Some(name.to_string());
        Ok(())
    }

    fn start_identity_member(
        &self,
        name: &str,
        target_type: Option<&str>,
    ) -> Result<(), Error> {
        let mut buffer = self.buffer.borrow_mut();
        if target_type.is_none() && !self.seen.borrow().contains(&"unknown".to_string()) {
            buffer.push_str("  unknown [shape=rect; label=\"Unknown\"; color=\"grey\"; fontcolor=\"grey\"];\n");
            self.seen.borrow_mut().push("unknown".to_string());
        }
        let target_type = if let Some(target_type) = target_type {
            buffer.push_str(&format!("  {} [shape=rect; label=\"{}\"; style=\"filled\"; fillcolor=\"azure\"];\n",
                                     target_type.to_lowercase(),
                                     target_type));
            target_type
        } else {
            "unknown"
        };
        buffer.push_str(&format!("  {} -> {} [label=\"{}\"];\n",
                                 self.entity.borrow().as_ref().map(|s|s.as_str()).unwrap_or("").to_lowercase(),
                                 target_type.to_lowercase(),
                                 name));
        Ok(())
    }

    fn start_by_value_member(
        &self,
        name: &str,
        to: Cardinality,
        target_type: Option<&str>,
    ) -> Result<(), Error> {
        let mut buffer = self.buffer.borrow_mut();
        let target_type = target_type.map(|s|s.to_lowercase()).unwrap_or("?".to_string());
        let to_str = if to == Cardinality::ref_target_default() {
            String::new()
        }else {
            to.to_uml_string()
        };
        buffer.push_str(&format!("  {} -> {} [label=\"{}\"; taillabel=\"{}\"; headlabel=\"{}\"];\n",
                                 self.entity.borrow().as_ref().map(|s|s.as_str()).unwrap_or("").to_lowercase(),
                                 target_type,
                                 name,
                                 from_str,
                                 to_str));
        Ok(())
    }

    fn start_by_reference_member(
        &self,
        name: &str,
        from: Cardinality,
        to: Cardinality,
        target_type: Option<&str>,
    ) -> Result<(), Error> {
        let mut buffer = self.buffer.borrow_mut();
        let target_type = target_type.map(|s|s.to_lowercase()).unwrap_or("?".to_string());
        let from_str = if from == Cardinality::ref_source_default() {
            String::new()
        }else {
            from.to_uml_string()
        };
        let to_str = if to == Cardinality::ref_target_default() {
            String::new()
        }else {
            to.to_uml_string()
        };
        buffer.push_str(&format!("  {} -> {} [label=\"{}\"; taillabel=\"{}\"; headlabel=\"{}\"];\n",
                                 self.entity.borrow().as_ref().map(|s|s.as_str()).unwrap_or("").to_lowercase(),
                                 target_type,
                                 name,
                                 from_str,
                                 to_str));
        Ok(())
    }

    fn end_module(&self, _name: &str) -> Result<(), Error> {
        self.buffer.borrow_mut().push_str("}\n");
        *self.entity.borrow_mut() = None;
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn mkformat(format: OutputFormat) -> Format {
    match format {
        OutputFormat::ImageJpeg => Format::Jpg,
        OutputFormat::ImagePng => Format::Png,
        OutputFormat::ImageSvg => Format::Svg,
        _ => unreachable!(),
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
