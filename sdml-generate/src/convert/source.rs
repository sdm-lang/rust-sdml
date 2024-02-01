/*!
This module provides a generator that recreates the surface syntax for a module given its
in-memory representation.

# Example

```rust
use sdml_core::cache::ModuleCache;
use sdml_core::model::modules::Module;
use sdml_generate::GenerateToWriter;
use sdml_generate::convert::source::SourceGenerator;
use std::io::stdout;
# use sdml_core::model::identifiers::Identifier;
# fn load_module() -> (Module, ModuleCache) { (Module::empty(Identifier::new_unchecked("example")), ModuleCache::default()) }
# sdml_generate::color::set_colorize(sdml_generate::color::UseColor::Never);

let (module, cache) = load_module();

let mut generator: SourceGenerator = Default::default();
let source = generator
    .write_to_string(&module, &cache)
    .expect("write to stdout failed");
assert_eq!(source.as_str(), "module example is end\n");

```
*/

use crate::color::sdml::{
    braces_end, braces_start, format_url, import, keyword, member_name, module_name_def, operator,
    paren_end, paren_start, property_name, sequence_end, sequence_start, type_name_def,
    type_name_ref, type_variant_name_def, type_variant_ref_def, value_variant_name_def,
};
use crate::GenerateToWriter;
use sdml_core::cache::ModuleCache;
use sdml_core::error::Error;
use sdml_core::model::annotations::{Annotation, AnnotationProperty, HasAnnotations};
use sdml_core::model::constraints::{Constraint, ConstraintBody};
use sdml_core::model::definitions::{
    DatatypeDef, Definition, EntityDef, EnumDef, EventDef, HasMembers, HasVariants, PropertyDef,
    RdfDef, StructureDef, TypeVariant, UnionDef, ValueVariant,
};
use sdml_core::model::members::{
    Cardinality, HasCardinality, HasType, Member, TypeReference, DEFAULT_CARDINALITY,
};
use sdml_core::model::modules::{Module, ModuleBody};
use sdml_core::model::{HasBody, HasName, HasNameReference, HasOptionalBody};
use std::{fmt::Debug, io::Write};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// The type that implements the generator.
#[derive(Debug, Default)]
pub struct SourceGenerator {}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SourceGenerationLevel {
    #[default]
    Full,
    Members,
    Definitions,
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

impl SourceGenerationLevel {
    fn indentation(&self, level: usize) -> String {
        let n = level * DEFAULT_INDENTATION;
        format!("{:n$}", "")
    }

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

impl GenerateToWriter<SourceGenerationLevel> for SourceGenerator {
    fn write_in_format<W>(
        &mut self,
        module: &Module,
        _: &ModuleCache,
        writer: &mut W,
        options: SourceGenerationLevel,
    ) -> Result<(), Error>
    where
        W: Write + Sized,
    {
        writer.write_all(
            format!("{} {} ", keyword("module"), module_name_def(module.name())).as_bytes(),
        )?;

        if let Some(base) = module.base_uri() {
            writer.write_all(format!("{} ", format_url(base)).as_bytes())?;
        }

        let body = module.body();
        if body.has_imports() || body.has_annotations() || body.has_definitions() {
            writer.write_all(format!("{}\n\n", keyword("is")).as_bytes())?;

            self.write_module_imports(body, writer, &options)?;
            if body.has_annotations() {
                self.write_annotations(
                    body.annotations(),
                    writer,
                    MODULE_ANNOTATION_INDENT,
                    &options,
                )?;
            }
            self.write_module_definitions(body, writer, &options)?;

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
        options: &SourceGenerationLevel,
    ) -> Result<(), Error> {
        let indentation = options.indentation(MODULE_IMPORT_INDENT);
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
            writer.write_all(EOL)?;
        }

        Ok(())
    }

    #[allow(single_use_lifetimes)]
    fn write_annotations<'a>(
        &mut self,
        annotations: impl Iterator<Item = &'a Annotation>,
        writer: &mut dyn Write,
        indent_level: usize,
        options: &SourceGenerationLevel,
    ) -> Result<(), Error> {
        for annotation in annotations {
            match annotation {
                Annotation::Property(v) => {
                    self.write_annotation_property(v, writer, indent_level, options)?
                }
                Annotation::Constraint(v) => {
                    self.write_constraint(v, writer, indent_level, options)?
                }
            }
        }

        Ok(())
    }

    fn write_annotation_property(
        &mut self,
        annotation: &AnnotationProperty,
        writer: &mut dyn Write,
        indent_level: usize,
        options: &SourceGenerationLevel,
    ) -> Result<(), Error> {
        let indentation = options.indentation(indent_level);
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
        options: &SourceGenerationLevel,
    ) -> Result<(), Error> {
        let indentation = options.indentation(indent_level);
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
        options: &SourceGenerationLevel,
    ) -> Result<(), Error> {
        let _indentation = options.indentation(1);
        if module_body.has_definitions() {
            for definition in module_body.definitions() {
                writer.write_all(EOL)?;
                match &definition {
                    Definition::Datatype(v) => self.write_datatype(v, writer, options)?,
                    Definition::Entity(v) => self.write_entity(v, writer, options)?,
                    Definition::Enum(v) => self.write_enum(v, writer, options)?,
                    Definition::Event(v) => self.write_event(v, writer, options)?,
                    Definition::Property(v) => self.write_property(v, writer, options)?,
                    Definition::Structure(v) => self.write_structure(v, writer, options)?,
                    Definition::Rdf(v) => self.write_rdf(v, writer, options)?,
                    Definition::TypeClass(_) => todo!(),
                    Definition::Union(v) => self.write_union(v, writer, options)?,
                }
            }
        }
        Ok(())
    }

    fn write_datatype(
        &mut self,
        defn: &DatatypeDef,
        writer: &mut dyn Write,
        options: &SourceGenerationLevel,
    ) -> Result<(), Error> {
        let indentation = options.indentation(MODULE_DEFINITION_INDENT);
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
            if options.generate_definition_bodies() {
                writer.write_all(format!(" {}\n", keyword("is")).as_bytes())?;
                if body.has_annotations() {
                    self.write_annotations(
                        body.annotations(),
                        writer,
                        DEFINITION_ANNOTATION_INDENT,
                        options,
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

    fn write_entity(
        &mut self,
        defn: &EntityDef,
        writer: &mut dyn Write,
        options: &SourceGenerationLevel,
    ) -> Result<(), Error> {
        let indentation = options.indentation(MODULE_DEFINITION_INDENT);
        writer.write_all(
            format!(
                "{indentation}{} {}",
                keyword("entity"),
                type_name_def(defn.name())
            )
            .as_bytes(),
        )?;

        if let Some(body) = defn.body() {
            if options.generate_definition_bodies() {
                writer.write_all(format!(" {}\n", keyword("is")).as_bytes())?;
                if body.has_annotations() {
                    self.write_annotations(
                        body.annotations(),
                        writer,
                        DEFINITION_ANNOTATION_INDENT,
                        options,
                    )?;
                }
                for member in body.members() {
                    self.write_member(member, writer, options)?;
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

    fn write_enum(
        &mut self,
        defn: &EnumDef,
        writer: &mut dyn Write,
        options: &SourceGenerationLevel,
    ) -> Result<(), Error> {
        let indentation = options.indentation(MODULE_DEFINITION_INDENT);
        writer.write_all(
            format!(
                "{indentation}{} {}",
                keyword("enum"),
                type_name_def(defn.name())
            )
            .as_bytes(),
        )?;

        if let Some(body) = defn.body() {
            if options.generate_definition_bodies() {
                writer.write_all(format!(" {}\n", keyword("of")).as_bytes())?;
                if body.has_annotations() {
                    self.write_annotations(
                        body.annotations(),
                        writer,
                        DEFINITION_ANNOTATION_INDENT,
                        options,
                    )?;
                }
                if body.has_variants() {
                    writer.write_all(b"\n")?;
                    for variant in body.variants() {
                        self.write_value_variant(variant, writer, options)?;
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
        options: &SourceGenerationLevel,
    ) -> Result<(), Error> {
        let indentation = options.indentation(DEFINITION_MEMBER_INDENT);
        writer.write_all(
            format!("{indentation}{}", value_variant_name_def(variant.name())).as_bytes(),
        )?;

        if let Some(body) = variant.body() {
            if options.generate_member_bodies() {
                writer.write_all(format!(" {}\n", keyword("is")).as_bytes())?;
                if body.has_annotations() {
                    self.write_annotations(
                        body.annotations(),
                        writer,
                        MEMBER_ANNOTATION_INDENT,
                        options,
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

    fn write_event(
        &mut self,
        defn: &EventDef,
        writer: &mut dyn Write,
        options: &SourceGenerationLevel,
    ) -> Result<(), Error> {
        let indentation = options.indentation(MODULE_DEFINITION_INDENT);
        writer.write_all(
            format!(
                "{indentation}{} {} {} {}",
                keyword("event"),
                type_name_def(defn.name()),
                keyword("source"),
                type_name_ref(defn.event_source())
            )
            .as_bytes(),
        )?;

        if let Some(body) = defn.body() {
            if options.generate_definition_bodies() {
                writer.write_all(format!(" {}\n", keyword("is")).as_bytes())?;
                if body.has_annotations() {
                    self.write_annotations(
                        body.annotations(),
                        writer,
                        DEFINITION_ANNOTATION_INDENT,
                        options,
                    )?;
                }
                for member in body.members() {
                    self.write_member(member, writer, options)?;
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

    fn write_member(
        &mut self,
        defn: &Member,
        writer: &mut dyn Write,
        options: &SourceGenerationLevel,
    ) -> Result<(), Error> {
        let indentation = options.indentation(DEFINITION_MEMBER_INDENT);
        writer.write_all(format!("{indentation}{} ", member_name(defn.name())).as_bytes())?;
        if let Some(role_ref) = defn.as_property_reference() {
            writer.write_all(
                format!("{} {}\n", keyword("as"), member_name(role_ref.member())).as_bytes(),
            )?;
        } else if let Some(defn) = defn.as_definition() {
            if let Some(inverse_name) = defn.inverse_name() {
                writer.write_all(
                    format!(
                        "{}{}{} ",
                        paren_start(),
                        member_name(inverse_name),
                        paren_end()
                    )
                    .as_bytes(),
                )?;
            }
            writer.write_all(format!("{} ", operator("->")).as_bytes())?;
            if *defn.target_cardinality() != DEFAULT_CARDINALITY {
                self.write_cardinality(defn.target_cardinality(), writer, options)?;
            }
            self.write_type_reference(defn.target_type(), writer, options)?;
            if let Some(body) = defn.body() {
                if options.generate_member_bodies() {
                    self.write_annotations(
                        body.annotations(),
                        writer,
                        DEFINITION_ANNOTATION_INDENT,
                        options,
                    )?;
                }
            }
        } else {
            unreachable!()
        }
        Ok(())
    }

    fn write_cardinality(
        &mut self,
        defn: &Cardinality,
        writer: &mut dyn Write,
        _options: &SourceGenerationLevel,
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
        options: &SourceGenerationLevel,
    ) -> Result<(), Error> {
        match defn {
            TypeReference::Unknown => {
                writer.write_all(keyword("unknown").as_bytes())?;
            }
            TypeReference::Type(name_ref) => {
                writer.write_all(type_name_ref(name_ref).as_bytes())?;
            }
            TypeReference::FeatureSet(name_ref) => {
                writer.write_all(
                    format!("{} {}", keyword("features"), type_name_ref(name_ref)).as_bytes(),
                )?;
            }
            TypeReference::MappingType(map_ref) => {
                writer.write_all(paren_start().as_bytes())?;
                self.write_type_reference(map_ref.domain(), writer, options)?;
                writer.write_all(operator("->").as_bytes())?;
                self.write_type_reference(map_ref.range(), writer, options)?;
                writer.write_all(paren_end().as_bytes())?;
            }
        }
        writer.write_all(EOL)?;
        Ok(())
    }

    fn write_property(
        &mut self,
        defn: &PropertyDef,
        writer: &mut dyn Write,
        options: &SourceGenerationLevel,
    ) -> Result<(), Error> {
        let indentation = options.indentation(MODULE_DEFINITION_INDENT);
        writer.write_all(
            format!(
                "{indentation}{} {}",
                keyword("property"),
                type_name_def(defn.name())
            )
            .as_bytes(),
        )?;

        if let Some(body) = defn.body() {
            if options.generate_definition_bodies() {
                writer.write_all(b" is\n")?;
                if body.has_annotations() {
                    self.write_annotations(
                        body.annotations(),
                        writer,
                        DEFINITION_ANNOTATION_INDENT,
                        options,
                    )?;
                }
                // TODO: roles
                writer.write_all(format!("\n{indentation}{}\n", keyword("end")).as_bytes())?;
            } else {
                writer.write_all(ELIPPSIS.as_bytes())?;
            }
        } else {
            writer.write_all(EOL)?;
        }

        Ok(())
    }

    fn write_rdf(
        &mut self,
        defn: &RdfDef,
        writer: &mut dyn Write,
        options: &SourceGenerationLevel,
    ) -> Result<(), Error> {
        let indentation = options.indentation(MODULE_DEFINITION_INDENT);
        writer.write_all(
            format!(
                "{indentation}{} {}",
                keyword("rdf"),
                type_name_def(defn.name())
            )
            .as_bytes(),
        )?;

        if options.generate_definition_bodies() {
            let body = defn.body();
            writer.write_all(format!(" {}\n", keyword("is")).as_bytes())?;
            if body.has_annotations() {
                self.write_annotations(
                    body.annotations(),
                    writer,
                    DEFINITION_ANNOTATION_INDENT,
                    options,
                )?;
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
        options: &SourceGenerationLevel,
    ) -> Result<(), Error> {
        let indentation = options.indentation(MODULE_DEFINITION_INDENT);
        writer.write_all(
            format!(
                "{indentation}{} {}",
                keyword("structure"),
                type_name_def(defn.name())
            )
            .as_bytes(),
        )?;

        if let Some(body) = defn.body() {
            if options.generate_definition_bodies() {
                writer.write_all(format!(" {}\n", keyword("is")).as_bytes())?;
                if body.has_annotations() {
                    self.write_annotations(
                        body.annotations(),
                        writer,
                        DEFINITION_ANNOTATION_INDENT,
                        options,
                    )?;
                }
                for member in body.members() {
                    self.write_member(member, writer, options)?;
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

    fn write_union(
        &mut self,
        defn: &UnionDef,
        writer: &mut dyn Write,
        options: &SourceGenerationLevel,
    ) -> Result<(), Error> {
        let indentation = options.indentation(MODULE_DEFINITION_INDENT);
        writer.write_all(
            format!(
                "{indentation}{} {}",
                keyword("union"),
                type_name_def(defn.name())
            )
            .as_bytes(),
        )?;

        if let Some(body) = defn.body() {
            if options.generate_definition_bodies() {
                writer.write_all(format!(" {}\n", keyword("of")).as_bytes())?;
                if body.has_annotations() {
                    self.write_annotations(
                        body.annotations(),
                        writer,
                        DEFINITION_ANNOTATION_INDENT,
                        options,
                    )?;
                }
                if body.has_variants() {
                    for variant in body.variants() {
                        self.write_type_variant(variant, writer, options)?;
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
        options: &SourceGenerationLevel,
    ) -> Result<(), Error> {
        let indentation = options.indentation(DEFINITION_MEMBER_INDENT);

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
            if options.generate_variant_bodies() {
                writer.write_all(format!(" {}\n", keyword("is")).as_bytes())?;
                if body.has_annotations() {
                    self.write_annotations(
                        body.annotations(),
                        writer,
                        MEMBER_ANNOTATION_INDENT,
                        options,
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
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
