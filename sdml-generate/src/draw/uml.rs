/*!
Provide a generator for UML class diagrams via PlantUML.

*/

use crate::draw::{filter::DiagramContentFilter, OutputFormat, UML_PROGRAM};
use crate::exec::{exec_with_temp_input, CommandArg};
use sdml_core::cache::ModuleStore;
use sdml_core::error::Error;
use sdml_core::model::annotations::AnnotationProperty;
use sdml_core::model::definitions::{
    DatatypeDef, EntityDef, EnumDef, EventDef, PropertyDef, RdfDef, StructureDef, TypeVariant,
    UnionDef, ValueVariant,
};
use sdml_core::model::identifiers::{Identifier, IdentifierReference};
use sdml_core::model::members::{
    Cardinality, Member, MemberDef, MemberKind, Ordering, TypeReference, Uniqueness,
    DEFAULT_CARDINALITY,
};
use sdml_core::model::modules::Module;
use sdml_core::model::walk::{walk_module_simple, SimpleModuleVisitor};
use sdml_core::model::{HasName, HasNameReference, HasOptionalBody, References};
use sdml_core::syntax::{KW_ORDERING_ORDERED, KW_UNIQUENESS_UNIQUE};
use std::collections::HashSet;
use std::io::Write;
use std::path::{Path, PathBuf};
use tracing::{debug, trace};

use super::filter::DefinitionKind;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, Default)]
pub struct UmlDiagramGenerator {
    buffer: String,
    imports: (String, String),
    output: Option<DiagramOutput>,
    assoc_src: Option<String>,
    refs: Option<String>,
    options: UmlDiagramOptions,
}

#[derive(Clone, Debug, Default)]
pub struct UmlDiagramOptions {
    emit_annotations: bool,
    content_filter: DiagramContentFilter,
    output_format: OutputFormat,
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, Default)]
struct DiagramOutput {
    file_name: String,
    output_dir: String,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl UmlDiagramOptions {
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

    pub fn emit_annotations(self, emit_annotations: bool) -> Self {
        Self {
            emit_annotations,
            ..self
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl crate::Generator for UmlDiagramGenerator {
    type Options = UmlDiagramOptions;

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
            "UmlDiagramGenerator",
            "write_to_file_in_format" =>
            "{}, _",
            module.name());

        self.options = options;
        self.imports = make_imports(module);

        walk_module_simple(module, self, true, true)?;

        if self.options.output_format == OutputFormat::Source {
            writer.write_all(self.buffer.as_bytes())?;
        } else {
            match exec_with_temp_input(
                UML_PROGRAM,
                // TODO: use path parameter instead!
                vec![
                    CommandArg::new(format!(
                        "-o{}",
                        self.output.as_ref().map(|o| &o.output_dir).unwrap()
                    )),
                    format_to_arg(self.options.output_format),
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

impl SimpleModuleVisitor for UmlDiagramGenerator {
    fn module_start(&mut self, module: &Module) -> Result<bool, Error> {
        trace!("start_module");

        let name = module.name();
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

        Self::INCLUDE_NESTED
    }

    fn module_end(&mut self, _: &Module) -> Result<(), Error> {
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

    fn annotation_property(&mut self, property: &AnnotationProperty) -> Result<(), Error> {
        if self.options.emit_annotations {
            let name = property.name_reference();
            let value = property.value();
            self.buffer.push_str(&format!("{{{name} = {value}}}\n"));
        }
        Ok(())
    }

    fn datatype_start(&mut self, datatype: &DatatypeDef) -> Result<bool, Error> {
        let name = datatype.name();
        if self
            .options
            .content_filter
            .draw_definition_named(DefinitionKind::Datatype, name)
        {
            self.buffer
                .push_str(&start_type_with_sterotype("class", name, "datatype"));

            // TODO: add opaque as stereotype on restriction

            let base_type = datatype.base_type();
            let restriction = format!("  {} --|> {}\n", make_id(name), make_id(base_type));
            self.refs = Some(
                self.refs
                    .clone()
                    .map(|r| format!("{r}{restriction}"))
                    .unwrap_or(restriction),
            );
            self.options.emit_annotations = true;
        }
        Self::INCLUDE_NESTED
    }

    fn datatype_end(&mut self, datatype: &DatatypeDef) -> Result<(), Error> {
        let name = datatype.name();
        self.buffer.push_str("  }\n");
        self.buffer.push_str(&format!(
            "  hide {} {}\n",
            make_id(name),
            if datatype.has_body() {
                "methods"
            } else {
                "members"
            }
        ));

        self.assoc_src = None;
        self.options.emit_annotations = false;

        Ok(())
    }

    fn entity_start(&mut self, entity: &EntityDef) -> Result<bool, Error> {
        let name = entity.name();
        if self
            .options
            .content_filter
            .draw_definition_named(DefinitionKind::Entity, name)
        {
            self.buffer.push_str(&start_type_with_sterotype(
                if entity.has_body() {
                    "class"
                } else {
                    "abstract"
                },
                name,
                "entity",
            ));
            self.assoc_src = Some(name.to_string());
        }
        Self::INCLUDE_NESTED
    }

    fn entity_end(&mut self, entity: &EntityDef) -> Result<(), Error> {
        self.buffer
            .push_str(&end_type(entity.name(), entity.has_body()));
        self.assoc_src = None;
        Ok(())
    }

    fn enum_start(&mut self, an_enum: &EnumDef) -> Result<bool, Error> {
        let name = an_enum.name();
        if self
            .options
            .content_filter
            .draw_definition_named(DefinitionKind::Enum, name)
        {
            self.buffer
                .push_str(&start_type_with_sterotype("class", an_enum.name(), "enum"));
        }
        Self::INCLUDE_NESTED
    }

    fn enum_end(&mut self, an_enum: &EnumDef) -> Result<(), Error> {
        self.buffer
            .push_str(&end_type(an_enum.name(), an_enum.has_body()));
        self.assoc_src = None;
        Ok(())
    }

    fn event_start(&mut self, event: &EventDef) -> Result<bool, Error> {
        let name = event.name();
        if self
            .options
            .content_filter
            .draw_definition_named(DefinitionKind::Enum, name)
        {
            let source = event.event_source();
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
        }
        Self::INCLUDE_NESTED
    }

    fn event_end(&mut self, event: &EventDef) -> Result<(), Error> {
        self.buffer
            .push_str(&end_type(event.name(), event.has_body()));
        self.assoc_src = None;
        Ok(())
    }

    fn property_start(&mut self, property: &PropertyDef) -> Result<bool, Error> {
        let defn = property.member_def();
        if self
            .options
            .content_filter
            .draw_definition_named(DefinitionKind::Enum, defn.name())
        {
            self.buffer
                .push_str(&start_type_with_sterotype("class", defn.name(), "property"));
        }
        Self::INCLUDE_NESTED
    }

    fn property_end(&mut self, property: &PropertyDef) -> Result<(), Error> {
        let defn = property.member_def();
        self.buffer
            .push_str(&end_type(defn.name(), defn.has_body()));
        Ok(())
    }

    fn structure_start(&mut self, structure: &StructureDef) -> Result<bool, Error> {
        let name = structure.name();
        if self
            .options
            .content_filter
            .draw_definition_named(DefinitionKind::Enum, name)
        {
            self.buffer
                .push_str(&start_type_with_sterotype("class", name, "structure"));
            self.assoc_src = Some(name.to_string());
        }
        Self::INCLUDE_NESTED
    }

    fn structure_end(&mut self, structure: &StructureDef) -> Result<(), Error> {
        self.buffer
            .push_str(&end_type(structure.name(), structure.has_body()));
        self.assoc_src = None;
        Ok(())
    }

    fn rdf_start(&mut self, rdf: &RdfDef) -> Result<bool, Error> {
        let name = rdf.name();
        if self
            .options
            .content_filter
            .draw_definition_named(DefinitionKind::Enum, name)
        {
            self.buffer
                .push_str(&start_type_with_sterotype("class", name, "rdf"));
            self.assoc_src = Some(name.to_string());
        }
        Self::INCLUDE_NESTED
    }

    fn rdf_end(&mut self, rdf: &RdfDef) -> Result<(), Error> {
        self.buffer.push_str(&end_type(rdf.name(), false));
        self.assoc_src = None;
        Ok(())
    }

    fn union_start(&mut self, union: &UnionDef) -> Result<bool, Error> {
        let name = union.name();
        if self
            .options
            .content_filter
            .draw_definition_named(DefinitionKind::Enum, name)
        {
            self.buffer
                .push_str(&start_type_with_sterotype("class", name, "union"));
            self.assoc_src = Some(name.to_string());
        }
        Self::INCLUDE_NESTED
    }

    fn union_end(&mut self, union: &UnionDef) -> Result<(), Error> {
        self.buffer
            .push_str(&end_type(union.name(), union.has_body()));
        self.assoc_src = None;
        Ok(())
    }

    fn member_start(&mut self, member: &Member) -> Result<bool, Error> {
        match self.make_member(self.assoc_src.as_ref().unwrap(), member) {
            (v, false) => {
                self.refs = Some(self.refs.clone().map(|r| format!("{r}{v}")).unwrap_or(v))
            }
            (v, true) => self.buffer.push_str(v.as_str()),
        }
        Self::INCLUDE_NESTED
    }

    fn identity_member_start(&mut self, _thing: &Member) -> Result<bool, Error> {
        Self::INCLUDE_NESTED
    }

    fn value_variant_start(&mut self, variant: &ValueVariant) -> Result<bool, Error> {
        self.buffer.push_str(&format!("    +{}\n", variant.name()));
        Self::INCLUDE_NESTED
    }

    fn type_variant_start(&mut self, variant: &TypeVariant) -> Result<bool, Error> {
        let name = variant.name();
        let rename = variant.rename();
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
        Self::INCLUDE_NESTED
    }
}

impl UmlDiagramGenerator {
    fn make_member(&self, source: &str, member: &Member) -> (String, bool) {
        match &member.kind() {
            MemberKind::Reference(v) => self.make_property_ref(v),
            MemberKind::Definition(v) => self.make_member_def(source, v),
        }
    }

    fn make_member_def(&self, source: &str, member_def: &MemberDef) -> (String, bool) {
        let name = member_def.name();
        let card = member_def.target_cardinality();
        let target_type = member_def.target_type();
        match &target_type {
            TypeReference::Type(type_ref) => {
                if *card == DEFAULT_CARDINALITY {
                    (
                        format!("    +{name}: {}\n", make_type_reference(target_type)),
                        true,
                    )
                } else {
                    (
                        // TODO: make references to entities "o" not "*"
                        format!(
                            "  s_{source} *--> \"+{name}\\n{}\" {}\n",
                            to_uml_string(card, true),
                            make_id(type_ref),
                        ),
                        false,
                    )
                }
            }
            TypeReference::MappingType(_) => (
                format!(
                    "    +{name}: {} {}\n",
                    to_uml_string(card, false),
                    make_type_reference(target_type)
                ),
                true,
            ),
            TypeReference::Unknown => (format!("    +{name}: unknown\n"), true),
        }
    }

    fn make_property_ref(&self, property_ref: &IdentifierReference) -> (String, bool) {
        (format!("    +<<ref>> {property_ref}\n"), true)
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

fn make_type_reference(type_ref: &TypeReference) -> String {
    match type_ref {
        TypeReference::Unknown => "unknown".to_string(),
        TypeReference::Type(v) => v.to_string(),
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

/// Note:
///  PlantUML does not take output file names, it derives the names from the input file names.
///  However, it will take the path of the directory to put output files in, which needs to be
///  specified else it is derived from the input path (a temp file name).
#[allow(dead_code)]
#[inline(always)]
fn path_to_output<P>(path: P, module_name: &Identifier) -> Result<DiagramOutput, Error>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    trace!("path_to_output({:?}, {})", path, module_name);

    let output_dir = if path.components().count() == 1 {
        std::env::current_dir()?.canonicalize()?
    } else {
        path.parent()
            .unwrap() // safe due to test above
            .canonicalize()?
    };
    trace!("path_to_output output_dir = {:?}", output_dir);

    Ok(DiagramOutput {
        file_name: path
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
            card.range()
                .max_occurs()
                .map(|i| i.to_string())
                .unwrap_or_else(|| String::from("*"))
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
