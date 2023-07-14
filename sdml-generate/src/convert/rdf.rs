/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use sdml_core::error::Error;
use sdml_core::model::{
    Annotation, AnnotationProperty, Constraint, DatatypeDef, Definition, EntityDef, EnumDef,
    EventDef, IdentifierReference, Import, Module, PropertyDef, StructureDef, UnionDef,
};
use std::io::Write;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct RdfModelGenerator {}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn write_as_rdf<W: Write>(module: &Module, w: &mut W) -> Result<(), Error> {
    if let Some(base) = module.base() {
        w.write_all(format!(r#"@base <{base}> .\n@prefix : <{base}> .\n"#).as_bytes())?;
    }

    let name = module.name();
    w.write_all(
        r#"@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix sdml: <https://sdml.io/sdml-owl.ttl#> .
"#
        .as_bytes(),
    )?;

    let body = module.body();

    for statement in body.imports() {
        for import in statement.imports() {
            let name = match import {
                Import::Module(v) => v,
                Import::Member(v) => v.module(),
            };
            w.write_all(format!("@prefix {name}: <> .\n").as_bytes())?;
        }
    }

    w.write_all(b": rdf:type owl:Ontology, sdml:Module ;\n")?;

    for statement in body.imports() {
        for import in statement.imports() {
            let _name = match import {
                Import::Module(v) => v,
                Import::Member(v) => v.module(),
            };
            w.write_all(format!("    owl:imports <{}> .\n", "").as_bytes())?;
            todo!();
        }
    }

    for annotation in body.annotations() {
        match &annotation {
            Annotation::Property(v) => write_annotation_property(v, w)?,
            Annotation::Constraint(v) => write_constraint(v, w)?,
        }
    }

    w.write_all(format!("    sdml:srcLabel \"{name}\" .\n").as_bytes())?;

    for definition in body.definitions() {
        match &definition {
            Definition::Datatype(v) => write_datatype(v, w)?,
            Definition::Entity(v) => write_entity(v, w)?,
            Definition::Enum(v) => write_enumeration(v, w)?,
            Definition::Event(v) => write_event(v, w)?,
            Definition::Structure(v) => write_structure(v, w)?,
            Definition::Union(v) => write_union(v, w)?,
            Definition::Property(v) => write_property(v, w)?,
        }
    }

    Ok(())
}

write_to_string!(to_rdf_string, write_as_rdf);

write_to_file!(to_rdf_file, write_as_rdf);

print_to_stdout!(print_rdf, write_as_rdf);

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn write_datatype<W: Write>(me: &DatatypeDef, w: &mut W) -> Result<(), Error> {
    let name = me.name();
    let base_type = me.base_type();

    w.write_all(
        format!(":{name} rdf:type rdfs:Datatype ;\n    owl:onDatatype {base_type} ;\n").as_bytes(),
    )?;

    // TODO: restrictions

    // owl:withRestrictions (
    //     [ xsd:minLength "5"^^xsd:nonNegativeInteger ]
    //     [ xsd:maxLength "25"^^xsd:nonNegativeInteger ]
    // ) ;

    // TODO: other annotations

    w.write_all(format!("    sdml:srcLabel \"{name}\" ;\n    rdfs:isDefinedBy : .\n").as_bytes())?;

    Ok(())
}

fn write_entity<W: Write>(me: &EntityDef, w: &mut W) -> Result<(), Error> {
    let name = me.name();

    w.write_all(format!(":{name} rdf:type owl:Class, sdml:Entity ;\n").as_bytes())?;

    // TODO: annotations

    // TODO: members and groups

    w.write_all(format!("    sdml:srcLabel \"{name}\" ;\n    rdfs:isDefinedBy : .\n").as_bytes())?;

    Ok(())
}

fn write_enumeration<W: Write>(me: &EnumDef, w: &mut W) -> Result<(), Error> {
    let name = me.name();

    w.write_all(format!(":{name} rdf:type owl:Class, sdml:Enumeration ;\n").as_bytes())?;

    // TODO: annotations

    // TODO: value variants

    w.write_all(format!("    sdml:srcLabel \"{name}\" ;\n    rdfs:isDefinedBy : .\n").as_bytes())?;

    Ok(())
}

fn write_event<W: Write>(me: &EventDef, w: &mut W) -> Result<(), Error> {
    let name = me.name();

    w.write_all(format!(":{name} rdf:type owl:Class, sdml:Event ;\n").as_bytes())?;

    // TODO: annotations

    // TODO: members and groups

    w.write_all(format!("    sdml:srcLabel \"{name}\" ;\n    rdfs:isDefinedBy : .\n").as_bytes())?;

    Ok(())
}

fn write_structure<W: Write>(me: &StructureDef, w: &mut W) -> Result<(), Error> {
    let name = me.name();

    w.write_all(format!(":{name} rdf:type owl:Class, sdml:Structure ;\n").as_bytes())?;

    // TODO: annotations

    // TODO: members and groups

    w.write_all(format!("    sdml:srcLabel \"{name}\" ;\n    rdfs:isDefinedBy : .\n").as_bytes())?;

    Ok(())
}

fn write_union<W: Write>(me: &UnionDef, w: &mut W) -> Result<(), Error> {
    let name = me.name();

    w.write_all(format!(":{name} rdf:type owl:Class, sdml:Union ;\n").as_bytes())?;

    // TODO: annotations

    // TODO: type variants

    w.write_all(format!("    sdml:srcLabel \"{name}\" ;\n    rdfs:isDefinedBy : .\n").as_bytes())?;

    Ok(())
}

fn write_property<W: Write>(me: &PropertyDef, w: &mut W) -> Result<(), Error> {
    let name = me.name();

    w.write_all(format!(":{name} rdf:type owl:Class, sdml:Property ;\n").as_bytes())?;

    // TODO: annotations

    // TODO: roles

    w.write_all(format!("    sdml:srcLabel \"{name}\" ;\n    rdfs:isDefinedBy : .\n").as_bytes())?;

    Ok(())
}

fn write_annotation_property<W: Write>(me: &AnnotationProperty, w: &mut W) -> Result<(), Error> {
    let _name = me.name();
    let value = me.value();

    let (prefix, name) = match me.name() {
        IdentifierReference::Identifier(name) => (":", name.to_string()),
        IdentifierReference::QualifiedIdentifier(name) => ("", name.to_string()),
    };
    w.write_all(format!("    {prefix}{name} {value} ;\n").as_bytes())?;
    todo!();
}

fn write_constraint<W: Write>(_me: &Constraint, _w: &mut W) -> Result<(), Error> {
    todo!();
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
