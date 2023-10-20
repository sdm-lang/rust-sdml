use crate::model::members::{Member, MemberGroup};
use std::fmt::Debug;

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

pub trait HasGroups {
    fn has_groups(&self) -> bool;

    fn groups_len(&self) -> usize;

    fn groups(&self) -> Box<dyn Iterator<Item = &MemberGroup> + '_>;

    fn groups_mut(&mut self) -> Box<dyn Iterator<Item = &mut MemberGroup> + '_>;

    fn add_to_groups(&mut self, value: MemberGroup);

    fn extend_groups<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = MemberGroup>;
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
    Structure(StructureDef),
    TypeClass(TypeClassDef),
    Union(UnionDef),
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

impl_from_for_variant!(Definition, Property, PropertyDef);

impl_from_for_variant!(Definition, Structure, StructureDef);

impl_from_for_variant!(Definition, TypeClass, TypeClassDef);

impl_from_for_variant!(Definition, Union, UnionDef);

impl_has_name_for!(Definition => variants Datatype, Entity, Enum, Event, Property, Structure, TypeClass, Union);

impl_has_source_span_for!(Definition => variants Datatype, Entity, Enum, Event, Property, Structure, TypeClass, Union);

impl_references_for!(Definition => variants Datatype, Entity, Enum, Event, Property, Structure, TypeClass, Union);

impl_validate_for!(Definition => variants Datatype, Entity, Enum, Event, Property, Structure, TypeClass, Union);

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

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
