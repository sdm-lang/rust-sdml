use crate::model::identifiers::Identifier;
use crate::model::{HasName, HasSourceSpan, Span};
use std::fmt::Debug;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub trait HasMembers<T> {
    fn has_members(&self) -> bool;

    fn members_len(&self) -> usize;

    fn members(&self) -> Box<dyn Iterator<Item = &T> + '_>;

    fn members_mut(&mut self) -> Box<dyn Iterator<Item = &mut T> + '_>;

    fn add_to_members(&mut self, value: T);

    fn extend_members<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = T>;
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

pub trait HasGroups<G, T>
where
    G: HasMembers<T>,
{
    fn has_groups(&self) -> bool;

    fn groups_len(&self) -> usize;

    fn groups(&self) -> Box<dyn Iterator<Item = &G> + '_>;

    fn groups_mut(&mut self) -> Box<dyn Iterator<Item = &mut G> + '_>;

    fn add_to_groups(&mut self, value: G);

    fn extend_groups<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = G>;
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
    Structure(StructureDef),
    Union(UnionDef),
    Property(PropertyDef),
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
// Implementations ❱ Type Definitions
// ------------------------------------------------------------------------------------------------

impl_from_for_variant!(Definition, Datatype, DatatypeDef);

impl_from_for_variant!(Definition, Entity, EntityDef);

impl_from_for_variant!(Definition, Enum, EnumDef);

impl_from_for_variant!(Definition, Event, EventDef);

impl_from_for_variant!(Definition, Structure, StructureDef);

impl_from_for_variant!(Definition, Union, UnionDef);

impl_from_for_variant!(Definition, Property, PropertyDef);

impl HasName for Definition {
    fn name(&self) -> &Identifier {
        match self {
            Self::Datatype(v) => v.name(),
            Self::Entity(v) => v.name(),
            Self::Enum(v) => v.name(),
            Self::Event(v) => v.name(),
            Self::Structure(v) => v.name(),
            Self::Union(v) => v.name(),
            Self::Property(v) => v.name(),
        }
    }

    fn set_name(&mut self, name: Identifier) {
        match self {
            Self::Datatype(v) => v.set_name(name),
            Self::Entity(v) => v.set_name(name),
            Self::Enum(v) => v.set_name(name),
            Self::Event(v) => v.set_name(name),
            Self::Structure(v) => v.set_name(name),
            Self::Union(v) => v.set_name(name),
            Self::Property(v) => v.set_name(name),
        }
    }
}

impl_references_for!(Definition => variants Datatype, Entity, Enum, Event, Structure, Union, Property);

impl_validate_for!(Definition => variants Datatype, Entity, Enum, Event, Structure, Union, Property);

impl Definition {
    pub fn source_span(&self) -> Option<&Span> {
        match self {
            Self::Datatype(v) => v.source_span(),
            Self::Entity(v) => v.source_span(),
            Self::Enum(v) => v.source_span(),
            Self::Event(v) => v.source_span(),
            Self::Structure(v) => v.source_span(),
            Self::Union(v) => v.source_span(),
            Self::Property(v) => v.source_span(),
        }
    }

    pub fn set_source_span(&mut self, span: Span) {
        match self {
            Self::Datatype(v) => v.set_source_span(span),
            Self::Entity(v) => v.set_source_span(span),
            Self::Enum(v) => v.set_source_span(span),
            Self::Event(v) => v.set_source_span(span),
            Self::Structure(v) => v.set_source_span(span),
            Self::Union(v) => v.set_source_span(span),
            Self::Property(v) => v.set_source_span(span),
        }
    }

    pub fn unset_source_span(&mut self) {
        match self {
            Self::Datatype(v) => v.unset_source_span(),
            Self::Entity(v) => v.unset_source_span(),
            Self::Enum(v) => v.unset_source_span(),
            Self::Event(v) => v.unset_source_span(),
            Self::Structure(v) => v.unset_source_span(),
            Self::Union(v) => v.unset_source_span(),
            Self::Property(v) => v.unset_source_span(),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod datatypes;
pub use datatypes::DatatypeDef;

mod entities;
pub use entities::{EntityBody, EntityDef, EntityGroup, EntityMember};

mod enums;
pub use enums::{EnumBody, EnumDef, ValueVariant};

mod events;
pub use events::EventDef;

mod properties;
pub use properties::{PropertyBody, PropertyDef, PropertyRole, PropertyRoleDef};

mod structures;
pub use structures::{StructureBody, StructureDef, StructureGroup};

mod unions;
pub use unions::{TypeVariant, UnionBody, UnionDef};
