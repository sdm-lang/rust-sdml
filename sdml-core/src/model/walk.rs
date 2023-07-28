/*!
Provides the capability to walk the in-memory model of an SDML module.

To use the model walker:

1. Provide a type, say `MyModuleWalker`.
2. Provide an implementation of [`ModuleWalker`] for `MyModuleWalker`.
2. Implement any methods from the trait [`ModuleWalker`] of interest to you.
3. Use the function [`walk_module`] and provide the module you wish to walk and your walker.

```rust,ignore
#[derive(Debug, Default)]
pub struct MyModuleWalker {}

impl ModuleWalker for MyModuleWalker {
    // implement some methods...
}

walk_module(&some_module, &mut MyModuleWalker::default());
```

*/

use crate::error::Error;
use crate::model::{
    Annotation, ByReferenceMember, ByReferenceMemberInner, ByValueMember, ByValueMemberInner,
    Cardinality, ConstraintBody, DatatypeDef, Definition, EntityDef, EntityGroup, EntityMember,
    EnumDef, EventDef, Identifier, IdentifierReference, IdentityMember, IdentityMemberInner,
    Import, Module, PropertyDef, Span, StructureDef, StructureGroup, TypeReference, UnionDef,
    Value,
};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The trait that captures the callbacks that [`walk_module`] uses as it traverses the module.
///
pub trait ModuleWalker {
    fn start_module(&mut self, _name: &Identifier, _span: Option<&Span>) -> Result<(), Error> {
        Ok(())
    }

    fn import(&mut self, _imported: &[Import], _span: Option<&Span>) -> Result<(), Error> {
        Ok(())
    }

    fn annotation_property(
        &mut self,
        _name: &IdentifierReference,
        _value: &Value,
        _span: Option<&Span>,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn informal_constraint(
        &mut self,
        _name: Option<&Identifier>,
        _value: &str,
        _span: Option<&Span>,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn start_datatype(
        &mut self,
        _name: &Identifier,
        _base_type: &IdentifierReference,
        _has_body: bool,
        _span: Option<&Span>,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn end_datatype(&mut self, _name: &Identifier, _had_body: bool) -> Result<(), Error> {
        Ok(())
    }

    fn start_entity(
        &mut self,
        _name: &Identifier,
        _has_body: bool,
        _span: Option<&Span>,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn start_identity_member(
        &mut self,
        _name: &Identifier,
        _inner: &IdentityMemberInner,
        _span: Option<&Span>,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn start_by_value_member(
        &mut self,
        _name: &Identifier,
        _inner: &ByValueMemberInner,
        _span: Option<&Span>,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn start_by_reference_member(
        &mut self,
        _name: &Identifier,
        _inner: &ByReferenceMemberInner,
        _span: Option<&Span>,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn end_member(&mut self, _name: &Identifier) -> Result<(), Error> {
        Ok(())
    }

    fn end_entity(&mut self, _name: &Identifier, _had_body: bool) -> Result<(), Error> {
        Ok(())
    }

    fn start_enum(
        &mut self,
        _name: &Identifier,
        _has_body: bool,
        _span: Option<&Span>,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn start_value_variant(
        &mut self,
        _identifier: &Identifier,
        _value: u32,
        _has_body: bool,
        _span: Option<&Span>,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn end_value_variant(&mut self, _name: &Identifier, _had_body: bool) -> Result<(), Error> {
        Ok(())
    }

    fn end_enum(&mut self, _name: &Identifier, _had_body: bool) -> Result<(), Error> {
        Ok(())
    }

    fn start_event(
        &mut self,
        _name: &Identifier,
        _source: &IdentifierReference,
        _has_body: bool,
        _span: Option<&Span>,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn start_group(&mut self, _span: Option<&Span>) -> Result<(), Error> {
        Ok(())
    }

    fn end_group(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn end_event(&mut self, _name: &Identifier, _had_body: bool) -> Result<(), Error> {
        Ok(())
    }

    fn start_structure(
        &mut self,
        _name: &Identifier,
        _has_body: bool,
        _span: Option<&Span>,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn end_structure(&mut self, _name: &Identifier, _had_body: bool) -> Result<(), Error> {
        Ok(())
    }

    fn start_union(
        &mut self,
        _name: &Identifier,
        _has_body: bool,
        _span: Option<&Span>,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn start_type_variant(
        &mut self,
        _identifier: &IdentifierReference,
        _rename: Option<&Identifier>,
        _has_body: bool,
        _span: Option<&Span>,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn end_type_variant(
        &mut self,
        _name: &IdentifierReference,
        _had_body: bool,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn end_union(&mut self, _name: &Identifier, _had_body: bool) -> Result<(), Error> {
        Ok(())
    }

    fn start_property(
        &mut self,
        _name: &Identifier,
        _has_body: bool,
        _span: Option<&Span>,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn start_property_role(
        &mut self,
        _name: &Identifier,
        _inverse_name: Option<&Option<Identifier>>,
        _target_cardinality: Option<&Cardinality>,
        _target_type: &TypeReference,
        _has_body: bool,
        _span: Option<&Span>,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn end_property_role(&mut self, _name: &Identifier, _had_body: bool) -> Result<(), Error> {
        Ok(())
    }

    fn end_property(&mut self, _name: &Identifier, _had_body: bool) -> Result<(), Error> {
        Ok(())
    }

    fn end_module(&mut self, _name: &Identifier) -> Result<(), Error> {
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

macro_rules! walk_annotations {
    ($walker: expr, $iterator: expr) => {
        for annotation in $iterator {
            match annotation {
                Annotation::Property(prop) => {
                    $walker.annotation_property(prop.name(), prop.value(), annotation.ts_span())?;
                }
                Annotation::Constraint(cons) => match cons.body() {
                    ConstraintBody::Informal(body) => {
                        $walker.informal_constraint(cons.name(), body, cons.ts_span())?;
                    }
                    ConstraintBody::Formal(_) => todo!(),
                },
            }
        }
    };
}
// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Walk the module `module` calling the relevant methods on `walker`.
///
pub fn walk_module(module: &Module, walker: &mut impl ModuleWalker) -> Result<(), Error> {
    walker.start_module(module.name(), module.ts_span())?;

    let body = module.body();

    for import in body.imports() {
        walker.import(import.as_slice(), import.ts_span())?;
    }

    walk_annotations!(walker, body.annotations());

    for type_def in body.definitions() {
        match &type_def {
            Definition::Datatype(def) => walk_datatype_def(def, walker)?,
            Definition::Entity(def) => walk_entity_def(def, walker)?,
            Definition::Enum(def) => walk_enum_def(def, walker)?,
            Definition::Event(def) => walk_event_def(def, walker)?,
            Definition::Structure(def) => walk_structure_def(def, walker)?,
            Definition::Union(def) => walk_union_def(def, walker)?,
            Definition::Property(def) => walk_property_def(def, walker)?,
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

fn walk_datatype_def(def: &DatatypeDef, walker: &mut impl ModuleWalker) -> Result<(), Error> {
    walker.start_datatype(def.name(), def.base_type(), def.has_body(), def.ts_span())?;

    if let Some(body) = def.body() {
        walk_annotations!(walker, body.annotations());
    }

    walker.end_datatype(def.name(), def.has_body())
}

fn walk_entity_def(def: &EntityDef, walker: &mut impl ModuleWalker) -> Result<(), Error> {
    walker.start_entity(def.name(), def.has_body(), def.ts_span())?;

    if let Some(body) = def.body() {
        walk_identity_member(body.identity(), walker)?;

        walk_annotations!(walker, body.annotations());

        for member in body.members() {
            match &member {
                EntityMember::ByValue(member) => walk_by_value_member(member, walker)?,
                EntityMember::ByReference(member) => walk_by_reference_member(member, walker)?,
            }
        }

        for group in body.groups() {
            walk_entity_group(group, walker)?;
        }
    }

    walker.end_entity(def.name(), def.has_body())
}

fn walk_entity_group(group: &EntityGroup, walker: &mut impl ModuleWalker) -> Result<(), Error> {
    walker.start_group(group.ts_span())?;

    walk_annotations!(walker, group.annotations());

    for member in group.members() {
        match &member {
            EntityMember::ByValue(member) => walk_by_value_member(member, walker)?,
            EntityMember::ByReference(member) => walk_by_reference_member(member, walker)?,
        }
    }

    walker.end_group()
}

fn walk_enum_def(def: &EnumDef, walker: &mut impl ModuleWalker) -> Result<(), Error> {
    walker.start_enum(def.name(), def.has_body(), def.ts_span())?;

    if let Some(body) = def.body() {
        walk_annotations!(walker, body.annotations());
        for variant in body.variants() {
            walker.start_value_variant(
                variant.name(),
                variant.value(),
                variant.has_body(),
                variant.ts_span(),
            )?;
            if let Some(body) = variant.body() {
                walk_annotations!(walker, body.annotations());
            }
            walker.end_value_variant(variant.name(), def.has_body())?;
        }
    }

    walker.end_enum(def.name(), def.has_body())
}

fn walk_event_def(def: &EventDef, walker: &mut impl ModuleWalker) -> Result<(), Error> {
    walker.start_event(
        def.name(),
        def.event_source(),
        def.has_body(),
        def.ts_span(),
    )?;

    if let Some(body) = def.body() {
        walk_annotations!(walker, body.annotations());

        for member in body.members() {
            walk_by_value_member(member, walker)?;
        }

        for group in body.groups() {
            walk_structure_group(group, walker)?;
        }
    }

    walker.end_event(def.name(), def.has_body())
}

fn walk_structure_def(def: &StructureDef, walker: &mut impl ModuleWalker) -> Result<(), Error> {
    walker.start_structure(def.name(), def.has_body(), def.ts_span())?;

    if let Some(body) = def.body() {
        walk_annotations!(walker, body.annotations());

        for member in body.members() {
            walk_by_value_member(member, walker)?;
        }

        for group in body.groups() {
            walk_structure_group(group, walker)?;
        }
    }

    walker.end_structure(def.name(), def.has_body())
}

fn walk_structure_group(
    group: &StructureGroup,
    walker: &mut impl ModuleWalker,
) -> Result<(), Error> {
    walker.start_group(group.ts_span())?;

    walk_annotations!(walker, group.annotations());

    for member in group.members() {
        walk_by_value_member(member, walker)?;
    }

    walker.end_group()
}

fn walk_union_def(def: &UnionDef, walker: &mut impl ModuleWalker) -> Result<(), Error> {
    walker.start_union(def.name(), def.has_body(), def.ts_span())?;

    if let Some(body) = def.body() {
        walk_annotations!(walker, body.annotations());
        for variant in body.variants() {
            walker.start_type_variant(
                variant.name(),
                variant.rename(),
                variant.has_body(),
                variant.ts_span(),
            )?;
            if let Some(body) = variant.body() {
                walk_annotations!(walker, body.annotations());
            }
            walker.end_type_variant(variant.name(), def.has_body())?;
        }
    }

    walker.end_union(def.name(), def.has_body())
}

fn walk_property_def(def: &PropertyDef, walker: &mut impl ModuleWalker) -> Result<(), Error> {
    walker.start_property(def.name(), def.has_body(), def.ts_span())?;

    if let Some(body) = def.body() {
        walk_annotations!(walker, body.annotations());
        for role in body.roles() {
            walker.start_property_role(
                role.name(),
                role.inverse_name(),
                role.target_cardinality(),
                role.target_type(),
                role.has_body(),
                role.ts_span(),
            )?;
            if let Some(body) = role.body() {
                walk_annotations!(walker, body.annotations());
            }
            walker.end_property_role(role.name(), def.has_body())?;
        }
    }

    walker.end_union(def.name(), def.has_body())
}

fn walk_identity_member(
    member: &IdentityMember,
    walker: &mut impl ModuleWalker,
) -> Result<(), Error> {
    walker.start_identity_member(member.name(), member.inner(), member.ts_span())?;

    if let Some(body) = member
        .inner()
        .as_defined()
        .map(|def| def.body())
        .unwrap_or_default()
    {
        walk_annotations!(walker, body.annotations());
    }

    walker.end_member(member.name())
}

fn walk_by_value_member(
    member: &ByValueMember,
    walker: &mut impl ModuleWalker,
) -> Result<(), Error> {
    walker.start_by_value_member(member.name(), member.inner(), member.ts_span())?;

    if let Some(body) = member
        .inner()
        .as_defined()
        .map(|def| def.body())
        .unwrap_or_default()
    {
        walk_annotations!(walker, body.annotations());
    }

    walker.end_member(member.name())
}

fn walk_by_reference_member(
    member: &ByReferenceMember,
    walker: &mut impl ModuleWalker,
) -> Result<(), Error> {
    walker.start_by_reference_member(member.name(), member.inner(), member.ts_span())?;

    if let Some(body) = member
        .inner()
        .as_defined()
        .map(|def| def.body())
        .unwrap_or_default()
    {
        walk_annotations!(walker, body.annotations());
    }

    walker.end_member(member.name())
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
