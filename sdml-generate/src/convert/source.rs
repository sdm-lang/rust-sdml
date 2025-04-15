/*!
This module provides a generator that recreates the surface syntax for a module given its
in-memory representation.

# Example

```rust
use sdml_core::store::InMemoryModuleCache;
use sdml_core::model::modules::Module;
use sdml_generate::Generator;
use sdml_generate::convert::source::{SourceGenerator, SourceGeneratorOptions};
use std::io::stdout;
# use sdml_core::model::identifiers::Identifier;
# fn load_module() -> (Module, InMemoryModuleCache) { (Module::empty(Identifier::new_unchecked("example")), InMemoryModuleCache::default()) }
# sdml_generate::color::set_colorize(sdml_errors::diagnostics::color::UseColor::Never);

let (module, cache) = load_module();

let mut generator: SourceGenerator = Default::default();
let options = SourceGeneratorOptions::default();
let source = generator
    .generate_to_string(&module, &cache, options, None)
    .expect("write to stdout failed");
assert_eq!(source.as_str(), "module example is end\n");

```
*/

use crate::color::sdml::{
    braces_end, braces_start, format_url, import, keyword, member_name, module_name_def, operator,
    paren_end, paren_start, property_name, sequence_end, sequence_start, type_name_def,
    type_name_ref, type_variant_name_def, type_variant_ref_def, value_variant_name_def,
};
use crate::Generator;
use sdml_core::error::Error;
use sdml_core::model::annotations::{Annotation, AnnotationProperty, HasAnnotations};
use sdml_core::model::constraints::{Constraint, ConstraintBody};
use sdml_core::model::definitions::{
    DatatypeDef, Definition, DimensionDef, EntityDef, EnumDef, EventDef, PropertyDef, RdfDef,
    StructureDef, TypeVariant, UnionDef, ValueVariant,
};
use sdml_core::model::identifiers::IdentifierReference;
use sdml_core::model::members::{
    Cardinality, Member, MemberDef, MemberKind, TypeReference, DEFAULT_CARDINALITY,
};
use sdml_core::model::modules::{Module, ModuleBody};
use sdml_core::model::{HasBody, HasName, HasNameReference, HasOptionalBody};
use sdml_core::store::ModuleStore;
use std::path::PathBuf;
use std::{fmt::Debug, io::Write};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// The type that implements the generator.
#[derive(Debug, Default)]
pub struct SourceGenerator {
    options: SourceGeneratorOptions,
}

/// The type that implements the generator.
#[derive(Debug)]
pub struct SourceGeneratorOptions {
    indentation: usize,
    level: SourceGenerationLevel,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SourceGenerationLevel {
    #[default]
    Full,
    Members,
    Definitions,
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

const DEFAULT_INDENTATION: usize = 2;

const MODULE_ANNOTATION_INDENT: usize = 1;
const MODULE_IMPORT_INDENT: usize = 1;
const MODULE_DEFINITION_INDENT: usize = 1;
const DEFINITION_ANNOTATION_INDENT: usize = 2;
const DEFINITION_MEMBER_INDENT: usize = 2;
const MEMBER_ANNOTATION_INDENT: usize = 3;

const ELIPPSIS: &str = " ;; ...\n";

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for SourceGeneratorOptions {
    fn default() -> Self {
        Self {
            indentation: DEFAULT_INDENTATION,
            level: Default::default(),
        }
    }
}

impl SourceGeneratorOptions {
    pub fn with_level(self, level: SourceGenerationLevel) -> Self {
        Self { level, ..self }
    }
    pub fn with_indentation(self, indentation: usize) -> Self {
        Self {
            indentation,
            ..self
        }
    }
    fn indentation_str(&self, level: usize) -> String {
        let n = level * self.indentation;
        format!("{:n$}", "")
    }
}

// ------------------------------------------------------------------------------------------------

impl SourceGenerationLevel {
    #[inline(always)]
    const fn generate_definition_bodies(&self) -> bool {
        matches!(self, Self::Full | Self::Members)
    }

    #[inline(always)]
    const fn generate_member_bodies(&self) -> bool {
        matches!(self, Self::Full)
    }

    #[inline(always)]
    const fn generate_variant_bodies(&self) -> bool {
        matches!(self, Self::Full)
    }
}

// ------------------------------------------------------------------------------------------------

const EOL: &[u8] = b"\n";

impl Generator for SourceGenerator {
    type Options = SourceGeneratorOptions;

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
        self.options = options;

        writer.write_all(
            format!("{} {} ", keyword("module"), module_name_def(module.name())).as_bytes(),
        )?;

        if let Some(base) = module.base_uri() {
            writer.write_all(format!("{} ", format_url(base.as_ref())).as_bytes())?;
        }

        let body = module.body();
        if body.has_imports() || body.has_annotations() || body.has_definitions() {
            writer.write_all(format!("{}\n", keyword("is")).as_bytes())?;

            if body.has_imports() {
                writer.write_all(EOL)?;
                self.write_module_imports(body, writer)?;
            }

            if body.has_annotations() {
                writer.write_all(EOL)?;
                self.write_annotations(body.annotations(), writer, MODULE_ANNOTATION_INDENT)?;
            }
            if body.has_definitions() {
                self.write_module_definitions(body, writer)?;
            }

            writer.write_all(format!("\n{}\n", keyword("end")).as_bytes())?;
        } else {
            writer.write_all(format!("{} {}\n", keyword("is"), keyword("end")).as_bytes())?;
        }
        Ok(())
    }
}

impl SourceGenerator {
    fn write_module_imports(
        &mut self,
        module_body: &ModuleBody,
        writer: &mut dyn Write,
    ) -> Result<(), Error> {
        let indentation = self.options.indentation_str(MODULE_IMPORT_INDENT);
        if module_body.has_imports() {
            for import_statement in module_body.imports() {
                let imported = if import_statement.imports_len() == 1 {
                    import_statement
                        .imports()
                        .map(import)
                        .collect::<Vec<String>>()
                        .join("")
                } else {
                    format!(
                        "{} {} {}",
                        sequence_start(),
                        import_statement
                            .imports()
                            .map(import)
                            .collect::<Vec<String>>()
                            .join(" "),
                        sequence_end(),
                    )
                };
                writer.write_all(
                    format!("{indentation}{} {imported}\n", keyword("import")).as_bytes(),
                )?;
            }
        }
        Ok(())
    }

    #[allow(single_use_lifetimes)]
    fn write_annotations<'a>(
        &mut self,
        annotations: impl Iterator<Item = &'a Annotation>,
        writer: &mut dyn Write,
        indent_level: usize,
    ) -> Result<(), Error> {
        for annotation in annotations {
            match annotation {
                Annotation::Property(v) => {
                    self.write_annotation_property(v, writer, indent_level)?
                }
                Annotation::Constraint(v) => self.write_constraint(v, writer, indent_level)?,
            }
        }

        Ok(())
    }

    fn write_annotation_property(
        &mut self,
        annotation: &AnnotationProperty,
        writer: &mut dyn Write,
        indent_level: usize,
    ) -> Result<(), Error> {
        let indentation = self.options.indentation_str(indent_level);
        // TODO: ensure wrapping
        writer.write_all(
            format!(
                "{indentation}{} {} {}\n",
                property_name(annotation.name_reference()),
                operator("="),
                annotation.value()
            )
            .as_bytes(),
        )?;
        Ok(())
    }

    fn write_constraint(
        &mut self,
        constraint: &Constraint,
        writer: &mut dyn Write,
        indent_level: usize,
    ) -> Result<(), Error> {
        let indentation = self.options.indentation_str(indent_level);
        writer.write_all(format!("{indentation}{} ", keyword("assert")).as_bytes())?;
        writer.write_all(format!("{} ", constraint.name()).as_bytes())?;
        match constraint.body() {
            ConstraintBody::Informal(v) => {
                writer.write_all(format!("{} {v:?}", operator("=")).as_bytes())?;
            }
            ConstraintBody::Formal(_) => {
                writer.write_all(format!("{}\n", keyword("is")).as_bytes())?;
                // TODO: add constraint sentence
                writer.write_all(format!("{indentation}{}\n", keyword("end")).as_bytes())?;
            }
        }
        Ok(())
    }

    fn write_module_definitions(
        &mut self,
        module_body: &ModuleBody,
        writer: &mut dyn Write,
    ) -> Result<(), Error> {
        let _indentation = self.options.indentation_str(1);
        if module_body.has_definitions() {
            for definition in module_body.definitions() {
                writer.write_all(EOL)?;
                match &definition {
                    Definition::Datatype(v) => self.write_datatype(v, writer)?,
                    Definition::Dimension(v) => self.write_dimension(v, writer)?,
                    Definition::Entity(v) => self.write_entity(v, writer)?,
                    Definition::Enum(v) => self.write_enum(v, writer)?,
                    Definition::Event(v) => self.write_event(v, writer)?,
                    Definition::Property(v) => self.write_property(v, writer)?,
                    Definition::Structure(v) => self.write_structure(v, writer)?,
                    Definition::Rdf(v) => self.write_rdf(v, writer)?,
                    Definition::TypeClass(_) => todo!(),
                    Definition::Union(v) => self.write_union(v, writer)?,
                }
            }
        }
        Ok(())
    }

    fn write_datatype(&mut self, defn: &DatatypeDef, writer: &mut dyn Write) -> Result<(), Error> {
        let indentation = self.options.indentation_str(MODULE_DEFINITION_INDENT);
        writer.write_all(
            format!(
                "{indentation}{} {} {} {}",
                keyword("datatype"),
                type_name_def(defn.name()),
                operator("<-"),
                type_name_ref(defn.base_type())
            )
            .as_bytes(),
        )?;

        if let Some(body) = defn.body() {
            if self.options.level.generate_definition_bodies() {
                writer.write_all(format!(" {}\n", keyword("is")).as_bytes())?;
                if body.has_annotations() {
                    self.write_annotations(
                        body.annotations(),
                        writer,
                        DEFINITION_ANNOTATION_INDENT,
                    )?;
                }
                writer.write_all(format!("{indentation}{}\n", keyword("end")).as_bytes())?;
            } else {
                writer.write_all(ELIPPSIS.as_bytes())?;
            }
        } else {
            writer.write_all(EOL)?;
        }

        Ok(())
    }

    fn write_dimension(
        &mut self,
        defn: &DimensionDef,
        writer: &mut dyn Write,
    ) -> Result<(), Error> {
        let indentation = self.options.indentation_str(MODULE_DEFINITION_INDENT);
        writer.write_all(
            format!(
                "{indentation}{} {}",
                keyword("dimension"),
                type_name_def(defn.name())
            )
            .as_bytes(),
        )?;

        if let Some(body) = defn.body() {
            if self.options.level.generate_definition_bodies() {
                writer.write_all(format!(" {}\n", keyword("is")).as_bytes())?;
                if body.has_annotations() {
                    self.write_annotations(
                        body.annotations(),
                        writer,
                        DEFINITION_ANNOTATION_INDENT,
                    )?;
                    if body.has_members() {
                        writer.write_all(EOL)?;
                    }
                }
                for member in body.members() {
                    self.write_member(member, writer)?;
                }
                writer.write_all(format!("{indentation}{}\n", keyword("end")).as_bytes())?;
            } else {
                writer.write_all(ELIPPSIS.as_bytes())?;
            }
        } else {
            writer.write_all(EOL)?;
        }

        Ok(())
    }

    fn write_entity(&mut self, defn: &EntityDef, writer: &mut dyn Write) -> Result<(), Error> {
        let indentation = self.options.indentation_str(MODULE_DEFINITION_INDENT);
        writer.write_all(
            format!(
                "{indentation}{} {}",
                keyword("entity"),
                type_name_def(defn.name())
            )
            .as_bytes(),
        )?;

        if let Some(body) = defn.body() {
            if self.options.level.generate_definition_bodies() {
                writer.write_all(format!(" {}\n", keyword("is")).as_bytes())?;
                if body.has_annotations() {
                    self.write_annotations(
                        body.annotations(),
                        writer,
                        DEFINITION_ANNOTATION_INDENT,
                    )?;
                    if body.has_members() {
                        writer.write_all(EOL)?;
                    }
                }
                for member in body.members() {
                    self.write_member(member, writer)?;
                }
                writer.write_all(format!("{indentation}{}\n", keyword("end")).as_bytes())?;
            } else {
                writer.write_all(ELIPPSIS.as_bytes())?;
            }
        } else {
            writer.write_all(EOL)?;
        }

        Ok(())
    }

    fn write_enum(&mut self, defn: &EnumDef, writer: &mut dyn Write) -> Result<(), Error> {
        let indentation = self.options.indentation_str(MODULE_DEFINITION_INDENT);
        writer.write_all(
            format!(
                "{indentation}{} {}",
                keyword("enum"),
                type_name_def(defn.name())
            )
            .as_bytes(),
        )?;

        if let Some(body) = defn.body() {
            if self.options.level.generate_definition_bodies() {
                writer.write_all(format!(" {}\n", keyword("of")).as_bytes())?;
                if body.has_annotations() {
                    self.write_annotations(
                        body.annotations(),
                        writer,
                        DEFINITION_ANNOTATION_INDENT,
                    )?;
                    if body.has_variants() {
                        writer.write_all(EOL)?;
                    }
                }
                if body.has_variants() {
                    for variant in body.variants() {
                        self.write_value_variant(variant, writer)?;
                    }
                }
                writer.write_all(format!("{indentation}{}\n", keyword("end")).as_bytes())?;
            } else {
                writer.write_all(ELIPPSIS.as_bytes())?;
            }
        } else {
            writer.write_all(EOL)?;
        }

        Ok(())
    }

    fn write_value_variant(
        &mut self,
        variant: &ValueVariant,
        writer: &mut dyn Write,
    ) -> Result<(), Error> {
        let indentation = self.options.indentation_str(DEFINITION_MEMBER_INDENT);
        writer.write_all(
            format!("{indentation}{}", value_variant_name_def(variant.name())).as_bytes(),
        )?;

        if let Some(body) = variant.body() {
            if self.options.level.generate_member_bodies() {
                writer.write_all(format!(" {}\n", keyword("is")).as_bytes())?;
                if body.has_annotations() {
                    self.write_annotations(body.annotations(), writer, MEMBER_ANNOTATION_INDENT)?;
                }
                writer.write_all(format!("{indentation}{}\n", keyword("end")).as_bytes())?;
            } else {
                writer.write_all(ELIPPSIS.as_bytes())?;
            }
        } else {
            writer.write_all(EOL)?;
        }

        Ok(())
    }

    fn write_event(&mut self, defn: &EventDef, writer: &mut dyn Write) -> Result<(), Error> {
        let indentation = self.options.indentation_str(MODULE_DEFINITION_INDENT);
        writer.write_all(
            format!(
                "{indentation}{} {}",
                keyword("event"),
                type_name_def(defn.name()),
            )
            .as_bytes(),
        )?;

        if let Some(body) = defn.body() {
            if self.options.level.generate_definition_bodies() {
                writer.write_all(format!(" {}\n", keyword("is")).as_bytes())?;
                if body.has_annotations() {
                    self.write_annotations(
                        body.annotations(),
                        writer,
                        DEFINITION_ANNOTATION_INDENT,
                    )?;
                    if body.has_members() {
                        writer.write_all(EOL)?;
                    }
                }
                let source = body.source_entity();
                writer.write_all(
                    format!(
                        "{} {}",
                        keyword("source"),
                        type_name_ref(source.target_entity())
                    )
                    .as_bytes(),
                )?;
                if source.has_members() {
                    if source.member_count() == 1 {
                        writer.write_all(
                            format!(" with {}", source.members().next().unwrap()).as_bytes(),
                        )?;
                    } else {
                        writer.write_all(
                            format!(
                                " with [ {} ]",
                                source
                                    .members()
                                    .map(|id| id.to_string())
                                    .collect::<Vec<String>>()
                                    .join(" ")
                            )
                            .as_bytes(),
                        )?;
                    }
                }
                for member in body.members() {
                    self.write_member(member, writer)?;
                }
                writer.write_all(format!("{indentation}{}\n", keyword("end")).as_bytes())?;
            } else {
                writer.write_all(ELIPPSIS.as_bytes())?;
            }
        } else {
            writer.write_all(EOL)?;
        }

        Ok(())
    }

    fn write_member(&mut self, defn: &Member, writer: &mut dyn Write) -> Result<(), Error> {
        let indentation = self.options.indentation_str(DEFINITION_MEMBER_INDENT);
        match defn.kind() {
            MemberKind::Reference(v) => self.write_member_reference(v, &indentation, writer),
            MemberKind::Definition(v) => self.write_member_definition(v, &indentation, writer),
        }
    }

    fn write_member_definition(
        &mut self,
        defn: &MemberDef,
        indentation: &str,
        writer: &mut dyn Write,
    ) -> Result<(), Error> {
        self.write_member_definition_inner(
            defn,
            indentation,
            MEMBER_ANNOTATION_INDENT,
            indentation,
            writer,
        )
    }

    fn write_member_definition_inner(
        &mut self,
        defn: &MemberDef,
        initial_indentation: &str,
        annotation_indentation: usize,
        end_indentation: &str,
        writer: &mut dyn Write,
    ) -> Result<(), Error> {
        writer.write_all(
            format!(
                "{initial_indentation}{} {} ",
                member_name(defn.name()),
                operator("->")
            )
            .as_bytes(),
        )?;
        if *defn.target_cardinality() != DEFAULT_CARDINALITY {
            self.write_cardinality(defn.target_cardinality(), writer)?;
        }
        self.write_type_reference(defn.target_type(), writer)?;
        if let Some(body) = defn.body() {
            if body.has_annotations() && self.options.level.generate_member_bodies() {
                writer.write_all(format!(" {}\n", keyword("is")).as_bytes())?;
                self.write_annotations(body.annotations(), writer, annotation_indentation)?;
                writer.write_all(format!("{end_indentation}{}\n", keyword("end")).as_bytes())?;
            }
        } else {
            writer.write_all(EOL)?;
        }
        Ok(())
    }

    fn write_member_reference(
        &mut self,
        defn: &IdentifierReference,
        indentation: &str,
        writer: &mut dyn Write,
    ) -> Result<(), Error> {
        writer.write_all(format!("{indentation}ref {}\n", defn).as_bytes())?;
        Ok(())
    }

    fn write_cardinality(
        &mut self,
        defn: &Cardinality,
        writer: &mut dyn Write,
    ) -> Result<(), Error> {
        writer.write_all(
            format!(
                "{}{}{}{}{}{} ",
                braces_start(),
                if let Some(uniqueness) = defn.uniqueness() {
                    format!("{} ", keyword(uniqueness.to_string()))
                } else {
                    String::new()
                },
                if let Some(ordering) = defn.ordering() {
                    format!("{} ", keyword(ordering.to_string()))
                } else {
                    String::new()
                },
                defn.min_occurs(),
                if let Some(max_occurs) = defn.max_occurs() {
                    if max_occurs == defn.min_occurs() {
                        String::new()
                    } else {
                        format!("{}{}", operator(".."), max_occurs)
                    }
                } else {
                    operator("..")
                },
                braces_end()
            )
            .as_bytes(),
        )?;
        Ok(())
    }

    #[allow(clippy::only_used_in_recursion)]
    fn write_type_reference(
        &mut self,
        defn: &TypeReference,
        writer: &mut dyn Write,
    ) -> Result<(), Error> {
        match defn {
            TypeReference::Unknown => {
                writer.write_all(keyword("unknown").as_bytes())?;
            }
            TypeReference::Type(name_ref) => {
                writer.write_all(type_name_ref(name_ref).as_bytes())?;
            }
            TypeReference::MappingType(map_ref) => {
                writer.write_all(paren_start().as_bytes())?;
                self.write_type_reference(map_ref.domain(), writer)?;
                writer.write_all(operator("->").as_bytes())?;
                self.write_type_reference(map_ref.range(), writer)?;
                writer.write_all(paren_end().as_bytes())?;
            }
        }
        Ok(())
    }

    fn write_property(&mut self, defn: &PropertyDef, writer: &mut dyn Write) -> Result<(), Error> {
        let indentation = self.options.indentation_str(MODULE_DEFINITION_INDENT);
        writer.write_all(format!("{indentation}{} ", keyword("property"),).as_bytes())?;

        self.write_member_definition_inner(
            defn.member_def(),
            "",
            DEFINITION_ANNOTATION_INDENT,
            &indentation,
            writer,
        )
    }

    fn write_rdf(&mut self, defn: &RdfDef, writer: &mut dyn Write) -> Result<(), Error> {
        let indentation = self.options.indentation_str(MODULE_DEFINITION_INDENT);
        writer.write_all(
            format!(
                "{indentation}{} {}",
                keyword("rdf"),
                type_name_def(defn.name())
            )
            .as_bytes(),
        )?;

        if self.options.level.generate_definition_bodies() {
            let body = defn.body();
            writer.write_all(format!(" {}\n", keyword("is")).as_bytes())?;
            if body.has_annotations() {
                self.write_annotations(body.annotations(), writer, DEFINITION_ANNOTATION_INDENT)?;
            }
            writer.write_all(format!("{indentation}{}\n", keyword("end")).as_bytes())?;
        } else {
            writer.write_all(
                format!(
                    " {}\n{indentation}{}{indentation}{}\n",
                    keyword("is"),
                    ELIPPSIS,
                    keyword("end"),
                )
                .as_bytes(),
            )?;
        }

        Ok(())
    }

    fn write_structure(
        &mut self,
        defn: &StructureDef,
        writer: &mut dyn Write,
    ) -> Result<(), Error> {
        let indentation = self.options.indentation_str(MODULE_DEFINITION_INDENT);
        writer.write_all(
            format!(
                "{indentation}{} {}",
                keyword("structure"),
                type_name_def(defn.name())
            )
            .as_bytes(),
        )?;

        if let Some(body) = defn.body() {
            if self.options.level.generate_definition_bodies() {
                writer.write_all(format!(" {}\n", keyword("is")).as_bytes())?;
                if body.has_annotations() {
                    self.write_annotations(
                        body.annotations(),
                        writer,
                        DEFINITION_ANNOTATION_INDENT,
                    )?;
                    if body.has_members() {
                        writer.write_all(EOL)?;
                    }
                }
                for member in body.members() {
                    self.write_member(member, writer)?;
                }
                writer.write_all(format!("{indentation}{}\n", keyword("end")).as_bytes())?;
            } else {
                writer.write_all(ELIPPSIS.as_bytes())?;
            }
        } else {
            writer.write_all(EOL)?;
        }

        Ok(())
    }

    fn write_union(&mut self, defn: &UnionDef, writer: &mut dyn Write) -> Result<(), Error> {
        let indentation = self.options.indentation_str(MODULE_DEFINITION_INDENT);
        writer.write_all(
            format!(
                "{indentation}{} {}",
                keyword("union"),
                type_name_def(defn.name())
            )
            .as_bytes(),
        )?;

        if let Some(body) = defn.body() {
            if self.options.level.generate_definition_bodies() {
                writer.write_all(format!(" {}\n", keyword("of")).as_bytes())?;
                if body.has_annotations() {
                    self.write_annotations(
                        body.annotations(),
                        writer,
                        DEFINITION_ANNOTATION_INDENT,
                    )?;
                    if body.has_variants() {
                        writer.write_all(EOL)?;
                    }
                }
                if body.has_variants() {
                    for variant in body.variants() {
                        self.write_type_variant(variant, writer)?;
                    }
                }
                writer.write_all(format!("{indentation}{}\n", keyword("end")).as_bytes())?;
            } else {
                writer.write_all(ELIPPSIS.as_bytes())?;
            }
        } else {
            writer.write_all(EOL)?;
        }

        Ok(())
    }

    fn write_type_variant(
        &mut self,
        variant: &TypeVariant,
        writer: &mut dyn Write,
    ) -> Result<(), Error> {
        let indentation = self.options.indentation_str(DEFINITION_MEMBER_INDENT);

        if let Some(rename) = variant.rename() {
            writer.write_all(
                format!("{indentation}{}", type_name_ref(variant.name_reference())).as_bytes(),
            )?;
            writer.write_all(
                format!(" {} {}", keyword("as"), type_variant_name_def(rename)).as_bytes(),
            )?;
        } else {
            writer.write_all(
                format!(
                    "{indentation}{}",
                    type_variant_ref_def(variant.name_reference())
                )
                .as_bytes(),
            )?;
        }

        if let Some(body) = variant.body() {
            if self.options.level.generate_variant_bodies() {
                writer.write_all(format!(" {}\n", keyword("is")).as_bytes())?;
                if body.has_annotations() {
                    self.write_annotations(body.annotations(), writer, MEMBER_ANNOTATION_INDENT)?;
                }
                writer.write_all(format!("{indentation}{}\n", keyword("end")).as_bytes())?;
            } else {
                writer.write_all(ELIPPSIS.as_bytes())?;
            }
        } else {
            writer.write_all(EOL)?;
        }

        Ok(())
    }
}
