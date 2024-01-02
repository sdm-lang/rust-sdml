/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::{GenerateToWriter, NoFormatOptions};
use sdml_core::error::Error;
use sdml_core::model::annotations::Annotation;
use sdml_core::model::annotations::HasAnnotations;
use sdml_core::model::definitions::{
    DatatypeDef, Definition, EntityDef, EnumDef, EventDef, HasVariants, PropertyDef, StructureDef,
    TypeClassDef, UnionDef,
};
use sdml_core::model::identifiers::Identifier;
use sdml_core::model::modules::Module;
use sdml_core::model::{HasBody, HasName, HasNameReference, HasOptionalBody};
use std::io::Write;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct OrgFileGenerator<'a> {
    source: Option<&'a str>,
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

impl GenerateToWriter<NoFormatOptions> for OrgFileGenerator<'_> {
    fn write_in_format(
        &mut self,
        module: &Module,
        writer: &mut dyn Write,
        _: NoFormatOptions,
    ) -> Result<(), Error> {
        let name = module.name();
        writer.write_all(
            format!(
                r#"#+TITLE: Module {name}
#+LANGUAGE: en
#+STARTUP: overview hidestars inlineimages entitiespretty
#+SETUPFILE: https://fniessen.github.io/org-html-themes/org/theme-readtheorg.setup
#+HTML_HEAD: <style>img {{ max-width: 800px; height: auto; }}</style>
#+HTML_HEAD: <style>div.figure {{ text-align: center; }}</style>
#+OPTIONS: toc:3

"#
            )
            .as_bytes(),
        )?;

        // TODO: add description

        if let Some(source) = self.source {
            writer.write_all(
                format!(
                    r#"#+NAME: lst:module-input-source
#+BEGIN_SRC sdml :cmdline draw --diagram uml-class --output-format svg :file ./fig-{name}-module-uml.svg :exports results :noweb yes
{source}
#+END_SRC

#+NAME: fig:module-input-source
#+CAPTION: UML Class Diagram
#+RESULTS: lst:module-input-source
[[file:./fig-{name}-module-uml.svg]]

"#)
                    .as_bytes(),
            )?;
        }

        let module_body = module.body();

        writer.write_all(
            b"* Dependencies

#+NAME: tbl:imported-modules
#+CAPTION: Imported Modules
| Name | Base IRI |
|------+----------|
",
        )?;

        let mut imported_modules: Vec<&Identifier> =
            module_body.imported_modules().into_iter().collect();
        imported_modules.sort();

        for import in imported_modules {
            // TODO: look up in cache
            writer.write_all(format!("| ~{}~ | |\n", import).as_bytes())?;
        }

        writer.write_all(b"\n")?;

        if module_body.has_annotation_properties() {
            write_annotations(module_body.annotations(), writer)?;
        }

        writer.write_all(b"* Definitions\n\n")?;
        write_definitions(module_body.definitions(), writer)?;

        if self.source.is_some() {
            writer.write_all(
                b"* Appendix: Module Source

#+NAME: lst:module-output-source
#+CAPTION: Original Module Source
#+BEGIN_SRC sdml :exports code :noweb yes
<<lst:module-input-source>>
#+END_SRC
",
            )?;
        }

        Ok(())
    }
}

impl<'a> OrgFileGenerator<'a> {
    pub fn without_source() -> Self {
        Self { source: None }
    }

    pub fn with_source(source: &'a str) -> Self {
        Self {
            source: Some(source),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn write_annotations(
    _annotations: Box<dyn Iterator<Item = &Annotation> + '_>,
    writer: &mut dyn Write,
) -> Result<(), Error> {
    writer.write_all(b"* Annotation Properties\n\n")?;

    Ok(())
}

#[allow(single_use_lifetimes)]
fn write_definitions<'a>(
    definitions: impl Iterator<Item = &'a Definition>,
    writer: &mut dyn Write,
) -> Result<(), Error> {
    for definition in definitions {
        match definition {
            Definition::Datatype(v) => write_datatype(v, writer)?,
            Definition::Entity(v) => write_entity(v, writer)?,
            Definition::Enum(v) => write_enum(v, writer)?,
            Definition::Event(v) => write_event(v, writer)?,
            Definition::Property(v) => write_property(v, writer)?,
            Definition::Structure(v) => write_structure(v, writer)?,
            Definition::TypeClass(v) => write_typeclass(v, writer)?,
            Definition::Union(v) => write_union(v, writer)?,
        }
    }
    Ok(())
}

fn write_datatype(datatype: &DatatypeDef, writer: &mut dyn Write) -> Result<(), Error> {
    let name = datatype.name();
    let label = name.to_type_label();
    writer.write_all(format!("** {} Datatype\n\n", label).as_bytes())?;

    if datatype.is_opaque() {
        writer.write_all(
            format!(
                "- *{label}* is an /opaque/ data type;
constraints may only use strict equality tests between values.\n"
            )
            .as_bytes(),
        )?;
    }
    writer.write_all(
        format!(
            "- *{label}* is based on the datatype ~{}~.\n",
            datatype.base_type()
        )
        .as_bytes(),
    )?;

    // TODO: special annotation properties.

    writer.write_all(b"\n")?;

    if let Some(body) = datatype.body() {
        write_annotations(body.annotations(), writer)?;
    }

    Ok(())
}

fn write_entity(entity: &EntityDef, writer: &mut dyn Write) -> Result<(), Error> {
    let name = entity.name();
    writer.write_all(format!("** {} Entity\n\n", name.to_type_label()).as_bytes())?;

    if let Some(body) = entity.body() {
        if body.has_annotation_properties() {
            write_annotations(body.annotations(), writer)?;
        }
    }

    Ok(())
}

fn write_enum(an_enum: &EnumDef, writer: &mut dyn Write) -> Result<(), Error> {
    let name = an_enum.name();
    writer.write_all(format!("** {} Enum\n\n", name.to_type_label()).as_bytes())?;

    if let Some(body) = an_enum.body() {
        if body.has_variants() {
            writer.write_all(
                format!(
                    r#"#+NAME: tbl:enum-{name}-variants
#+CAPTION: {name} Variants
| Label    | Description |
|----------+-------------|
"#
                )
                .as_bytes(),
            )?;

            for variant in body.variants() {
                writer.write_all(format!("| ~{}~ | {} |\n", variant.name(), "").as_bytes())?;
            }

            writer.write_all(b"\n")?;
        }

        if body.has_annotation_properties() {
            write_annotations(body.annotations(), writer)?;
        }
    }

    Ok(())
}

fn write_event(event: &EventDef, writer: &mut dyn Write) -> Result<(), Error> {
    let name = event.name();
    writer.write_all(format!("** {} Event\n\n", name.to_type_label()).as_bytes())?;

    if let Some(body) = event.body() {
        if body.has_annotation_properties() {
            write_annotations(body.annotations(), writer)?;
        }
    }

    Ok(())
}

fn write_property(property: &PropertyDef, writer: &mut dyn Write) -> Result<(), Error> {
    let name = property.name();
    writer.write_all(format!("** {} Property\n\n", name.to_type_label()).as_bytes())?;

    if let Some(body) = property.body() {
        if body.has_annotation_properties() {
            write_annotations(body.annotations(), writer)?;
        }
    }

    Ok(())
}

fn write_structure(structure: &StructureDef, writer: &mut dyn Write) -> Result<(), Error> {
    let name = structure.name();
    writer.write_all(format!("** {} Structure\n\n", name.to_type_label()).as_bytes())?;

    if let Some(body) = structure.body() {
        if body.has_annotation_properties() {
            write_annotations(body.annotations(), writer)?;
        }
    }

    Ok(())
}

fn write_typeclass(typeclass: &TypeClassDef, writer: &mut dyn Write) -> Result<(), Error> {
    let name = typeclass.name();
    writer.write_all(format!("** {} Typeclass\n\n", name.to_type_label()).as_bytes())?;

    if let Some(body) = typeclass.body() {
        if body.has_annotation_properties() {
            write_annotations(body.annotations(), writer)?;
        }
    }

    Ok(())
}

fn write_union(union: &UnionDef, writer: &mut dyn Write) -> Result<(), Error> {
    let name = union.name();
    writer.write_all(format!("** {} Union\n\n", name.to_type_label()).as_bytes())?;

    if let Some(body) = union.body() {
        if body.has_variants() {
            writer.write_all(
                format!(
                    r#"#+NAME: tbl:union-{name}-variants
#+CAPTION: {name} Variants
| Label    | Type | Description |
|----------+------+-------------|
"#
                )
                .as_bytes(),
            )?;

            for variant in body.variants() {
                writer.write_all(
                    format!(
                        "| ~{}~ | ~{}~ | {} |\n",
                        if let Some(name) = variant.rename() {
                            name.to_string()
                        } else {
                            variant.name_reference().to_string()
                        },
                        variant.name_reference(),
                        ""
                    )
                    .as_bytes(),
                )?;
            }

            writer.write_all(b"\n")?;
        }

        if body.has_annotation_properties() {
            write_annotations(body.annotations(), writer)?;
        }
    }

    Ok(())
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
