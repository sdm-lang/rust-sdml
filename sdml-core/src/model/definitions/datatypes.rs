use crate::load::ModuleLoader;
use crate::model::annotations::HasAnnotations;
use crate::model::annotations::{AnnotationBuilder, AnnotationProperty};
use crate::model::check::{find_definition, MaybeIncomplete, Validate};
use crate::model::definitions::Definition;
use crate::model::modules::Module;
use crate::model::values::Value;
use crate::model::References;
use crate::model::{
    annotations::AnnotationOnlyBody,
    identifiers::{Identifier, IdentifierReference},
    HasName, Span,
};
use crate::model::{HasOptionalBody, HasSourceSpan};
use crate::store::ModuleStore;
use crate::syntax::{
    KW_FACET_EXPLICIT_TIMEZONE, KW_FACET_FRACTION_DIGITS, KW_FACET_LENGTH, KW_FACET_MAX_EXCLUSIVE,
    KW_FACET_MAX_INCLUSIVE, KW_FACET_MAX_LENGTH, KW_FACET_MIN_EXCLUSIVE, KW_FACET_MIN_INCLUSIVE,
    KW_FACET_MIN_LENGTH, KW_FACET_PATTERN, KW_FACET_TIMEZONE_OPTIONAL,
    KW_FACET_TIMEZONE_PROHIBITED, KW_FACET_TIMEZONE_REQUIRED, KW_FACET_TOTAL_DIGITS,
};
use sdml_errors::diagnostics::functions::{
    datatype_invalid_base_type, type_definition_not_found, IdentifierCaseConvention,
};
use std::fmt::Display;
use std::str::FromStr;
use std::{collections::BTreeSet, fmt::Debug};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Definitions ❱ Datatypes
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum ExplicitTimezoneFlag {
    #[default]
    Optional,
    Required,
    Prohibited,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum RestrictionFacet {
    // ---------------------------------------------------------------------------------------------
    /// The XML Schema facet `xsd:fractionDigits`
    FractionDigits(u32, bool),
    /// The XML Schema facet `xsd:totalDigits`
    TotalDigits(u32, bool),
    // ---------------------------------------------------------------------------------------------
    /// The XML Schema facet `xsd:length`
    Length(u32, bool),
    /// The XML Schema facet `xsd:maxLength`
    MaxLength(u32, bool),
    /// The XML Schema facet `xsd:minLength`
    MinLength(u32, bool),
    // ---------------------------------------------------------------------------------------------
    /// The XML Schema facet `xsd:maxExclusive`
    MaxExclusive(u32, bool),
    /// The XML Schema facet `xsd:minExclusive`
    MinExclusive(u32, bool),
    /// The XML Schema facet `xsd:maxInclusive`
    MaxInclusive(u32, bool),
    /// The XML Schema facet `xsd:minInclusive`
    MinInclusive(u32, bool),
    // ---------------------------------------------------------------------------------------------
    /// The XML Schema facet `xsd:explicitTimezone`
    ExplicitTimezone(ExplicitTimezoneFlag, bool),
    // ---------------------------------------------------------------------------------------------
    /// The XML Schema facet `xsd:pattern`
    Pattern(Vec<String>),
}

/// Corresponds to the grammar rule `data_type_def`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct DatatypeDef {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    name: Identifier,
    opaque: bool,
    /// Corresponds to the grammar rule `data_type_base`.
    base_type: IdentifierReference,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    restrictions: Vec<RestrictionFacet>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    body: Option<AnnotationOnlyBody>,
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Definitions ❱ DatatypeDef
// ------------------------------------------------------------------------------------------------

impl HasSourceSpan for DatatypeDef {
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

    fn has_source_span(&self) -> bool {
        self.source_span().is_some()
    }
}

impl HasName for DatatypeDef {
    fn name(&self) -> &Identifier {
        &self.name
    }

    fn set_name(&mut self, name: Identifier) {
        self.name = name;
    }
}

impl HasOptionalBody for DatatypeDef {
    type Body = AnnotationOnlyBody;

    fn body(&self) -> Option<&Self::Body> {
        self.body.as_ref()
    }

    fn body_mut(&mut self) -> Option<&mut Self::Body> {
        self.body.as_mut()
    }

    fn set_body(&mut self, body: Self::Body) {
        self.body = Some(body);
    }

    fn unset_body(&mut self) {
        self.body = None;
    }
}

impl MaybeIncomplete for DatatypeDef {
    fn is_incomplete(&self, _: &Module, _: &impl ModuleStore) -> bool {
        false
    }
}

impl AnnotationBuilder for DatatypeDef {
    fn with_predicate<I, V>(self, predicate: I, value: V) -> Self
    where
        Self: Sized,
        I: Into<IdentifierReference>,
        V: Into<Value>,
    {
        let mut self_mut = self;
        if let Some(ref mut inner) = self_mut.body {
            inner.add_to_annotations(AnnotationProperty::new(predicate.into(), value.into()));
        }
        self_mut
    }
}

impl Validate for DatatypeDef {
    fn validate(
        &self,
        top: &Module,
        cache: &impl ModuleStore,
        loader: &impl ModuleLoader,
        _check_constraints: bool,
    ) {
        self.name().validate(
            top,
            loader,
            Some(IdentifierCaseConvention::DatatypeDefinition),
        );
        if let Some(defn) = find_definition(self.base_type(), top, cache) {
            if let Definition::Datatype(_base) = defn {
                // TODO: check restriction annotations.
            } else if let Definition::Rdf(base) = defn {
                if !base.is_datatype() {
                    loader
                        .report(&datatype_invalid_base_type(
                            top.file_id().copied().unwrap_or_default(),
                            self.base_type()
                                .source_span()
                                .as_ref()
                                .map(|span| (*span).into()),
                            self.base_type(),
                        ))
                        .unwrap();
                }
                // TODO: check type and restrictions
            } else {
                loader
                    .report(&datatype_invalid_base_type(
                        top.file_id().copied().unwrap_or_default(),
                        self.base_type()
                            .source_span()
                            .as_ref()
                            .map(|span| (*span).into()),
                        self.base_type(),
                    ))
                    .unwrap();
            }
        } else {
            loader
                .report(&type_definition_not_found(
                    top.file_id().copied().unwrap_or_default(),
                    self.span.as_ref().map(|span| span.clone().into()),
                    self.base_type(),
                ))
                .unwrap();
        }
    }
}

impl References for DatatypeDef {
    fn referenced_types<'a>(&'a self, names: &mut BTreeSet<&'a IdentifierReference>) {
        names.insert(&self.base_type);
    }

    fn referenced_annotations<'a>(&'a self, names: &mut BTreeSet<&'a IdentifierReference>) {
        self.body
            .as_ref()
            .map(|b| b.referenced_annotations(names))
            .unwrap_or_default()
    }
}

impl DatatypeDef {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub const fn new(name: Identifier, base_type: IdentifierReference) -> Self {
        Self {
            span: None,
            name,
            opaque: false,
            base_type,
            restrictions: Vec::new(),
            body: None,
        }
    }

    pub const fn new_opaque(name: Identifier, base_type: IdentifierReference) -> Self {
        Self {
            span: None,
            name,
            opaque: true,
            base_type,
            restrictions: Vec::new(),
            body: None,
        }
    }

    pub fn with_body(self, body: AnnotationOnlyBody) -> Self {
        let mut self_mut = self;
        self_mut.body = Some(body);
        self_mut
    }

    pub fn with_restrictions(self, restrictions: Vec<RestrictionFacet>) -> Self {
        let mut self_mut = self;
        self_mut.restrictions = restrictions;
        self_mut
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    pub const fn is_opaque(&self) -> bool {
        self.opaque
    }

    pub fn set_opaque(&mut self, opaque: bool) {
        self.opaque = opaque;
    }

    pub const fn base_type(&self) -> &IdentifierReference {
        &self.base_type
    }

    pub fn set_base_type(&mut self, base_type: IdentifierReference) {
        self.base_type = base_type;
    }

    pub fn has_restrictions(&self) -> bool {
        !self.restrictions.is_empty()
    }

    pub fn restriction_count(&self) -> usize {
        self.restrictions.len()
    }

    pub fn restrictions(&self) -> impl Iterator<Item = &RestrictionFacet> {
        self.restrictions.iter()
    }

    pub fn restrictions_mut(&mut self) -> impl Iterator<Item = &mut RestrictionFacet> {
        self.restrictions.iter_mut()
    }

    pub fn add_to_restrictions(&mut self, restriction: RestrictionFacet) {
        self.restrictions.push(restriction);
    }

    pub fn extend_restrictions<I>(&mut self, restrictions: I)
    where
        I: IntoIterator<Item = RestrictionFacet>,
    {
        self.restrictions.extend(restrictions);
    }

    pub fn set_restrictions(&mut self, restrictions: Vec<RestrictionFacet>) {
        self.restrictions = restrictions;
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Definitions ❱ RestrictionFacet
// ------------------------------------------------------------------------------------------------

impl RestrictionFacet {
    pub fn name(&self) -> &'static str {
        match self {
            Self::FractionDigits(_, _) => KW_FACET_FRACTION_DIGITS,
            Self::TotalDigits(_, _) => KW_FACET_TOTAL_DIGITS,
            Self::Length(_, _) => KW_FACET_LENGTH,
            Self::MaxLength(_, _) => KW_FACET_MAX_LENGTH,
            Self::MinLength(_, _) => KW_FACET_MIN_LENGTH,
            Self::MaxExclusive(_, _) => KW_FACET_MAX_EXCLUSIVE,
            Self::MinExclusive(_, _) => KW_FACET_MIN_EXCLUSIVE,
            Self::MaxInclusive(_, _) => KW_FACET_MAX_INCLUSIVE,
            Self::MinInclusive(_, _) => KW_FACET_MIN_INCLUSIVE,
            Self::ExplicitTimezone(_, _) => KW_FACET_EXPLICIT_TIMEZONE,
            Self::Pattern(_) => KW_FACET_PATTERN,
        }
    }

    pub fn fixed(&self) -> Option<bool> {
        match self {
            Self::FractionDigits(_, fixed) => Some(*fixed),
            Self::TotalDigits(_, fixed) => Some(*fixed),
            Self::Length(_, fixed) => Some(*fixed),
            Self::MaxLength(_, fixed) => Some(*fixed),
            Self::MinLength(_, fixed) => Some(*fixed),
            Self::MaxExclusive(_, fixed) => Some(*fixed),
            Self::MinExclusive(_, fixed) => Some(*fixed),
            Self::MaxInclusive(_, fixed) => Some(*fixed),
            Self::MinInclusive(_, fixed) => Some(*fixed),
            Self::ExplicitTimezone(_, fixed) => Some(*fixed),
            Self::Pattern(_) => None,
        }
    }

    pub fn count(&self) -> usize {
        match self {
            Self::Pattern(vec) => vec.len(),
            _ => 1,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Definitions ❱ ExplicitTimezoneFlag
// ------------------------------------------------------------------------------------------------

impl Display for ExplicitTimezoneFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Optional => KW_FACET_TIMEZONE_OPTIONAL,
                Self::Required => KW_FACET_TIMEZONE_REQUIRED,
                Self::Prohibited => KW_FACET_TIMEZONE_PROHIBITED,
            }
        )
    }
}

impl FromStr for ExplicitTimezoneFlag {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            KW_FACET_TIMEZONE_OPTIONAL => Ok(Self::Optional),
            KW_FACET_TIMEZONE_REQUIRED => Ok(Self::Required),
            KW_FACET_TIMEZONE_PROHIBITED => Ok(Self::Prohibited),
            _ => panic!(),
        }
    }
}
