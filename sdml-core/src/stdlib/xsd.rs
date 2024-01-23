/*
This Rust module contains the SDML model of the SDML library module `xsd`.
*/

use crate::model::annotations::AnnotationBuilder;
use crate::model::identifiers::Identifier;
use crate::model::modules::{ImportStatement, Module};
use crate::model::HasBody;
use url::Url;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub const MODULE_NAME: &str = "xsd";
pub const MODULE_URL: &str = "http://www.w3.org/2001/XMLSchema#";

// ------------------------------------------------------------------------------------------------
// Public Types ❱ "ur" types
// ------------------------------------------------------------------------------------------------

pub const DT_ANY_TYPE_NAME: &str = "anyType";
pub const DT_ANY_SIMPLE_TYPE_NAME: &str = "anySimpleType";

// ------------------------------------------------------------------------------------------------
// Public Types ❱ built-in primitive types
// ------------------------------------------------------------------------------------------------

pub const DT_ANY_URI_NAME: &str = "anyURI";
pub const DT_BASE64_BINARY_NAME: &str = "base64Binary";
pub const DT_BOOLEAN_NAME: &str = "boolean";
pub const DT_DATE_NAME: &str = "date";
pub const DT_DATETIME_NAME: &str = "dateTime";
pub const DT_DECIMAL_NAME: &str = "decimal";
pub const DT_DOUBLE_NAME: &str = "double";
pub const DT_DURATION_NAME: &str = "duration";
pub const DT_FLOAT_NAME: &str = "float";
pub const DT_GDAY_NAME: &str = "gDay";
pub const DT_GMONTH_NAME: &str = "gMonth";
pub const DT_GMONTH_DAY_NAME: &str = "gMonthDay";
pub const DT_GYEAR_NAME: &str = "gYear";
pub const DT_GYEAR_MONTH_NAME: &str = "gYearMonth";
pub const DT_HEX_BINARY_NAME: &str = "hexBinary";
pub const DT_QNAME_NAME: &str = "QName";
pub const DT_QNOTATION_NAME: &str = "QNotation";
pub const DT_STRING_NAME: &str = "string";
pub const DT_TIME_NAME: &str = "time";

// ------------------------------------------------------------------------------------------------
// Public Types ❱ built-in derived types
// ------------------------------------------------------------------------------------------------

pub const DT_NORMALIZED_STRING_NAME: &str = "normalizedString";
pub const DT_TOKEN_NAME: &str = "token";
pub const DT_LANGUAGE_NAME: &str = "language";
pub const DT_NAME_NAME: &str = "Name";
pub const DT_NMTOKEN_NAME: &str = "NMTOKEN";
pub const DT_NCNAME_NAME: &str = "NCName";
pub const DT_ID_NAME: &str = "ID";
pub const DT_IDREF_NAME: &str = "IDREF";
pub const DT_ENTITY_NAME: &str = "ENTITY";

pub const DT_INTEGER_NAME: &str = "string";
pub const DT_NONPOSITIVE_INTEGER_NAME: &str = "string";
pub const DT_NEGATIVE_INTEGER_NAME: &str = "string";
pub const DT_LONG_NAME: &str = "string";
pub const DT_INT_NAME: &str = "string";
pub const DT_SHORT_NAME: &str = "string";
pub const DT_BYTE_NAME: &str = "string";
pub const DT_NONNEGATIVE_INTEGER_NAME: &str = "string";
pub const DT_UNSIGNED_LONG_NAME: &str = "string";
pub const DT_UNSIGNED_INT_NAME: &str = "string";
pub const DT_UNSIGNED_SHORT_NAME: &str = "string";
pub const DT_UNSIGNED_BYTE_NAME: &str = "string";
pub const DT_POSITIVE_INTEGER_NAME: &str = "string";

// ------------------------------------------------------------------------------------------------
// Public Types ❱ constraining facets
// ------------------------------------------------------------------------------------------------

pub const PROP_ENUMERATION_NAME: &str = "enumeration";
pub const PROP_FRACTION_DIGITS_NAME: &str = "fractionDigits";
pub const PROP_LENGTH_NAME: &str = "length";
pub const PROP_MAX_EXCLUSIVE_NAME: &str = "maxExclusive";
pub const PROP_MAX_INCLUSIVE_NAME: &str = "maxInclusive";
pub const PROP_MAX_LENGTH_NAME: &str = "maxLength";
pub const PROP_MIN_EXCLUSIVE_NAME: &str = "minExclusive";
pub const PROP_MIN_INCLUSIVE_NAME: &str = "minInclusive";
pub const PROP_MIN_LENGTH_NAME: &str = "minLength";
pub const PROP_PATTERN_NAME: &str = "pattern";
pub const PROP_TOTAL_DIGITS_NAME: &str = "totalDigits";
pub const PROP_WHITE_SPACE_NAME: &str = "whiteSpace";

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn module() -> Module {
    #[allow(non_snake_case)]
    let MODULE_IRI: url::Url = url::Url::parse(MODULE_URL).unwrap();
    let mut module = Module::empty(id!(MODULE_NAME)).with_base_uri(Url::parse(MODULE_URL).unwrap());

    module
        .body_mut()
        .add_to_imports(ImportStatement::new_module(id!(super::rdf::MODULE_NAME)));
    module
        .body_mut()
        .add_to_imports(ImportStatement::new_module(id!(super::rdfs::MODULE_NAME)));

    module.body_mut().extend_definitions(vec![
        // Purple
        rdf!(datatype DT_ANY_TYPE_NAME, MODULE_IRI).into(),
        rdf!(datatype DT_ANY_SIMPLE_TYPE_NAME, MODULE_IRI; DT_ANY_TYPE_NAME).into(),
        // Blue
        rdf!(datatype DT_ANY_URI_NAME, MODULE_IRI; DT_ANY_SIMPLE_TYPE_NAME).into(),
        rdf!(datatype DT_BASE64_BINARY_NAME, MODULE_IRI; DT_ANY_SIMPLE_TYPE_NAME).into(),
        rdf!(datatype DT_BOOLEAN_NAME, MODULE_IRI; DT_ANY_SIMPLE_TYPE_NAME).into(),
        rdf!(datatype DT_DATE_NAME, MODULE_IRI; DT_ANY_SIMPLE_TYPE_NAME).into(),
        rdf!(datatype DT_DATETIME_NAME, MODULE_IRI; DT_ANY_SIMPLE_TYPE_NAME).into(),
        rdf!(datatype DT_DECIMAL_NAME, MODULE_IRI; DT_ANY_SIMPLE_TYPE_NAME).into(),
        rdf!(datatype DT_DOUBLE_NAME, MODULE_IRI; DT_ANY_SIMPLE_TYPE_NAME).into(),
        rdf!(datatype DT_DURATION_NAME, MODULE_IRI; DT_ANY_SIMPLE_TYPE_NAME).into(),
        rdf!(datatype DT_FLOAT_NAME, MODULE_IRI; DT_ANY_SIMPLE_TYPE_NAME).into(),
        rdf!(datatype DT_GDAY_NAME, MODULE_IRI; DT_ANY_SIMPLE_TYPE_NAME).into(),
        rdf!(datatype DT_GMONTH_NAME, MODULE_IRI; DT_ANY_SIMPLE_TYPE_NAME).into(),
        rdf!(datatype DT_GMONTH_DAY_NAME, MODULE_IRI; DT_ANY_SIMPLE_TYPE_NAME).into(),
        rdf!(datatype DT_GYEAR_NAME, MODULE_IRI; DT_ANY_SIMPLE_TYPE_NAME).into(),
        rdf!(datatype DT_GYEAR_MONTH_NAME, MODULE_IRI; DT_ANY_SIMPLE_TYPE_NAME).into(),
        rdf!(datatype DT_HEX_BINARY_NAME, MODULE_IRI; DT_ANY_SIMPLE_TYPE_NAME).into(),
        rdf!(datatype DT_QNAME_NAME, MODULE_IRI; DT_ANY_SIMPLE_TYPE_NAME).into(),
        rdf!(datatype DT_QNOTATION_NAME, MODULE_IRI; DT_ANY_SIMPLE_TYPE_NAME).into(),
        rdf!(datatype DT_STRING_NAME, MODULE_IRI; DT_ANY_SIMPLE_TYPE_NAME).into(),
        rdf!(datatype DT_TIME_NAME, MODULE_IRI; DT_ANY_SIMPLE_TYPE_NAME).into(),
        // Green
        rdf!(datatype DT_NORMALIZED_STRING_NAME, MODULE_IRI; DT_STRING_NAME).into(),
        rdf!(datatype DT_TOKEN_NAME, MODULE_IRI; DT_NORMALIZED_STRING_NAME).into(),
        rdf!(datatype DT_LANGUAGE_NAME, MODULE_IRI; DT_TOKEN_NAME).into(),
        rdf!(datatype DT_NAME_NAME, MODULE_IRI; DT_TOKEN_NAME).into(),
        rdf!(datatype DT_NMTOKEN_NAME, MODULE_IRI; DT_TOKEN_NAME).into(),
        rdf!(datatype DT_NCNAME_NAME, MODULE_IRI; DT_NAME_NAME).into(),
        rdf!(datatype DT_NONPOSITIVE_INTEGER_NAME, MODULE_IRI; DT_INTEGER_NAME).into(),
        rdf!(datatype DT_NEGATIVE_INTEGER_NAME, MODULE_IRI; DT_NONPOSITIVE_INTEGER_NAME).into(),
        rdf!(datatype DT_LONG_NAME, MODULE_IRI; DT_INTEGER_NAME).into(),
        rdf!(datatype DT_INT_NAME, MODULE_IRI; DT_LONG_NAME).into(),
        rdf!(datatype DT_SHORT_NAME, MODULE_IRI; DT_INT_NAME).into(),
        rdf!(datatype DT_BYTE_NAME, MODULE_IRI; DT_SHORT_NAME).into(),
        rdf!(datatype DT_NONNEGATIVE_INTEGER_NAME, MODULE_IRI; DT_INTEGER_NAME).into(),
        rdf!(datatype DT_UNSIGNED_LONG_NAME, MODULE_IRI; DT_NONNEGATIVE_INTEGER_NAME).into(),
        rdf!(datatype DT_UNSIGNED_INT_NAME, MODULE_IRI; DT_UNSIGNED_LONG_NAME).into(),
        rdf!(datatype DT_UNSIGNED_SHORT_NAME, MODULE_IRI; DT_UNSIGNED_INT_NAME).into(),
        rdf!(datatype DT_UNSIGNED_BYTE_NAME, MODULE_IRI; DT_UNSIGNED_SHORT_NAME).into(),
        rdf!(datatype DT_POSITIVE_INTEGER_NAME, MODULE_IRI; DT_NONNEGATIVE_INTEGER_NAME).into(),
        // Facets
        rdf!(property PROP_ENUMERATION_NAME, MODULE_IRI;
             (super::rdfs::MODULE_NAME, super::rdfs::CLASS_DATATYPE_NAME))
        .into(),
        rdf!(property PROP_FRACTION_DIGITS_NAME, MODULE_IRI;
             (super::rdfs::MODULE_NAME, super::rdfs::CLASS_DATATYPE_NAME))
        .into(),
        rdf!(property PROP_LENGTH_NAME, MODULE_IRI;
             (super::rdfs::MODULE_NAME, super::rdfs::CLASS_DATATYPE_NAME))
        .into(),
        rdf!(property PROP_MAX_EXCLUSIVE_NAME, MODULE_IRI;
             (super::rdfs::MODULE_NAME, super::rdfs::CLASS_DATATYPE_NAME))
        .into(),
        rdf!(property PROP_MAX_INCLUSIVE_NAME, MODULE_IRI;
             (super::rdfs::MODULE_NAME, super::rdfs::CLASS_DATATYPE_NAME))
        .into(),
        rdf!(property PROP_MAX_LENGTH_NAME, MODULE_IRI;
             (super::rdfs::MODULE_NAME, super::rdfs::CLASS_DATATYPE_NAME))
        .into(),
        rdf!(property PROP_MIN_EXCLUSIVE_NAME, MODULE_IRI;
             (super::rdfs::MODULE_NAME, super::rdfs::CLASS_DATATYPE_NAME))
        .into(),
        rdf!(property PROP_MIN_INCLUSIVE_NAME, MODULE_IRI;
             (super::rdfs::MODULE_NAME, super::rdfs::CLASS_DATATYPE_NAME))
        .into(),
        rdf!(property PROP_MIN_LENGTH_NAME, MODULE_IRI;
             (super::rdfs::MODULE_NAME, super::rdfs::CLASS_DATATYPE_NAME))
        .into(),
        rdf!(property PROP_ENUMERATION_NAME, MODULE_IRI;
             (super::rdfs::MODULE_NAME, super::rdfs::CLASS_DATATYPE_NAME))
        .into(),
        rdf!(property PROP_TOTAL_DIGITS_NAME, MODULE_IRI;
             (super::rdfs::MODULE_NAME, super::rdfs::CLASS_DATATYPE_NAME))
        .into(),
        rdf!(property PROP_WHITE_SPACE_NAME, MODULE_IRI;
             (super::rdfs::MODULE_NAME, super::rdfs::CLASS_DATATYPE_NAME))
        .into(),
    ]);

    module
}

pub fn is_constraining_facet(name: &Identifier) -> bool {
    is_constraining_facet_str(name.as_ref())
}

pub fn is_constraining_facet_str(name: &str) -> bool {
    [
        PROP_ENUMERATION_NAME,
        PROP_FRACTION_DIGITS_NAME,
        PROP_LENGTH_NAME,
        PROP_MAX_EXCLUSIVE_NAME,
        PROP_MAX_INCLUSIVE_NAME,
        PROP_MAX_LENGTH_NAME,
        PROP_MIN_EXCLUSIVE_NAME,
        PROP_MIN_INCLUSIVE_NAME,
        PROP_MIN_LENGTH_NAME,
        PROP_PATTERN_NAME,
        PROP_TOTAL_DIGITS_NAME,
        PROP_WHITE_SPACE_NAME,
    ]
    .contains(&name)
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
