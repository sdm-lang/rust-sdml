/*!
Provide the Rust types that implement *value*-related components of the SDML Grammar.
*/
use crate::model::{
    identifiers::{Identifier, IdentifierReference, QualifiedIdentifier},
    members::{Ordering, Uniqueness},
    HasSourceSpan, Span,
};
use lazy_static::lazy_static;
use ordered_float::OrderedFloat;
use regex::Regex;
use rust_decimal::Decimal;
use sdml_errors::diagnostics::functions::invalid_language_tag;
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
    Mapping(MappingValue),
    Reference(IdentifierReference),
    Sequence(SequenceOfValues),
}

/// Corresponds to the grammar rule `simple_value`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum SimpleValue {
    /// Corresponds to the grammar rule `boolean`.
    Boolean(bool),
    /// Corresponds to the grammar rule `double`.
    Double(OrderedFloat<f64>),
    /// Corresponds to the grammar rule `decimal`.
    Decimal(Decimal),
    /// Corresponds to the grammar rule `integer`.
    Integer(i64),
    /// Corresponds to the grammar rule `unsigned`.
    Unsigned(u64),
    /// Corresponds to the grammar rule `string`.
    String(LanguageString),
    /// Corresponds to the grammar rule `iri_reference`.
    IriReference(Url),
    /// Corresponds to the grammar rule `binary`.
    Binary(Binary),
}

/// Corresponds to the grammar rule `binary`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Binary(Vec<u8>);

/// Corresponds to the grammar rule `string`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct LanguageString {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    /// Corresponds to the grammar rule `quoted_string`.
    value: String,
    language: Option<LanguageTag>,
}

/// Corresponds to the grammar rule `language_tag`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct LanguageTag {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    value: language_tags::LanguageTag,
}

/// Corresponds to the grammar rule `mapping_value`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct MappingValue {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    domain: SimpleValue,
    range: Box<Value>,
}

/// Corresponds to the grammar rule `list_of_values`.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct SequenceOfValues {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    ordering: Option<Ordering>,
    uniqueness: Option<Uniqueness>,
    values: Vec<SequenceMember>,
}

/// Corresponds to the grammar rule `name`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum SequenceMember {
    Simple(SimpleValue),
    ValueConstructor(ValueConstructor),
    Reference(IdentifierReference),
    Mapping(MappingValue),
}

/// Corresponds to the grammar rule `value_constructor`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ValueConstructor {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    type_name: IdentifierReference,
    value: SimpleValue,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

lazy_static! {
    static ref LANGUAGE_TAG: Regex =
        Regex::new(r"^[a-z]{2,3}(-[A-Z]{3})?(-[A-Z][a-z]{3})?(-([A-Z]{2}|[0-9]{3}))?$").unwrap();
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Value
// ------------------------------------------------------------------------------------------------

impl<T: Into<SimpleValue>> From<T> for Value {
    fn from(v: T) -> Self {
        Self::Simple(v.into())
    }
}

impl From<&ValueConstructor> for Value {
    fn from(v: &ValueConstructor) -> Self {
        Self::ValueConstructor(v.clone())
    }
}

impl From<ValueConstructor> for Value {
    fn from(v: ValueConstructor) -> Self {
        Self::ValueConstructor(v)
    }
}

impl From<&Identifier> for Value {
    fn from(value: &Identifier) -> Self {
        Self::Reference(value.clone().into())
    }
}

impl From<Identifier> for Value {
    fn from(value: Identifier) -> Self {
        Self::Reference(value.into())
    }
}

impl From<&QualifiedIdentifier> for Value {
    fn from(value: &QualifiedIdentifier) -> Self {
        Self::Reference(value.clone().into())
    }
}

impl From<QualifiedIdentifier> for Value {
    fn from(value: QualifiedIdentifier) -> Self {
        Self::Reference(value.into())
    }
}

impl From<&IdentifierReference> for Value {
    fn from(value: &IdentifierReference) -> Self {
        Self::Reference(value.clone())
    }
}

impl From<IdentifierReference> for Value {
    fn from(value: IdentifierReference) -> Self {
        Self::Reference(value)
    }
}

impl From<&MappingValue> for Value {
    fn from(v: &MappingValue) -> Self {
        Self::Mapping(v.clone())
    }
}

impl From<MappingValue> for Value {
    fn from(v: MappingValue) -> Self {
        Self::Mapping(v)
    }
}

impl From<&SequenceOfValues> for Value {
    fn from(v: &SequenceOfValues) -> Self {
        Self::Sequence(v.clone())
    }
}

impl From<SequenceOfValues> for Value {
    fn from(v: SequenceOfValues) -> Self {
        Self::Sequence(v)
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Simple(v) => v.to_string(),
                Self::ValueConstructor(v) => v.to_string(),
                Self::Reference(v) => v.to_string(),
                Self::Mapping(v) => v.to_string(),
                Self::Sequence(v) => v.to_string(),
            }
        )
    }
}

impl Value {
    // --------------------------------------------------------------------------------------------
    // Variants
    // --------------------------------------------------------------------------------------------

    pub const fn is_simple(&self) -> bool {
        match self {
            Self::Simple(_) => true,
            _ => false,
        }
    }

    pub const fn as_simple(&self) -> Option<&SimpleValue> {
        match self {
            Self::Simple(v) => Some(v),
            _ => None,
        }
    }

    pub const fn is_value_constructor(&self) -> bool {
        match self {
            Self::ValueConstructor(_) => true,
            _ => false,
        }
    }

    pub const fn as_value_constructor(&self) -> Option<&ValueConstructor> {
        match self {
            Self::ValueConstructor(v) => Some(v),
            _ => None,
        }
    }

    pub const fn is_mapping_value(&self) -> bool {
        match self {
            Self::Mapping(_) => true,
            _ => false,
        }
    }

    pub const fn as_mapping_value(&self) -> Option<&MappingValue> {
        match self {
            Self::Mapping(v) => Some(v),
            _ => None,
        }
    }

    pub const fn is_reference(&self) -> bool {
        match self {
            Self::Reference(_) => true,
            _ => false,
        }
    }

    pub const fn as_reference(&self) -> Option<&IdentifierReference> {
        match self {
            Self::Reference(v) => Some(v),
            _ => None,
        }
    }
    pub const fn is_sequence(&self) -> bool {
        match self {
            Self::Sequence(_) => true,
            _ => false,
        }
    }

    pub const fn as_sequence(&self) -> Option<&SequenceOfValues> {
        match self {
            Self::Sequence(v) => Some(v),
            _ => None,
        }
    }

    // --------------------------------------------------------------------------------------------
    // Variants ❱ SimpleTypes
    // --------------------------------------------------------------------------------------------

    pub const fn is_boolean(&self) -> bool {
        matches!(self, Self::Simple(SimpleValue::Boolean(_)))
    }

    pub const fn as_boolean(&self) -> Option<bool> {
        match self {
            Self::Simple(SimpleValue::Boolean(v)) => Some(*v),
            _ => None,
        }
    }

    pub const fn is_double(&self) -> bool {
        matches!(self, Self::Simple(SimpleValue::Double(_)))
    }

    pub const fn as_double(&self) -> Option<OrderedFloat<f64>> {
        match self {
            Self::Simple(SimpleValue::Double(v)) => Some(*v),
            _ => None,
        }
    }

    pub const fn is_decimal(&self) -> bool {
        matches!(self, Self::Simple(SimpleValue::Decimal(_)))
    }

    pub const fn as_decimal(&self) -> Option<Decimal> {
        match self {
            Self::Simple(SimpleValue::Decimal(v)) => Some(*v),
            _ => None,
        }
    }

    pub const fn is_integer(&self) -> bool {
        matches!(self, Self::Simple(SimpleValue::Integer(_)))
    }

    pub fn as_integer(&self) -> Option<i64> {
        match self {
            Self::Simple(SimpleValue::Integer(v)) => Some(*v),
            _ => None,
        }
    }

    pub const fn is_unsigned(&self) -> bool {
        matches!(self, Self::Simple(SimpleValue::Unsigned(_)))
    }

    pub const fn as_unsigned(&self) -> Option<u64> {
        match self {
            Self::Simple(SimpleValue::Unsigned(v)) => Some(*v),
            _ => None,
        }
    }

    pub const fn is_string(&self) -> bool {
        matches!(self, Self::Simple(SimpleValue::String(_)))
    }

    pub const fn as_string(&self) -> Option<&LanguageString> {
        match self {
            Self::Simple(SimpleValue::String(v)) => Some(v),
            _ => None,
        }
    }

    pub const fn is_iri(&self) -> bool {
        matches!(self, Self::Simple(SimpleValue::IriReference(_)))
    }

    pub const fn as_iri(&self) -> Option<&Url> {
        match self {
            Self::Simple(SimpleValue::IriReference(v)) => Some(v),
            _ => None,
        }
    }

    pub const fn is_binary(&self) -> bool {
        matches!(self, Self::Simple(SimpleValue::Binary(_)))
    }

    pub const fn as_binary(&self) -> Option<&Binary> {
        match self {
            Self::Simple(SimpleValue::Binary(v)) => Some(v),
            _ => None,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ SimpleValue
// ------------------------------------------------------------------------------------------------

impl From<&bool> for SimpleValue {
    fn from(v: &bool) -> Self {
        Self::Boolean(*v)
    }
}

impl From<bool> for SimpleValue {
    fn from(v: bool) -> Self {
        Self::Boolean(v)
    }
}

impl From<&f64> for SimpleValue {
    fn from(v: &f64) -> Self {
        Self::Double(OrderedFloat::from(*v))
    }
}

impl From<f64> for SimpleValue {
    fn from(v: f64) -> Self {
        Self::Double(OrderedFloat::from(v))
    }
}

impl From<&OrderedFloat<f64>> for SimpleValue {
    fn from(v: &OrderedFloat<f64>) -> Self {
        Self::Double(*v)
    }
}

impl From<OrderedFloat<f64>> for SimpleValue {
    fn from(v: OrderedFloat<f64>) -> Self {
        Self::Double(v)
    }
}

impl From<&Decimal> for SimpleValue {
    fn from(v: &Decimal) -> Self {
        Self::Decimal(*v)
    }
}

impl From<Decimal> for SimpleValue {
    fn from(v: Decimal) -> Self {
        Self::Decimal(v)
    }
}

impl From<&i64> for SimpleValue {
    fn from(v: &i64) -> Self {
        Self::Integer(*v)
    }
}

impl From<i64> for SimpleValue {
    fn from(v: i64) -> Self {
        Self::Integer(v)
    }
}

impl From<&u64> for SimpleValue {
    fn from(v: &u64) -> Self {
        Self::Unsigned(*v)
    }
}

impl From<u64> for SimpleValue {
    fn from(v: u64) -> Self {
        Self::Unsigned(v)
    }
}

impl From<&str> for SimpleValue {
    fn from(v: &str) -> Self {
        Self::plain(v)
    }
}

impl From<String> for SimpleValue {
    fn from(v: String) -> Self {
        Self::plain(&v)
    }
}

impl From<&LanguageString> for SimpleValue {
    fn from(v: &LanguageString) -> Self {
        Self::String(v.clone())
    }
}

impl From<LanguageString> for SimpleValue {
    fn from(v: LanguageString) -> Self {
        Self::String(v)
    }
}

impl From<&Url> for SimpleValue {
    fn from(v: &Url) -> Self {
        Self::IriReference(v.clone())
    }
}

impl From<Url> for SimpleValue {
    fn from(v: Url) -> Self {
        Self::IriReference(v)
    }
}

impl From<&Binary> for SimpleValue {
    fn from(v: &Binary) -> Self {
        Self::Binary(v.clone())
    }
}

impl From<Binary> for SimpleValue {
    fn from(v: Binary) -> Self {
        Self::Binary(v)
    }
}

impl Display for SimpleValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Double(v) => v.to_string(),
                Self::Decimal(v) => v.to_string(),
                Self::Integer(v) => v.to_string(),
                Self::Unsigned(v) => v.to_string(),
                Self::Boolean(v) => v.to_string(),
                Self::IriReference(v) => format!("<{v}>"),
                Self::String(v) => v.to_string(),
                Self::Binary(v) => v.to_string(),
            }
        )
    }
}

impl SimpleValue {
    pub fn plain<S>(content: S) -> Self
    where
        S: AsRef<str>,
    {
        LanguageString::plain(content.as_ref()).into()
    }

    // --------------------------------------------------------------------------------------------
    // Variants
    // --------------------------------------------------------------------------------------------

    pub const fn is_boolean(&self) -> bool {
        match self {
            Self::Boolean(_) => true,
            _ => false,
        }
    }

    pub const fn as_boolean(&self) -> Option<&bool> {
        match self {
            Self::Boolean(v) => Some(v),
            _ => None,
        }
    }

    pub const fn is_double(&self) -> bool {
        match self {
            Self::Double(_) => true,
            _ => false,
        }
    }

    pub const fn as_double(&self) -> Option<&OrderedFloat<f64>> {
        match self {
            Self::Double(v) => Some(v),
            _ => None,
        }
    }

    pub const fn is_decimal(&self) -> bool {
        match self {
            Self::Decimal(_) => true,
            _ => false,
        }
    }

    pub const fn as_decimal(&self) -> Option<&Decimal> {
        match self {
            Self::Decimal(v) => Some(v),
            _ => None,
        }
    }

    pub const fn is_integer(&self) -> bool {
        match self {
            Self::Integer(_) => true,
            _ => false,
        }
    }

    pub const fn as_integer(&self) -> Option<&i64> {
        match self {
            Self::Integer(v) => Some(v),
            _ => None,
        }
    }

    pub const fn is_unsigned(&self) -> bool {
        match self {
            Self::Unsigned(_) => true,
            _ => false,
        }
    }

    pub const fn as_unsigned(&self) -> Option<&u64> {
        match self {
            Self::Unsigned(v) => Some(v),
            _ => None,
        }
    }

    pub const fn is_string(&self) -> bool {
        match self {
            Self::String(_) => true,
            _ => false,
        }
    }

    pub const fn as_string(&self) -> Option<&LanguageString> {
        match self {
            Self::String(v) => Some(v),
            _ => None,
        }
    }

    pub const fn is_iri(&self) -> bool {
        match self {
            Self::IriReference(_) => true,
            _ => false,
        }
    }

    pub const fn as_iri(&self) -> Option<&Url> {
        match self {
            Self::IriReference(v) => Some(v),
            _ => None,
        }
    }

    pub const fn is_binary(&self) -> bool {
        match self {
            Self::Binary(_) => true,
            _ => false,
        }
    }

    pub const fn as_binary(&self) -> Option<&Binary> {
        match self {
            Self::Binary(v) => Some(v),
            _ => None,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ LanguageString
// ------------------------------------------------------------------------------------------------

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

impl HasSourceSpan for LanguageString {
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

impl LanguageString {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn new(value: &str, language: Option<LanguageTag>) -> Self {
        Self {
            span: None,
            value: value.to_string(),
            language,
        }
    }

    pub fn plain(value: &str) -> Self {
        Self::new(value, None)
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    pub const fn value(&self) -> &String {
        &self.value
    }

    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }

    pub const fn has_language(&self) -> bool {
        self.language.is_some()
    }

    pub const fn language(&self) -> Option<&LanguageTag> {
        self.language.as_ref()
    }

    pub fn set_language(&mut self, language: LanguageTag) {
        self.language = Some(language);
    }

    pub fn unset_language(&mut self) {
        self.language = None;
    }

    // --------------------------------------------------------------------------------------------
    // Helpers
    // --------------------------------------------------------------------------------------------

    pub fn is_plain_literal(&self) -> bool {
        !self.has_language()
    }

    pub fn eq_with_span(&self, other: &Self) -> bool {
        self.span == other.span && self.value == other.value && self.language == other.language
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ LanguageTag
// ------------------------------------------------------------------------------------------------

impl From<LanguageTag> for language_tags::LanguageTag {
    fn from(value: LanguageTag) -> Self {
        value.value
    }
}

impl From<LanguageTag> for String {
    fn from(value: LanguageTag) -> Self {
        value.value.to_string()
    }
}

impl FromStr for LanguageTag {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Self::is_valid_str(s) {
            Ok(Self {
                span: None,
                value: language_tags::LanguageTag::parse(s)?,
            })
        } else {
            Err(invalid_language_tag(0, None, s).into())
        }
    }
}

impl AsRef<language_tags::LanguageTag> for LanguageTag {
    fn as_ref(&self) -> &language_tags::LanguageTag {
        &self.value
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

impl PartialEq<language_tags::LanguageTag> for LanguageTag {
    fn eq(&self, other: &language_tags::LanguageTag) -> bool {
        self.value == *other
    }
}

impl PartialEq<str> for LanguageTag {
    fn eq(&self, other: &str) -> bool {
        self.value.as_str() == other
    }
}

impl Eq for LanguageTag {}

impl Display for LanguageTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "@{}", self.value)
    }
}

impl HasSourceSpan for LanguageTag {
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

impl LanguageTag {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn new(value: language_tags::LanguageTag) -> Self {
        Self { span: None, value }
    }

    pub fn new_unchecked(s: &str) -> Self {
        Self {
            span: None,
            value: language_tags::LanguageTag::parse(s).unwrap(),
        }
    }

    // --------------------------------------------------------------------------------------------
    // Helpers
    // --------------------------------------------------------------------------------------------

    pub fn is_valid_str(s: &str) -> bool {
        language_tags::LanguageTag::parse(s).is_ok()
    }

    pub fn eq_with_span(&self, other: &Self) -> bool {
        self.span == other.span && self.value == other.value
    }

    pub fn inner(&self) -> &language_tags::LanguageTag {
        &self.value
    }

    pub fn into_inner(self) -> language_tags::LanguageTag {
        self.value
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Binary
// ------------------------------------------------------------------------------------------------

impl Display for Binary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for byte in self.as_bytes() {
            write!(f, "{:02X}", byte)?;
        }
        write!(f, "[")
    }
}

impl From<Vec<u8>> for Binary {
    fn from(v: Vec<u8>) -> Self {
        Self(v)
    }
}

impl FromIterator<u8> for Binary {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        Self(Vec::from_iter(iter))
    }
}

impl AsRef<Vec<u8>> for Binary {
    fn as_ref(&self) -> &Vec<u8> {
        &self.0
    }
}

impl Binary {
    // --------------------------------------------------------------------------------------------
    // Helpers
    // --------------------------------------------------------------------------------------------

    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_slice()
    }

    pub fn default_format(&self) -> String {
        self.format(1, 2)
    }

    pub fn format(&self, indent_level: u8, indent_spaces: u8) -> String {
        let mut buffer = String::new();
        let n = (indent_level * indent_spaces) as usize;
        let indent_outer = format!("{:n$}", "");
        let n = ((indent_level + 1) * indent_spaces) as usize;
        let indent_inner = format!("{:n$}", "");
        if self.0.len() <= 16 {
            buffer.push_str("#[");
            buffer.push_str(&format_byte_block(self.0.as_slice(), &indent_inner));
            buffer.push(']');
        } else {
            buffer.push_str(&format!("#[\n{indent_outer}"));
            buffer.push_str(&format_byte_block(self.0.as_slice(), &indent_inner));
            buffer.push_str(&format!("\n{indent_outer}]"));
        }
        buffer
    }
}

fn format_byte_block(bytes: &[u8], indent: &str) -> String {
    if bytes.len() <= 8 {
        bytes
            .iter()
            .map(|b| format!("{:02X}", b))
            .collect::<Vec<String>>()
            .join(" ")
    } else if bytes.len() <= 16 {
        format!(
            "{}   {}",
            format_byte_block(&bytes[0..8], indent),
            format_byte_block(&bytes[9..], indent),
        )
    } else {
        format!(
            "{indent}{}\n{}",
            format_byte_block(&bytes[0..16], indent),
            format_byte_block(&bytes[17..], indent),
        )
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ MappingValue
// ------------------------------------------------------------------------------------------------

impl Display for MappingValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.domain, self.range)
    }
}

impl HasSourceSpan for MappingValue {
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

impl MappingValue {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn new(domain: SimpleValue, range: Value) -> Self {
        Self {
            span: None,
            domain,
            range: Box::new(range),
        }
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    pub const fn domain(&self) -> &SimpleValue {
        &self.domain
    }

    pub fn set_domain<T>(&mut self, domain: T)
    where
        T: Into<SimpleValue>,
    {
        self.domain = domain.into();
    }

    pub const fn range(&self) -> &Value {
        &self.range
    }

    pub fn set_range<T>(&mut self, range: T)
    where
        T: Into<Value>,
    {
        self.range = Box::new(range.into());
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ SequenceOfValues
// ------------------------------------------------------------------------------------------------

impl From<Vec<SequenceMember>> for SequenceOfValues {
    fn from(values: Vec<SequenceMember>) -> Self {
        Self {
            span: None,
            ordering: None,
            uniqueness: None,
            values,
        }
    }
}

impl FromIterator<SequenceMember> for SequenceOfValues {
    fn from_iter<T: IntoIterator<Item = SequenceMember>>(iter: T) -> Self {
        Self::from(Vec::from_iter(iter))
    }
}

impl Display for SequenceOfValues {
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

impl HasSourceSpan for SequenceOfValues {
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

impl SequenceOfValues {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn with_ordering(self, ordering: Ordering) -> Self {
        Self {
            ordering: Some(ordering),
            ..self
        }
    }

    pub fn with_uniqueness(self, uniqueness: Uniqueness) -> Self {
        Self {
            uniqueness: Some(uniqueness),
            ..self
        }
    }

    // --------------------------------------------------------------------------------------------
    // Values
    // --------------------------------------------------------------------------------------------

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &SequenceMember> {
        self.values.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut SequenceMember> {
        self.values.iter_mut()
    }

    pub fn push<I>(&mut self, value: I)
    where
        I: Into<SequenceMember>,
    {
        self.values.push(value.into())
    }

    pub fn extend<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = SequenceMember>,
    {
        self.values.extend(extension)
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    pub const fn has_ordering(&self) -> bool {
        self.ordering.is_some()
    }

    pub const fn ordering(&self) -> Option<&Ordering> {
        self.ordering.as_ref()
    }

    pub fn set_ordering(&mut self, ordering: Ordering) {
        self.ordering = Some(ordering);
    }

    pub fn unset_ordering(&mut self) {
        self.ordering = None;
    }

    // --------------------------------------------------------------------------------------------

    pub const fn has_uniqueness(&self) -> bool {
        self.uniqueness.is_some()
    }

    pub const fn uniqueness(&self) -> Option<&Uniqueness> {
        self.uniqueness.as_ref()
    }

    pub fn set_uniqueness(&mut self, uniqueness: Uniqueness) {
        self.uniqueness = Some(uniqueness);
    }

    pub fn unset_uniqueness(&mut self) {
        self.uniqueness = None;
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ SequenceMember
// ------------------------------------------------------------------------------------------------

impl<T: Into<SimpleValue>> From<T> for SequenceMember {
    fn from(value: T) -> Self {
        Self::Simple(value.into())
    }
}

impl From<&ValueConstructor> for SequenceMember {
    fn from(v: &ValueConstructor) -> Self {
        Self::ValueConstructor(v.clone())
    }
}

impl From<ValueConstructor> for SequenceMember {
    fn from(v: ValueConstructor) -> Self {
        Self::ValueConstructor(v)
    }
}

impl From<&Identifier> for SequenceMember {
    fn from(value: &Identifier) -> Self {
        Self::Reference(value.clone().into())
    }
}

impl From<Identifier> for SequenceMember {
    fn from(value: Identifier) -> Self {
        Self::Reference(value.into())
    }
}

impl From<&QualifiedIdentifier> for SequenceMember {
    fn from(value: &QualifiedIdentifier) -> Self {
        Self::Reference(value.clone().into())
    }
}

impl From<QualifiedIdentifier> for SequenceMember {
    fn from(value: QualifiedIdentifier) -> Self {
        Self::Reference(value.into())
    }
}

impl From<&IdentifierReference> for SequenceMember {
    fn from(value: &IdentifierReference) -> Self {
        Self::Reference(value.clone())
    }
}

impl From<IdentifierReference> for SequenceMember {
    fn from(value: IdentifierReference) -> Self {
        Self::Reference(value)
    }
}

impl From<&MappingValue> for SequenceMember {
    fn from(v: &MappingValue) -> Self {
        Self::Mapping(v.clone())
    }
}

impl From<MappingValue> for SequenceMember {
    fn from(v: MappingValue) -> Self {
        Self::Mapping(v)
    }
}

impl Display for SequenceMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Simple(v) => v.to_string(),
                Self::ValueConstructor(v) => v.to_string(),
                Self::Reference(v) => v.to_string(),
                Self::Mapping(v) => v.to_string(),
            }
        )
    }
}

impl SequenceMember {
    // --------------------------------------------------------------------------------------------
    // Variants
    // --------------------------------------------------------------------------------------------

    pub fn is_simple(&self) -> bool {
        matches!(self, Self::Simple(_))
    }

    pub fn as_simple(&self) -> Option<&SimpleValue> {
        match self {
            Self::Simple(v) => Some(v),
            _ => None,
        }
    }

    pub fn is_value_constructor(&self) -> bool {
        matches!(self, Self::ValueConstructor(_))
    }

    pub fn as_value_constructor(&self) -> Option<&ValueConstructor> {
        match self {
            Self::ValueConstructor(v) => Some(v),
            _ => None,
        }
    }

    pub fn is_reference(&self) -> bool {
        matches!(self, Self::Reference(_))
    }

    pub fn as_reference(&self) -> Option<&IdentifierReference> {
        match self {
            Self::Reference(v) => Some(v),
            _ => None,
        }
    }

    pub fn is_mapping(&self) -> bool {
        matches!(self, Self::Mapping(_))
    }

    pub fn as_mapping(&self) -> Option<&MappingValue> {
        match self {
            Self::Mapping(v) => Some(v),
            _ => None,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ ValueConstructor
// ------------------------------------------------------------------------------------------------

impl Display for ValueConstructor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.type_name, self.value)
    }
}

impl HasSourceSpan for ValueConstructor {
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

impl ValueConstructor {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub const fn new(type_name: IdentifierReference, value: SimpleValue) -> Self {
        Self {
            span: None,
            type_name,
            value,
        }
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    pub const fn type_name(&self) -> &IdentifierReference {
        &self.type_name
    }

    pub fn set_type_name(&mut self, type_name: IdentifierReference) {
        self.type_name = type_name;
    }

    pub const fn value(&self) -> &SimpleValue {
        &self.value
    }

    pub fn set_value(&mut self, value: SimpleValue) {
        self.value = value;
    }
}
