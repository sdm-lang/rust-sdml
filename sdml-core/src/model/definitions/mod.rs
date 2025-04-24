/*!
Provide the Rust types that implement *definition*-related components of the SDML Grammar.
*/

use crate::{
    load::ModuleLoader,
    model::{
        check::{MaybeIncomplete, Validate},
        identifiers::{Identifier, IdentifierReference},
        modules::Module,
        HasName, HasSourceSpan, References, Span,
    },
    store::ModuleStore,
};
use sdml_errors::diagnostics::functions::definition_is_incomplete;
use std::{collections::BTreeSet, fmt::Debug};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Traits ❱  Multi-Methods
// ------------------------------------------------------------------------------------------------

pub trait HasMultiMembers {
    fn has_any_members(&self) -> bool;

    fn contains_any_member(&self, name: &Identifier) -> bool;

    fn all_member_count(&self) -> usize;

    fn all_member_names(&self) -> impl Iterator<Item = &Identifier>;
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Definitions ❱  Definition
// ------------------------------------------------------------------------------------------------

/// Corresponds to the grammar rule `type_def`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Definition {
    Datatype(DatatypeDef),
    Dimension(DimensionDef),
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
// Public Types ❱ Definitions ❱  FromDefinition
// ------------------------------------------------------------------------------------------------

/// Corresponds to the grammar rule `from_definition`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct FromDefinition {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    definition: IdentifierReference,
    with_members: BTreeSet<Identifier>,
}

pub trait HasOptionalFromDefinition {
    fn with_from_definition(self, from_definition: FromDefinition) -> Self
    where
        Self: Sized,
    {
        let mut self_mut = self;
        self_mut.set_from_definition(from_definition);
        self_mut
    }
    fn has_from_definition(&self) -> bool {
        self.from_definition().is_some()
    }
    #[allow(clippy::wrong_self_convention)]
    fn from_definition(&self) -> Option<&FromDefinition>;
    #[allow(clippy::wrong_self_convention)]
    fn from_definition_mut(&mut self) -> Option<&mut FromDefinition>;
    fn set_from_definition(&mut self, from_definition: FromDefinition);
    fn unset_from_definition(&mut self);
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Definitions ❱ Definition
// ------------------------------------------------------------------------------------------------

impl From<&DatatypeDef> for Definition {
    fn from(v: &DatatypeDef) -> Self {
        Self::Datatype(v.clone())
    }
}

impl From<DatatypeDef> for Definition {
    fn from(v: DatatypeDef) -> Self {
        Self::Datatype(v)
    }
}
impl From<&DimensionDef> for Definition {
    fn from(v: &DimensionDef) -> Self {
        Self::Dimension(v.clone())
    }
}

impl From<DimensionDef> for Definition {
    fn from(v: DimensionDef) -> Self {
        Self::Dimension(v)
    }
}

impl From<&EntityDef> for Definition {
    fn from(v: &EntityDef) -> Self {
        Self::Entity(v.clone())
    }
}

impl From<EntityDef> for Definition {
    fn from(v: EntityDef) -> Self {
        Self::Entity(v)
    }
}

impl From<&EnumDef> for Definition {
    fn from(v: &EnumDef) -> Self {
        Self::Enum(v.clone())
    }
}

impl From<EnumDef> for Definition {
    fn from(v: EnumDef) -> Self {
        Self::Enum(v)
    }
}

impl From<&EventDef> for Definition {
    fn from(v: &EventDef) -> Self {
        Self::Event(v.clone())
    }
}

impl From<EventDef> for Definition {
    fn from(v: EventDef) -> Self {
        Self::Event(v)
    }
}

impl From<&PropertyDef> for Definition {
    fn from(v: &PropertyDef) -> Self {
        Self::Property(v.clone())
    }
}

impl From<PropertyDef> for Definition {
    fn from(v: PropertyDef) -> Self {
        Self::Property(v)
    }
}

impl From<&RdfDef> for Definition {
    fn from(v: &RdfDef) -> Self {
        Self::Rdf(v.clone())
    }
}

impl From<RdfDef> for Definition {
    fn from(v: RdfDef) -> Self {
        Self::Rdf(v)
    }
}

impl From<&StructureDef> for Definition {
    fn from(v: &StructureDef) -> Self {
        Self::Structure(v.clone())
    }
}

impl From<StructureDef> for Definition {
    fn from(v: StructureDef) -> Self {
        Self::Structure(v)
    }
}

impl From<&TypeClassDef> for Definition {
    fn from(v: &TypeClassDef) -> Self {
        Self::TypeClass(v.clone())
    }
}

impl From<TypeClassDef> for Definition {
    fn from(v: TypeClassDef) -> Self {
        Self::TypeClass(v)
    }
}

impl From<&UnionDef> for Definition {
    fn from(v: &UnionDef) -> Self {
        Self::Union(v.clone())
    }
}

impl From<UnionDef> for Definition {
    fn from(v: UnionDef) -> Self {
        Self::Union(v)
    }
}

impl HasName for Definition {
    fn name(&self) -> &Identifier {
        match self {
            Self::Datatype(v) => v.name(),
            Self::Dimension(v) => v.name(),
            Self::Entity(v) => v.name(),
            Self::Enum(v) => v.name(),
            Self::Event(v) => v.name(),
            Self::Property(v) => v.name(),
            Self::Rdf(v) => v.name(),
            Self::Structure(v) => v.name(),
            Self::TypeClass(v) => v.name(),
            Self::Union(v) => v.name(),
        }
    }

    fn set_name(&mut self, name: Identifier) {
        match self {
            Self::Datatype(v) => v.set_name(name),
            Self::Dimension(v) => v.set_name(name),
            Self::Entity(v) => v.set_name(name),
            Self::Enum(v) => v.set_name(name),
            Self::Event(v) => v.set_name(name),
            Self::Property(v) => v.set_name(name),
            Self::Rdf(v) => v.set_name(name),
            Self::Structure(v) => v.set_name(name),
            Self::TypeClass(v) => v.set_name(name),
            Self::Union(v) => v.set_name(name),
        }
    }
}

impl HasSourceSpan for Definition {
    #[inline]
    fn with_source_span(self, span: Span) -> Self {
        match self {
            Self::Datatype(v) => Self::Datatype(v.with_source_span(span)),
            Self::Dimension(v) => Self::Dimension(v.with_source_span(span)),
            Self::Entity(v) => Self::Entity(v.with_source_span(span)),
            Self::Enum(v) => Self::Enum(v.with_source_span(span)),
            Self::Event(v) => Self::Event(v.with_source_span(span)),
            Self::Property(v) => Self::Property(v.with_source_span(span)),
            Self::Rdf(v) => Self::Rdf(v.with_source_span(span)),
            Self::Structure(v) => Self::Structure(v.with_source_span(span)),
            Self::TypeClass(v) => Self::TypeClass(v.with_source_span(span)),
            Self::Union(v) => Self::Union(v.with_source_span(span)),
        }
    }

    #[inline]
    fn source_span(&self) -> Option<&Span> {
        match self {
            Self::Datatype(v) => v.source_span(),
            Self::Dimension(v) => v.source_span(),
            Self::Entity(v) => v.source_span(),
            Self::Enum(v) => v.source_span(),
            Self::Event(v) => v.source_span(),
            Self::Property(v) => v.source_span(),
            Self::Rdf(v) => v.source_span(),
            Self::Structure(v) => v.source_span(),
            Self::TypeClass(v) => v.source_span(),
            Self::Union(v) => v.source_span(),
        }
    }

    #[inline]
    fn set_source_span(&mut self, span: Span) {
        match self {
            Self::Datatype(v) => v.set_source_span(span),
            Self::Dimension(v) => v.set_source_span(span),
            Self::Entity(v) => v.set_source_span(span),
            Self::Enum(v) => v.set_source_span(span),
            Self::Event(v) => v.set_source_span(span),
            Self::Property(v) => v.set_source_span(span),
            Self::Rdf(v) => v.set_source_span(span),
            Self::Structure(v) => v.set_source_span(span),
            Self::TypeClass(v) => v.set_source_span(span),
            Self::Union(v) => v.set_source_span(span),
        }
    }

    #[inline]
    fn unset_source_span(&mut self) {
        match self {
            Self::Datatype(v) => v.unset_source_span(),
            Self::Dimension(v) => v.unset_source_span(),
            Self::Entity(v) => v.unset_source_span(),
            Self::Enum(v) => v.unset_source_span(),
            Self::Event(v) => v.unset_source_span(),
            Self::Property(v) => v.unset_source_span(),
            Self::Rdf(v) => v.unset_source_span(),
            Self::Structure(v) => v.unset_source_span(),
            Self::TypeClass(v) => v.unset_source_span(),
            Self::Union(v) => v.unset_source_span(),
        }
    }
}

impl References for Definition {
    fn referenced_annotations<'a>(&'a self, names: &mut BTreeSet<&'a IdentifierReference>) {
        match self {
            Self::Datatype(v) => v.referenced_annotations(names),
            Self::Dimension(v) => v.referenced_annotations(names),
            Self::Entity(v) => v.referenced_annotations(names),
            Self::Enum(v) => v.referenced_annotations(names),
            Self::Event(v) => v.referenced_annotations(names),
            Self::Property(v) => v.referenced_annotations(names),
            Self::Rdf(v) => v.referenced_annotations(names),
            Self::Structure(v) => v.referenced_annotations(names),
            Self::TypeClass(v) => v.referenced_annotations(names),
            Self::Union(v) => v.referenced_annotations(names),
        }
    }

    fn referenced_types<'a>(&'a self, names: &mut BTreeSet<&'a IdentifierReference>) {
        match self {
            Self::Datatype(v) => v.referenced_types(names),
            Self::Dimension(v) => v.referenced_types(names),
            Self::Entity(v) => v.referenced_types(names),
            Self::Enum(v) => v.referenced_types(names),
            Self::Event(v) => v.referenced_types(names),
            Self::Property(v) => v.referenced_types(names),
            Self::Rdf(v) => v.referenced_types(names),
            Self::Structure(v) => v.referenced_types(names),
            Self::TypeClass(v) => v.referenced_types(names),
            Self::Union(v) => v.referenced_types(names),
        }
    }
}

impl MaybeIncomplete for Definition {
    fn is_incomplete(&self, top: &Module, cache: &impl ModuleStore) -> bool {
        match self {
            Self::Datatype(v) => v.is_incomplete(top, cache),
            Self::Dimension(v) => v.is_incomplete(top, cache),
            Self::Entity(v) => v.is_incomplete(top, cache),
            Self::Enum(v) => v.is_incomplete(top, cache),
            Self::Event(v) => v.is_incomplete(top, cache),
            Self::Property(v) => v.is_incomplete(top, cache),
            Self::Rdf(v) => v.is_incomplete(top, cache),
            Self::Structure(v) => v.is_incomplete(top, cache),
            Self::TypeClass(v) => v.is_incomplete(top, cache),
            Self::Union(v) => v.is_incomplete(top, cache),
        }
    }
}

impl Validate for Definition {
    fn validate(
        &self,
        top: &Module,
        cache: &impl ModuleStore,
        loader: &impl ModuleLoader,
        check_constraints: bool,
    ) {
        match self {
            Definition::Datatype(v) => v.validate(top, cache, loader, check_constraints),
            Definition::Dimension(v) => v.validate(top, cache, loader, check_constraints),
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
    // --------------------------------------------------------------------------------------------
    // Variants
    // --------------------------------------------------------------------------------------------

    pub const fn is_datatype(&self) -> bool {
        matches!(self, Self::Datatype(_))
    }

    pub const fn as_datatype(&self) -> Option<&DatatypeDef> {
        match self {
            Self::Datatype(v) => Some(v),
            _ => None,
        }
    }

    pub const fn is_dimension(&self) -> bool {
        matches!(self, Self::Dimension(_))
    }

    pub const fn as_dimension(&self) -> Option<&DimensionDef> {
        match self {
            Self::Dimension(v) => Some(v),
            _ => None,
        }
    }

    pub const fn is_entity(&self) -> bool {
        matches!(self, Self::Entity(_))
    }

    pub const fn as_entity(&self) -> Option<&EntityDef> {
        match self {
            Self::Entity(v) => Some(v),
            _ => None,
        }
    }

    pub const fn is_enum(&self) -> bool {
        matches!(self, Self::Enum(_))
    }

    pub const fn as_enum(&self) -> Option<&EnumDef> {
        match self {
            Self::Enum(v) => Some(v),
            _ => None,
        }
    }

    pub const fn is_event(&self) -> bool {
        matches!(self, Self::Event(_))
    }

    pub const fn as_event(&self) -> Option<&EventDef> {
        match self {
            Self::Event(v) => Some(v),
            _ => None,
        }
    }

    pub const fn is_property(&self) -> bool {
        matches!(self, Self::Property(_))
    }

    pub const fn as_property(&self) -> Option<&PropertyDef> {
        match self {
            Self::Property(v) => Some(v),
            _ => None,
        }
    }

    pub const fn is_rdf(&self) -> bool {
        matches!(self, Self::Rdf(_))
    }

    pub const fn as_rdf(&self) -> Option<&RdfDef> {
        match self {
            Self::Rdf(v) => Some(v),
            _ => None,
        }
    }

    pub const fn is_structure(&self) -> bool {
        matches!(self, Self::Structure(_))
    }

    pub const fn as_structure(&self) -> Option<&StructureDef> {
        match self {
            Self::Structure(v) => Some(v),
            _ => None,
        }
    }

    pub const fn is_type_class(&self) -> bool {
        matches!(self, Self::TypeClass(_))
    }

    pub const fn as_type_class(&self) -> Option<&TypeClassDef> {
        match self {
            Self::TypeClass(v) => Some(v),
            _ => None,
        }
    }

    pub const fn is_union(&self) -> bool {
        matches!(self, Self::Union(_))
    }

    pub const fn as_union(&self) -> Option<&UnionDef> {
        match self {
            Self::Union(v) => Some(v),
            _ => None,
        }
    }

    // --------------------------------------------------------------------------------------------
    // Helpers
    // --------------------------------------------------------------------------------------------

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
// Implementations ❱ Definitions ❱ FromDefinition
// ------------------------------------------------------------------------------------------------

impl From<IdentifierReference> for FromDefinition {
    fn from(definition: IdentifierReference) -> Self {
        Self {
            span: Default::default(),
            definition,
            with_members: Default::default(),
        }
    }
}

impl HasSourceSpan for FromDefinition {
    fn with_source_span(self, span: Span) -> Self {
        let mut self_mut = self;
        self_mut.span = Some(span);
        self_mut
    }

    fn source_span(&self) -> Option<&Span> {
        self.span.as_ref()
    }

    fn set_source_span(&mut self, span: Span) {
        self.span = Some(span);
    }

    fn unset_source_span(&mut self) {
        self.span = None;
    }
}

impl FromDefinition {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------
    pub fn new<I>(definition: IdentifierReference, with_members: I) -> Self
    where
        I: IntoIterator<Item = Identifier>,
    {
        Self::from(definition).with_members(with_members)
    }

    pub fn with_members<I>(self, with_members: I) -> Self
    where
        I: IntoIterator<Item = Identifier>,
    {
        let mut self_mut = self;
        self_mut.extend_members(with_members);
        self_mut
    }

    // --------------------------------------------------------------------------------------------
    // Members
    // --------------------------------------------------------------------------------------------

    pub fn definition(&self) -> &IdentifierReference {
        &self.definition
    }

    pub fn set_definition(&mut self, definition: IdentifierReference) {
        self.definition = definition;
    }

    // --------------------------------------------------------------------------------------------

    pub fn is_wildcard(&self) -> bool {
        self.with_members.is_empty()
    }

    pub fn set_member_wildcard(&mut self) {
        self.with_members.clear();
    }

    pub fn has_members(&self) -> bool {
        !self.with_members.is_empty()
    }

    pub fn member_count(&self) -> usize {
        self.with_members.len()
    }

    pub fn members(&self) -> impl Iterator<Item = &Identifier> {
        self.with_members.iter()
    }

    pub fn add_to_members(&mut self, member: Identifier) {
        self.with_members.insert(member);
    }

    pub fn extend_members<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = Identifier>,
    {
        self.with_members.extend(extension);
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
pub use datatypes::{
    is_restriction_facet_name, is_restriction_facet_name_str, DatatypeDef, ExplicitTimezoneFlag,
    RestrictionFacet,
};

mod dimensions;
pub use dimensions::{
    DimensionBody, DimensionDef, DimensionIdentity, DimensionParent, SourceEntity,
};

mod entities;
pub use entities::{EntityBody, EntityDef};

mod enums;
pub use enums::{EnumBody, EnumDef, ValueVariant};

mod events;
pub use events::{EventBody, EventDef};

mod properties;
pub use properties::PropertyDef;

mod structures;
pub use structures::{StructureBody, StructureDef};

mod unions;
pub use unions::{TypeVariant, UnionBody, UnionDef};

mod rdf;
pub use rdf::RdfDef;
