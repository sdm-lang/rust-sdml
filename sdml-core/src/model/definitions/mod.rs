use crate::model::{Identifier, IdentifierReference, ModelElement, Span};
use std::{collections::HashSet, fmt::Debug};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

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

//enum_display_impl!(Definition => Datatype, Entity, Enum, Event, Structure, Union, Property);

impl ModelElement for Definition {
    fn ts_span(&self) -> Option<&Span> {
        match self {
            Self::Datatype(v) => v.ts_span(),
            Self::Entity(v) => v.ts_span(),
            Self::Enum(v) => v.ts_span(),
            Self::Event(v) => v.ts_span(),
            Self::Structure(v) => v.ts_span(),
            Self::Union(v) => v.ts_span(),
            Self::Property(v) => v.ts_span(),
        }
    }

    fn set_ts_span(&mut self, span: Span) {
        match self {
            Self::Datatype(v) => v.set_ts_span(span),
            Self::Entity(v) => v.set_ts_span(span),
            Self::Enum(v) => v.set_ts_span(span),
            Self::Event(v) => v.set_ts_span(span),
            Self::Structure(v) => v.set_ts_span(span),
            Self::Union(v) => v.set_ts_span(span),
            Self::Property(v) => v.set_ts_span(span),
        }
    }

    fn unset_ts_span(&mut self) {
        match self {
            Self::Datatype(v) => v.unset_ts_span(),
            Self::Entity(v) => v.unset_ts_span(),
            Self::Enum(v) => v.unset_ts_span(),
            Self::Event(v) => v.unset_ts_span(),
            Self::Structure(v) => v.unset_ts_span(),
            Self::Union(v) => v.unset_ts_span(),
            Self::Property(v) => v.unset_ts_span(),
        }
    }

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

    fn is_complete(&self) -> bool {
        match self {
            Self::Datatype(v) => v.is_complete(),
            Self::Entity(v) => v.is_complete(),
            Self::Enum(v) => v.is_complete(),
            Self::Event(v) => v.is_complete(),
            Self::Structure(v) => v.is_complete(),
            Self::Union(v) => v.is_complete(),
            Self::Property(v) => v.is_complete(),
        }
    }

    fn referenced_types(&self) -> HashSet<&IdentifierReference> {
        match self {
            Self::Datatype(v) => v.referenced_types(),
            Self::Entity(v) => v.referenced_types(),
            Self::Enum(v) => v.referenced_types(),
            Self::Event(v) => v.referenced_types(),
            Self::Structure(v) => v.referenced_types(),
            Self::Union(v) => v.referenced_types(),
            Self::Property(v) => v.referenced_types(),
        }
    }

    fn referenced_annotations(&self) -> HashSet<&IdentifierReference> {
        match self {
            Self::Datatype(v) => v.referenced_annotations(),
            Self::Entity(v) => v.referenced_annotations(),
            Self::Enum(v) => v.referenced_annotations(),
            Self::Event(v) => v.referenced_annotations(),
            Self::Structure(v) => v.referenced_annotations(),
            Self::Union(v) => v.referenced_annotations(),
            Self::Property(v) => v.referenced_annotations(),
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
pub use datatypes::{AnnotationOnlyBody, DatatypeDef};

mod entities;
pub use entities::{EntityBody, EntityDef, EntityGroup, EntityMember};

mod enums;
pub use enums::{EnumBody, EnumDef, ValueVariant};

mod events;
pub use events::EventDef;

mod properties;
pub use properties::{PropertyBody, PropertyDef, PropertyRole};

mod structures;
pub use structures::{StructureBody, StructureDef, StructureGroup};

mod unions;
pub use unions::{TypeVariant, UnionBody, UnionDef};
