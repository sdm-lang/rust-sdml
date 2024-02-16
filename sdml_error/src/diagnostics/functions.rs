/*!
One-line description.

More detailed description, with

# Example

End of file during parsingSymbol’s value as variable is void: rustEnd of file during parsing

 */

use crate::diagnostics::{Diagnostic, ErrorCode};
use crate::{FileId, Span};
use codespan_reporting::diagnostic::Label;

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
            let diagnostic = diagnostic.with_labels(vec![
                Label::primary(file_id, location).with_message(i18n!("lbl_this_import"))
            ]);
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

#[inline]
#[allow(clippy::redundant_closure_call)]
pub fn property_reference_not_property<S>(
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
