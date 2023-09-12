use crate::{
    error::Error,
    model::{
        annotations::{Annotation, AnnotationOnlyBody, HasAnnotations},
        check::Validate,
        definitions::{HasMembers, HasVariants, TypeVariant},
        identifiers::{Identifier, IdentifierReference},
        members::{Cardinality, TypeReference, DEFAULT_BY_VALUE_CARDINALITY},
        modules::Module,
        References, Span,
    },
};
use std::{collections::HashSet, fmt::Debug};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Feature Sets
// ------------------------------------------------------------------------------------------------

/// Corresponds to the grammar rule `union_def`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct FeatureSetDef {
    span: Option<Span>,
    name: Identifier,
    body: Option<FeatureSetBody>,
}

/// Corresponds to the inner part of the grammar rule `entity_group`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum FeatureSetBody {
    Conjunctive(FeatureSetProductBody),
    Disjunctive(FeatureSetSumBody),
    ExclusiveDisjunction(FeatureSetSumBody),
}

/// Corresponds to the grammar rule `union_body`.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct FeatureSetSumBody {
    span: Option<Span>,
    annotations: Vec<Annotation>,
    variants: Vec<TypeVariant>, // assert!(!variants.is_empty());
}

/// Corresponds to the grammar rule `union_body`.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct FeatureSetProductBody {
    span: Option<Span>,
    annotations: Vec<Annotation>,
    members: Vec<FeatureMemberDef>, // assert!(!variants.is_empty());
}

/// Corresponds to the grammar rule `union_body`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct FeatureMemberDef {
    span: Option<Span>,
    name: Identifier,
    target_cardinality: Cardinality,
    target_type: TypeReference,
    body: Option<AnnotationOnlyBody>,
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Type Definitions ❱ Feature Sets
// ------------------------------------------------------------------------------------------------

impl_has_name_for!(FeatureSetDef);

impl_has_optional_body_for!(FeatureSetDef, FeatureSetBody);

impl_has_source_span_for!(FeatureSetDef);

impl_references_for!(FeatureSetDef => delegate optional body);

impl_validate_for!(FeatureSetDef => delegate optional body, false, true);

// ------------------------------------------------------------------------------------------------

impl_has_annotations_for!(FeatureSetBody => variants Conjunctive, Disjunctive, ExclusiveDisjunction);

impl_has_source_span_for!(FeatureSetBody => variants Conjunctive, Disjunctive, ExclusiveDisjunction);

impl_references_for!(FeatureSetBody => variants Conjunctive, Disjunctive, ExclusiveDisjunction);

impl_validate_for!(FeatureSetBody => variants Conjunctive, Disjunctive, ExclusiveDisjunction);

impl FeatureSetBody {
    // --------------------------------------------------------------------------------------------
    // Variants
    // --------------------------------------------------------------------------------------------

    is_as_variant!(Conjunctive (FeatureSetProductBody) => is_conjunctive, as_conjunctive);

    is_as_variant!(Disjunctive (FeatureSetSumBody) => is_disjunctive, as_disjunctive);

    is_as_variant!(ExclusiveDisjunction (FeatureSetSumBody) => is_exclusive_disjunction, as_exclusive_disjunction);
}

// ------------------------------------------------------------------------------------------------

impl_has_annotations_for!(FeatureSetSumBody);

impl_has_source_span_for!(FeatureSetSumBody);

impl_has_variants_for!(FeatureSetSumBody, TypeVariant);

impl_validate_for_annotations_and_variants!(FeatureSetSumBody);

impl References for FeatureSetSumBody {
    fn referenced_types<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        self.variants().for_each(|m| m.referenced_types(names));
    }

    fn referenced_annotations<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        self.variants()
            .for_each(|m| m.referenced_annotations(names));
    }
}

// ------------------------------------------------------------------------------------------------

impl_has_annotations_for!(FeatureSetProductBody);

impl_has_source_span_for!(FeatureSetProductBody);

impl_has_members_for!(FeatureSetProductBody, FeatureMemberDef);

impl_validate_for_annotations_and_members!(FeatureSetProductBody);

impl References for FeatureSetProductBody {
    fn referenced_types<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        self.members().for_each(|m| m.referenced_types(names));
    }

    fn referenced_annotations<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        self.members().for_each(|m| m.referenced_annotations(names));
    }
}

// ------------------------------------------------------------------------------------------------

impl_has_cardinality_for!(FeatureMemberDef);

impl_has_name_for!(FeatureMemberDef);

impl_has_optional_body_for!(FeatureMemberDef);

impl_has_source_span_for!(FeatureMemberDef);

impl_has_type_for!(FeatureMemberDef);

impl_validate_for!(FeatureMemberDef => todo!);

impl References for FeatureMemberDef {
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

impl FeatureMemberDef {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn new<T>(name: Identifier, target_type: T) -> Self
    where
        T: Into<TypeReference>,
    {
        Self {
            span: Default::default(),
            name,
            target_cardinality: DEFAULT_BY_VALUE_CARDINALITY,
            target_type: target_type.into(),
            body: None,
        }
    }

    pub fn new_unknown(name: Identifier) -> Self {
        Self {
            span: Default::default(),
            name,
            target_cardinality: DEFAULT_BY_VALUE_CARDINALITY,
            target_type: TypeReference::Unknown,
            body: None,
        }
    }
}
