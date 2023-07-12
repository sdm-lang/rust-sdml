use super::{ConstraintBody, Identifier, IdentifierReference, Span, Value};
use std::{collections::HashSet, fmt::Debug};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// Corresponds to the grammar rule `line_comment`.
#[derive(Clone, Debug)]
pub struct Comment {
    span: Option<Span>,
    value: String,
}

/// Corresponds to the grammar rule `annotation`.
#[derive(Clone, Debug)]
#[allow(clippy::large_enum_variant)] // TODO: why is this reported as an issue?
pub enum Annotation {
    Property(AnnotationProperty),
    Constraint(Constraint),
}

/// Corresponds to the grammar rule `annotation_property`.
#[derive(Clone, Debug)]
pub struct AnnotationProperty {
    span: Option<Span>,
    comments: Vec<Comment>,
    name: IdentifierReference,
    value: Value,
}

/// Corresponds to the grammar rule `constraint`.
#[derive(Clone, Debug)]
pub struct Constraint {
    span: Option<Span>,
    comments: Vec<Comment>,
    name: Option<Identifier>,
    body: ConstraintBody,
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
// Implementations ❱ Comments
// ------------------------------------------------------------------------------------------------

impl From<String> for Comment {
    fn from(v: String) -> Self {
        Self::new(&v)
    }
}

impl From<&str> for Comment {
    fn from(v: &str) -> Self {
        Self::new(v)
    }
}

simple_display_impl!(Comment, value);
as_str_impl!(Comment, value);
into_string_impl!(Comment, value);

impl PartialEq for Comment {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Comment {}

impl Comment {
    pub fn new(s: &str) -> Self {
        Self {
            span: None,
            value: s.to_string(),
        }
    }

    with!(pub span (ts_span) => option Span);
    get_and_mutate!(pub span (ts_span) => option Span);

    pub fn eq_with_span(&self, other: &Self) -> bool {
        self.span == other.span && self.value == other.value
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Annotations
// ------------------------------------------------------------------------------------------------

impl From<AnnotationProperty> for Annotation {
    fn from(value: AnnotationProperty) -> Self {
        Self::Property(value)
    }
}

impl From<Constraint> for Annotation {
    fn from(value: Constraint) -> Self {
        Self::Constraint(value)
    }
}

impl Annotation {
    is_as_variant!(pub annotation_property => Property, AnnotationProperty);
    is_as_variant!(pub constraint => Constraint, Constraint);

    pub fn has_ts_span(&self) -> bool {
        match self {
            Annotation::Property(v) => v.has_ts_span(),
            Annotation::Constraint(v) => v.has_ts_span(),
        }
    }

    pub fn ts_span(&self) -> Option<&Span> {
        match self {
            Annotation::Property(v) => v.ts_span(),
            Annotation::Constraint(v) => v.ts_span(),
        }
    }

    pub fn referenced_types(&self) -> HashSet<&IdentifierReference> {
        Default::default()
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Annotations ❱ Annotation Properties
// ------------------------------------------------------------------------------------------------

impl AnnotationProperty {
    pub fn new(name: IdentifierReference, value: Value) -> Self {
        Self {
            span: None,
            comments: Default::default(),
            name,
            value,
        }
    }

    // --------------------------------------------------------------------------------------------

    with!(pub span (ts_span) => option Span);
    get_and_mutate!(pub span (ts_span) => option Span);

    get_and_mutate_collection_of!(pub comments => Vec, Comment);

    get_and_mutate!(pub name => IdentifierReference);

    get_and_mutate!(pub value => Value);
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Annotations ❱ Constraints
// ------------------------------------------------------------------------------------------------

impl<B> From<B> for Constraint
where
    B: Into<ConstraintBody>,
{
    fn from(value: B) -> Self {
        Self::anonymous(value)
    }
}

impl Constraint {
    pub fn new<B>(name: Option<Identifier>, body: B) -> Self
    where
        B: Into<ConstraintBody>,
    {
        Self {
            span: None,
            comments: Default::default(),
            name,
            body: body.into(),
        }
    }

    pub fn named<B>(name: Identifier, body: B) -> Self
    where
        B: Into<ConstraintBody>,
    {
        Self::new(Some(name), body)
    }

    pub fn anonymous<B>(body: B) -> Self
    where
        B: Into<ConstraintBody>,
    {
        Self::new(None, body)
    }

    // --------------------------------------------------------------------------------------------

    with!(pub span (ts_span) => option Span);
    get_and_mutate!(pub span (ts_span) => option Span);

    get_and_mutate_collection_of!(pub comments => Vec, Comment);

    get_and_mutate!(pub name => option Identifier);

    get_and_mutate!(pub body => ConstraintBody);
}

// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
