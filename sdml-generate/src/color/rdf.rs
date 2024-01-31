/*!
One-line description.

More detailed description, with

# Example

 */

use std::fmt::Display;

use sdml_core::model::values::LanguageString;

use super::ColorElement;

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

// ------------------------------------------------------------------------------------------------
// Public Functions ❱ Turtle Directives
// ------------------------------------------------------------------------------------------------

#[inline]
pub fn base_directive<U>(url: U) -> String
where
    U: AsRef<str>,
{
    format!(
        "{} {}{}",
        ColorElement::Keyword.colorize("@base"),
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
        ColorElement::Keyword.colorize("@prefix"),
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
        ColorElement::ModuleDefinition.colorize(module.as_ref()),
        ColorElement::PunctuationSeparator.colorize(":"),
    )
}

#[inline]
pub fn module_ref_qname<S>(module: S) -> String
where
    S: AsRef<str>,
{
    format!(
        "{}{}",
        ColorElement::Module.colorize(module.as_ref()),
        ColorElement::PunctuationSeparator.colorize(":"),
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
        ColorElement::Type.colorize(ty_name)
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
        ColorElement::Variable.colorize(thing_name)
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
    ColorElement::LiteralStringSpecial.colorize(format!("<{}>", url.as_ref()))
}

#[inline]
pub fn format_str<S>(v: S) -> String
where
    S: AsRef<str>,
{
    ColorElement::LiteralString.colorize(format!("{:?}", v.as_ref()))
}

#[inline]
pub fn format_lang_str(v: &LanguageString) -> String {
    format!(
        "{}{}",
        format_str(v.value()),
        if let Some(lang) = v.language() {
            ColorElement::LiteralString.colorize(format!("{lang}"))
        } else {
            String::new()
        }
    )
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
        ColorElement::Operator.colorize("^^"),
        type_ref_qname(module, ty_name),
    )
}

#[inline]
pub fn format_number<N>(v: N) -> String
where
    N: Into<String>,
{
    ColorElement::LiteralNumber.colorize(v.into())
}

#[inline]
pub fn format_boolean(v: bool) -> String {
    ColorElement::LiteralBoolean.colorize(v.to_string())
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
        ColorElement::TypeDefinition.colorize(ty_name),
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
    ColorElement::PunctuationBracket.colorize("[")
}

#[inline]
pub fn start_bnode_eol() -> String {
    format!(" {}\n", start_bnode())
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
    ColorElement::PunctuationBracket.colorize("]")
}

// ------------------------------------------------------------------------------------------------
// Public Functions ❱ Collections
// ------------------------------------------------------------------------------------------------

#[inline]
pub fn start_collection() -> String {
    ColorElement::PunctuationBracket.colorize("(")
}

#[inline]
pub fn start_collection_eol() -> String {
    format!(" {}\n", start_collection())
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
    ColorElement::PunctuationBracket.colorize(")")
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
                    format!(" {}\n", ColorElement::PunctuationDelimiter.colorize(".")),
                Separator::Predicate =>
                    format!(" {}\n", ColorElement::PunctuationDelimiter.colorize(";")),
                Separator::Object =>
                    format!(" {}\n", ColorElement::PunctuationDelimiter.colorize(",")),
                Separator::InlineObject =>
                    format!("{} ", ColorElement::PunctuationDelimiter.colorize(",")),
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
