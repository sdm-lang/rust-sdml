/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::draw::OutputFormat;
use crate::error::Error;
use crate::model::walk::{walk_module, ModuleWalker};
use crate::model::{
    Cardinality, Identifier, IdentifierReference, Import, Module, Span, TypeReference,
};
use graphviz_rust::{cmd::CommandArg, cmd::Format, exec_dot};
use std::cell::RefCell;
use std::io::Write;
use std::path::Path;
use tracing::{debug, trace};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn write_erd_diagram<W: Write>(
    module: &Module,
    w: &mut W,
    format: OutputFormat,
) -> Result<(), Error> {
    let state = DiagramState::default();
    walk_module(module, &state)?;

    let source = state.buffer.into_inner();

    if format == OutputFormat::Source {
        w.write_all(source.as_bytes())?;
    } else {
        match exec_dot(source, vec![CommandArg::Format(mkformat(format))]) {
            Ok(result) => {
                w.write_all(result.as_bytes())?;
            }
            Err(e) => {
                panic!("exec_dot failed: {:?}", e);
            }
        }
    }

    Ok(())
}

pub fn erd_diagram_to_file<P>(module: &Module, path: P, format: OutputFormat) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    let state = DiagramState::default();
    walk_module(module, &state)?;

    let source = state.buffer.into_inner();

    if format == OutputFormat::Source {
        std::fs::write(path.as_ref(), source)?;
    } else {
        match exec_dot(
            source,
            vec![
                CommandArg::Output(path.as_ref().to_str().unwrap().to_string()),
                CommandArg::Format(mkformat(format)),
            ],
        ) {
            Ok(result) => {
                debug!("Response from dot: {:?}", result);
            }
            Err(e) => {
                panic!("exec_dot failed: {:?}", e);
            }
        }
    }

    Ok(())
}

write_to_string!(erd_diagram_to_string, write_erd_diagram, OutputFormat);

print_to_stdout!(print_erd_diagram, write_erd_diagram, OutputFormat);

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Default)]
struct DiagramState {
    buffer: RefCell<String>,
    entity: RefCell<Option<String>>,
    seen: RefCell<Vec<String>>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl ModuleWalker for DiagramState {
    fn start_module(&self, name: &Identifier, _: Option<&Span>) -> Result<(), Error> {
        let mut buffer = self.buffer.borrow_mut();
        buffer.push_str(&format!(
            r#"digraph G {{
  bgcolor="transparent";
  rankdir="TB";
  fontname="Helvetica,Arial,sans-serif";
  node [fontname="Helvetica,Arial,sans-serif"; fontsize=10];
  edge [fontname="Helvetica,Arial,sans-serif"; fontsize=9; fontcolor="dimgrey";
        labelfontcolor="blue"; labeldistance=2.0];
  graph [pad="0.5", nodesep="1", ranksep="1"];
  splines="ortho";
  label="module {}";

"#,
            name
        ));
        Ok(())
    }

    fn import(&self, imported: &[Import], _: Option<&Span>) -> Result<(), Error> {
        trace!("import: {:?}", imported);
        let mut buffer = self.buffer.borrow_mut();
        for name in imported {
            buffer.push_str(&format!(
                "  {} [label=\"{}\"; style=\"dashed\"; color=\"dimgrey\"; fontcolor=\"dimgrey\"];\n",
                name_to_ref(&name.to_string()),
                name
            ));
        }
        // ?        *self.entity.borrow_mut() = Some(name.to_string());
        Ok(())
    }

    fn start_entity(&self, name: &Identifier, _: Option<&Span>) -> Result<(), Error> {
        trace!("entity: {}", name);
        let mut buffer = self.buffer.borrow_mut();
        buffer.push_str(&format!(
            "  {} [label=\"{}\"; penwidth=1.5];\n",
            name_to_ref(name.as_ref()),
            name
        ));
        *self.entity.borrow_mut() = Some(name.to_string());
        Ok(())
    }

    fn start_datatype(
        &self,
        name: &Identifier,
        _base_type: &IdentifierReference,
        _: Option<&Span>,
    ) -> Result<(), Error> {
        trace!("datatype: {}", name);
        let mut buffer = self.buffer.borrow_mut();
        buffer.push_str(&format!(
            "  {} [label=\"■ {}\"; style=\"dashed\"; color=\"dimgrey\"; fontcolor=\"dimgrey\"];\n",
            name_to_ref(name.as_ref()),
            name
        ));
        *self.entity.borrow_mut() = Some(name.to_string());
        Ok(())
    }

    fn start_enum(&self, name: &Identifier, _: Option<&Span>) -> Result<(), Error> {
        trace!("enum: {}", name);
        let mut buffer = self.buffer.borrow_mut();
        buffer.push_str(&format!(
            "  {} [label=\"≣ {}\"; style=\"dashed\"; color=\"dimgrey\"; fontcolor=\"dimgrey\"];\n",
            name_to_ref(name.as_ref()),
            name
        ));
        *self.entity.borrow_mut() = Some(name.to_string());
        Ok(())
    }

    fn start_event(
        &self,
        name: &Identifier,
        _source: &IdentifierReference,
        _: Option<&Span>,
    ) -> Result<(), Error> {
        trace!("event: {}", name);
        let mut buffer = self.buffer.borrow_mut();
        buffer.push_str(&format!(
            "  {} [label=\"☇ {}\"; style=\"dashed\"; color=\"dimgrey\"; fontcolor=\"dimgrey\"];\n",
            name_to_ref(name.as_ref()),
            name
        ));
        *self.entity.borrow_mut() = Some(name.to_string());
        Ok(())
    }

    fn start_structure(&self, name: &Identifier, _: Option<&Span>) -> Result<(), Error> {
        trace!("structure: {}", name);
        let mut buffer = self.buffer.borrow_mut();
        buffer.push_str(&format!(
            "  {} [label=\"{}\"; style=\"dashed\"; color=\"dimgrey\"; fontcolor=\"dimgrey\"];\n",
            name_to_ref(name.as_ref()),
            name
        ));
        *self.entity.borrow_mut() = Some(name.to_string());
        Ok(())
    }

    fn start_identity_member(
        &self,
        name: &Identifier,
        target_type: &TypeReference,
        _: Option<&Span>,
    ) -> Result<(), Error> {
        let mut buffer = self.buffer.borrow_mut();
        if matches!(target_type, TypeReference::Unknown)
            && !self.seen.borrow().contains(&"unknown".to_string())
        {
            buffer.push_str(
                "  unknown [shape=rect; label=\"Unknown\"; color=\"grey\"; fontcolor=\"grey\"];\n",
            );
            self.seen.borrow_mut().push("unknown".to_string());
        }
        let target_type = if let TypeReference::Reference(target_type) = target_type {
            name_to_ref(&target_type.to_string())
        } else {
            "unknown".to_string()
        };
        buffer.push_str(&format!(
            "  {} -> {} [tooltip=\"{}\";dir=\"both\";arrowtail=\"teetee\";arrowhead=\"teetee\"];\n",
            self.entity
                .borrow()
                .as_ref()
                .map(|s| s.as_str())
                .unwrap_or("")
                .to_lowercase(),
            target_type,
            name
        ));
        Ok(())
    }

    fn start_by_value_member(
        &self,
        name: &Identifier,
        target_cardinality: Option<&Cardinality>,
        target_type: &TypeReference,
        _: Option<&Span>,
    ) -> Result<(), Error> {
        let mut buffer = self.buffer.borrow_mut();
        let target_type = if let TypeReference::Reference(target_type) = target_type {
            name_to_ref(&target_type.to_string())
        } else {
            "unknown".to_string()
        };
        let target_cardinality = if let Some(target_cardinality) = target_cardinality {
            arrow_end("head", target_cardinality)
        } else {
            String::new()
        };
        buffer.push_str(&format!(
            "  {} -> {} [tooltip=\"{}\";dir=\"both\";arrowtail=\"teetee\"{}];\n",
            self.entity
                .borrow()
                .as_ref()
                .map(|s| s.as_str())
                .unwrap_or("")
                .to_lowercase(),
            target_type,
            name,
            target_cardinality
        ));
        Ok(())
    }

    fn start_by_reference_member(
        &self,
        name: &Identifier,
        source_cardinality: Option<&Cardinality>,
        target_cardinality: Option<&Cardinality>,
        target_type: &TypeReference,
        _: Option<&Span>,
    ) -> Result<(), Error> {
        let mut buffer = self.buffer.borrow_mut();
        let target_type = if let TypeReference::Reference(target_type) = target_type {
            name_to_ref(&target_type.to_string())
        } else {
            "unknown".to_string()
        };
        let source_cardinality = if let Some(source_cardinality) = source_cardinality {
            arrow_end("tail", source_cardinality)
        } else {
            String::new()
        };
        let target_cardinality = if let Some(target_cardinality) = target_cardinality {
            arrow_end("head", target_cardinality)
        } else {
            String::new()
        };
        buffer.push_str(&format!(
            "  {} -> {} [tooltip=\"{}\";dir=\"both\"{}{}];\n",
            self.entity
                .borrow()
                .as_ref()
                .map(|s| s.as_str())
                .unwrap_or("")
                .to_lowercase(),
            target_type,
            name,
            source_cardinality,
            target_cardinality
        ));
        Ok(())
    }

    fn end_module(&self, _: &Identifier) -> Result<(), Error> {
        self.buffer.borrow_mut().push_str("}\n");
        *self.entity.borrow_mut() = None;
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

#[inline(always)]
fn mkformat(format: OutputFormat) -> Format {
    match format {
        OutputFormat::ImageJpeg => Format::Jpg,
        OutputFormat::ImagePng => Format::Png,
        OutputFormat::ImageSvg => Format::Svg,
        _ => unreachable!(),
    }
}

const CARD_ONLY_ONE: &str = "teetee";
const CARD_ZERO_OR_ONE: &str = "teeodot";

const CARD_MANY: &str = "ocrow";
const CARD_ONE_OR_MANY: &str = "ocrowtee";
const CARD_ZERO_OR_MANY: &str = "ocrowodot";

#[inline(always)]
fn arrow_end(end: &str, cardinality: &Cardinality) -> String {
    format!(
        "; arrow{}=\"{}\"",
        end,
        match (cardinality.min_occurs(), cardinality.max_occurs()) {
            (0, None) => CARD_ZERO_OR_MANY,
            (1, None) => CARD_ONE_OR_MANY,
            (0, Some(1)) => CARD_ZERO_OR_ONE,
            (1, Some(1)) => CARD_ONLY_ONE,
            _ => CARD_MANY,
        }
    )
}

#[inline(always)]
fn name_to_ref(name: &str) -> String {
    name.replace(':', "-").to_lowercase()
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
