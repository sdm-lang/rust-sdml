/*!
One-line description.

More detailed description, with

```text
start_module
++ import ...
++ annotation ...
++ (
++   (
++   start_datatype
++++   annotation ...
++   end_datatype
++   )
++   |
++   (
++   start_entity
++++   start_identity_member
++++++   annotation...
++++   end_member
++++   annotation ...
++++   (
++++     (
++++       start_by_value_member
++++++       annotation...
++++       end_member
++++     )
++++     |
++++     (
++++       start_by_value_member
++++++       annotation...
++++       end_member
++++     )
++++     |
++++     (
++++       start_group
++++++       annotation...
++++++       (
++++++         (
++++++           start_by_value_member
++++++++           annotation...
++++++           end_member
++++++         )
++++++         |
++++++         (
++++++           start_by_value_member
++++++++           annotation...
++++++           end_member
++++++         )
++++++       )*
++++       end_group
++++     )
++++   )*
++++   annotation ...
++   end_entity
++   )
++   | enum | event | structure
++ ) ...
end_module
```

# Example

YYYYY

*/

use crate::error::Error;
use crate::model::{
    ByReferenceMember, ByValueMember, Cardinality, DatatypeDef, EntityDef, EntityGroup,
    EntityMember, EnumDef, EventDef, Identifier, IdentifierReference, IdentityMember, Import,
    Module, Span, StructureDef, StructureGroup, TypeDefinition, TypeReference, Value,
};

use super::UnionDef;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub trait ModuleWalker {
    fn start_module(&self, _name: &Identifier, _span: Option<&Span>) -> Result<(), Error> {
        Ok(())
    }

    fn import(&self, _imported: &[Import], _span: Option<&Span>) -> Result<(), Error> {
        Ok(())
    }

    fn annotation(
        &self,
        _name: &IdentifierReference,
        _value: &Value,
        _span: Option<&Span>,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn start_datatype(
        &self,
        _name: &Identifier,
        _base_type: &IdentifierReference,
        _span: Option<&Span>,
    ) -> Result<(), Error> {
        Ok(())
    }
    // annotation
    fn end_datatype(&self, _name: &Identifier) -> Result<(), Error> {
        Ok(())
    }

    fn start_entity(&self, _name: &Identifier, _span: Option<&Span>) -> Result<(), Error> {
        Ok(())
    }
    // annotation
    fn start_identity_member(
        &self,
        _name: &Identifier,
        _target_type: &TypeReference,
        _span: Option<&Span>,
    ) -> Result<(), Error> {
        Ok(())
    }
    fn start_by_value_member(
        &self,
        _name: &Identifier,
        _target_cardinality: Option<&Cardinality>,
        _target_type: &TypeReference,
        _span: Option<&Span>,
    ) -> Result<(), Error> {
        Ok(())
    }
    fn start_by_reference_member(
        &self,
        _name: &Identifier,
        _source_cardinality: Option<&Cardinality>,
        _target_cardinality: Option<&Cardinality>,
        _target_type: &TypeReference,
        _span: Option<&Span>,
    ) -> Result<(), Error> {
        Ok(())
    }
    // annotation
    fn end_member(&self, _name: &Identifier) -> Result<(), Error> {
        Ok(())
    }
    fn end_entity(&self, _name: &Identifier) -> Result<(), Error> {
        Ok(())
    }

    fn start_enum(&self, _name: &Identifier, _span: Option<&Span>) -> Result<(), Error> {
        Ok(())
    }
    // annotation
    fn start_enum_variant(
        &self,
        _identifier: &Identifier,
        _value: u32,
        _span: Option<&Span>,
    ) -> Result<(), Error> {
        Ok(())
    }
    // annotation
    fn end_enum_variant(&self, _name: &Identifier) -> Result<(), Error> {
        Ok(())
    }
    fn end_enum(&self, _name: &Identifier) -> Result<(), Error> {
        Ok(())
    }

    fn start_event(
        &self,
        _name: &Identifier,
        _source: &IdentifierReference,
        _span: Option<&Span>,
    ) -> Result<(), Error> {
        Ok(())
    }
    // annotation
    // start_member..end_member
    fn start_group(&self, _span: Option<&Span>) -> Result<(), Error> {
        Ok(())
    }
    // annotation
    // start_member..end_member
    fn end_group(&self) -> Result<(), Error> {
        Ok(())
    }
    fn end_event(&self, _name: &Identifier) -> Result<(), Error> {
        Ok(())
    }

    fn start_structure(&self, _name: &Identifier, _span: Option<&Span>) -> Result<(), Error> {
        Ok(())
    }
    // annotation
    // start_member..end_member
    // start_group..end_group
    fn end_structure(&self, _name: &Identifier) -> Result<(), Error> {
        Ok(())
    }

    fn start_union(&self, _name: &Identifier, _span: Option<&Span>) -> Result<(), Error> {
        Ok(())
    }
    // annotation
    fn start_type_variant(
        &self,
        _identifier: &IdentifierReference,
        _rename: Option<&Identifier>,
        _span: Option<&Span>,
    ) -> Result<(), Error> {
        Ok(())
    }
    // annotation
    fn end_type_variant(&self, _name: &IdentifierReference) -> Result<(), Error> {
        Ok(())
    }
    fn end_union(&self, _name: &Identifier) -> Result<(), Error> {
        Ok(())
    }

    fn end_module(&self, _name: &Identifier) -> Result<(), Error> {
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn walk_module(module: &Module, walker: &impl ModuleWalker) -> Result<(), Error> {
    walker.start_module(module.name(), module.ts_span())?;

    let body = module.body();

    for import in body.imports() {
        walker.import(import.as_slice(), import.ts_span())?;
    }

    for annotation in body.annotations() {
        walker.annotation(annotation.name(), annotation.value(), annotation.ts_span())?;
    }

    for type_def in body.definitions() {
        match &type_def {
            TypeDefinition::Datatype(def) => walk_datatype_def(def, walker)?,
            TypeDefinition::Entity(def) => walk_entity_def(def, walker)?,
            TypeDefinition::Enum(def) => walk_enum_def(def, walker)?,
            TypeDefinition::Event(def) => walk_event_def(def, walker)?,
            TypeDefinition::Structure(def) => walk_structure_def(def, walker)?,
            TypeDefinition::Union(def) => walk_union_def(def, walker)?,
        }
    }

    walker.end_module(module.name())
}

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

pub fn walk_datatype_def(def: &DatatypeDef, walker: &impl ModuleWalker) -> Result<(), Error> {
    walker.start_datatype(def.name(), def.base_type(), def.ts_span())?;

    if let Some(body) = def.body() {
        for annotation in body.annotations() {
            walker.annotation(annotation.name(), annotation.value(), annotation.ts_span())?;
        }
    }

    walker.end_datatype(def.name())
}

pub fn walk_entity_def(def: &EntityDef, walker: &impl ModuleWalker) -> Result<(), Error> {
    walker.start_entity(def.name(), def.ts_span())?;

    if let Some(body) = def.body() {
        walk_identity_member(body.identity(), walker)?;

        for annotation in body.annotations() {
            walker.annotation(annotation.name(), annotation.value(), annotation.ts_span())?;
        }

        for member in body.members() {
            match member {
                EntityMember::ByValue(member) => walk_by_value_member(member, walker)?,
                EntityMember::ByReference(member) => walk_by_reference_member(member, walker)?,
            }
        }

        for group in body.groups() {
            walk_entity_group(group, walker)?;
        }
    }

    walker.end_entity(def.name())
}

pub fn walk_entity_group(group: &EntityGroup, walker: &impl ModuleWalker) -> Result<(), Error> {
    walker.start_group(group.ts_span())?;

    for annotation in group.annotations() {
        walker.annotation(annotation.name(), annotation.value(), annotation.ts_span())?;
    }

    for member in group.members() {
        match member {
            EntityMember::ByValue(member) => walk_by_value_member(member, walker)?,
            EntityMember::ByReference(member) => walk_by_reference_member(member, walker)?,
        }
    }

    walker.end_group()
}

pub fn walk_enum_def(def: &EnumDef, walker: &impl ModuleWalker) -> Result<(), Error> {
    walker.start_enum(def.name(), def.ts_span())?;

    if let Some(body) = def.body() {
        for annotation in body.annotations() {
            walker.annotation(annotation.name(), annotation.value(), annotation.ts_span())?;
        }
        for variant in body.variants() {
            walker.start_enum_variant(variant.name(), variant.value(), variant.ts_span())?;
            if let Some(body) = variant.body() {
                for annotation in body.annotations() {
                    walker.annotation(
                        annotation.name(),
                        annotation.value(),
                        annotation.ts_span(),
                    )?;
                }
            }
            walker.end_enum_variant(variant.name())?;
        }
    }

    walker.end_enum(def.name())
}

pub fn walk_event_def(def: &EventDef, walker: &impl ModuleWalker) -> Result<(), Error> {
    walker.start_event(def.name(), def.event_source(), def.ts_span())?;

    if let Some(body) = def.body() {
        for annotation in body.annotations() {
            walker.annotation(annotation.name(), annotation.value(), annotation.ts_span())?;
        }
    }

    walker.end_event(def.name())
}

pub fn walk_structure_def(def: &StructureDef, walker: &impl ModuleWalker) -> Result<(), Error> {
    walker.start_structure(def.name(), def.ts_span())?;

    if let Some(body) = def.body() {
        for annotation in body.annotations() {
            walker.annotation(annotation.name(), annotation.value(), annotation.ts_span())?;
        }
    }

    walker.end_structure(def.name())
}

pub fn walk_structure_group(
    group: &StructureGroup,
    walker: &impl ModuleWalker,
) -> Result<(), Error> {
    walker.start_group(group.ts_span())?;

    for annotation in group.annotations() {
        walker.annotation(annotation.name(), annotation.value(), annotation.ts_span())?;
    }

    for member in group.members() {
        walk_by_value_member(member, walker)?;
    }

    walker.end_group()
}

pub fn walk_union_def(def: &UnionDef, walker: &impl ModuleWalker) -> Result<(), Error> {
    walker.start_union(def.name(), def.ts_span())?;

    if let Some(body) = def.body() {
        for annotation in body.annotations() {
            walker.annotation(annotation.name(), annotation.value(), annotation.ts_span())?;
        }
        for variant in body.variants() {
            walker.start_type_variant(variant.name(), variant.rename(), variant.ts_span())?;
            if let Some(body) = variant.body() {
                for annotation in body.annotations() {
                    walker.annotation(
                        annotation.name(),
                        annotation.value(),
                        annotation.ts_span(),
                    )?;
                }
            }
            walker.end_type_variant(variant.name())?;
        }
    }

    walker.end_union(def.name())
}

pub fn walk_identity_member(
    member: &IdentityMember,
    walker: &impl ModuleWalker,
) -> Result<(), Error> {
    walker.start_identity_member(member.name(), member.target_type(), member.ts_span())?;

    if let Some(body) = member.body() {
        for annotation in body.annotations() {
            walker.annotation(annotation.name(), annotation.value(), annotation.ts_span())?;
        }
    }

    walker.end_member(member.name())
}

pub fn walk_by_value_member(
    member: &ByValueMember,
    walker: &impl ModuleWalker,
) -> Result<(), Error> {
    walker.start_by_value_member(
        member.name(),
        member.target_cardinality(),
        member.target_type(),
        member.ts_span(),
    )?;

    if let Some(body) = member.body() {
        for annotation in body.annotations() {
            walker.annotation(annotation.name(), annotation.value(), annotation.ts_span())?;
        }
    }

    walker.end_member(member.name())
}

pub fn walk_by_reference_member(
    member: &ByReferenceMember,
    walker: &impl ModuleWalker,
) -> Result<(), Error> {
    walker.start_by_reference_member(
        member.name(),
        member.source_cardinality(),
        member.target_cardinality(),
        member.target_type(),
        member.ts_span(),
    )?;

    if let Some(body) = member.body() {
        for annotation in body.annotations() {
            walker.annotation(annotation.name(), annotation.value(), annotation.ts_span())?;
        }
    }

    walker.end_member(member.name())
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
