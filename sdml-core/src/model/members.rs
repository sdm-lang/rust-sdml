use super::{
    AnnotationOnlyBody, Comment, Identifier, IdentifierReference, QualifiedIdentifier, Span,
};
use std::{collections::HashSet, fmt::Debug};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

macro_rules! member_types {
    ($prefix: ident, $rule: literal $(, $addname: ident, $addtype: ty )*) => {
        paste::paste! {
            #[doc = "Corresponds to the grammar rule `" $rule "_member`."]
            #[derive(Clone, Debug)]
            #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
            pub struct [< $prefix Member >] {
                span: Option<Span>,
                comments: Vec<Comment>,
                name: Identifier,
                inner: [< $prefix MemberInner >],
            }

            #[doc = "Corresponds to the choice component within grammar rule `" $rule "_member`."]
            #[derive(Clone, Debug)]
            #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
            pub enum [< $prefix MemberInner >] {
                PropertyRole(Identifier),
                Defined([< $prefix MemberDef >]),
            }

            #[doc = "Corresponds to the definition component within grammar rule `" $rule "_member`."]
            #[derive(Clone, Debug)]
            #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
            pub struct [< $prefix MemberDef >] {
                target_type: TypeReference,
                $(
                    $addname: Option<$addtype>,
                )*
                    body: Option<AnnotationOnlyBody>,
            }
        }
    };
}

macro_rules! member_impl {
    ($prefix: ident $(, $addname: ident, $addtype: ty )*) => {
        paste::paste! {
            impl [< $prefix Member >] {
                pub fn new_with_role(name: Identifier, role: Identifier) -> Self {
                    Self {
                        span: None,
                        comments: Default::default(),
                        name,
                        inner: role.into(),
                    }
                }

                pub fn new_with_definition(name: Identifier, def: [< $prefix MemberDef >]) -> Self {
                    Self {
                        span: None,
                        comments: Default::default(),
                        name,
                        inner: def.into(),
                    }
                }

                with!(pub span (ts_span) => option Span);
                get_and_mutate!(pub span (ts_span) => option Span);

                get_and_mutate_collection_of!(pub comments => Vec, Comment);

                get_and_mutate!(pub name => Identifier);

                get_and_mutate!(pub inner => [< $prefix MemberInner >]);

                delegate_is_as_variant!(pub property_role, inner => [< $prefix MemberInner >], PropertyRole, Identifier);

                delegate_is_as_variant!(pub defined, inner => [< $prefix MemberInner >], Defined, [< $prefix MemberDef >]);

                pub fn is_complete(&self) -> bool {
                    self.inner.is_complete()
                }
            }

            impl From<Identifier> for [< $prefix MemberInner >] {
                fn from(value: Identifier) -> Self {
                    Self::PropertyRole(value)
                }
            }

            impl From<[< $prefix MemberDef >]> for [< $prefix MemberInner >] {
                fn from(value: [< $prefix MemberDef >]) -> Self {
                    Self::Defined(value)
                }
            }

            impl [< $prefix MemberInner >] {
                is_as_variant!(pub property_role => PropertyRole, Identifier);
                is_as_variant!(pub defined => Defined, [< $prefix MemberDef >]);

                pub fn is_complete(&self) -> bool {
                    if let Self::Defined(defined) = self {
                        defined.is_complete()
                    } else {
                        true
                    }
                }
            }

            impl [< $prefix MemberDef >] {
                pub fn new(target_type: TypeReference) -> Self {
                    Self {
                        target_type,
                        body: None,
                        $(
                            $addname: Default::default(),
                        )*
                    }
                }

                get_and_mutate!(pub target_type => TypeReference);

                $(
                    with!(pub $addname => option $addtype);
                    get_and_mutate!(pub $addname => option $addtype);
                )*

                    get_and_mutate!(pub body => option AnnotationOnlyBody);

                referenced_optional_body_annotations!();

                pub fn is_complete(&self) -> bool {
                    self.target_type().is_complete()
                }
            }
        }
    };
}
// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Members
// ------------------------------------------------------------------------------------------------

member_types!(Identity, "identity");

member_types!(ByValue, "by_value", target_cardinality, Cardinality);

member_types!(
    ByReference,
    "by_reference",
    inverse_name,
    Identifier,
    target_cardinality,
    Cardinality
);

/// Corresponds to the grammar rule `type_reference`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum TypeReference {
    Reference(IdentifierReference),
    Unknown,
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Members ❱ Cardinality
// ------------------------------------------------------------------------------------------------

/// Corresponds to the grammar rule `cardinality`.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Cardinality {
    span: Option<Span>,
    min: u32,
    max: Option<u32>,
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
// Implementations ❱ Members
// ------------------------------------------------------------------------------------------------

member_impl!(Identity);

member_impl!(ByValue, target_cardinality, Cardinality);

member_impl!(
    ByReference,
    inverse_name,
    Identifier,
    target_cardinality,
    Cardinality
);

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Members ❱ Type Reference
// ------------------------------------------------------------------------------------------------

impl From<IdentifierReference> for TypeReference {
    fn from(value: IdentifierReference) -> Self {
        Self::Reference(value)
    }
}

impl From<Identifier> for TypeReference {
    fn from(value: Identifier) -> Self {
        Self::Reference(value.into())
    }
}

impl From<QualifiedIdentifier> for TypeReference {
    fn from(value: QualifiedIdentifier) -> Self {
        Self::Reference(value.into())
    }
}

impl TypeReference {
    is_as_variant!(pub reference => Reference, IdentifierReference);

    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }

    pub fn is_complete(&self) -> bool {
        !self.is_unknown()
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Members ❱ Cardinality
// ------------------------------------------------------------------------------------------------

impl Cardinality {
    pub fn new_range(min: u32, max: u32) -> Self {
        Self {
            span: None,
            min,
            max: Some(max),
        }
    }

    pub fn new_unbounded(min: u32) -> Self {
        Self {
            span: None,
            min,
            max: None,
        }
    }

    pub fn new_single(min_and_max: u32) -> Self {
        Self {
            span: None,
            min: min_and_max,
            max: Some(min_and_max),
        }
    }

    pub fn value_target_default() -> Self {
        Self {
            span: None,
            min: 1,
            max: Some(1),
        }
    }

    pub fn ref_source_default() -> Self {
        Self {
            span: None,
            min: 0,
            max: None,
        }
    }
    pub fn ref_target_default() -> Self {
        Self {
            span: None,
            min: 0,
            max: Some(1),
        }
    }

    // --------------------------------------------------------------------------------------------

    with!(pub span (ts_span) => option Span);
    get_and_mutate!(pub span (ts_span) => option Span);

    // --------------------------------------------------------------------------------------------

    pub fn min_occurs(&self) -> u32 {
        self.min
    }

    pub fn max_occurs(&self) -> Option<u32> {
        self.max
    }

    pub fn is_range(&self) -> bool {
        self.max.map(|i| i != self.min).unwrap_or(true)
    }

    pub fn to_uml_string(&self) -> String {
        if self.is_range() {
            format!(
                "{}..{}",
                self.min,
                self.max
                    .map(|i| i.to_string())
                    .unwrap_or_else(|| "*".to_string())
            )
        } else {
            self.min.to_string()
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
