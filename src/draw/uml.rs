/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::draw::OutputFormat;
use crate::error::Error;
use crate::model::{Module, TypeDefinition, DatatypeDef, EntityDef, EnumDef, EventDef, StructureDef, UnionDef, Identifier, ByValueMember, TypeReference, IdentityMember, ByReferenceMember, EntityMember};
use std::io::Write;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub fn write_uml_diagram<W: Write>(
    module: &Module,
    w: &mut W,
    _format: OutputFormat,
) -> Result<(), Error> {
    w.write_all(b"@startuml\n")?;
    w.write_all(b"skin rose\n")?;
    w.write_all(b"hide methods\n")?;
    w.write_all(b"hide circle\n")?;
    w.write_all(b"show << datatype >> circle\n")?;
    w.write_all(b"show << event >> circle\n")?;
    w.write_all(b"show << union >> circle\n\n")?;

    w.write_all(format!("title Module {}\n\n", module.name()).as_bytes())?;

    for other in module.imported_modules() {
        w.write_all(format!("package \"{}\" as s_{} {{\n", other, other).as_bytes())?;
        for imported in module.imported_types().iter().filter(|qi|qi.module() == other) {
            w.write_all(format!("  class \"{}\" as s_{}\n", imported.member(), imported).as_bytes())?;
        }
        w.write_all(b"}\n")?;
        w.write_all(format!("s_{} ..> s_{}\n\n", module.name(), other).as_bytes())?;
    }

    w.write_all(format!("package \"{}\" as s_{} {{\n", module.name(), module.name()).as_bytes())?;

    for definition in module.body().definitions() {
        match &definition {
            TypeDefinition::Datatype(v) => write_uml_datatype(v, w)?,
            TypeDefinition::Entity(v) => write_uml_entity(v, w)?,
            TypeDefinition::Enum(v) => write_uml_enum(v, w)?,
            TypeDefinition::Event(v) => write_uml_event(v, w)?,
            TypeDefinition::Structure(v) => write_uml_structure(v, w)?,
            TypeDefinition::Union(v) => write_uml_union(v, w)?,
        }
    }

    w.write_all(b"}\n")?;
    w.write_all(b"@enduml\n")?;

    Ok(())
}

write_to_string!(to_uml_diagram_string, write_uml_diagram, OutputFormat);

write_to_file!(uml_diagram_to_file, write_uml_diagram, OutputFormat);

print_to_stdout!(print_uml_diagram, write_uml_diagram, OutputFormat);

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

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn write_start_type<W: Write>(
    type_class: &str,
    type_name: &Identifier,
    w: &mut W,
) -> Result<(), Error> {
    w.write_all(format!(
        "  {} \"{}\" as s_{} {{\n",
        type_class, type_name, type_name).as_bytes())?;
    Ok(())
}

fn write_start_type_with_sterotype<W: Write>(
    type_class: &str,
    type_name: &Identifier,
    stereo_name: &str,
    w: &mut W,
) -> Result<(), Error> {
    w.write_all(format!(
        "  {} \"{}\" as s_{} << (D, orchid) {} >> {{\n",
        type_class, type_name, type_name, stereo_name).as_bytes())?;
    Ok(())
}

fn write_end_type<W: Write>(
    type_name: &Identifier,
    has_body: bool,
    w: &mut W,
) -> Result<(), Error> {
    if !has_body {
        w.write_all(format!(
            "  }}\n  hide s_{} members\n\n",
            type_name).as_bytes())?;
    } else {
        w.write_all(b"  }\n\n")?;
    }
    Ok(())
}

fn write_uml_datatype<W: Write>(
    me: &DatatypeDef,
    w: &mut W,
) -> Result<(), Error> {
    let name = me.name();
    write_start_type_with_sterotype("class", name, "datatype", w)?;
    write_end_type(name, me.has_body(), w)?;
    w.write_all(format!(
        "  s_{} ..|> s_{}\n",
        me.name(), me.base_type()).as_bytes())?;

    Ok(())
}

fn write_uml_entity<W: Write>(
    me: &EntityDef,
    w: &mut W,
) -> Result<(), Error> {
    let name = me.name();
    write_start_type("entity", name, w)?;
    if let Some(body) = me.body() {
        w.write_all(b"    __identity__\n")?;
        write_identity_member(body.identity(), w)?;
        if body.has_members() {
            w.write_all(b"    ..\n")?;
        }
        for member in body.members() {
            match member {
                EntityMember::ByValue(member) => write_by_value_member(member, w)?,
                EntityMember::ByReference(member) => write_by_reference_member(member, w)?,
            }
        }
        for group in body.groups() {
            w.write_all(b"    --\n")?;
            for member in group.members() {
                match member {
                    EntityMember::ByValue(member) => write_by_value_member(member, w)?,
                    EntityMember::ByReference(member) => write_by_reference_member(member, w)?,
                }
            }
        }
    }
    write_end_type(name, me.has_body(), w)?;

    Ok(())
}

fn write_uml_enum<W: Write>(
    me: &EnumDef,
    w: &mut W,
) -> Result<(), Error> {
    let name = me.name();
    write_start_type("enum", name, w)?;
    if let Some(body) = me.body() {
        for variant in body.variants() {
            w.write_all(format!("    {} = {}\n", variant.name(), variant.value()).as_bytes())?;
        }
    }
    write_end_type(name, me.has_body(), w)?;

    Ok(())
}

fn write_uml_event<W: Write>(
    me: &EventDef,
    w: &mut W,
) -> Result<(), Error> {
    let name = me.name();
    write_start_type_with_sterotype("class", name, "event", w)?;
    if let Some(body) = me.body() {
        for member in body.members() {
            write_by_value_member(member, w)?;
        }
        for group in body.groups() {
            w.write_all(b"    --\n")?;
            for member in group.members() {
                write_by_value_member(member, w)?;
            }
        }
    }
    write_end_type(name, me.has_body(), w)?;
    w.write_all(format!(
        "  s_{} o--> s_{}\n",
        me.name(), me.event_source()).as_bytes())?;

    Ok(())
}

fn write_uml_structure<W: Write>(
    me: &StructureDef,
    w: &mut W,
) -> Result<(), Error> {
    let name = me.name();
    write_start_type("class", name, w)?;
    if let Some(body) = me.body() {
        for member in body.members() {
            write_by_value_member(member, w)?;
        }
        for group in body.groups() {
            w.write_all(b"  --\n")?;
            for member in group.members() {
                write_by_value_member(member, w)?;
            }
        }
    }
    write_end_type(name, me.has_body(), w)?;

    Ok(())
}

fn write_identity_member<W: Write>(
    me: &IdentityMember,
    w: &mut W,
) -> Result<(), Error> {
    w.write_all(format!(
        "    {}",
        me.name()).as_bytes())?;
    if let TypeReference::Reference(target_type) = me.target_type() {
        w.write_all(format!(
            ": {}\n",
            target_type).as_bytes())?;
    } else {
        w.write_all(b"\n")?;
    }
    Ok(())
}

fn write_by_value_member<W: Write>(
    me: &ByValueMember,
    w: &mut W,
) -> Result<(), Error> {
    w.write_all(format!(
        "    {}",
        me.name()).as_bytes())?;
    if let TypeReference::Reference(target_type) = me.target_type() {
        w.write_all(format!(
            ": {}\n",
            target_type).as_bytes())?;
    } else {
        w.write_all(b"\n")?;
    }
    Ok(())
}

fn write_by_reference_member<W: Write>(
    me: &ByReferenceMember,
    w: &mut W,
) -> Result<(), Error> {
    w.write_all(format!(
        "    {}",
        me.name()).as_bytes())?;
    if let TypeReference::Reference(target_type) = me.target_type() {
        w.write_all(format!(
            ": {}\n",
            target_type).as_bytes())?;
    } else {
        w.write_all(b"\n")?;
    }
    Ok(())
}

fn write_uml_union<W: Write>(
    me: &UnionDef,
    w: &mut W,
) -> Result<(), Error> {
    let name = me.name();
    write_start_type_with_sterotype("enum", name, "union", w)?;
    if let Some(body) = me.body() {
        for variant in body.variants() {
            if let Some(rename) = variant.rename() {
                w.write_all(format!("    {} ({})\n", rename, variant.name()).as_bytes())?;
            } else {
                w.write_all(format!("    {}\n", variant.name()).as_bytes())?;
            }
        }
    }
    write_end_type(name, me.has_body(), w)?;

    Ok(())
}


// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
