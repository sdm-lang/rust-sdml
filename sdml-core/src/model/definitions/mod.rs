use crate::{
    cache::ModuleCache,
    load::ModuleLoader,
    model::check::MaybeIncomplete,
    model::members::Member,
    model::{HasName, HasSourceSpan},
};
use std::fmt::Debug;

use sdml_error::diagnostics::functions::definition_is_incomplete;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub trait HasMembers {
    fn has_members(&self) -> bool;

    fn members_len(&self) -> usize;

    fn members(&self) -> Box<dyn Iterator<Item = &Member> + '_>;

    fn members_mut(&mut self) -> Box<dyn Iterator<Item = &mut Member> + '_>;

    fn add_to_members(&mut self, value: Member);

    fn extend_members<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = Member>;
}

pub trait HasVariants<T> {
    fn has_variants(&self) -> bool;

    fn variants_len(&self) -> usize;

    fn variants(&self) -> Box<dyn Iterator<Item = &T> + '_>;

    fn variants_mut(&mut self) -> Box<dyn Iterator<Item = &mut T> + '_>;

    fn add_to_variants(&mut self, value: T);

    fn extend_variants<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = T>;
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions
// ------------------------------------------------------------------------------------------------

/// Corresponds to the grammar rule `type_def`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Definition {
    Datatype(DatatypeDef),
    Entity(EntityDef),
    Enum(EnumDef),
    Event(EventDef),
    Property(PropertyDef),
    Rdf(RdfDef),
    Structure(StructureDef),
    TypeClass(TypeClassDef),
    Union(UnionDef),
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Type Definitions
// ------------------------------------------------------------------------------------------------

impl_from_for_variant!(Definition, Datatype, DatatypeDef);

impl_from_for_variant!(Definition, Entity, EntityDef);

impl_from_for_variant!(Definition, Enum, EnumDef);

impl_from_for_variant!(Definition, Event, EventDef);

impl_from_for_variant!(Definition, Property, PropertyDef);

impl_from_for_variant!(Definition, Rdf, RdfDef);

impl_from_for_variant!(Definition, Structure, StructureDef);

impl_from_for_variant!(Definition, TypeClass, TypeClassDef);

impl_from_for_variant!(Definition, Union, UnionDef);

impl_has_name_for!(Definition => variants Datatype, Entity, Enum, Event, Property, Rdf, Structure, TypeClass, Union);

impl_has_source_span_for!(Definition => variants Datatype, Entity, Enum, Event, Property, Rdf, Structure, TypeClass, Union);

impl_references_for!(Definition => variants Datatype, Entity, Enum, Event, Property, Rdf, Structure, TypeClass, Union);

impl_maybe_invalid_for!(Definition; variants Datatype, Entity, Enum, Event, Property, Rdf, Structure, TypeClass, Union);

impl Validate for Definition {
    fn validate(
        &self,
        top: &Module,
        cache: &ModuleCache,
        loader: &impl ModuleLoader,
        check_constraints: bool,
    ) {
        match self {
            Definition::Datatype(v) => v.validate(top, cache, loader, check_constraints),
            Definition::Entity(v) => v.validate(top, cache, loader, check_constraints),
            Definition::Enum(v) => v.validate(top, cache, loader, check_constraints),
            Definition::Event(v) => v.validate(top, cache, loader, check_constraints),
            Definition::Property(v) => v.validate(top, cache, loader, check_constraints),
            Definition::Rdf(v) => v.validate(top, cache, loader, check_constraints),
            Definition::Structure(v) => v.validate(top, cache, loader, check_constraints),
            Definition::TypeClass(v) => v.validate(top, cache, loader, check_constraints),
            Definition::Union(v) => v.validate(top, cache, loader, check_constraints),
        }
        if self.is_incomplete(top, cache) {
            loader
                .report(&definition_is_incomplete(
                    top.file_id().copied().unwrap_or_default(),
                    self.source_span().map(|span| span.byte_range()),
                    top.name(),
                ))
                .unwrap()
        }
    }
}

impl Definition {
    #[inline(always)]
    pub fn is_datatype(&self) -> bool {
        matches!(self, Self::Datatype(_))
    }

    #[inline(always)]
    pub fn is_structured_type(&self) -> bool {
        matches!(
            self,
            Self::Entity(_) | Self::Enum(_) | Self::Event(_) | Self::Structure(_) | Self::Union(_)
        )
    }

    #[inline(always)]
    pub fn is_type(&self) -> bool {
        matches!(
            self,
            Self::Datatype(_)
                | Self::Entity(_)
                | Self::Enum(_)
                | Self::Event(_)
                | Self::Structure(_)
                | Self::Union(_)
        )
    }

    #[inline(always)]
    pub fn is_library_definition(&self) -> bool {
        matches!(self, Self::Rdf(_) | Self::TypeClass(_))
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod classes;
pub use classes::{
    MethodDef, TypeClassArgument, TypeClassBody, TypeClassDef, TypeClassReference, TypeVariable,
};

mod datatypes;
pub use datatypes::DatatypeDef;

mod entities;
pub use entities::{EntityBody, EntityDef, EntityIdentity, EntityIdentityDef};

mod enums;
pub use enums::{EnumBody, EnumDef, ValueVariant};

mod events;
pub use events::EventDef;

mod properties;
pub use properties::{PropertyBody, PropertyDef, PropertyRole, PropertyRoleDef};

mod structures;
pub use structures::{StructureBody, StructureDef};

mod unions;
pub use unions::{TypeVariant, UnionBody, UnionDef};

mod rdf;
pub use rdf::RdfDef;

use super::{check::Validate, modules::Module};
