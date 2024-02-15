/*!
Provides project-wide diagnostic types that describe more fine-grained error conditions..

 */

use crate::codes::ErrorCode;
use crate::{errors::Error, Span};
use crate::{FileId, SourceFiles};
use codespan_reporting::diagnostic::Label;
use codespan_reporting::{
    diagnostic::Severity,
    term::{
        emit,
        termcolor::{ColorChoice, StandardStream, WriteColor},
        Chars, Config,
    },
};
use std::cell::RefCell;
use std::io::Write;
use std::ops::{Add, AddAssign};
use tracing::{error, info, warn};
use std::sync::OnceLock;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types  Diagnostics and Reporter
// ------------------------------------------------------------------------------------------------

///
/// The type of diagnostic reports.
///
pub type Diagnostic = codespan_reporting::diagnostic::Diagnostic<FileId>;

///
/// This trait describes a facility to report diagnostics.
///
pub trait Reporter: Default {
    ///
    /// Emit a diagnostic, providing a mapping for source code.
    ///
    fn emit(&self, diagnostic: &Diagnostic, sources: &SourceFiles) -> Result<(), Error>;

    fn emit_without_source(&self, diagnostic: &Diagnostic) -> Result<(), Error> {
        self.emit(diagnostic, &SourceFiles::new())
    }

    fn done(&self, module_name: Option<String>) -> Result<(), Error>;

    fn log(diagnostic: &Diagnostic) {
        match diagnostic.severity {
            Severity::Bug | Severity::Error => error!(
                "[{}] {}",
                diagnostic.code.as_ref().unwrap(),
                diagnostic.message
            ),
            Severity::Warning => warn!(
                "[{}] {}",
                diagnostic.code.as_ref().unwrap(),
                diagnostic.message
            ),
            Severity::Note | Severity::Help => info!(
                "[{}] {}",
                diagnostic.code.as_ref().unwrap(),
                diagnostic.message
            ),
        }
    }
}

#[derive(Debug)]
pub struct StandardStreamReporter {
    stream: StandardStream,
    config: Config,
    counters: RefCell<ErrorCounters>,
}

#[derive(Debug, Default)]
pub struct BailoutReporter;

// ------------------------------------------------------------------------------------------------
// Diagnostic Level
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SeverityFilter {
    Bug,
    Error,
    Warning,
    Note,
    Help,
    None,
}

static DIAGNOSTIC_LEVEL: OnceLock<SeverityFilter> = OnceLock::new();

pub fn get_diagnostic_level_filter() -> SeverityFilter {
    *DIAGNOSTIC_LEVEL.get_or_init(|| SeverityFilter::None)
}

pub fn set_diagnostic_level_filter(level: SeverityFilter) -> Result<(), SeverityFilter> {
    DIAGNOSTIC_LEVEL.set(level)
}

pub fn diagnostic_level_enabled(level: Severity) -> bool {
    match get_diagnostic_level_filter() {
        SeverityFilter::Bug => level >= Severity::Bug,
        SeverityFilter::Error => level >= Severity::Error,
        SeverityFilter::Warning => level >= Severity::Warning,
        SeverityFilter::Note => level >= Severity::Note,
        SeverityFilter::Help => level >= Severity::Help,
        SeverityFilter::None => false,
    }
}

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

macro_rules! new_diagnostic {
    ($code: ident, $callback_fn: expr) => {
        $callback_fn(Diagnostic::from(ErrorCode::$code)).with_notes(vec![i18n!(
            "help_more_details_url",
            url = ErrorCode::$code.url_string()
        )])
    };
}

// ------------------------------------------------------------------------------------------------
// Public Functions  Bugs
// ------------------------------------------------------------------------------------------------

///
/// Note: tree-sitter originated errors will *always* have a location.
///
#[inline]
#[allow(clippy::redundant_closure_call)]
pub fn found_error_node<S>(file_id: FileId, location: Span, in_rule: S) -> Diagnostic
where
    S: Into<String>,
{
    new_diagnostic!(TreeSitterErrorNode, |diagnostic: Diagnostic| diagnostic
        .with_labels(vec![
            Label::primary(file_id, location).with_message(i18n!("lbl_here"))
        ])
        .with_notes(vec![i18n!("lbl_in_grammar_rule", name = in_rule.into())]))
}

///
/// Note: tree-sitter originated errors will *always* have a location.
///
#[inline]
#[allow(clippy::redundant_closure_call)]
pub fn unexpected_node_kind<S1, S2>(
    file_id: FileId,
    location: Span,
    expecting: S1,
    got: S2,
) -> Diagnostic
where
    S1: Into<String>,
    S2: Into<String>,
{
    let expecting = expecting.into();
    let expecting = if expecting.contains('|') {
        i18n!("lbl_expecting_one_of_node_kind", kind = expecting)
    } else {
        i18n!("lbl_expecting_node_kind", kind = expecting)
    };
    new_diagnostic!(TreeSitterUnexpectedNode, |diagnostic: Diagnostic| {
        diagnostic.with_labels(vec![
            Label::primary(file_id, location.clone())
                .with_message(i18n!("lbl_actual_node_kind", kind = got.into())),
            Label::secondary(file_id, location).with_message(expecting),
        ])
    })
}

///
/// Note: tree-sitter originated errors will *always* have a location.
///
#[inline]
#[allow(clippy::redundant_closure_call)]
pub fn missing_node<S1, S2>(
    file_id: FileId,
    location: Span,
    expecting: S1,
    in_variable: Option<S2>,
) -> Diagnostic
where
    S1: Into<String>,
    S2: Into<String>,
{
    let message = if let Some(in_variable) = in_variable {
        i18n!(
            "lbl_missing_node_kind_in_variable",
            kind = expecting.into(),
            variable = in_variable.into()
        )
    } else {
        i18n!("lbl_missing_node_kind", kind = expecting.into())
    };
    new_diagnostic!(TreeSitterMissingNode, |diagnostic: Diagnostic| diagnostic
        .with_labels(vec![
            Label::primary(file_id, location).with_message(message)
        ]))
}

///
/// Note: tree-sitter originated errors will *always* have a location.
///
#[inline]
#[allow(clippy::redundant_closure_call)]
pub fn missing_variable_in_node<S1, S2>(
    file_id: FileId,
    location: Span,
    expecting: S1,
    in_node: Option<S2>,
) -> Diagnostic
where
    S1: Into<String>,
    S2: Into<String>,
{
    let message = if let Some(in_node) = in_node {
        format!(
            "missing a variable named `{}` in grammar node kind `{}`",
            expecting.into(),
            in_node.into()
        )
    } else {
        format!("missing a variable named `{}`", expecting.into())
    };
    new_diagnostic!(TreeSitterMissingVariable, |diagnostic: Diagnostic| {
        diagnostic.with_labels(vec![Label::primary(file_id, location).with_message(message)])
    })
}

// ------------------------------------------------------------------------------------------------
// Public Functions  Errors
// ------------------------------------------------------------------------------------------------

#[inline]
#[allow(clippy::redundant_closure_call)]
pub fn module_not_found<S>(name: S) -> Diagnostic
where
    S: Into<String>,
{
    new_diagnostic!(ModuleNotFound, |diagnostic: Diagnostic| diagnostic
        .with_notes(vec![i18n!("lbl_module_name", name = name.into())]))
}

#[inline]
#[allow(clippy::redundant_closure_call)]
pub fn imported_module_not_found<S>(file_id: FileId, location: Option<Span>, name: S) -> Diagnostic
where
    S: Into<String>,
{
    new_diagnostic!(
        ImportedModuleNotFound,
        |diagnostic: Diagnostic| if let Some(location) = location {
            diagnostic.with_labels(vec![
                Label::primary(file_id, location).with_message(i18n!("lbl_this_import"))
            ])
        } else {
            diagnostic.with_notes(vec![i18n!("lbl_module_name", name = name.into())])
        }
    )
}

#[inline]
#[allow(clippy::redundant_closure_call)]
pub fn module_version_not_found<S1, S2>(
    file_id: FileId,
    expecting_location: Option<Span>,
    expecting: S1,
    actual_file_id: FileId,
    actual_location: Option<Span>,
    actual: S2,
) -> Diagnostic
where
    S1: Into<String>,
    S2: Into<String>,
{
    new_diagnostic!(
        ModuleVersionNotFound,
        |diagnostic: Diagnostic| if let Some(location) = expecting_location {
            let diagnostic = diagnostic.with_labels(vec![Label::primary(file_id, location)
                .with_message(i18n!("lbl_this_import"))]);
            if let Some(location) = actual_location {
                diagnostic.with_labels(vec![Label::secondary(actual_file_id, location)
                    .with_message(i18n!("lbl_this_module"))])
            } else {
                diagnostic
            }
        } else {
            diagnostic.with_notes(vec![
                i18n!("lbl_expected_version_uri", url = expecting.into()),
                i18n!("lbl_module_name", name = actual.into()),
            ])
        }
    )
}

#[inline]
#[allow(clippy::redundant_closure_call)]
pub fn module_version_mismatch<S1, S2>(
    file_id: FileId,
    expecting_location: Option<Span>,
    expecting: S1,
    actual_file_id: FileId,
    actual_location: Option<Span>,
    actual: S2,
) -> Diagnostic
where
    S1: Into<String>,
    S2: Into<String>,
{
    new_diagnostic!(
        ModuleVersionMismatch,
        |diagnostic: Diagnostic| if let Some(location) = expecting_location {
            let diagnostic = diagnostic.with_labels(vec![Label::primary(file_id, location)
                .with_message(i18n!("lbl_expected_this_version_uri"))]);
            if let Some(location) = actual_location {
                diagnostic.with_labels(vec![Label::secondary(actual_file_id, location)
                    .with_message(i18n!("lbl_actual_this_version_uri"))])
            } else {
                diagnostic
            }
        } else {
            diagnostic.with_notes(vec![
                i18n!("lbl_expected_version_uri", url = expecting.into()),
                i18n!("lbl_actual_version_uri", url = actual.into()),
            ])
        }
    )
}

#[inline]
#[allow(clippy::redundant_closure_call)]
pub fn duplicate_definition(file_id: FileId, first: Span, second: Span) -> Diagnostic {
    new_diagnostic!(DuplicateDefinitionName, |diagnostic: Diagnostic| diagnostic
        .with_labels(vec![
            Label::primary(file_id, second).with_message(i18n!("lbl_this_definition_name")),
            Label::secondary(file_id, first).with_message(i18n!("lbl_previously_defined_here")),
        ]))
}

#[inline]
#[allow(clippy::redundant_closure_call)]
pub fn duplicate_member(file_id: FileId, first: Span, second: Span) -> Diagnostic {
    new_diagnostic!(DuplicateMemberName, |diagnostic: Diagnostic| diagnostic
        .with_labels(vec![
            Label::primary(file_id, second).with_message(i18n!("lbl_this_member_name")),
            Label::secondary(file_id, first).with_message(i18n!("lbl_previously_defined_here")),
        ]))
}

#[inline]
#[allow(clippy::redundant_closure_call)]
pub fn duplicate_variant(file_id: FileId, first: Span, second: Span) -> Diagnostic {
    new_diagnostic!(DuplicateDefinitionName, |diagnostic: Diagnostic| diagnostic
        .with_labels(vec![
            Label::primary(file_id, second).with_message(i18n!("lbl_this_variant_name")),
            Label::secondary(file_id, first).with_message(i18n!("lbl_previously_defined_here")),
        ]))
}

#[inline]
#[allow(clippy::redundant_closure_call)]
pub fn invalid_identifier<S>(file_id: FileId, location: Option<Span>, value: S) -> Diagnostic
where
    S: Into<String>,
{
    new_diagnostic!(
        InvalidIdentifier,
        |diagnostic: Diagnostic| if let Some(location) = location {
            diagnostic.with_labels(vec![
                Label::primary(file_id, location).with_message(i18n!("lbl_this_identifier"))
            ])
        } else {
            diagnostic.with_notes(vec![i18n!("lbl_value", val = value.into())])
        }
    )
}

#[inline]
#[allow(clippy::redundant_closure_call)]
pub fn invalid_language_tag<S>(file_id: FileId, location: Option<Span>, value: S) -> Diagnostic
where
    S: Into<String>,
{
    new_diagnostic!(
        InvalidLanguageTag,
        |diagnostic: Diagnostic| if let Some(location) = location {
            diagnostic.with_labels(vec![
                Label::primary(file_id, location).with_message(i18n!("lbl_this_language_tag"))
            ])
        } else {
            diagnostic.with_notes(vec![i18n!("lbl_value", val = value.into())])
        }
    )
}

#[inline]
#[allow(clippy::redundant_closure_call)]
pub fn invalid_value_for_type<S1, S2>(
    value_file_id: FileId,
    value_location: Option<Span>,
    value: S1,
    type_file_id: FileId,
    type_location: Option<Span>,
    type_name: S2,
) -> Diagnostic
where
    S1: Into<String>,
    S2: Into<String>,
{
    new_diagnostic!(InvalidValueForType, |diagnostic: Diagnostic| {
        let diagnostic = if let Some(location) = value_location {
            diagnostic.with_labels(vec![
                Label::primary(value_file_id, location).with_message(i18n!("lbl_this_value"))
            ])
        } else {
            diagnostic.with_notes(vec![i18n!("lbl_value", val = value.into())])
        };
        if let Some(location) = type_location {
            diagnostic.with_labels(vec![
                Label::primary(type_file_id, location).with_message(i18n!("lbl_this_type"))
            ])
        } else {
            diagnostic.with_notes(vec![i18n!("lbl_type_name", name = type_name.into())])
        }
    })
}

#[inline]
#[allow(clippy::redundant_closure_call)]
pub fn invalid_value_for_type_named<S1, S2>(
    value_file_id: FileId,
    value_location: Option<Span>,
    value: S1,
    type_name: S2,
) -> Diagnostic
where
    S1: Into<String>,
    S2: Into<String>,
{
    new_diagnostic!(InvalidValueForType, |diagnostic: Diagnostic| {
        if let Some(location) = value_location {
            diagnostic
                .with_labels(vec![
                    Label::primary(value_file_id, location).with_message(i18n!("lbl_this_value"))
                ])
                .with_notes(vec![i18n!("lbl_type_name", name = type_name.into())])
        } else {
            diagnostic.with_notes(vec![
                i18n!("lbl_value", val = value.into()),
                i18n!("lbl_type_name", name = type_name.into()),
            ])
        }
    })
}

#[inline]
#[allow(clippy::redundant_closure_call)]
pub fn definition_not_found<S>(
    file_id: FileId,
    reference_location: Option<Span>,
    name: S,
) -> Diagnostic
where
    S: Into<String>,
{
    new_diagnostic!(
        DefinitionNotFound,
        |diagnostic: Diagnostic| if let Some(reference_location) = reference_location {
            diagnostic.with_labels(vec![Label::primary(file_id, reference_location)
                .with_message(i18n!("lbl_this_reference"))])
        } else {
            diagnostic.with_notes(vec![i18n!("lbl_definition_name", name = name.into())])
        }
    )
}

#[inline]
#[allow(clippy::redundant_closure_call)]
pub fn type_definition_not_found<S>(
    file_id: FileId,
    reference_location: Option<Span>,
    name: S,
) -> Diagnostic
where
    S: Into<String>,
{
    new_diagnostic!(
        DefinitionNotFound,
        |diagnostic: Diagnostic| if let Some(reference_location) = reference_location {
            diagnostic.with_labels(vec![Label::primary(file_id, reference_location)
                .with_message(i18n!("lbl_this_reference"))])
        } else {
            diagnostic.with_notes(vec![i18n!("lbl_type_name", name = name.into())])
        }
    )
}

#[inline]
#[allow(clippy::redundant_closure_call)]
pub fn datatype_invalid_base_type<S>(
    file_id: FileId,
    reference_location: Option<Span>,
    name: S,
) -> Diagnostic
where
    S: Into<String>,
{
    new_diagnostic!(
        DatatypeInvalidBase,
        |diagnostic: Diagnostic| if let Some(reference_location) = reference_location {
            diagnostic.with_labels(vec![Label::primary(file_id, reference_location)
                .with_message(i18n!("lbl_this_reference"))])
        } else {
            diagnostic.with_notes(vec![i18n!("lbl_type_name", name = name.into())])
        }
    )
}

#[inline]
#[allow(clippy::redundant_closure_call)]
pub fn type_class_incompatible_usage<S>(
    file_id: FileId,
    reference_location: Option<Span>,
    name: S,
) -> Diagnostic
where
    S: Into<String>,
{
    new_diagnostic!(TypeClassIncompatible, |diagnostic: Diagnostic| if let Some(
        reference_location,
    ) = reference_location
    {
        diagnostic.with_labels(vec![
            Label::primary(file_id, reference_location).with_message(i18n!("lbl_this_usage"))
        ])
    } else {
        diagnostic.with_notes(vec![i18n!("lbl_typeclass_name", name = name.into())])
    })
}

#[inline]
#[allow(clippy::redundant_closure_call)]
pub fn property_incompatible_usage<S>(
    file_id: FileId,
    reference_location: Option<Span>,
    name: S,
) -> Diagnostic
where
    S: Into<String>,
{
    new_diagnostic!(
        PropertyIncompatible,
        |diagnostic: Diagnostic| if let Some(reference_location) = reference_location {
            diagnostic
                .with_labels(vec![Label::primary(file_id, reference_location)
                    .with_message(i18n!("lbl_this_usage"))])
        } else {
            diagnostic.with_notes(vec![i18n!("lbl_property_name", name = name.into())])
        }
    )
}

#[inline]
#[allow(clippy::redundant_closure_call)]
pub fn rdf_definition_incompatible_usage<S>(
    file_id: FileId,
    reference_location: Option<Span>,
    name: S,
) -> Diagnostic
where
    S: Into<String>,
{
    new_diagnostic!(
        RdfDefinitionIncompatible,
        |diagnostic: Diagnostic| if let Some(reference_location) = reference_location {
            diagnostic
                .with_labels(vec![Label::primary(file_id, reference_location)
                    .with_message(i18n!("lbl_this_usage"))])
        } else {
            diagnostic.with_notes(vec![i18n!("lbl_rdf_name", name = name.into())])
        }
    )
}

#[inline]
#[allow(clippy::redundant_closure_call)]
pub fn feature_set_not_a_union<S>(
    file_id: FileId,
    reference_location: Option<Span>,
    name: S,
) -> Diagnostic
where
    S: Into<String>,
{
    new_diagnostic!(
        FeatureSetNotUnion,
        |diagnostic: Diagnostic| if let Some(reference_location) = reference_location {
            diagnostic.with_labels(vec![Label::primary(file_id, reference_location)
                .with_message(i18n!("lbl_this_reference"))])
        } else {
            diagnostic.with_notes(vec![i18n!("lbl_type_name", name = name.into())])
        }
    )
}

// ------------------------------------------------------------------------------------------------
// Public Functions  Warnings
// ------------------------------------------------------------------------------------------------

#[inline]
#[allow(clippy::redundant_closure_call)]
pub fn duplicate_module_import(file_id: FileId, first: Span, second: Span) -> Diagnostic {
    new_diagnostic!(DuplicateModuleImport, |diagnostic: Diagnostic| diagnostic
        .with_labels(vec![
            Label::primary(file_id, second).with_message(i18n!("lbl_this_module")),
            Label::secondary(file_id, first).with_message(i18n!("lbl_previously_imported_here")),
        ]))
}

#[inline]
#[allow(clippy::redundant_closure_call)]
pub fn duplicate_definition_import(file_id: FileId, first: Span, second: Span) -> Diagnostic {
    new_diagnostic!(DuplicateDefinitionImport, |diagnostic: Diagnostic| {
        diagnostic.with_labels(vec![
            Label::primary(file_id, second).with_message(i18n!("lbl_this_member")),
            Label::secondary(file_id, first).with_message(i18n!("lbl_previously_imported_here")),
        ])
    })
}

#[inline]
#[allow(clippy::redundant_closure_call)]
pub fn type_validation_incomplete<S>(
    file_id: FileId,
    location: Option<Span>,
    type_name: S,
) -> Diagnostic
where
    S: Into<String>,
{
    new_diagnostic!(
        ValidationIncomplete,
        |diagnostic: Diagnostic| if let Some(location) = location {
            diagnostic.with_labels(vec![
                Label::primary(file_id, location).with_message(i18n!("lbl_this_definition"))
            ])
        } else {
            diagnostic.with_notes(vec![i18n!("lbl_type_name", name = type_name.into())])
        }
    )
}

#[inline]
#[allow(clippy::redundant_closure_call)]
pub fn module_version_info_empty(file_id: FileId, location: Option<Span>) -> Diagnostic {
    new_diagnostic!(
        ModuleVersionInfoEmpty,
        |diagnostic: Diagnostic| if let Some(location) = location {
            diagnostic.with_labels(vec![
                Label::primary(file_id, location).with_message(i18n!("lbl_this_value"))
            ])
        } else {
            diagnostic
        }
    )
}

// ------------------------------------------------------------------------------------------------
// Public Functions  Informational
// ------------------------------------------------------------------------------------------------

#[inline]
#[allow(clippy::redundant_closure_call)]
pub fn module_is_incomplete<S>(file_id: FileId, location: Option<Span>, name: S) -> Diagnostic
where
    S: Into<String>,
{
    new_diagnostic!(
        IncompleteModule,
        |diagnostic: Diagnostic| if let Some(location) = location {
            diagnostic.with_labels(vec![
                Label::primary(file_id, location).with_message(i18n!("lbl_this_module"))
            ])
        } else {
            diagnostic.with_notes(vec![i18n!("lbl_module_name", name = name.into())])
        }
    )
}

#[inline]
#[allow(clippy::redundant_closure_call)]
pub fn definition_is_incomplete<S>(file_id: FileId, location: Option<Span>, name: S) -> Diagnostic
where
    S: Into<String>,
{
    new_diagnostic!(
        IncompleteDefinition,
        |diagnostic: Diagnostic| if let Some(location) = location {
            diagnostic.with_labels(vec![
                Label::primary(file_id, location).with_message(i18n!("lbl_this_definition"))
            ])
        } else {
            diagnostic.with_notes(vec![i18n!("lbl_definition_name", name = name.into())])
        }
    )
}

#[inline]
#[allow(clippy::redundant_closure_call)]
pub fn member_is_incomplete<S>(file_id: FileId, location: Option<Span>, name: S) -> Diagnostic
where
    S: Into<String>,
{
    new_diagnostic!(
        IncompleteMember,
        |diagnostic: Diagnostic| if let Some(location) = location {
            diagnostic.with_labels(vec![
                Label::primary(file_id, location).with_message(i18n!("lbl_this_member"))
            ])
        } else {
            diagnostic.with_notes(vec![i18n!("lbl_member_name", name = name.into())])
        }
    )
}

#[inline]
#[allow(clippy::redundant_closure_call)]
pub fn string_without_language<S>(file_id: FileId, location: Option<Span>, value: S) -> Diagnostic
where
    S: Into<String>,
{
    new_diagnostic!(
        StringWithoutLanguage,
        |diagnostic: Diagnostic| if let Some(location) = location {
            diagnostic.with_labels(vec![
                Label::primary(file_id, location).with_message(i18n!("lbl_this_value"))
            ])
        } else {
            diagnostic.with_notes(vec![i18n!("lbl_value", val = value.into())])
        }
    )
}

#[inline]
#[allow(clippy::redundant_closure_call)]
pub fn using_unconstrained_datatype<S>(
    file_id: FileId,
    location: Option<Span>,
    name: S,
) -> Diagnostic
where
    S: Into<String>,
{
    new_diagnostic!(
        UnconstrainedDatatype,
        |diagnostic: Diagnostic| if let Some(location) = location {
            diagnostic.with_labels(vec![
                Label::primary(file_id, location).with_message(i18n!("lbl_this_type"))
            ])
        } else {
            diagnostic.with_notes(vec![i18n!("lbl_type_name", name = name.into())])
        }
    )
}

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, Default)]
struct ErrorCounters {
    bugs: u32,
    errors: u32,
    warnings: u32,
    info: u32,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl PartialEq<Severity> for SeverityFilter {
    fn eq(&self, other: &Severity) -> bool {
        match (self, other) {
            (SeverityFilter::Bug, Severity::Bug) => true,
            (SeverityFilter::Error, Severity::Error) => true,
            (SeverityFilter::Warning, Severity::Warning) => true,
            (SeverityFilter::Note, Severity::Note) => true,
            (SeverityFilter::Help, Severity::Help) => true,
            _ => false,
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl From<ErrorCode> for Diagnostic {
    fn from(code: ErrorCode) -> Self {
        Self::new(code.severity())
            .with_code(code.to_string())
            .with_message(code.message().to_string())
    }
}

// ------------------------------------------------------------------------------------------------

impl Add for ErrorCounters {
    type Output = ErrorCounters;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            bugs: self.bugs + rhs.bugs,
            errors: self.errors + rhs.errors,
            warnings: self.warnings + rhs.warnings,
            info: self.info,
        }
    }
}

impl AddAssign for ErrorCounters {
    fn add_assign(&mut self, rhs: Self) {
        self.bugs += rhs.bugs;
        self.errors += rhs.errors;
        self.warnings += rhs.warnings;
        self.info += rhs.info;
    }
}

impl ErrorCounters {
    #[inline(always)]
    fn report(&mut self, severity: Severity) {
        match severity {
            Severity::Bug => self.bugs += 1,
            Severity::Error => self.errors += 1,
            Severity::Warning => self.warnings += 1,
            Severity::Note => self.info += 1,
            Severity::Help => self.info += 1,
        }
    }

    #[inline(always)]
    fn total(&self) -> u64 {
        (self.bugs + self.errors + self.warnings + self.info) as u64
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for StandardStreamReporter {
    fn default() -> Self {
        Self::stderr(Default::default())
    }
}

impl Reporter for StandardStreamReporter {
    fn emit(&self, diagnostic: &Diagnostic, sources: &SourceFiles) -> Result<(), Error> {
        if diagnostic_level_enabled(diagnostic.severity) {
            <StandardStreamReporter as Reporter>::log(diagnostic);
            let mut counters = self.counters.borrow_mut();
            counters.report(diagnostic.severity);
            Ok(emit(
                &mut self.stream.lock(),
                &self.config,
                sources,
                diagnostic,
            )?)
        } else {
            Ok(())
        }
    }

    fn done(&self, module_name: Option<String>) -> Result<(), Error> {
        self.done_stats(module_name)?;
        let _ = self.counters.replace(ErrorCounters::default());
        Ok(())
    }
}

impl StandardStreamReporter {
    pub fn stderr(color_choice: ColorChoice) -> Self {
        Self {
            stream: StandardStream::stderr(color_choice),
            config: Self::default_config(),
            counters: Default::default(),
        }
    }

    pub fn stdout(color_choice: ColorChoice) -> Self {
        Self {
            stream: StandardStream::stdout(color_choice),
            config: Self::default_config(),
            counters: Default::default(),
        }
    }

    fn default_config() -> Config {
        Config {
            chars: Chars::box_drawing(),
            ..Default::default()
        }
    }

    fn done_stats(&self, module_name: Option<String>) -> Result<(), Error> {
        let counters = self.counters.borrow();
        if counters.total() > 0 {
            let severity = if counters.bugs > 0 {
                Severity::Bug
            } else if counters.errors > 0 {
                Severity::Error
            } else if counters.warnings > 0 {
                Severity::Warning
            } else if counters.info > 0 {
                Severity::Note
            } else {
                unreachable!();
            };

            let mut writer = self.stream.lock();

            writer.set_color(self.config.styles.header(severity))?;
            writer.write_all(
                match severity {
                    Severity::Bug => i18n!("word_bug"),
                    Severity::Error => i18n!("word_error"),
                    Severity::Warning => i18n!("word_warning"),
                    Severity::Note => i18n!("word_note"),
                    Severity::Help => i18n!("word_help"),
                }
                .as_bytes(),
            )?;
            writer.reset()?;
            writer.write_all(b": ")?;
            if let Some(name) = module_name {
                writer.write_all(i18n!("lbl_module_name", name = name).as_bytes())?;
            }
            let mut count_strings: Vec<String> = Default::default();
            if counters.bugs > 0 {
                count_strings.push(i18n!("count_of_bugs", count = counters.bugs));
            }
            if counters.errors > 0 {
                count_strings.push(i18n!("count_of_errors", count = counters.errors));
            }
            if counters.warnings > 0 {
                count_strings.push(i18n!("count_of_warnings", count = counters.warnings));
            }
            if counters.info > 0 {
                count_strings.push(i18n!("count_of_informational", count = counters.info));
            }
            writer.write_all(
                i18n!(
                    "counts_generated_summary",
                    counts = count_strings.join(", ")
                )
                .as_bytes(),
            )?;
            writer.write_all(b"\n")?;
        }
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------

impl Reporter for BailoutReporter {
    fn emit(&self, diagnostic: &Diagnostic, _: &SourceFiles) -> Result<(), Error> {
        if diagnostic_level_enabled(diagnostic.severity) {
            <BailoutReporter as Reporter>::log(diagnostic);
            Err(diagnostic.clone().into())
        } else {
            Ok(())
        }
    }

    fn done(&self, _: Option<String>) -> Result<(), Error> {
        Ok(())
    }
}
