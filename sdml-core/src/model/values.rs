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

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Values
// ------------------------------------------------------------------------------------------------

/// Corresponds to the grammar rule `value`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Value {
    Simple(SimpleValue),
    ValueConstructor(ValueConstructor),
    Reference(IdentifierReference),
    Mapping(MappingValue),
    List(ListOfValues),
}

/// Corresponds to the grammar rule `simple_value`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum SimpleValue {
    /// Corresponds to the grammar rule `string`.
    String(LanguageString),
    /// Corresponds to the grammar rule `double`.
    Double(OrderedFloat<f64>),
    /// Corresponds to the grammar rule `decimal`.
    Decimal(Decimal),
    /// Corresponds to the grammar rule `integer`.
    Integer(i64),
    /// Corresponds to the grammar rule `boolean`.
    Boolean(bool),
    /// Corresponds to the grammar rule `iri_reference`.
    IriReference(Url),
}

/// Corresponds to the grammar rule `string`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct LanguageString {
    span: Option<Span>,
    /// Corresponds to the grammar rule `quoted_string`.
    value: String,
    language: Option<LanguageTag>,
}

/// Corresponds to the grammar rule `language_tag`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct LanguageTag {
    span: Option<Span>,
    value: String,
}

/// Corresponds to the grammar rule `mapping_value`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct MappingValue {
    span: Option<Span>,
    domain: SimpleValue,
    range: Box<Value>,
}

/// Corresponds to the grammar rule `list_of_values`.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ListOfValues {
    span: Option<Span>,
    values: Vec<ListMember>,
}

/// Corresponds to the grammar rule `name`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum ListMember {
    Simple(SimpleValue),
    ValueConstructor(ValueConstructor),
    Reference(IdentifierReference),
    Mapping(MappingValue),
}

/// Corresponds to the grammar rule `value_constructor`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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

impl From<MappingValue> for Value {
    fn from(v: MappingValue) -> Self {
        Self::Mapping(v)
    }
}

impl From<ListOfValues> for Value {
    fn from(v: ListOfValues) -> Self {
        Self::List(v)
    }
}

enum_display_impl!(Value => Simple, ValueConstructor, Reference, Mapping, List);

// ------------------------------------------------------------------------------------------------

impl_from_for_variant!(SimpleValue, String, LanguageString);

impl_from_for_variant!(SimpleValue, Double, OrderedFloat<f64>);

impl_from_for_variant!(SimpleValue, Decimal, Decimal);

impl_from_for_variant!(SimpleValue, Integer, i64);

impl_from_for_variant!(SimpleValue, Boolean, bool);

impl_from_for_variant!(SimpleValue, IriReference, Url);

enum_display_impl!(SimpleValue => String, Double, Decimal, Integer, Boolean, IriReference);

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

    pub fn with_ts_span(self, ts_span: Span) -> Self {
        Self {
            span: Some(ts_span),
            ..self
        }
    }

    pub fn has_ts_span(&self) -> bool {
        self.ts_span().is_some()
    }
    pub fn ts_span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
    pub fn set_ts_span(&mut self, span: Span) {
        self.span = Some(span);
    }
    pub fn unset_ts_span(&mut self) {
        self.span = None;
    }

    pub fn value(&self) -> &String {
        &self.value
    }
    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }

    pub fn language(&self) -> Option<&LanguageTag> {
        self.language.as_ref()
    }
    pub fn set_language(&mut self, language: LanguageTag) {
        self.language = Some(language);
    }
    pub fn unset_language(&mut self) {
        self.language = None;
    }

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

impl From<LanguageTag> for String {
    fn from(value: LanguageTag) -> Self {
        value.value
    }
}

impl AsRef<str> for LanguageTag {
    fn as_ref(&self) -> &str {
        self.value.as_str()
    }
}

impl PartialEq for LanguageTag {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for LanguageTag {}

impl LanguageTag {
    pub fn new_unchecked(s: &str) -> Self {
        Self {
            span: None,
            value: s.to_string(),
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn with_ts_span(self, ts_span: Span) -> Self {
        Self {
            span: Some(ts_span),
            ..self
        }
    }

    pub fn has_ts_span(&self) -> bool {
        self.ts_span().is_some()
    }
    pub fn ts_span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
    pub fn set_ts_span(&mut self, span: Span) {
        self.span = Some(span);
    }
    pub fn unset_ts_span(&mut self) {
        self.span = None;
    }

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

impl Display for MappingValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.domain, self.range)
    }
}

impl MappingValue {
    pub fn new(domain: SimpleValue, range: Value) -> Self {
        Self {
            span: Default::default(),
            domain,
            range: Box::new(range),
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn with_ts_span(self, ts_span: Span) -> Self {
        Self {
            span: Some(ts_span),
            ..self
        }
    }

    pub fn has_ts_span(&self) -> bool {
        self.ts_span().is_some()
    }
    pub fn ts_span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
    pub fn set_ts_span(&mut self, span: Span) {
        self.span = Some(span);
    }
    pub fn unset_ts_span(&mut self) {
        self.span = None;
    }

    // --------------------------------------------------------------------------------------------

    pub fn domain(&self) -> &SimpleValue {
        &self.domain
    }

    pub fn set_domain(&mut self, domain: SimpleValue) {
        self.domain = domain;
    }

    // --------------------------------------------------------------------------------------------

    pub fn range(&self) -> &Value {
        &self.range
    }

    pub fn set_range(&mut self, range: Value) {
        self.range = Box::new(range);
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
    pub fn with_ts_span(self, ts_span: Span) -> Self {
        Self {
            span: Some(ts_span),
            ..self
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_ts_span(&self) -> bool {
        self.ts_span().is_some()
    }
    pub fn ts_span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
    pub fn set_ts_span(&mut self, span: Span) {
        self.span = Some(span);
    }
    pub fn unset_ts_span(&mut self) {
        self.span = None;
    }

    // --------------------------------------------------------------------------------------------

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn iter(&self) -> impl Iterator<Item = &ListMember> {
        self.values.iter()
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut ListMember> {
        self.values.iter_mut()
    }
    pub fn push<I>(&mut self, value: I)
    where
        I: Into<ListMember>,
    {
        self.values.push(value.into())
    }
    pub fn extend<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = ListMember>,
    {
        self.values.extend(extension)
    }
}

// ------------------------------------------------------------------------------------------------

impl_from_for_variant!(ListMember, Simple, SimpleValue);

impl_from_for_variant!(ListMember, ValueConstructor, ValueConstructor);

impl_from_for_variant!(ListMember, Reference, IdentifierReference);

enum_display_impl!(ListMember => Simple, ValueConstructor, Reference, Mapping);

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

    pub fn with_ts_span(self, ts_span: Span) -> Self {
        Self {
            span: Some(ts_span),
            ..self
        }
    }

    pub fn has_ts_span(&self) -> bool {
        self.ts_span().is_some()
    }
    pub fn ts_span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
    pub fn set_ts_span(&mut self, span: Span) {
        self.span = Some(span);
    }
    pub fn unset_ts_span(&mut self) {
        self.span = None;
    }

    pub fn type_name(&self) -> &IdentifierReference {
        &self.type_name
    }
    pub fn set_type_name(&mut self, type_name: IdentifierReference) {
        self.type_name = type_name;
    }

    pub fn value(&self) -> &SimpleValue {
        &self.value
    }
    pub fn set_value(&mut self, value: SimpleValue) {
        self.value = value;
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
