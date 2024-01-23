use crate::cache::ModuleCache;
use crate::error::Error;
use crate::model::check::Validate;
use crate::model::constraints::ConstraintSentence;
use crate::model::identifiers::{Identifier, IdentifierReference};
use crate::model::members::{CardinalityRange, MappingType, Ordering, Uniqueness};
use crate::model::modules::Module;
use crate::model::Span;
use crate::syntax::KW_WILDCARD;
use std::fmt::Display;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Formal Constraints ❱ Environments ❱ Functions
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct FunctionDef {
    span: Option<Span>,
    signature: FunctionSignature,
    body: ConstraintSentence,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct FunctionSignature {
    span: Option<Span>,
    parameters: Vec<FunctionParameter>,
    target_type: FunctionType,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct FunctionParameter {
    span: Option<Span>,
    name: Identifier,
    target_type: FunctionType,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct FunctionType {
    span: Option<Span>,
    target_cardinality: FunctionCardinality,
    target_type: FunctionTypeReference,
}

/// Corresponds to the grammar rule `cardinality`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct FunctionCardinality {
    span: Option<Span>,
    ordering: Option<Ordering>,
    uniqueness: Option<Uniqueness>,
    range: Option<CardinalityRange>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct FunctionTypeReference {
    optional: bool,
    inner: FunctionTypeReferenceInner,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum FunctionTypeReferenceInner {
    Wildcard,
    Reference(IdentifierReference),
    // builtin_simple_type is converted into a reference
    MappingType(MappingType),
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Formal Constraints ❱ Environments ❱ Functions
// ------------------------------------------------------------------------------------------------

impl_has_body_for!(FunctionDef, ConstraintSentence);

impl_has_source_span_for!(FunctionDef);

impl FunctionDef {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub const fn new(signature: FunctionSignature, body: ConstraintSentence) -> Self {
        Self {
            span: None,
            signature,
            body,
        }
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    get_and_set!(pub signature, set_signature => FunctionSignature);
}

// ------------------------------------------------------------------------------------------------

impl_has_source_span_for!(FunctionSignature);

impl FunctionSignature {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn new(parameters: Vec<FunctionParameter>, target_type: FunctionType) -> Self {
        Self {
            span: Default::default(),
            parameters,
            target_type,
        }
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    get_and_set_vec!(
        pub
        has has_parameters,
        parameters_len,
        parameters,
        parameters_mut,
        add_to_parameters,
        extend_parameters
            => parameters, FunctionParameter
    );

    get_and_set!(pub target_type, set_target_type => FunctionType);
}

// ------------------------------------------------------------------------------------------------

impl_has_name_for!(FunctionParameter);

impl_has_source_span_for!(FunctionParameter);

impl FunctionParameter {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub const fn new(name: Identifier, target_type: FunctionType) -> Self {
        Self {
            span: None,
            name,
            target_type,
        }
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    get_and_set!(pub target_type, set_target_type => FunctionType);
}

// ------------------------------------------------------------------------------------------------

impl_has_source_span_for!(FunctionType);

impl FunctionType {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn new(
        target_cardinality: FunctionCardinality,
        target_type: FunctionTypeReference,
    ) -> Self {
        Self {
            span: Default::default(),
            target_cardinality,
            target_type,
        }
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    pub fn with_wildcard_cardinality(self) -> Self {
        Self {
            target_cardinality: FunctionCardinality::new_wildcard(),
            ..self
        }
    }

    pub fn with_target_cardinality(self, target_cardinality: FunctionCardinality) -> Self {
        Self {
            target_cardinality,
            ..self
        }
    }

    get_and_set!(pub target_cardinality, set_target_cardinality => FunctionCardinality);

    pub fn with_target_type(self, target_type: FunctionTypeReference) -> Self {
        Self {
            target_type,
            ..self
        }
    }

    get_and_set!(pub target_type, set_target_type => FunctionTypeReference);
}

// ------------------------------------------------------------------------------------------------

impl Display for FunctionCardinality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{{}{}{}}}",
            self.ordering.map(|c| format!("{} ", c)).unwrap_or_default(),
            self.uniqueness
                .map(|c| format!("{} ", c))
                .unwrap_or_default(),
            if let Some(range) = &self.range {
                range.to_string()
            } else {
                KW_WILDCARD.to_string()
            }
        )
    }
}

impl_has_source_span_for!(FunctionCardinality);

impl Validate for FunctionCardinality {
    fn is_complete(&self, top: &Module, cache: &ModuleCache) -> Result<bool, Error> {
        if let Some(range) = &self.range {
            range.is_complete(top, cache)
        } else {
            Ok(true)
        }
    }

    fn is_valid(
        &self,
        check_constraints: bool,
        top: &Module,
        cache: &ModuleCache,
    ) -> Result<bool, Error> {
        if let Some(range) = &self.range {
            range.is_valid(check_constraints, top, cache)
        } else {
            Ok(true)
        }
    }
}

impl FunctionCardinality {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub const fn new(
        ordering: Option<Ordering>,
        uniqueness: Option<Uniqueness>,
        range: Option<CardinalityRange>,
    ) -> Self {
        Self {
            span: None,
            ordering,
            uniqueness,
            range,
        }
    }

    pub const fn new_range(min: u32, max: u32) -> Self {
        Self {
            span: None,
            ordering: None,
            uniqueness: None,
            range: Some(CardinalityRange::new_range(min, max)),
        }
    }

    pub const fn new_unbounded(min: u32) -> Self {
        Self {
            span: None,
            ordering: None,
            uniqueness: None,
            range: Some(CardinalityRange::new_unbounded(min)),
        }
    }

    pub const fn new_single(min_and_max: u32) -> Self {
        Self {
            span: None,
            ordering: None,
            uniqueness: None,
            range: Some(CardinalityRange::new_single(min_and_max)),
        }
    }

    pub const fn new_wildcard() -> Self {
        Self {
            span: None,
            ordering: None,
            uniqueness: None,
            range: None,
        }
    }

    #[inline(always)]
    pub const fn one() -> Self {
        Self::new_single(1)
    }

    #[inline(always)]
    pub const fn zero_or_one() -> Self {
        Self::new_range(0, 1)
    }

    #[inline(always)]
    pub const fn one_or_more() -> Self {
        Self::new_unbounded(1)
    }

    #[inline(always)]
    pub const fn zero_or_more() -> Self {
        Self::new_unbounded(0)
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    pub const fn with_ordering(self, ordering: Ordering) -> Self {
        Self {
            ordering: Some(ordering),
            ..self
        }
    }

    get_and_set!(pub ordering, set_ordering, unset_ordering => optional copy has_ordering, Ordering);

    #[inline(always)]
    pub fn is_ordered(&self) -> Option<bool> {
        self.ordering().map(|o| o == Ordering::Ordered)
    }

    // --------------------------------------------------------------------------------------------

    #[inline(always)]
    pub const fn with_uniqueness(self, uniqueness: Uniqueness) -> Self {
        Self {
            uniqueness: Some(uniqueness),
            ..self
        }
    }

    get_and_set!(pub uniqueness, set_uniqueness, unset_uniqueness => optional copy has_uniqueness, Uniqueness);

    #[inline(always)]
    pub fn is_unique(&self) -> Option<bool> {
        self.uniqueness().map(|u| u == Uniqueness::Unique)
    }

    // --------------------------------------------------------------------------------------------

    get_and_set!(pub range, set_range, unset_range => optional has_range, CardinalityRange);

    pub fn is_wildcard(&self) -> bool {
        self.range.is_none()
    }
}

// ------------------------------------------------------------------------------------------------

impl From<FunctionTypeReferenceInner> for FunctionTypeReference {
    fn from(inner: FunctionTypeReferenceInner) -> Self {
        Self {
            optional: false,
            inner,
        }
    }
}

impl AsRef<FunctionTypeReferenceInner> for FunctionTypeReference {
    fn as_ref(&self) -> &FunctionTypeReferenceInner {
        &self.inner
    }
}

impl FunctionTypeReference {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn optional(inner: FunctionTypeReferenceInner) -> Self {
        Self {
            optional: true,
            inner,
        }
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    pub fn is_optional(&self) -> bool {
        self.optional
    }

    pub fn inner(&self) -> &FunctionTypeReferenceInner {
        &self.inner
    }
}

// ------------------------------------------------------------------------------------------------

impl From<IdentifierReference> for FunctionTypeReferenceInner {
    fn from(value: IdentifierReference) -> Self {
        Self::Reference(value)
    }
}

impl From<MappingType> for FunctionTypeReferenceInner {
    fn from(value: MappingType) -> Self {
        Self::MappingType(value)
    }
}

impl FunctionTypeReferenceInner {
    // --------------------------------------------------------------------------------------------
    // Variants
    // --------------------------------------------------------------------------------------------

    is_as_variant!(Reference (IdentifierReference) => is_type_reference, as_type_reference);

    is_as_variant!(MappingType (MappingType) => is_mapping_type, as_mapping_type);

    is_variant!(Wildcard => is_wildcard);
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
