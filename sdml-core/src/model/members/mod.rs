use crate::cache::ModuleCache;
use crate::load::ModuleLoader;
use crate::model::annotations::AnnotationOnlyBody;
use crate::model::check::{find_definition, MaybeIncomplete, Validate};
use crate::model::definitions::Definition;
use crate::model::identifiers::{Identifier, IdentifierReference};
use crate::model::modules::Module;
use crate::model::{HasName, HasSourceSpan, References, Span};
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
    name: Identifier,
    kind: MemberKind,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum MemberKind {
    PropertyReference(IdentifierReference),
    Definition(MemberDef),
}

/// Corresponds to the definition component within grammar rule `by_reference_member`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct MemberDef {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    inverse_name: Option<Identifier>,
    target_cardinality: Cardinality,
    target_type: TypeReference,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    body: Option<AnnotationOnlyBody>,
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Members ❱ Member
// ------------------------------------------------------------------------------------------------

impl_has_name_for!(Member);

impl_has_source_span_for!(Member);

impl MaybeIncomplete for Member {
    fn is_incomplete(&self, top: &Module, cache: &ModuleCache) -> bool {
        match &self.kind {
            MemberKind::PropertyReference(name) => {
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
        cache: &ModuleCache,
        loader: &impl ModuleLoader,
        check_constraints: bool,
    ) {
        self.name()
            .validate(top, loader, Some(IdentifierCaseConvention::Member));
        match &self.kind {
            MemberKind::PropertyReference(name) => {
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
                            self.name(),
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
            MemberKind::PropertyReference(v) => {
                names.insert(v);
            }
            MemberKind::Definition(v) => v.referenced_annotations(names),
        }
    }

    fn referenced_types<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        match &self.kind {
            MemberKind::PropertyReference(v) => {
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

    pub const fn new_property_reference(
        role: Identifier,
        in_property: IdentifierReference,
    ) -> Self {
        Self {
            span: None,
            name: role,
            kind: MemberKind::PropertyReference(in_property),
        }
    }

    pub const fn new_definition(name: Identifier, definition: MemberDef) -> Self {
        Self {
            span: None,
            name,
            kind: MemberKind::Definition(definition),
        }
    }

    // --------------------------------------------------------------------------------------------
    // Member :: Variants
    // --------------------------------------------------------------------------------------------

    pub const fn kind(&self) -> &MemberKind {
        &self.kind
    }

    pub const fn is_property_reference(&self) -> bool {
        matches!(self.kind, MemberKind::PropertyReference(_))
    }

    pub const fn as_property_reference(&self) -> Option<&IdentifierReference> {
        if let MemberKind::PropertyReference(v) = &self.kind {
            Some(v)
        } else {
            None
        }
    }

    pub const fn is_definition(&self) -> bool {
        matches!(self.kind, MemberKind::Definition(_))
    }

    pub const fn as_definition(&self) -> Option<&MemberDef> {
        if let MemberKind::Definition(v) = &self.kind {
            Some(v)
        } else {
            None
        }
    }
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
    fn is_incomplete(&self, top: &Module, cache: &ModuleCache) -> bool {
        self.target_type.is_incomplete(top, cache)
    }
}

impl Validate for MemberDef {
    fn validate(
        &self,
        top: &Module,
        cache: &ModuleCache,
        loader: &impl ModuleLoader,
        check_constraints: bool,
    ) {
        // TODO: check inverse name exists
        // TODO: check target type exists
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

    pub fn new<T>(target_type: T) -> Self
    where
        T: Into<TypeReference>,
    {
        Self {
            span: None,
            target_type: target_type.into(),
            target_cardinality: Cardinality::one(),
            inverse_name: None,
            body: None,
        }
    }

    pub const fn new_unknown() -> Self {
        Self {
            span: None,
            target_type: TypeReference::Unknown,
            target_cardinality: Cardinality::one(),
            inverse_name: None,
            body: None,
        }
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    get_and_set!(pub inverse_name, set_inverse_name, unset_inverse_name => optional has_inverse_name, Identifier);
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
