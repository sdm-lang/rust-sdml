/*!
This module provides an generator that recreates the surface syntax for a module given its
in-memory representation.

# Example

```rust
use sdml_core::generate::source::SourceGenerator;
use sdml_core::generate::GenerateToWriter;
use sdml_core::model::identifiers::Identifier;
use sdml_core::model::modules::Module;

let module = Module::empty(Identifier::new_unchecked("example"));

let mut generator: SourceGenerator = Default::default();

let source = generator.write_to_string(&module, None).unwrap();
assert_eq!(source.as_str(), "module example is end\n");

```
*/

use crate::error::Error;
use crate::generate::GenerateToWriter;
use crate::load::ModuleLoader;
use crate::model::annotations::{Annotation, AnnotationProperty, HasAnnotations};
use crate::model::constraints::{Constraint, ConstraintBody};
use crate::model::definitions::{
    DatatypeDef, Definition, EntityDef, EnumDef, EventDef, HasVariants, PropertyDef, StructureDef,
    TypeVariant, UnionDef, ValueVariant,
};
use crate::model::modules::{Module, ModuleBody};
use crate::model::{HasBody, HasName, HasNameReference, HasOptionalBody};
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

/// Options that control the generation style or layout.
#[derive(Debug)]
pub struct SourceOptions {
    indent_spaces: usize,
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

const MODULE_ANNOTATION_INDENT: usize = 1;
const MODULE_IMPORT_INDENT: usize = 1;
const MODULE_DEFINITION_INDENT: usize = 1;
const DEFINITION_ANNOTATION_INDENT: usize = 2;
const DEFINITION_MEMBER_INDENT: usize = 2;
const MEMBER_ANNOTATION_INDENT: usize = 3;
//const GROUP_ANNOTATION_INDENT: usize = 4;
//const GROUP_MEMBER_INDENT: usize = 4;
//const GROUP_MEMBER_ANNOTATION_INDENT: usize = 5;

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for SourceOptions {
    fn default() -> Self {
        Self { indent_spaces: 2 }
    }
}

impl SourceOptions {
    pub fn indent_spaces(self, value: usize) -> Self {
        let mut self_mut = self;
        self_mut.indent_spaces = value;
        self_mut
    }

    fn indentation(&self, level: usize) -> String {
        let n = level * self.indent_spaces;
        format!("{:n$}", "")
    }
}

// ------------------------------------------------------------------------------------------------

impl GenerateToWriter<SourceOptions> for SourceGenerator {
    fn write_in_format(
        &mut self,
        module: &Module,
        _: Option<&mut dyn ModuleLoader>,
        writer: &mut dyn Write,
        options: SourceOptions,
    ) -> Result<(), Error> {
        writer.write_all(format!("module {} ", module.name()).as_bytes())?;

        if let Some(base) = module.base() {
            writer.write_all(format!("base <{base}> ").as_bytes())?;
        }

        let body = module.body();
        if body.has_imports() || body.has_annotations() || body.has_definitions() {
            writer.write_all(b"is\n")?;

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

            writer.write_all(b"end\n")?;
        } else {
            writer.write_all(b"is end\n")?;
        }
        Ok(())
    }
}

impl SourceGenerator {
    fn write_module_imports(
        &mut self,
        module_body: &ModuleBody,
        writer: &mut dyn Write,
        options: &SourceOptions,
    ) -> Result<(), Error> {
        let indentation = options.indentation(MODULE_IMPORT_INDENT);
        if module_body.has_imports() {
            writer.write_all(b"\n")?;
            for import_statement in module_body.imports() {
                let imported = if import_statement.imports_len() == 1 {
                    import_statement
                        .imports()
                        .map(|i| i.to_string())
                        .collect::<Vec<String>>()
                        .join("")
                } else {
                    format!(
                        "[ {} ]",
                        import_statement
                            .imports()
                            .map(|i| i.to_string())
                            .collect::<Vec<String>>()
                            .join("")
                    )
                };
                writer.write_all(format!("{indentation}import {imported}\n").as_bytes())?;
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
        options: &SourceOptions,
    ) -> Result<(), Error> {
        writer.write_all(b"\n")?;

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
        options: &SourceOptions,
    ) -> Result<(), Error> {
        let indentation = options.indentation(indent_level);
        // TODO: ensure wrapping
        writer.write_all(
            format!(
                "{indentation}@{} = {}",
                annotation.name_reference(),
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
        options: &SourceOptions,
    ) -> Result<(), Error> {
        let indentation = options.indentation(indent_level);
        writer.write_all(format!("{indentation}assert ").as_bytes())?;
        writer.write_all(format!("{} ", constraint.name()).as_bytes())?;
        match constraint.body() {
            ConstraintBody::Informal(v) => {
                writer.write_all(format!("= {v:?}").as_bytes())?;
            }
            ConstraintBody::Formal(_) => {
                writer.write_all(b"is\n")?;
                // TODO: add constraint sentence
                writer.write_all(format!("{indentation}end\n").as_bytes())?;
            }
        }
        Ok(())
    }

    fn write_module_definitions(
        &mut self,
        module_body: &ModuleBody,
        writer: &mut dyn Write,
        options: &SourceOptions,
    ) -> Result<(), Error> {
        let _indentation = options.indentation(1);
        if module_body.has_definitions() {
            writer.write_all(b"\n")?;
            for definition in module_body.definitions() {
                match &definition {
                    Definition::Datatype(v) => self.write_datatype(v, writer, options)?,
                    Definition::Entity(v) => self.write_entity(v, writer, options)?,
                    Definition::Enum(v) => self.write_enum(v, writer, options)?,
                    Definition::Event(v) => self.write_event(v, writer, options)?,
                    Definition::Property(v) => self.write_property(v, writer, options)?,
                    Definition::Structure(v) => self.write_structure(v, writer, options)?,
                    Definition::TypeClass(_) => todo!(),
                    Definition::Union(v) => self.write_union(v, writer, options)?,
                }
            }
        }
        todo!()
    }

    fn write_datatype(
        &mut self,
        defn: &DatatypeDef,
        writer: &mut dyn Write,
        options: &SourceOptions,
    ) -> Result<(), Error> {
        let indentation = options.indentation(MODULE_DEFINITION_INDENT);
        writer.write_all(
            format!(
                "{indentation}datatype {} <- {}",
                defn.name(),
                defn.base_type()
            )
            .as_bytes(),
        )?;

        if let Some(body) = defn.body() {
            writer.write_all(b" is\n")?;
            if body.has_annotations() {
                self.write_annotations(
                    body.annotations(),
                    writer,
                    DEFINITION_ANNOTATION_INDENT,
                    options,
                )?;
            }
            writer.write_all(format!("{indentation}end\n").as_bytes())?;
        } else {
            writer.write_all(b"\n")?;
        }

        Ok(())
    }

    fn write_entity(
        &mut self,
        defn: &EntityDef,
        writer: &mut dyn Write,
        options: &SourceOptions,
    ) -> Result<(), Error> {
        let indentation = options.indentation(MODULE_DEFINITION_INDENT);
        writer.write_all(format!("{indentation}entity {}", defn.name()).as_bytes())?;

        if let Some(body) = defn.body() {
            writer.write_all(b" is\n")?;
            if body.has_annotations() {
                self.write_annotations(
                    body.annotations(),
                    writer,
                    DEFINITION_ANNOTATION_INDENT,
                    options,
                )?;
            }
            // TODO: members
            // TODO: groups
            writer.write_all(format!("{indentation}end\n").as_bytes())?;
        } else {
            writer.write_all(b"\n")?;
        }

        Ok(())
    }

    fn write_enum(
        &mut self,
        defn: &EnumDef,
        writer: &mut dyn Write,
        options: &SourceOptions,
    ) -> Result<(), Error> {
        let indentation = options.indentation(MODULE_DEFINITION_INDENT);
        writer.write_all(format!("{indentation}enum {}", defn.name()).as_bytes())?;

        if let Some(body) = defn.body() {
            writer.write_all(b" of\n")?;
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
            writer.write_all(format!("{indentation}end\n").as_bytes())?;
        } else {
            writer.write_all(b"\n")?;
        }

        Ok(())
    }

    fn write_value_variant(
        &mut self,
        variant: &ValueVariant,
        writer: &mut dyn Write,
        options: &SourceOptions,
    ) -> Result<(), Error> {
        let indentation = options.indentation(DEFINITION_MEMBER_INDENT);
        writer.write_all(format!("{indentation}{}", variant.name()).as_bytes())?;

        if let Some(body) = variant.body() {
            writer.write_all(b" is\n")?;
            if body.has_annotations() {
                self.write_annotations(
                    body.annotations(),
                    writer,
                    MEMBER_ANNOTATION_INDENT,
                    options,
                )?;
            }
            writer.write_all(format!("{indentation}end\n").as_bytes())?;
        } else {
            writer.write_all(b"\n")?;
        }

        Ok(())
    }

    fn write_event(
        &mut self,
        defn: &EventDef,
        writer: &mut dyn Write,
        options: &SourceOptions,
    ) -> Result<(), Error> {
        let indentation = options.indentation(MODULE_DEFINITION_INDENT);
        writer.write_all(
            format!(
                "{indentation}event {} source {}",
                defn.name(),
                defn.event_source()
            )
            .as_bytes(),
        )?;

        if let Some(body) = defn.body() {
            writer.write_all(b" is\n")?;
            if body.has_annotations() {
                self.write_annotations(
                    body.annotations(),
                    writer,
                    DEFINITION_ANNOTATION_INDENT,
                    options,
                )?;
            }
            // TODO: members
            // TODO: groups
            writer.write_all(format!("{indentation}end\n").as_bytes())?;
        } else {
            writer.write_all(b"\n")?;
        }

        Ok(())
    }

    fn write_property(
        &mut self,
        defn: &PropertyDef,
        writer: &mut dyn Write,
        options: &SourceOptions,
    ) -> Result<(), Error> {
        let indentation = options.indentation(MODULE_DEFINITION_INDENT);
        writer.write_all(format!("{indentation}property {}", defn.name()).as_bytes())?;

        if let Some(body) = defn.body() {
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
            writer.write_all(format!("{indentation}end\n").as_bytes())?;
        } else {
            writer.write_all(b"\n")?;
        }

        Ok(())
    }

    fn write_structure(
        &mut self,
        defn: &StructureDef,
        writer: &mut dyn Write,
        options: &SourceOptions,
    ) -> Result<(), Error> {
        let indentation = options.indentation(MODULE_DEFINITION_INDENT);
        writer.write_all(format!("{indentation}structure {}", defn.name()).as_bytes())?;

        if let Some(body) = defn.body() {
            writer.write_all(b" is\n")?;
            if body.has_annotations() {
                self.write_annotations(
                    body.annotations(),
                    writer,
                    DEFINITION_ANNOTATION_INDENT,
                    options,
                )?;
            }
            // TODO: members
            // TODO: groups
            writer.write_all(format!("{indentation}end\n").as_bytes())?;
        } else {
            writer.write_all(b"\n")?;
        }

        Ok(())
    }

    fn write_union(
        &mut self,
        defn: &UnionDef,
        writer: &mut dyn Write,
        options: &SourceOptions,
    ) -> Result<(), Error> {
        let indentation = options.indentation(MODULE_DEFINITION_INDENT);
        writer.write_all(format!("{indentation}union {}", defn.name()).as_bytes())?;

        if let Some(body) = defn.body() {
            writer.write_all(b" of\n")?;
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
                    self.write_type_variant(variant, writer, options)?;
                }
            }
            writer.write_all(format!("{indentation}end\n").as_bytes())?;
        } else {
            writer.write_all(b"\n")?;
        }

        Ok(())
    }

    fn write_type_variant(
        &mut self,
        variant: &TypeVariant,
        writer: &mut dyn Write,
        options: &SourceOptions,
    ) -> Result<(), Error> {
        let indentation = options.indentation(DEFINITION_MEMBER_INDENT);
        writer.write_all(format!("{indentation}{}", variant.name_reference()).as_bytes())?;

        if let Some(rename) = variant.rename() {
            writer.write_all(format!(" as {rename}").as_bytes())?;
        }

        if let Some(body) = variant.body() {
            writer.write_all(b" is\n")?;
            if body.has_annotations() {
                self.write_annotations(
                    body.annotations(),
                    writer,
                    MEMBER_ANNOTATION_INDENT,
                    options,
                )?;
            }
            writer.write_all(format!("{indentation}end\n").as_bytes())?;
        } else {
            writer.write_all(b"\n")?;
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

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::generate::source::SourceGenerator;
    use crate::generate::GenerateToWriter;
    use crate::model::identifiers::Identifier;
    use crate::model::modules::Module;
    use pretty_assertions::assert_eq;
    use url::Url;

    #[test]
    fn test_generate_module_empty() {
        let module = Module::empty(Identifier::new_unchecked("example"));
        let mut generator: SourceGenerator = Default::default();
        let source = generator.write_to_string(&module, None).unwrap();
        assert_eq!(source.as_str(), "module example is end\n");
    }

    #[test]
    fn test_generate_module_empty_with_base() {
        let module = Module::empty(Identifier::new_unchecked("example"))
            .with_base(Url::parse("http://example.com").unwrap());
        let mut generator: SourceGenerator = Default::default();
        let source = generator.write_to_string(&module, None).unwrap();
        assert_eq!(
            source.as_str(),
            "module example base <http://example.com/> is end\n"
        );
    }
}
