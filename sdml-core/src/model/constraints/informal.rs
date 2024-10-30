use crate::{
    load::ModuleLoader,
    model::{check::Validate, modules::Module, HasSourceSpan, References, Span},
    store::ModuleStore,
};
use lazy_static::lazy_static;
use regex::Regex;
use sdml_errors::diagnostics::functions::invalid_language_tag;
use std::{fmt::Display, str::FromStr};
use tracing::warn;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Constraints ❱ Informal     ,
// ------------------------------------------------------------------------------------------------

///
/// Corresponds to the grammar rule `informal_constraint`.
///
/// This structure captures an informal, or semi-formal constraint as a natural language string
/// string.
///
/// 1. `"some cars have manual transmissions"` is an informal constraint in some unidentified
///    natural language.
/// 2. `"some cars have manual transmissions"@en` is an informal constraint in English.
/// 3. `"there is a car that has a a:manual transmission."@en-ACE` is a semi-formal constraint in
///    Attempto Controlled English (ACE).
///
/// We classify the last example as *semi-formal*, even though ACE is formally defined,
/// because SDML does not expect (although does not prohibit) the translation from this form into
/// the logical structure of a `ConstraintSentence`.
///
/// In the last example above the prefix `a:` on manual identifies the term *manual* it as an
/// adjective applied to the word term *transmission*.
///
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ControlledLanguageString {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    /// Corresponds to the grammar rule `quoted_string`.
    value: String,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    language: Option<ControlledLanguageTag>,
}

///
/// Corresponds to the grammar rule `controlled_language_tag`.
///
/// 1. Required natural language identifier, either a 2 or 3 character
///    code from ISO-639.
/// 2. An optional identifier representing the controlled language scheme.
///
/// There is no registry for controlled language schemes, and SDML makes no requirement
/// for the support of any particular scheme. The following are commonly used schemes
/// and their identifiers:
///
/// - **CLCE**: Common Logic Controlled English (see [Sowa, 2004](http://www.jfsowa.com/clce/specs.htm)).
/// - **ACE** or **Attempto**: Attempto Controlled English (ACE) (see
///   [attempto.ifi.uzh.ch](http://attempto.ifi.uzh.ch/site/)).
///
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ControlledLanguageTag {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    value: String,
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Constraints ❱ ControlledLanguageString
// ------------------------------------------------------------------------------------------------

impl From<String> for ControlledLanguageString {
    fn from(value: String) -> Self {
        Self {
            span: Default::default(),
            value,
            language: Default::default(),
        }
    }
}

impl HasSourceSpan for ControlledLanguageString {
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

impl References for ControlledLanguageString {}

impl Validate for ControlledLanguageString {
    fn validate(
        &self,
        _top: &Module,
        _cache: &impl ModuleStore,
        _loader: &impl ModuleLoader,
        _check_constraints: bool,
    ) {
        warn!("Missing Validation for ControlledLanguageString");
    }
}

impl ControlledLanguageString {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn new<S>(value: S, language: ControlledLanguageTag) -> Self
    where
        S: Into<String>,
    {
        Self {
            span: Default::default(),
            value: value.into(),
            language: Some(language),
        }
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

    // --------------------------------------------------------------------------------------------

    pub const fn has_language(&self) -> bool {
        self.language.is_some()
    }

    pub const fn language(&self) -> Option<&ControlledLanguageTag> {
        self.language.as_ref()
    }

    pub fn set_language(&mut self, language: ControlledLanguageTag) {
        self.language = Some(language);
    }

    pub fn unset_language(&mut self) {
        self.language = None;
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Constraints ❱ ControlledLanguageTag
// ------------------------------------------------------------------------------------------------

lazy_static! {
    static ref LANGUAGE_TAG: Regex = Regex::new(r"^[a-z]{2,3}(-[A-Z][A-Za-z]{1,9})?$").unwrap();
}

impl Display for ControlledLanguageTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "@{}", self.value)
    }
}

impl FromStr for ControlledLanguageTag {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Self::is_valid_str(s) {
            Ok(Self {
                span: None,
                value: s.to_string(),
            })
        } else {
            Err(invalid_language_tag(0, None, s).into())
        }
    }
}

impl From<ControlledLanguageTag> for String {
    fn from(value: ControlledLanguageTag) -> Self {
        value.value
    }
}

impl AsRef<str> for ControlledLanguageTag {
    fn as_ref(&self) -> &str {
        self.value.as_str()
    }
}

impl PartialEq for ControlledLanguageTag {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl PartialEq<str> for ControlledLanguageTag {
    fn eq(&self, other: &str) -> bool {
        self.value == other
    }
}

impl Eq for ControlledLanguageTag {}

impl HasSourceSpan for ControlledLanguageTag {
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

impl Validate for ControlledLanguageTag {
    fn validate(
        &self,
        _top: &Module,
        _cache: &impl ModuleStore,
        _loader: &impl ModuleLoader,
        _check_constraints: bool,
    ) {
        assert!(Self::is_valid_str(&self.value))
    }
}

impl ControlledLanguageTag {
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
    // Fields
    // --------------------------------------------------------------------------------------------

    pub const fn value(&self) -> &String {
        &self.value
    }

    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }

    // --------------------------------------------------------------------------------------------
    // Helpers
    // --------------------------------------------------------------------------------------------

    pub fn is_valid_str(s: &str) -> bool {
        LANGUAGE_TAG.is_match(s)
    }
}
