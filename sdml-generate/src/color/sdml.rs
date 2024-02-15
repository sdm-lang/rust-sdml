/*!
One-line description.

More detailed description, with

# Example

End of file during parsingSymbol’s value as variable is void: rustEnd of file during parsing

 */

use crate::color::{Colorizer, ConsoleColor};
use sdml_core::model::identifiers::{Identifier, IdentifierReference};
use sdml_core::model::modules::Import;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

const COLORIZER: ConsoleColor = ConsoleColor::new();

#[inline]
pub fn keyword<S>(kw: S) -> String
where
    S: AsRef<str>,
{
    COLORIZER.keyword(kw)
}

#[inline]
pub fn operator<S>(op: S) -> String
where
    S: AsRef<str>,
{
    COLORIZER.operator(op)
}

#[inline]
pub fn format_url<S>(url: S) -> String
where
    S: AsRef<str>,
{
    COLORIZER.url(format!("<{}>", url.as_ref()))
}

#[inline]
pub fn module_name_def(name: &Identifier) -> String {
    COLORIZER.module_definition(name)
}

#[inline]
pub fn module_name_ref(name: &Identifier) -> String {
    COLORIZER.module(name)
}

#[inline]
pub fn type_name_def(name: &Identifier) -> String {
    COLORIZER.type_definition(name)
}

#[inline]
pub fn value_variant_name_def(name: &Identifier) -> String {
    COLORIZER.constant_definition(name)
}

#[inline]
pub fn type_variant_ref_def(name: &IdentifierReference) -> String {
    match name {
        IdentifierReference::Identifier(id) => type_variant_name_def(id),
        IdentifierReference::QualifiedIdentifier(qid) => format!(
            "{}{}{}",
            COLORIZER.module(qid.module()),
            COLORIZER.delimiter(":"),
            type_variant_name_def(qid.member()),
        ),
    }
}

#[inline]
pub fn type_variant_name_def(name: &Identifier) -> String {
    COLORIZER.type_definition(name)
}

#[inline]
pub fn type_name_ref(name: &IdentifierReference) -> String {
    match name {
        IdentifierReference::Identifier(id) => COLORIZER.type_ref(id),
        IdentifierReference::QualifiedIdentifier(qid) => format!(
            "{}{}{}",
            COLORIZER.module(qid.module()),
            COLORIZER.delimiter(":"),
            COLORIZER.type_ref(qid.member()),
        ),
    }
}

#[inline]
pub fn sequence_start() -> String {
    COLORIZER.bracket("[")
}

#[inline]
pub fn sequence_end() -> String {
    COLORIZER.bracket("]")
}

#[inline]
pub fn paren_start() -> String {
    COLORIZER.bracket("(")
}

#[inline]
pub fn paren_end() -> String {
    COLORIZER.bracket(")")
}

#[inline]
pub fn braces_start() -> String {
    COLORIZER.bracket("{")
}

#[inline]
pub fn braces_end() -> String {
    COLORIZER.bracket("}")
}

#[inline]
pub fn property_name(name: &IdentifierReference) -> String {
    COLORIZER.property(format!("@{}", name))
}

#[inline]
pub fn member_name(name: &Identifier) -> String {
    COLORIZER.variable_field(name)
}

#[inline]
pub fn import(import: &Import) -> String {
    match import {
        Import::Module(import) => {
            if let Some(version_uri) = import.version_uri() {
                format!(
                    "{} {}",
                    COLORIZER.module(import.name()),
                    format_url(version_uri.as_ref()),
                )
            } else {
                COLORIZER.module(import.name())
            }
        }
        Import::Member(qid) => format!(
            "{}{}{}",
            COLORIZER.module(qid.module()),
            COLORIZER.delimiter(":"),
            COLORIZER.type_ref(qid.member()),
        ),
    }
}

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
