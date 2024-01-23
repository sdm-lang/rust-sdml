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
use crate::model::annotations::{Annotation, HasAnnotations};
use crate::model::constraints::{ConstraintBody, ControlledLanguageTag};
use crate::model::constraints::{ConstraintSentence, EnvironmentDef};
use crate::model::definitions::{
    DatatypeDef, Definition, EntityDef, EntityIdentity, EnumDef, EventDef, HasMembers, HasVariants,
    PropertyDef, PropertyRole, PropertyRoleDef, StructureDef, TypeVariant, UnionDef,
};
use crate::model::identifiers::{Identifier, IdentifierReference};
use crate::model::members::HasType;
use crate::model::members::{Cardinality, HasCardinality, Member, TypeReference};
use crate::model::modules::{Import, Module};
use crate::model::values::Value;
use crate::model::{HasBody, HasName, HasNameReference, HasOptionalBody, HasSourceSpan, Span};

use super::definitions::RdfDef;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The trait that captures the callbacks that [`walk_module`] uses as it traverses the module.
///
pub trait SimpleModuleWalker {
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
        _name: &Identifier,
        _value: &str,
        _language: Option<&ControlledLanguageTag>,
        _span: Option<&Span>,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn formal_constraint<'a>(
        &'a mut self,
        _name: &Identifier,
        _environment: &impl Iterator<Item = &'a EnvironmentDef>,
        _body: &ConstraintSentence,
        _span: Option<&Span>,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn start_datatype(
        &mut self,
        _name: &Identifier,
        _is_opaque: bool,
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

    fn start_entity_identity(
        &mut self,
        _name: &Identifier,
        _target_type: &TypeReference,
        _has_body: bool,
        _span: Option<&Span>,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn start_entity_identity_role_ref(
        &mut self,
        _role_name: &Identifier,
        _in_property: &IdentifierReference,
        _span: Option<&Span>,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn start_member(
        &mut self,
        _name: &Identifier,
        _inverse_name: Option<&Identifier>,
        _target_cardinality: &Cardinality,
        _target_type: &TypeReference,
        _has_body: bool,
        _span: Option<&Span>,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn start_member_role_ref(
        &mut self,
        _role_name: &Identifier,
        _in_property: &IdentifierReference,
        _span: Option<&Span>,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn end_member(&mut self, _name: &Identifier, _had_body: bool) -> Result<(), Error> {
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

    fn end_event(&mut self, _name: &Identifier, _had_body: bool) -> Result<(), Error> {
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

    fn start_identity_role(
        &mut self,
        _name: &Identifier,
        _target_type: &TypeReference,
        _has_body: bool,
        _span: Option<&Span>,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn start_member_role(
        &mut self,
        _name: &Identifier,
        _inverse_name: Option<&Identifier>,
        _target_cardinality: &Cardinality,
        _target_type: &TypeReference,
        _has_body: bool,
        _span: Option<&Span>,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn end_property_role(&mut self, _name: &Identifier, _has_body: bool) -> Result<(), Error> {
        Ok(())
    }

    fn end_property(&mut self, _name: &Identifier, _had_body: bool) -> Result<(), Error> {
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

    fn start_rdf(&mut self, _name: &Identifier, _span: Option<&Span>) -> Result<(), Error> {
        Ok(())
    }

    fn end_rdf(&mut self, _name: &Identifier) -> Result<(), Error> {
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
                    $walker.annotation_property(
                        prop.name_reference(),
                        prop.value(),
                        annotation.source_span(),
                    )?;
                }
                Annotation::Constraint(cons) => match cons.body() {
                    ConstraintBody::Informal(body) => {
                        $walker.informal_constraint(
                            cons.name(),
                            body.value(),
                            body.language(),
                            body.source_span(),
                        )?;
                    }
                    ConstraintBody::Formal(body) => {
                        $walker.formal_constraint(
                            cons.name(),
                            &body.definitions(),
                            body.body(),
                            body.source_span(),
                        )?;
                    }
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
pub fn walk_module_simple(
    module: &Module,
    walker: &mut impl SimpleModuleWalker,
) -> Result<(), Error> {
    walker.start_module(module.name(), module.source_span())?;

    let body = module.body();

    for import in body.imports() {
        walker.import(import.as_slice(), import.source_span())?;
    }

    walk_annotations!(walker, body.annotations());

    for type_def in body.definitions() {
        match &type_def {
            Definition::Datatype(def) => walk_datatype_def(def, walker)?,
            Definition::Entity(def) => walk_entity_def(def, walker)?,
            Definition::Enum(def) => walk_enum_def(def, walker)?,
            Definition::Event(def) => walk_event_def(def, walker)?,
            Definition::Property(def) => walk_property_def(def, walker)?,
            Definition::Rdf(def) => walk_rdf_def(def, walker)?,
            Definition::Structure(def) => walk_structure_def(def, walker)?,
            Definition::TypeClass(_) => todo!(),
            Definition::Union(def) => walk_union_def(def, walker)?,
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

fn walk_datatype_def(def: &DatatypeDef, walker: &mut impl SimpleModuleWalker) -> Result<(), Error> {
    walker.start_datatype(
        def.name(),
        def.is_opaque(),
        def.base_type(),
        def.has_body(),
        def.source_span(),
    )?;

    if let Some(body) = def.body() {
        walk_annotations!(walker, body.annotations());
    }

    walker.end_datatype(def.name(), def.has_body())
}

fn walk_entity_def(def: &EntityDef, walker: &mut impl SimpleModuleWalker) -> Result<(), Error> {
    walker.start_entity(def.name(), def.has_body(), def.source_span())?;

    if let Some(body) = def.body() {
        walk_entity_identity(body.identity(), walker)?;

        walk_annotations!(walker, body.annotations());

        for member in body.members() {
            walk_member(member, walker)?;
        }
    }

    walker.end_entity(def.name(), def.has_body())
}

fn walk_enum_def(def: &EnumDef, walker: &mut impl SimpleModuleWalker) -> Result<(), Error> {
    walker.start_enum(def.name(), def.has_body(), def.source_span())?;

    if let Some(body) = def.body() {
        walk_annotations!(walker, body.annotations());
        for variant in body.variants() {
            walker.start_value_variant(
                variant.name(),
                variant.has_body(),
                variant.source_span(),
            )?;
            if let Some(body) = variant.body() {
                walk_annotations!(walker, body.annotations());
            }
            walker.end_value_variant(variant.name(), def.has_body())?;
        }
    }

    walker.end_enum(def.name(), def.has_body())
}

fn walk_event_def(def: &EventDef, walker: &mut impl SimpleModuleWalker) -> Result<(), Error> {
    walker.start_event(
        def.name(),
        def.event_source(),
        def.has_body(),
        def.source_span(),
    )?;

    if let Some(body) = def.body() {
        walk_annotations!(walker, body.annotations());

        for member in body.members() {
            walk_member(member, walker)?;
        }
    }

    walker.end_event(def.name(), def.has_body())
}

fn walk_property_def(def: &PropertyDef, walker: &mut impl SimpleModuleWalker) -> Result<(), Error> {
    walker.start_property(def.name(), def.has_body(), def.source_span())?;

    if let Some(body) = def.body() {
        walk_annotations!(walker, body.annotations());

        for role in body.roles() {
            walk_property_role(role, walker)?;
        }
    }

    walker.end_union(def.name(), def.has_body())
}

fn walk_property_role(
    role: &PropertyRole,
    walker: &mut impl SimpleModuleWalker,
) -> Result<(), Error> {
    match role.definition() {
        PropertyRoleDef::Identity(inner) => walker.start_identity_role(
            role.name(),
            inner.target_type(),
            inner.has_body(),
            role.source_span(),
        )?,
        PropertyRoleDef::Member(inner) => walker.start_member_role(
            role.name(),
            inner.inverse_name(),
            inner.target_cardinality(),
            inner.target_type(),
            inner.has_body(),
            role.source_span(),
        )?,
    };

    let had_body = if let Some(body) = role.body() {
        walk_annotations!(walker, body.annotations());
        true
    } else {
        false
    };
    walker.end_property_role(role.name(), had_body)
}

fn walk_structure_def(
    def: &StructureDef,
    walker: &mut impl SimpleModuleWalker,
) -> Result<(), Error> {
    walker.start_structure(def.name(), def.has_body(), def.source_span())?;

    if let Some(body) = def.body() {
        walk_annotations!(walker, body.annotations());

        for member in body.members() {
            walk_member(member, walker)?;
        }
    }

    walker.end_structure(def.name(), def.has_body())
}

fn walk_rdf_def(def: &RdfDef, walker: &mut impl SimpleModuleWalker) -> Result<(), Error> {
    walker.start_rdf(def.name(), def.source_span())?;
    walk_annotations!(walker, def.body().annotations());
    walker.end_rdf(def.name())
}

fn walk_union_def(def: &UnionDef, walker: &mut impl SimpleModuleWalker) -> Result<(), Error> {
    walker.start_union(def.name(), def.has_body(), def.source_span())?;

    if let Some(body) = def.body() {
        walk_annotations!(walker, body.annotations());
        for variant in body.variants() {
            walk_type_variant(variant, walker)?;
        }
    }

    walker.end_union(def.name(), def.has_body())
}

fn walk_type_variant(
    variant: &TypeVariant,
    walker: &mut impl SimpleModuleWalker,
) -> Result<(), Error> {
    walker.start_type_variant(
        variant.name_reference(),
        variant.rename(),
        variant.has_body(),
        variant.source_span(),
    )?;
    if let Some(body) = variant.body() {
        walk_annotations!(walker, body.annotations());
    }
    walker.end_type_variant(variant.name_reference(), variant.has_body())
}

fn walk_entity_identity(
    member: &EntityIdentity,
    walker: &mut impl SimpleModuleWalker,
) -> Result<(), Error> {
    let had_body = if let Some(name) = member.as_property_reference() {
        walker.start_entity_identity_role_ref(member.name(), name, member.source_span())?;
        false
    } else if let Some(defn) = member.as_definition() {
        walker.start_entity_identity(
            member.name(),
            defn.target_type(),
            defn.has_body(),
            member.source_span(),
        )?;
        if let Some(body) = defn.body() {
            walk_annotations!(walker, body.annotations());
            true
        } else {
            false
        }
    } else {
        unreachable!()
    };

    walker.end_member(member.name(), had_body)
}

fn walk_member(member: &Member, walker: &mut impl SimpleModuleWalker) -> Result<(), Error> {
    let had_body = if let Some(name) = member.as_property_reference() {
        walker.start_member_role_ref(member.name(), name, member.source_span())?;
        false
    } else if let Some(defn) = member.as_definition() {
        walker.start_member(
            member.name(),
            defn.inverse_name(),
            defn.target_cardinality(),
            defn.target_type(),
            defn.has_body(),
            member.source_span(),
        )?;
        if let Some(body) = defn.body() {
            walk_annotations!(walker, body.annotations());
            true
        } else {
            false
        }
    } else {
        unreachable!()
    };

    walker.end_member(member.name(), had_body)
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
