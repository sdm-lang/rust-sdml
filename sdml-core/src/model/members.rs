use super::{AnnotationOnlyBody, Comment, Identifier, IdentifierReference, Span};
use std::{collections::HashSet, fmt::Debug};

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

macro_rules! member_types {
    ($outer: ident, $inner: ident, $def: ident $(, $addname: ident, $addtype: ty )*) => {
        #[derive(Clone, Debug)]
        pub struct $outer {
            span: Option<Span>,
            comments: Vec<Comment>,
            name: Identifier,
            inner: $inner,
        }

        #[derive(Clone, Debug)]
        pub enum $inner {
            PropertyRole(Identifier),
            Defined($def),
        }

        #[derive(Clone, Debug)]
        pub struct $def {
            target_type: TypeReference,
            $(
                $addname: Option<$addtype>,
            )*
            body: Option<AnnotationOnlyBody>,
        }
    };
}

macro_rules! member_impl {
    ($outer: ident, $inner: ident, $def: ident $(, $addname: ident, $addtype: ty )*) => {
        impl $outer {
            pub fn new_with_role(name: Identifier, role: Identifier) -> Self {
                Self {
                    span: None,
                    comments: Default::default(),
                    name,
                    inner: role.into(),
                }
            }

            pub fn new_with_definition(name: Identifier, def: $def) -> Self {
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

            get_and_mutate!(pub inner => $inner);

            delegate_is_as_variant!(pub property_role, inner => $inner, PropertyRole, Identifier);

            delegate_is_as_variant!(pub defined, inner => $inner, Defined, $def);

            pub fn is_complete(&self) -> bool {
                self.inner.is_complete()
            }
        }

        impl From<Identifier> for $inner {
            fn from(value: Identifier) -> Self {
                Self::PropertyRole(value)
            }
        }

        impl From<$def> for $inner {
            fn from(value: $def) -> Self {
                Self::Defined(value)
            }
        }

        impl $inner {
            is_as_variant!(pub property_role => PropertyRole, Identifier);
            is_as_variant!(pub defined => Defined, $def);

            pub fn is_complete(&self) -> bool {
                if let Self::Defined(defined) = self {
                   defined.is_complete()
                } else {
                    true
                }
            }
        }

        impl $def {
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
    };
}
// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Members
// ------------------------------------------------------------------------------------------------

member_types!(IdentityMember, IdentityMemberInner, IdentityMemberDef);

member_types!(
    ByValueMember,
    ByValueMemberInner,
    ByValueMemberDef,
    target_cardinality,
    Cardinality
);

member_types!(
    ByReferenceMember,
    ByReferenceMemberInner,
    ByReferenceMemberDef,
    source_cardinality,
    Cardinality,
    target_cardinality,
    Cardinality
);

#[derive(Clone, Debug)]
pub enum TypeReference {
    Reference(IdentifierReference),
    Unknown,
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Members ❱ Cardinality
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq)]
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

member_impl!(IdentityMember, IdentityMemberInner, IdentityMemberDef);

member_impl!(
    ByValueMember,
    ByValueMemberInner,
    ByValueMemberDef,
    target_cardinality,
    Cardinality
);

member_impl!(
    ByReferenceMember,
    ByReferenceMemberInner,
    ByReferenceMemberDef,
    source_cardinality,
    Cardinality,
    target_cardinality,
    Cardinality
);

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
        self.max.map(|i| i != self.min).unwrap_or_default()
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
// Implementations ❱ Members ❱ Type Reference
// ------------------------------------------------------------------------------------------------

impl TypeReference {
    is_as_variant!(pub reference => Reference, IdentifierReference);

    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }

    pub fn is_complete(&self) -> bool {
        self.is_reference()
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
