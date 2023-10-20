use crate::model::{IdentifierReference, Span, members::{Ordering, Uniqueness}};
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
    Mapping(MappingValue),
    Reference(IdentifierReference),
    List(SequenceOfValues),
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
pub struct SequenceOfValues {
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

impl From<i32> for Value {
    fn from(v: i32) -> Self {
        Self::Simple(SimpleValue::Integer(v as i64))
    }
}

impl From<u64> for Value {
    fn from(v: u64) -> Self {
        Self::Simple(SimpleValue::Unsigned(v))
    }
}

impl From<u32> for Value {
    fn from(v: u32) -> Self {
        Self::Simple(SimpleValue::Unsigned(v as u64))
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

impl From<Binary> for Value {
    fn from(v: Binary) -> Self {
        Self::Simple(SimpleValue::Binary(v))
    }
}

//impl From<Box<Binary>> for Value {
//    fn from(v: Box<Binary>) -> Self {
//        Self::Simple(SimpleValue::Binary(v))
//    }
//}

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

impl From<SequenceOfValues> for Value {
    fn from(v: SequenceOfValues) -> Self {
        Self::List(v)
    }
}

enum_display_impl!(Value => Simple, ValueConstructor, Reference, Mapping, List);

// ------------------------------------------------------------------------------------------------

impl_from_for_variant!(SimpleValue, String, LanguageString);

impl_from_for_variant!(SimpleValue, Double, OrderedFloat<f64>);

impl_from_for_variant!(SimpleValue, Decimal, Decimal);

impl_from_for_variant!(SimpleValue, Integer, i64);

impl_from_for_variant!(SimpleValue, Unsigned, u64);

impl_from_for_variant!(SimpleValue, Boolean, bool);

impl_from_for_variant!(SimpleValue, IriReference, Url);

impl_from_for_variant!(SimpleValue, Binary, Binary);

enum_display_impl!(
    SimpleValue => Double, Decimal, Integer, Unsigned, Boolean, IriReference, String, Binary
);

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

impl_has_source_span_for!(LanguageString);

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

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    get_and_set!(pub value, set_value => String);

    get_and_set!(pub language, set_language, unset_language => optional has_language, LanguageTag);

    // --------------------------------------------------------------------------------------------
    // Helpers
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
        if Self::is_valid_str(s) {
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

impl PartialEq<str> for LanguageTag {
    fn eq(&self, other: &str) -> bool {
        self.value == other
    }
}

impl Eq for LanguageTag {}

impl_has_source_span_for!(LanguageTag);

impl LanguageTag {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn new_unchecked(s: &str) -> Self {
        Self {
            span: None,
            value: s.to_string(),
        }
    }

    // --------------------------------------------------------------------------------------------
    // Helpers
    // --------------------------------------------------------------------------------------------

    pub fn is_valid_str(s: &str) -> bool {
        LANGUAGE_TAG.is_match(s)
    }

    pub fn eq_with_span(&self, other: &Self) -> bool {
        self.span == other.span && self.value == other.value
    }
}

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

impl Display for MappingValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.domain, self.range)
    }
}

impl_has_source_span_for!(MappingValue);

impl MappingValue {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn new(domain: SimpleValue, range: Value) -> Self {
        Self {
            span: Default::default(),
            domain,
            range: Box::new(range),
        }
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    get_and_set!(pub domain, set_domain => into SimpleValue);

    get_and_set!(pub range, set_range => boxed into Value);
}

// ------------------------------------------------------------------------------------------------

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

impl From<Vec<SequenceMember>> for SequenceOfValues {
    fn from(values: Vec<SequenceMember>) -> Self {
        Self { span: None, ordering: None, uniqueness: None, values }
    }
}

impl FromIterator<SequenceMember> for SequenceOfValues {
    fn from_iter<T: IntoIterator<Item = SequenceMember>>(iter: T) -> Self {
        Self::from(Vec::from_iter(iter))
    }
}

impl_has_source_span_for!(SequenceOfValues);

impl_as_sequence!(pub SequenceOfValues => SequenceMember);

impl SequenceOfValues {
    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    pub fn with_ordering(self, ordering: Ordering) -> Self {
        Self {
            ordering: Some(ordering),
            ..self
        }
    }

    get_and_set!(pub ordering, set_ordering, unset_ordering => optional has_ordering, Ordering);

    pub fn with_uniqueness(self, uniqueness: Uniqueness) -> Self {
        Self {
            uniqueness: Some(uniqueness),
            ..self
        }
    }

    get_and_set!(pub uniqueness, set_uniqueness, unset_uniqueness => optional has_uniqueness, Uniqueness);
}

// ------------------------------------------------------------------------------------------------

impl_from_for_variant!(SequenceMember, Simple, SimpleValue);

impl_from_for_variant!(SequenceMember, ValueConstructor, ValueConstructor);

impl_from_for_variant!(SequenceMember, Reference, IdentifierReference);

enum_display_impl!(SequenceMember => Simple, ValueConstructor, Reference, Mapping);

// ------------------------------------------------------------------------------------------------

impl Display for ValueConstructor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.type_name, self.value)
    }
}

impl_has_source_span_for!(ValueConstructor);

impl ValueConstructor {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn new(type_name: IdentifierReference, value: SimpleValue) -> Self {
        Self {
            span: None,
            type_name,
            value,
        }
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    get_and_set!(pub type_name, set_type_name => IdentifierReference);

    get_and_set!(pub value, set_value => SimpleValue);
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
