use super::{IdentifierReference, Span};
use crate::error::invalid_language_tag_error;
use lazy_static::lazy_static;
use ordered_float::OrderedFloat;
use regex::Regex;
use rust_decimal::Decimal;
use std::{
    fmt::{Debug, Display},
    str::FromStr,
};
use url::Url;

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Values
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub enum Value {
    Simple(SimpleValue),
    ValueConstructor(ValueConstructor),
    Reference(IdentifierReference),
    List(ListOfValues),
}

#[derive(Clone, Debug)]
pub enum SimpleValue {
    String(LanguageString),
    Double(OrderedFloat<f64>),
    Decimal(Decimal),
    Integer(i64),
    Boolean(bool),
    IriReference(Url),
}

#[derive(Clone, Debug)]
pub struct LanguageString {
    span: Option<Span>,
    value: String,
    language: Option<LanguageTag>,
}

#[derive(Clone, Debug)]
pub struct LanguageTag {
    span: Option<Span>,
    value: String,
}

#[derive(Clone, Debug, Default)]
pub struct ListOfValues {
    span: Option<Span>,
    values: Vec<ListMember>,
}

#[derive(Clone, Debug)]
pub enum ListMember {
    Simple(SimpleValue),
    ValueConstructor(ValueConstructor),
    Reference(IdentifierReference),
}

#[derive(Clone, Debug)]
pub struct ValueConstructor {
    span: Option<Span>,
    type_name: IdentifierReference,
    value: SimpleValue,
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

lazy_static! {
    static ref LANGUAGE_TAG: Regex =
        Regex::new(r"^[a-z]{2,3}(-[A-Z]{3})?(-[A-Z][a-z]{3})?(-([A-Z]{2}|[0-9]{3}))?$").unwrap();
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Annotations ❱ Values
// ------------------------------------------------------------------------------------------------

impl From<SimpleValue> for Value {
    fn from(v: SimpleValue) -> Self {
        Self::Simple(v)
    }
}

impl From<LanguageString> for Value {
    fn from(v: LanguageString) -> Self {
        Self::Simple(SimpleValue::String(v))
    }
}

impl From<f64> for Value {
    fn from(v: f64) -> Self {
        Self::Simple(SimpleValue::Double(v.into()))
    }
}

impl From<OrderedFloat<f64>> for Value {
    fn from(v: OrderedFloat<f64>) -> Self {
        Self::Simple(SimpleValue::Double(v))
    }
}

impl From<Decimal> for Value {
    fn from(v: Decimal) -> Self {
        Self::Simple(SimpleValue::Decimal(v))
    }
}

impl From<i64> for Value {
    fn from(v: i64) -> Self {
        Self::Simple(SimpleValue::Integer(v))
    }
}

impl From<bool> for Value {
    fn from(v: bool) -> Self {
        Self::Simple(SimpleValue::Boolean(v))
    }
}

impl From<Url> for Value {
    fn from(v: Url) -> Self {
        Self::Simple(SimpleValue::IriReference(v))
    }
}

impl From<ValueConstructor> for Value {
    fn from(v: ValueConstructor) -> Self {
        Self::ValueConstructor(v)
    }
}

impl From<IdentifierReference> for Value {
    fn from(v: IdentifierReference) -> Self {
        Self::Reference(v)
    }
}

impl From<ListOfValues> for Value {
    fn from(v: ListOfValues) -> Self {
        Self::List(v)
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Value::Simple(v) => v.to_string(),
                Value::ValueConstructor(v) => v.to_string(),
                Value::Reference(v) => v.to_string(),
                Value::List(v) => v.to_string(),
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl_from_for_variant!(SimpleValue, String, LanguageString);

impl_from_for_variant!(SimpleValue, Double, OrderedFloat<f64>);

impl_from_for_variant!(SimpleValue, Decimal, Decimal);

impl_from_for_variant!(SimpleValue, Integer, i64);

impl_from_for_variant!(SimpleValue, Boolean, bool);

impl_from_for_variant!(SimpleValue, IriReference, Url);

impl Display for SimpleValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::String(v) => v.to_string(),
                Self::Double(v) => v.to_string(),
                Self::Decimal(v) => v.to_string(),
                Self::Integer(v) => v.to_string(),
                Self::Boolean(v) => v.to_string(),
                Self::IriReference(v) => v.to_string(),
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for LanguageString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}{}",
            self.value,
            if let Some(language) = &self.language {
                language.to_string()
            } else {
                String::new()
            }
        )
    }
}

impl From<String> for LanguageString {
    fn from(v: String) -> Self {
        Self::new(&v, None)
    }
}

impl From<&str> for LanguageString {
    fn from(v: &str) -> Self {
        Self::new(v, None)
    }
}

impl PartialEq for LanguageString {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.language == other.language
    }
}

impl Eq for LanguageString {}

impl LanguageString {
    pub fn new(value: &str, language: Option<LanguageTag>) -> Self {
        Self {
            span: None,
            value: value.to_string(),
            language,
        }
    }

    // --------------------------------------------------------------------------------------------

    with!(pub span (ts_span) => option Span);
    get_and_mutate!(pub span (ts_span) => option Span);

    get_and_mutate!(pub value => String);

    get_and_mutate!(pub language => option LanguageTag);

    // --------------------------------------------------------------------------------------------

    pub fn eq_with_span(&self, other: &Self) -> bool {
        self.span == other.span && self.value == other.value && self.language == other.language
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for LanguageTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "@{}", self.value)
    }
}

impl FromStr for LanguageTag {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Self::is_valid(s) {
            Ok(Self {
                span: None,
                value: s.to_string(),
            })
        } else {
            Err(invalid_language_tag_error(s))
        }
    }
}

into_string_impl!(LanguageTag, value);
as_str_impl!(LanguageTag, value);

impl PartialEq for LanguageTag {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for LanguageTag {}

impl LanguageTag {
    #[allow(dead_code)]
    pub fn new_unchecked(s: &str) -> Self {
        Self {
            span: None,
            value: s.to_string(),
        }
    }

    // --------------------------------------------------------------------------------------------

    with!(pub span (ts_span) => option Span);
    get_and_mutate!(pub span (ts_span) => option Span);

    // --------------------------------------------------------------------------------------------

    pub fn is_valid(s: &str) -> bool {
        LANGUAGE_TAG.is_match(s)
    }

    // --------------------------------------------------------------------------------------------

    pub fn eq_with_span(&self, other: &Self) -> bool {
        self.span == other.span && self.value == other.value
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for ListOfValues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}]",
            self.values
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

impl From<Vec<ListMember>> for ListOfValues {
    fn from(values: Vec<ListMember>) -> Self {
        Self { span: None, values }
    }
}

impl FromIterator<ListMember> for ListOfValues {
    fn from_iter<T: IntoIterator<Item = ListMember>>(iter: T) -> Self {
        Self::from(Vec::from_iter(iter))
    }
}

impl ListOfValues {
    with!(pub span (ts_span) => option Span);
    get_and_mutate!(pub span (ts_span) => option Span);

    get_and_mutate_collection_of!(pub values => Vec, ListMember);
}

// ------------------------------------------------------------------------------------------------

impl_from_for_variant!(ListMember, Simple, SimpleValue);

impl_from_for_variant!(ListMember, ValueConstructor, ValueConstructor);

impl_from_for_variant!(ListMember, Reference, IdentifierReference);

enum_display_impl!(ListMember => Simple, ValueConstructor, Reference);

// ------------------------------------------------------------------------------------------------

impl Display for ValueConstructor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.type_name, self.value)
    }
}

impl ValueConstructor {
    pub fn new(type_name: IdentifierReference, value: SimpleValue) -> Self {
        Self {
            span: None,
            type_name,
            value,
        }
    }

    // --------------------------------------------------------------------------------------------

    with!(pub span (ts_span) => option Span);
    get_and_mutate!(pub span (ts_span) => option Span);

    get_and_mutate!(pub type_name => IdentifierReference);

    get_and_mutate!(pub value => SimpleValue);
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
