/*!
One-line description.

More detailed description, with

# Example

 */

use crate::color::{Colorizer, ConsoleColor};
use sdml_core::model::values::LanguageString;
use std::fmt::Display;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Separator {
    None,
    InlineNone,
    Statement,
    Predicate,
    Object,
    InlineObject,
}

pub const INDENT_PREDICATE: &str = "    ";
pub const INDENT_OBJECT: &str = "        ";

pub const SYNTAX_KW_BASE: &str = "@base";
pub const SYNTAX_KW_PREFIX: &str = "@prefix";
pub const SYNTAX_NAME_SEP: &str = ":";
pub const SYNTAX_OPERATOR_TYPE: &str = "^^";
pub const SYNTAX_COLLECTION_START: &str = "(";
pub const SYNTAX_COLLECTION_END: &str = ")";
pub const SYNTAX_BNODE_START: &str = "[";
pub const SYNTAX_BNODE_END: &str = "]";
pub const SYNTAX_IRI_START: &str = "<";
pub const SYNTAX_IRI_END: &str = ">";
pub const SYNTAX_STATEMENT_DELIM: &str = ".";
pub const SYNTAX_PREDICATE_DELIM: &str = ";";
pub const SYNTAX_OBJECT_DELIM: &str = ",";

// ------------------------------------------------------------------------------------------------
// Public Functions ❱ Turtle Directives
// ------------------------------------------------------------------------------------------------

const COLORIZER: ConsoleColor = ConsoleColor::new();

#[inline]
pub fn base_directive<U>(url: U) -> String
where
    U: AsRef<str>,
{
    format!(
        "{} {}{}",
        COLORIZER.keyword(SYNTAX_KW_BASE),
        format_url(url),
        statement_sep()
    )
}

#[inline]
pub fn prefix_directive<S, U>(module: S, url: U) -> String
where
    S: AsRef<str>,
    U: AsRef<str>,
{
    format!(
        "{} {} {}{}",
        COLORIZER.keyword(SYNTAX_KW_PREFIX),
        module_ref_qname(module),
        format_url(url),
        statement_sep(),
    )
}

// ------------------------------------------------------------------------------------------------
// Public Functions ❱ Qualified Names
// ------------------------------------------------------------------------------------------------

#[inline]
pub fn mv_name<S1, S2>(parent: S1, name: S2) -> String
where
    S1: AsRef<str>,
    S2: AsRef<str>,
{
    format!("{}__{}", parent.as_ref(), name.as_ref())
}

// ------------------------------------------------------------------------------------------------
// Public Functions ❱ Qualified Names ❱ Modules
// ------------------------------------------------------------------------------------------------

#[inline]
pub fn module_def_qname<S>(module: S) -> String
where
    S: AsRef<str>,
{
    format!(
        "{}{}",
        COLORIZER.module_definition(module),
        COLORIZER.separator(SYNTAX_NAME_SEP),
    )
}

#[inline]
pub fn module_ref_qname<S>(module: S) -> String
where
    S: AsRef<str>,
{
    format!(
        "{}{}",
        COLORIZER.module(module),
        COLORIZER.separator(SYNTAX_NAME_SEP),
    )
}

// ------------------------------------------------------------------------------------------------
// Public Functions ❱ Qualified Names ❱ Predicates
// ------------------------------------------------------------------------------------------------

#[inline]
pub fn predicate_def_qname<S1, S2>(module: S1, property: S2) -> String
where
    S1: AsRef<str>,
    S2: AsRef<str>,
{
    format!("{}{}", module_ref_qname(module), property.as_ref(),)
}

#[inline]
pub fn predicate_qname<S1, S2>(module: S1, property: S2) -> String
where
    S1: AsRef<str>,
    S2: AsRef<str>,
{
    format!("{}{}", module_ref_qname(module), property.as_ref(),)
}

// ------------------------------------------------------------------------------------------------
// Public Functions ❱ Qualified Names ❱ Classes
// ------------------------------------------------------------------------------------------------

#[inline]
pub fn type_ref_qname<S1, S2>(module: S1, ty_name: S2) -> String
where
    S1: AsRef<str>,
    S2: AsRef<str>,
{
    format!(
        "{}{}",
        module_ref_qname(module),
        COLORIZER.type_ref(ty_name)
    )
}

#[inline]
pub fn type_ref_list<S1, S2>(ty_names: &[(S1, S2)]) -> String
where
    S1: AsRef<str>,
    S2: AsRef<str>,
{
    let list_sep = Separator::InlineObject.to_string();
    ty_names
        .iter()
        .map(|(module, ty_name)| type_ref_qname(module, ty_name))
        .collect::<Vec<_>>()
        .join(&list_sep)
}

// ------------------------------------------------------------------------------------------------
// Public Functions ❱ Qualified Names ❱ Things (Individuals)
// ------------------------------------------------------------------------------------------------

#[inline]
pub fn thing_qname<S1, S2>(module: S1, thing_name: S2) -> String
where
    S1: AsRef<str>,
    S2: AsRef<str>,
{
    format!(
        "{}{}",
        module_ref_qname(module),
        COLORIZER.variable(thing_name)
    )
}

// ------------------------------------------------------------------------------------------------
// Public Functions ❱ Simple Types
// ------------------------------------------------------------------------------------------------

#[inline]
pub fn format_url<U>(url: U) -> String
where
    U: AsRef<str>,
{
    COLORIZER.url(format!(
        "{}{}{}",
        SYNTAX_IRI_START,
        url.as_ref(),
        SYNTAX_IRI_END
    ))
}

#[inline]
pub fn format_str<S>(v: S) -> String
where
    S: AsRef<str>,
{
    COLORIZER.string(v.as_ref()))
}

#[inline]
pub fn format_lang_str(v: &LanguageString) -> String {
    format_str(v.to_string())
}

#[inline]
pub fn format_type_constructor<S1, S2, S3>(module: S1, ty_name: S2, value: S3) -> String
where
    S1: AsRef<str>,
    S2: AsRef<str>,
    S3: AsRef<str>,
{
    format!(
        "{}{}{}",
        format_str(value),
        COLORIZER.operator(SYNTAX_OPERATOR_TYPE),
        type_ref_qname(module, ty_name),
    )
}

#[inline]
pub fn format_number<N>(v: N) -> String
where
    N: Into<String>,
{
    COLORIZER.number(v.into())
}

#[inline]
pub fn format_boolean(v: bool) -> String {
    COLORIZER.boolean(v.to_string())
}

// ------------------------------------------------------------------------------------------------
// Public Functions ❱ Subjects
// ------------------------------------------------------------------------------------------------

#[inline]
pub fn module_subject<S1>(module: S1) -> String
where
    S1: AsRef<str>,
{
    format!("{}{}", module_def_qname(module), Separator::None)
}

#[inline]
pub fn type_subject<S1, S2>(module: S1, ty_name: S2) -> String
where
    S1: AsRef<str>,
    S2: AsRef<str>,
{
    format!(
        "{}{}{}",
        module_ref_qname(module),
        COLORIZER.type_definition(ty_name),
        Separator::None,
    )
}

#[inline]
pub fn property_subject<S1, S2>(module: S1, predicate: S2) -> String
where
    S1: AsRef<str>,
    S2: AsRef<str>,
{
    format!(
        "{}{}{}",
        module_ref_qname(module),
        predicate.as_ref(),
        Separator::None,
    )
}

#[inline]
pub fn thing_subject<S1, S2>(module: S1, name: S2) -> String
where
    S1: AsRef<str>,
    S2: AsRef<str>,
{
    format!(
        "{}{}{}",
        module_ref_qname(module),
        name.as_ref(),
        Separator::None,
    )
}

// ------------------------------------------------------------------------------------------------
// Public Functions ❱ Predicates
// ------------------------------------------------------------------------------------------------

#[inline]
pub fn predicate_no_value<S1, S2>(module: S1, property: S2, sep: Separator) -> String
where
    S1: AsRef<str>,
    S2: AsRef<str>,
{
    format!(
        "{INDENT_PREDICATE}{}{}",
        predicate_qname(module, property),
        sep,
    )
}

#[inline]
pub fn predicate_with_value<S1, S2, S3>(
    module: S1,
    property: S2,
    value: S3,
    sep: Separator,
) -> String
where
    S1: AsRef<str>,
    S2: AsRef<str>,
    S3: AsRef<str>,
{
    format!(
        "{INDENT_PREDICATE}{} {}{}",
        predicate_qname(module, property),
        value.as_ref(),
        sep,
    )
}

#[inline]
pub fn predicate_with_value_list<S1, S2, S3>(
    module: S1,
    property: S2,
    values: &[S3],
    sep: Separator,
) -> String
where
    S1: AsRef<str>,
    S2: AsRef<str>,
    S3: AsRef<str>,
{
    format!(
        "{}{}{}",
        predicate_no_value(module, property, Separator::None),
        object_list(values),
        sep
    )
}

// ------------------------------------------------------------------------------------------------
// Public Functions ❱ Objects
// ------------------------------------------------------------------------------------------------

#[inline]
pub fn object_list<S>(values: &[S]) -> String
where
    S: AsRef<str>,
{
    values
        .iter()
        .map(|s| format!("{INDENT_OBJECT}{}", s.as_ref()))
        .collect::<Vec<_>>()
        .join(&Separator::Object.to_string())
}

#[inline]
pub fn inline_object_list<S>(values: &[S]) -> String
where
    S: AsRef<str>,
{
    values
        .iter()
        .map(|s| s.as_ref())
        .collect::<Vec<_>>()
        .join(&Separator::InlineObject.to_string())
}

// ------------------------------------------------------------------------------------------------
// Public Functions ❱ Blank Nodes (Anonymous Objects)
// ------------------------------------------------------------------------------------------------

#[inline]
pub fn start_bnode() -> String {
    COLORIZER.bracket(SYNTAX_BNODE_START)
}

#[inline]
pub fn bnode_predicate_with_value<S1, S2, S3>(
    module: S1,
    property: S2,
    value: S3,
    sep: Separator,
) -> String
where
    S1: AsRef<str>,
    S2: AsRef<str>,
    S3: AsRef<str>,
{
    format!(
        "{INDENT_OBJECT}{} {} {} {}{}",
        start_bnode(),
        predicate_qname(module, property),
        value.as_ref(),
        end_bnode(),
        sep,
    )
}

#[inline]
pub fn end_bnode() -> String {
    COLORIZER.bracket(SYNTAX_BNODE_END)
}

// ------------------------------------------------------------------------------------------------
// Public Functions ❱ Collections
// ------------------------------------------------------------------------------------------------

#[inline]
pub fn start_collection() -> String {
    COLORIZER.bracket(SYNTAX_COLLECTION_START)
}

#[inline]
pub fn collection_element<S>(element: S) -> String
where
    S: AsRef<str>,
{
    format!("{INDENT_OBJECT}{}{}", element.as_ref(), Separator::None)
}

#[inline]
pub fn collection_element_list<S>(elements: &[S], sep: Separator) -> String
where
    S: AsRef<str>,
{
    format!(
        "{}\n{}\n{}{}",
        start_collection(),
        elements
            .iter()
            .map(collection_element)
            .collect::<Vec<_>>()
            .join(&Separator::None.to_string()),
        end_collection(),
        sep
    )
}

#[inline]
pub fn end_collection() -> String {
    COLORIZER.bracket(SYNTAX_COLLECTION_END)
}

// ------------------------------------------------------------------------------------------------
// Public Functions ❱ Separators
// ------------------------------------------------------------------------------------------------

#[inline]
pub fn statement_sep() -> String {
    Separator::Statement.to_string()
}

#[inline]
pub fn predicate_sep() -> String {
    Separator::Predicate.to_string()
}

#[inline]
pub fn object_sep() -> String {
    Separator::Object.to_string()
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

impl Display for Separator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Separator::None => "\n".to_string(),
                Separator::InlineNone => "".to_string(),
                Separator::Statement =>
                    format!(" {}\n", COLORIZER.delimiter(SYNTAX_STATEMENT_DELIM)),
                Separator::Predicate =>
                    format!(" {}\n", COLORIZER.delimiter(SYNTAX_PREDICATE_DELIM)),
                Separator::Object => format!(" {}\n", COLORIZER.delimiter(SYNTAX_OBJECT_DELIM)),
                Separator::InlineObject => format!("{} ", COLORIZER.delimiter(SYNTAX_OBJECT_DELIM)),
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
