/*!
Provide the Rust types that implement *member*-related components of the SDML Grammar.
*/
use crate::load::ModuleLoader;
use crate::model::annotations::AnnotationOnlyBody;
use crate::model::check::{find_definition, MaybeIncomplete, Validate};
use crate::model::definitions::Definition;
use crate::model::identifiers::{Identifier, IdentifierReference};
use crate::model::modules::Module;
use crate::model::{HasSourceSpan, References, Span};
use crate::store::ModuleStore;
use sdml_errors::diagnostics::functions::{
    member_is_incomplete, property_reference_not_property, type_definition_not_found,
    IdentifierCaseConvention,
};
use std::collections::HashSet;
use std::fmt::Debug;
use tracing::error;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Members ❱ Member
// ------------------------------------------------------------------------------------------------

/// Corresponds to the grammar rules `member` and `entity_identity`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Member {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    kind: MemberKind,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum MemberKind {
    Reference(IdentifierReference),
    Definition(MemberDef),
}

/// Corresponds to the definition component within grammar rule `by_reference_member`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct MemberDef {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    name: Identifier,
    target_cardinality: Cardinality,
    target_type: TypeReference,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    body: Option<AnnotationOnlyBody>,
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Members ❱ Member
// ------------------------------------------------------------------------------------------------

impl<T> From<T> for Member
where
    T: Into<MemberKind>,
{
    fn from(kind: T) -> Self {
        Self {
            span: Default::default(),
            kind: kind.into(),
        }
    }
}

impl_has_source_span_for!(Member);

impl MaybeIncomplete for Member {
    fn is_incomplete(&self, top: &Module, cache: &impl ModuleStore) -> bool {
        match &self.kind {
            MemberKind::Reference(name) => {
                if let Some(defn) = find_definition(name, top, cache) {
                    if matches!(defn, Definition::Property(_)) {
                        defn.is_incomplete(top, cache)
                    } else {
                        error!("Member property reference not a property");
                        false
                    }
                } else {
                    error!("Member property reference not found");
                    false
                }
            }
            MemberKind::Definition(v) => v.is_incomplete(top, cache),
        }
    }
}

impl Validate for Member {
    fn validate(
        &self,
        top: &Module,
        cache: &impl ModuleStore,
        loader: &impl ModuleLoader,
        check_constraints: bool,
    ) {
        match &self.kind {
            MemberKind::Reference(name) => {
                if let Some(defn) = find_definition(name, top, cache) {
                    if !matches!(defn, Definition::Property(_)) {
                        error!("Member property reference not a property");
                        loader
                            .report(&property_reference_not_property(
                                top.file_id().copied().unwrap_or_default(),
                                name.source_span().map(|span| span.byte_range()),
                                name,
                            ))
                            .unwrap()
                    }
                } else {
                    error!("Member property reference not found");
                    loader
                        .report(&type_definition_not_found(
                            top.file_id().copied().unwrap_or_default(),
                            name.source_span().map(|span| span.byte_range()),
                            name,
                        ))
                        .unwrap()
                }
            }
            MemberKind::Definition(v) => {
                v.validate(top, cache, loader, check_constraints);
                if self.is_incomplete(top, cache) {
                    loader
                        .report(&member_is_incomplete(
                            top.file_id().copied().unwrap_or_default(),
                            self.source_span().map(|span| span.byte_range()),
                            v.name(),
                        ))
                        .unwrap()
                }
            }
        }
    }
}

impl References for Member {
    fn referenced_annotations<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        match &self.kind {
            MemberKind::Reference(v) => {
                names.insert(v);
            }
            MemberKind::Definition(v) => v.referenced_annotations(names),
        }
    }

    fn referenced_types<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        match &self.kind {
            MemberKind::Reference(v) => {
                names.insert(v);
            }
            MemberKind::Definition(v) => v.referenced_types(names),
        }
    }
}

impl Member {
    // --------------------------------------------------------------------------------------------
    // Member :: Constructors
    // --------------------------------------------------------------------------------------------

    pub const fn new_reference(in_property: IdentifierReference) -> Self {
        Self {
            span: None,
            kind: MemberKind::Reference(in_property),
        }
    }

    pub const fn new_definition(definition: MemberDef) -> Self {
        Self {
            span: None,
            kind: MemberKind::Definition(definition),
        }
    }

    // --------------------------------------------------------------------------------------------
    // Member :: Variants
    // --------------------------------------------------------------------------------------------

    pub const fn kind(&self) -> &MemberKind {
        &self.kind
    }

    delegate!(pub const is_definition, bool, kind);
    delegate!(pub const as_definition, Option<&MemberDef>, kind);

    delegate!(pub const is_property_reference, bool, kind);
    delegate!(pub const as_property_reference, Option<&IdentifierReference>, kind);

    // --------------------------------------------------------------------------------------------
    // Member :: Delegated
    // --------------------------------------------------------------------------------------------

    pub fn name(&self) -> &Identifier {
        match self.kind() {
            MemberKind::Reference(v) => v.member(),
            MemberKind::Definition(defn) => defn.name(),
        }
    }

    pub fn resolve_target_type(
        &self,
        module: &Module,
        cache: &impl ModuleStore,
    ) -> Option<TypeReference> {
        match self.kind() {
            MemberKind::Reference(v) => {
                if let Some(Definition::Property(property)) = cache.resolve_or_in(v, module.name())
                {
                    Some(property.member_def().target_type().clone())
                } else {
                    None
                }
            }
            MemberKind::Definition(defn) => Some(defn.target_type().clone()),
        }
    }

    pub fn resolve_target_cardinality(
        &self,
        module: &Module,
        cache: &impl ModuleStore,
    ) -> Option<Cardinality> {
        match self.kind() {
            MemberKind::Reference(v) => {
                if let Some(Definition::Property(property)) = cache.resolve_or_in(v, module.name())
                {
                    Some(property.member_def().target_cardinality().clone())
                } else {
                    None
                }
            }
            MemberKind::Definition(defn) => Some(defn.target_cardinality().clone()),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Members ❱ MemberKind
// ------------------------------------------------------------------------------------------------

impl From<MemberDef> for MemberKind {
    fn from(value: MemberDef) -> Self {
        Self::Definition(value)
    }
}

impl From<IdentifierReference> for MemberKind {
    fn from(value: IdentifierReference) -> Self {
        Self::Reference(value)
    }
}

impl MemberKind {
    is_as_variant!(Definition (MemberDef) => is_definition, as_definition);
    is_as_variant!(Reference (IdentifierReference) => is_property_reference, as_property_reference);
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Members ❱ MemberDef
// ------------------------------------------------------------------------------------------------

impl_has_cardinality_for!(MemberDef);

impl_has_optional_body_for!(MemberDef);

impl_has_source_span_for!(MemberDef);

impl_has_type_for!(MemberDef);

impl_annotation_builder!(MemberDef, optional body);

impl References for MemberDef {
    fn referenced_annotations<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        self.body
            .as_ref()
            .map(|b| b.referenced_annotations(names))
            .unwrap_or_default()
    }

    fn referenced_types<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        self.target_type.referenced_types(names);
    }
}

impl MaybeIncomplete for MemberDef {
    fn is_incomplete(&self, top: &Module, cache: &impl ModuleStore) -> bool {
        self.target_type.is_incomplete(top, cache)
    }
}

impl Validate for MemberDef {
    fn validate(
        &self,
        top: &Module,
        cache: &impl ModuleStore,
        loader: &impl ModuleLoader,
        check_constraints: bool,
    ) {
        self.name()
            .validate(top, loader, Some(IdentifierCaseConvention::Member));
        self.target_type()
            .validate(top, cache, loader, check_constraints);
        self.target_cardinality()
            .validate(top, cache, loader, check_constraints);
    }
}

impl MemberDef {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn new<T>(name: Identifier, target_type: T) -> Self
    where
        T: Into<TypeReference>,
    {
        Self {
            span: None,
            name,
            target_type: target_type.into(),
            target_cardinality: Cardinality::one(),
            body: None,
        }
    }

    pub const fn new_unknown(name: Identifier) -> Self {
        Self {
            span: None,
            name,
            target_type: TypeReference::Unknown,
            target_cardinality: Cardinality::one(),
            body: None,
        }
    }

    builder_fn!(pub with_target_type, target_type => TypeReference);
    builder_fn!(pub with_target_cardinality, target_cardinality => Cardinality);
    builder_fn!(pub with_body, body => optional AnnotationOnlyBody);

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    get_and_set!(pub name, set_name  => Identifier);
    get_and_set!(pub target_type, set_target_type  => TypeReference);
    get_and_set!(pub target_cardinality, set_target_cardinality  => Cardinality);
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod cardinality;
pub use cardinality::{
    Cardinality, CardinalityRange, HasCardinality, Ordering, PseudoSequenceType, Uniqueness,
    DEFAULT_CARDINALITY, DEFAULT_CARDINALITY_RANGE, TYPE_BAG_CARDINALITY, TYPE_LIST_CARDINALITY,
    TYPE_MAYBE_CARDINALITY, TYPE_ORDERED_SET_CARDINALITY, TYPE_SET_CARDINALITY,
};

mod types;
pub use types::{HasType, MappingType, TypeReference};

use super::HasName;
