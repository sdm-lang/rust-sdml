/*!
Provide a generator for Entity-Relationship diagrams via GraphViz.

*/

use crate::draw::{
    filter::{DefinitionKind, DiagramContentFilter},
    OutputFormat, DOT_PROGRAM,
};
use crate::exec::exec_with_temp_input;
use crate::Generator;
use sdml_core::error::Error;
use sdml_core::model::definitions::{DatatypeDef, EntityDef, EnumDef, EventDef, StructureDef};
use sdml_core::model::members::{Cardinality, Member, TypeReference, DEFAULT_CARDINALITY};
use sdml_core::model::modules::{ImportStatement, Module};
use sdml_core::model::walk::{walk_module_simple, SimpleModuleVisitor};
use sdml_core::model::HasName;
use sdml_core::{cache::ModuleStore, model::members::MemberKind};
use std::io::Write;
use std::path::PathBuf;
use tracing::trace;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct ErdDiagramGenerator {
    buffer: String,
    entity: Option<String>,
    seen: Vec<String>,
    options: ErdDiagramOptions,
}

#[derive(Debug, Default)]
pub struct ErdDiagramOptions {
    content_filter: DiagramContentFilter,
    output_format: OutputFormat,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl ErdDiagramOptions {
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

impl Generator for ErdDiagramGenerator {
    type Options = ErdDiagramOptions;

    fn generate_with_options<W>(
        &mut self,
        module: &Module,
        _: &impl ModuleStore,
        options: Self::Options,
        _: Option<PathBuf>,
        writer: &mut W,
    ) -> Result<(), Error>
    where
        W: Write + Sized,
    {
        trace_entry!(
            "ErdDiagramGenerator",
            "generate_with_options" =>
                "{}, _, _",
            module.name());

        self.options = options;

        walk_module_simple(module, self, false, false)?;

        if self.options.output_format == OutputFormat::Source {
            writer.write_all(self.buffer.as_bytes())?;
        } else {
            match exec_with_temp_input(
                DOT_PROGRAM,
                vec![self.options.output_format.into()],
                &self.buffer,
            ) {
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

impl SimpleModuleVisitor for ErdDiagramGenerator {
    fn module_start(&mut self, _: &Module) -> Result<bool, Error> {
        self.buffer.push_str(
            r#"digraph G {
  bgcolor="transparent";
  rankdir="TB";
  fontname="Helvetica,Arial,sans-serif";
  node [fontname="Helvetica,Arial,sans-serif"; fontsize=10];
  edge [fontname="Helvetica,Arial,sans-serif"; fontsize=9; fontcolor="dimgrey";
        labelfontcolor="blue"; labeldistance=2.0];
  graph [pad="0.5", nodesep="1", ranksep="1"];
  splines="ortho";

"#,
        );
        Ok(true)
    }

    fn import_statement_start(&mut self, stmt: &ImportStatement) -> Result<bool, Error> {
        trace!("import: {:?}", stmt);
        for import in stmt.imports() {
            if self.options.content_filter.draw_import(import) {
                self.buffer
                    .push_str(&node(import.module().as_ref(), import.module().as_ref()));
            }
        }
        Self::INCLUDE_NESTED
    }

    fn entity_start(&mut self, defn: &EntityDef) -> Result<bool, Error> {
        if self
            .options
            .content_filter
            .draw_definition_named(DefinitionKind::Entity, defn.name())
        {
            let name = defn.name();
            trace!("entity: {}", name);
            self.buffer.push_str(&format!(
                "  {} [label=\"{}\"; penwidth=1.5];\n",
                name_to_ref(name.as_ref()),
                name
            ));
            self.entity = Some(name.to_string());
        }
        Self::INCLUDE_NESTED
    }

    fn datatype_start(&mut self, defn: &DatatypeDef) -> Result<bool, Error> {
        if self
            .options
            .content_filter
            .draw_definition_named(DefinitionKind::Datatype, defn.name())
        {
            let name = defn.name();
            trace!("datatype: {}", name);
            self.buffer.push_str(&node_with_icon(
                &name_to_ref(name.as_ref()),
                name.as_ref(),
                '■',
            ));
            self.entity = Some(name.to_string());
        }
        Self::INCLUDE_NESTED
    }

    fn enum_start(&mut self, defn: &EnumDef) -> Result<bool, Error> {
        if self
            .options
            .content_filter
            .draw_definition_named(DefinitionKind::Enum, defn.name())
        {
            let name = defn.name();
            trace!("enum: {}", name);
            self.buffer.push_str(&node_with_icon(
                &name_to_ref(name.as_ref()),
                name.as_ref(),
                '≣',
            ));
            self.entity = Some(name.to_string());
        }
        Self::INCLUDE_NESTED
    }

    fn event_start(&mut self, defn: &EventDef) -> Result<bool, Error> {
        if self
            .options
            .content_filter
            .draw_definition_named(DefinitionKind::Event, defn.name())
        {
            let name = defn.name();
            trace!("event: {}", name);
            self.buffer.push_str(&node_with_icon(
                &name_to_ref(name.as_ref()),
                name.as_ref(),
                '☇',
            ));
            self.entity = Some(name.to_string());
        }
        Self::INCLUDE_NESTED
    }

    fn structure_start(&mut self, defn: &StructureDef) -> Result<bool, Error> {
        if self
            .options
            .content_filter
            .draw_definition_named(DefinitionKind::Structure, defn.name())
        {
            let name = defn.name();
            trace!("structure: {}", name);
            self.buffer
                .push_str(&node(&name_to_ref(name.as_ref()), name.as_ref()));
            self.entity = Some(name.to_string());
        }
        Self::INCLUDE_NESTED
    }

    fn identity_member_start(&mut self, member: &Member) -> Result<bool, Error> {
        self.member_common(member, false)?;
        Self::INCLUDE_NESTED
    }

    fn member_start(&mut self, member: &Member) -> Result<bool, Error> {
        self.member_common(member, false)?;
        Self::INCLUDE_NESTED
    }

    fn module_end(&mut self, _: &Module) -> Result<(), Error> {
        self.buffer.push_str("}\n");
        self.entity = None;
        Ok(())
    }
}

impl ErdDiagramGenerator {
    fn member_common(&mut self, member: &Member, is_identity: bool) -> Result<(), Error> {
        match member.kind() {
            MemberKind::Reference(v) => {
                let name = v.to_string();
                self.buffer.push_str(&edge(
                    &self.entity.as_deref().unwrap_or_default().to_lowercase(),
                    None,
                    &name_to_ref(&name),
                    None,
                    name.as_ref(),
                ));
            }
            MemberKind::Definition(v) => {
                let name = v.name();

                let target_type = match v.target_type() {
                    TypeReference::Unknown => {
                        if !self.seen.contains(&NAME_UNKNOWN.to_string()) {
                            self.seen.push(NAME_UNKNOWN.to_string());
                            self.buffer.push_str(NODE_UNKNOWN);
                        }
                        NAME_UNKNOWN.to_string()
                    }
                    TypeReference::Type(target_type) => name_to_ref(&target_type.to_string()),
                    TypeReference::MappingType(_) => todo!(),
                };

                let target_cardinality = v.target_cardinality();
                let arrow_to = if *target_cardinality != DEFAULT_CARDINALITY {
                    Some(arrow_end("head", target_cardinality))
                } else {
                    None
                };

                let arrow_from = if is_identity {
                    Some(CARD_ONLY_ONE)
                } else {
                    None
                };

                self.buffer.push_str(&edge(
                    &self.entity.as_deref().unwrap_or_default().to_lowercase(),
                    arrow_from,
                    &target_type,
                    arrow_to.as_deref(),
                    name.as_ref(),
                ));
            }
        }
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

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
    name.replace(':', "__").to_lowercase()
}

#[inline(always)]
fn node(name: &str, label: &str) -> String {
    format!(
        r#"  {} [label="{}"; style="dashed"; color="dimgrey"; fontcolor="dimgrey"];\n"#,
        name, label
    )
}

const NAME_UNKNOWN: &str = "unknown";

const NODE_UNKNOWN: &str =
    r#"  unknown [shape=rect; label="Unknown"; color="grey"; fontcolor="grey"];\n"#;

#[inline(always)]
fn node_with_icon(name: &str, label: &str, icon: char) -> String {
    format!(
        r#"  {} [label="{} {}"; style="dashed"; color="dimgrey"; fontcolor="dimgrey"];\n"#,
        name, icon, label
    )
}

fn edge(
    from_node: &str,
    arrow_from: Option<&str>,
    to_node: &str,
    arrow_to: Option<&str>,
    tooltip: &str,
) -> String {
    format!(
        r#"  {from_node} -> {to_node} [tooltip="{tooltip}";dir="both"{}{}];\n"#,
        arrow_from.unwrap_or_default(),
        arrow_to.unwrap_or_default(),
    )
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
