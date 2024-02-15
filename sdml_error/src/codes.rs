/*!
One-line description.

More detailed description, with

# Example

End of file during parsingSymbol’s value as variable is void: rustEnd of file during parsing

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
    #[inline(always)]
    pub fn number(&self) -> u32 {
        *self as u32
    }

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
            | Self::FeatureSetNotUnion => Severity::Error,
            Self::DuplicateModuleImport
            | Self::DuplicateDefinitionImport
            | Self::ValidationIncomplete
            | Self::ModuleVersionInfoEmpty => Severity::Warning,
            Self::IncompleteModule
            | Self::IncompleteDefinition
            | Self::IncompleteMember
            | Self::StringWithoutLanguage
            | Self::UnconstrainedDatatype => Severity::Note,
        }
    }

    pub fn message(&self) -> String {
        match self {
            ErrorCode::TreeSitterErrorNode => i18n!("msg_treesitter_error_node"),
            ErrorCode::TreeSitterUnexpectedNode => i18n!("msg_treesitter_unexpected_node"),
            ErrorCode::TreeSitterMissingNode => i18n!("msg_treesitter_missing_node"),
            ErrorCode::TreeSitterMissingVariable => i18n!("msg_treesitter_missing_variable"),
            ErrorCode::ModuleNotFound => i18n!("msg_module_not_found"),
            ErrorCode::ImportedModuleNotFound => i18n!("msg_imported_module_not_found"),
            ErrorCode::ModuleVersionNotFound => i18n!("msg_module_version_not_found"),
            ErrorCode::ModuleVersionMismatch => {
                i18n!("msg_module_version_mismatch")
            }
            ErrorCode::DuplicateDefinitionName => {
                i18n!("msg_duplicate_definition_name")
            }
            ErrorCode::DuplicateMemberName => {
                i18n!("msg_duplicate_member_name")
            }
            ErrorCode::DuplicateVariantName => {
                i18n!("msg_duplicate_variant_name")
            }
            ErrorCode::InvalidIdentifier => i18n!("msg_invalid_identifier"),
            ErrorCode::InvalidLanguageTag => i18n!("msg_invalid_language_tag"),
            ErrorCode::InvalidValueForType => i18n!("msg_invalid_value_for_type"),
            ErrorCode::InvalidModuleBaseUrl => i18n!("msg_invalid_module_base_url"),
            ErrorCode::InvalidModuleVersionUrl => i18n!("msg_invalid_module_version_url"),
            ErrorCode::DefinitionNotFound => i18n!("msg_definition_not_found"),
            ErrorCode::TypeDefinitionNotFound => i18n!("msg_type_definition_not_found"),
            ErrorCode::DatatypeInvalidBase => i18n!("msg_datatype_invalid_base"),
            ErrorCode::TypeClassIncompatible => {
                i18n!("msg_typeclass_incompatible")
            }
            ErrorCode::PropertyIncompatible => {
                i18n!("msg_property_incompatible")
            }
            ErrorCode::RdfDefinitionIncompatible => {
                i18n!("msg_rdf_definition_incompatible")
            }
            ErrorCode::FeatureSetNotUnion => i18n!("msg_featureset_not_union"),
            ErrorCode::DuplicateModuleImport => i18n!("msg_duplicate_module_import"),
            ErrorCode::DuplicateDefinitionImport => i18n!("msg_duplicate_definition_import"),
            ErrorCode::ValidationIncomplete => i18n!("msg_validation_incomplete"),
            ErrorCode::ModuleVersionInfoEmpty => i18n!("msg_module_version_info_empty"),
            ErrorCode::IncompleteModule => i18n!("msg_incomplete_module"),
            ErrorCode::IncompleteDefinition => i18n!("msg_incomplete_definition"),
            ErrorCode::IncompleteMember => i18n!("msg_incomplete_member"),
            ErrorCode::StringWithoutLanguage => i18n!("msg_string_without_language"),
            ErrorCode::UnconstrainedDatatype => i18n!("msg_unconstrained_datatype"),
        }
    }

    #[inline(always)]
    pub fn url_string(&self) -> String {
        format!("https://sdml.io/errors/#{self}")
    }
}
