/*!
The [`ErrorCode`] type represents the set of conditions reported by the Diagnostics system.
 */

use codespan_reporting::diagnostic::Severity;
use std::fmt::Display;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u32)]
pub enum ErrorCode {
    // --------------------------------------------------------------------------------------------
    // Bugs
    // --------------------------------------------------------------------------------------------
    TreeSitterErrorNode = 2,
    TreeSitterUnexpectedNode = 3,
    TreeSitterMissingNode = 4,
    TreeSitterMissingVariable = 5,

    // --------------------------------------------------------------------------------------------
    // Errors
    // --------------------------------------------------------------------------------------------
    ModuleNotFound = 100,
    ImportedModuleNotFound = 101,
    ModuleVersionNotFound = 102,
    ModuleVersionMismatch = 103,
    DuplicateDefinitionName = 104,
    DuplicateMemberName = 105,
    DuplicateVariantName = 106,
    InvalidIdentifier = 107,
    InvalidLanguageTag = 108,
    InvalidValueForType = 109,
    InvalidModuleBaseUrl = 110,
    InvalidModuleVersionUrl = 112,
    DefinitionNotFound = 113,
    TypeDefinitionNotFound = 114,
    DatatypeInvalidBase = 115,
    TypeClassIncompatible = 116,
    PropertyIncompatible = 117,
    RdfDefinitionIncompatible = 118,
    FeatureSetNotUnion = 119,
    PropertyReferenceNotProperty = 120,

    // --------------------------------------------------------------------------------------------
    // Warnings
    // --------------------------------------------------------------------------------------------
    DuplicateModuleImport = 301,
    DuplicateDefinitionImport = 302,
    ValidationIncomplete = 303,
    ModuleVersionInfoEmpty = 304,

    // --------------------------------------------------------------------------------------------
    // Informational
    // --------------------------------------------------------------------------------------------
    IncompleteModule = 500,
    IncompleteDefinition = 501,
    IncompleteMember = 502,
    StringWithoutLanguage = 503,
    UnconstrainedDatatype = 504,
    DoubleUnderscoredIdentifier = 505,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{:04}",
            match self.severity() {
                Severity::Bug => "B",
                Severity::Error => "E",
                Severity::Warning => "W",
                Severity::Note => "I",
                Severity::Help => "H",
            },
            self.number()
        )
    }
}

impl ErrorCode {
    /// Return the numeric value for this code.
    #[inline(always)]
    pub fn number(&self) -> u32 {
        *self as u32
    }

    /// Return the severity of this code.
    #[inline(always)]
    pub fn severity(&self) -> Severity {
        match self {
            Self::TreeSitterErrorNode
            | Self::TreeSitterUnexpectedNode
            | Self::TreeSitterMissingNode
            | Self::TreeSitterMissingVariable => Severity::Bug,
            Self::ModuleNotFound
            | Self::ImportedModuleNotFound
            | Self::ModuleVersionNotFound
            | Self::ModuleVersionMismatch
            | Self::DuplicateDefinitionName
            | Self::DuplicateMemberName
            | Self::DuplicateVariantName
            | Self::InvalidIdentifier
            | Self::InvalidLanguageTag
            | Self::InvalidValueForType
            | Self::InvalidModuleBaseUrl
            | Self::InvalidModuleVersionUrl
            | Self::DefinitionNotFound
            | Self::TypeDefinitionNotFound
            | Self::DatatypeInvalidBase
            | Self::TypeClassIncompatible
            | Self::PropertyIncompatible
            | Self::RdfDefinitionIncompatible
            | Self::FeatureSetNotUnion
            | Self::PropertyReferenceNotProperty => Severity::Error,
            Self::DuplicateModuleImport
            | Self::DuplicateDefinitionImport
            | Self::ValidationIncomplete
            | Self::ModuleVersionInfoEmpty => Severity::Warning,
            Self::IncompleteModule
            | Self::IncompleteDefinition
            | Self::IncompleteMember
            | Self::StringWithoutLanguage
            | Self::UnconstrainedDatatype
            | Self::DoubleUnderscoredIdentifier => Severity::Note,
        }
    }

    /// Return the descriptive message for this code.
    pub fn message(&self) -> String {
        match self {
            Self::TreeSitterErrorNode => i18n!("msg_treesitter_error_node"),
            Self::TreeSitterUnexpectedNode => i18n!("msg_treesitter_unexpected_node"),
            Self::TreeSitterMissingNode => i18n!("msg_treesitter_missing_node"),
            Self::TreeSitterMissingVariable => i18n!("msg_treesitter_missing_variable"),
            Self::ModuleNotFound => i18n!("msg_module_not_found"),
            Self::ImportedModuleNotFound => i18n!("msg_imported_module_not_found"),
            Self::ModuleVersionNotFound => i18n!("msg_module_version_not_found"),
            Self::ModuleVersionMismatch => {
                i18n!("msg_module_version_mismatch")
            }
            Self::DuplicateDefinitionName => {
                i18n!("msg_duplicate_definition_name")
            }
            Self::DuplicateMemberName => {
                i18n!("msg_duplicate_member_name")
            }
            Self::DuplicateVariantName => {
                i18n!("msg_duplicate_variant_name")
            }
            Self::InvalidIdentifier => i18n!("msg_invalid_identifier"),
            Self::InvalidLanguageTag => i18n!("msg_invalid_language_tag"),
            Self::InvalidValueForType => i18n!("msg_invalid_value_for_type"),
            Self::InvalidModuleBaseUrl => i18n!("msg_invalid_module_base_url"),
            Self::InvalidModuleVersionUrl => i18n!("msg_invalid_module_version_url"),
            Self::DefinitionNotFound => i18n!("msg_definition_not_found"),
            Self::TypeDefinitionNotFound => i18n!("msg_type_definition_not_found"),
            Self::DatatypeInvalidBase => i18n!("msg_datatype_invalid_base"),
            Self::TypeClassIncompatible => {
                i18n!("msg_typeclass_incompatible")
            }
            Self::PropertyIncompatible => {
                i18n!("msg_property_incompatible")
            }
            Self::RdfDefinitionIncompatible => {
                i18n!("msg_rdf_definition_incompatible")
            }
            Self::FeatureSetNotUnion => i18n!("msg_featureset_not_union"),
            Self::PropertyReferenceNotProperty => i18n!("msg_property_reference_not_property"),
            Self::DuplicateModuleImport => i18n!("msg_duplicate_module_import"),
            Self::DuplicateDefinitionImport => i18n!("msg_duplicate_definition_import"),
            Self::ValidationIncomplete => i18n!("msg_validation_incomplete"),
            Self::ModuleVersionInfoEmpty => i18n!("msg_module_version_info_empty"),
            Self::IncompleteModule => i18n!("msg_incomplete_module"),
            Self::IncompleteDefinition => i18n!("msg_incomplete_definition"),
            Self::IncompleteMember => i18n!("msg_incomplete_member"),
            Self::StringWithoutLanguage => i18n!("msg_string_without_language"),
            Self::UnconstrainedDatatype => i18n!("msg_unconstrained_datatype"),
            Self::DoubleUnderscoredIdentifier => i18n!("msg_double_underscored_identifier"),
        }
    }

    /// Return a URL (as String) for the associated help documentation.
    #[inline(always)]
    pub fn url_string(&self) -> String {
        format!("https://sdml.io/errors/#{self}")
    }
}
