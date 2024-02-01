/*
This Rust module contains the SDML model of the SDML library module `xsd`.
*/

use crate::model::annotations::AnnotationBuilder;
use crate::model::identifiers::Identifier;
use crate::model::modules::Module;
use crate::model::HasBody;
use crate::stdlib::{rdf, rdfs};
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

pub const ANY_TYPE: &str = "anyType";
pub const ANY_SIMPLE_TYPE: &str = "anySimpleType";

// ------------------------------------------------------------------------------------------------
// Public Types ❱ built-in primitive types
// ------------------------------------------------------------------------------------------------

pub const ANY_URI: &str = "anyURI";
pub const BASE64_BINARY: &str = "base64Binary";
pub const BOOLEAN: &str = "boolean";
pub const DATE: &str = "date";
pub const DATETIME: &str = "dateTime";
pub const DECIMAL: &str = "decimal";
pub const DOUBLE: &str = "double";
pub const DURATION: &str = "duration";
pub const FLOAT: &str = "float";
pub const GDAY: &str = "gDay";
pub const GMONTH: &str = "gMonth";
pub const GMONTH_DAY: &str = "gMonthDay";
pub const GYEAR: &str = "gYear";
pub const GYEAR_MONTH: &str = "gYearMonth";
pub const HEX_BINARY: &str = "hexBinary";
pub const QNAME: &str = "QName";
pub const QNOTATION: &str = "QNotation";
pub const STRING: &str = "string";
pub const TIME: &str = "time";

// ------------------------------------------------------------------------------------------------
// Public Types ❱ built-in derived types
// ------------------------------------------------------------------------------------------------

pub const NORMALIZED_STRING: &str = "normalizedString";
pub const TOKEN: &str = "token";
pub const LANGUAGE: &str = "language";
pub const NAME: &str = "Name";
pub const NMTOKEN: &str = "NMTOKEN";
pub const NCNAME: &str = "NCName";
pub const ID: &str = "ID";
pub const IDREF: &str = "IDREF";
pub const ENTITY: &str = "ENTITY";

pub const INTEGER: &str = "integer";
pub const NONPOSITIVE_INTEGER: &str = "nonPositiveInteger";
pub const NEGATIVE_INTEGER: &str = "negativeInteger";
pub const LONG: &str = "long";
pub const INT: &str = "int";
pub const SHORT: &str = "short";
pub const BYTE: &str = "byte";
pub const NONNEGATIVE_INTEGER: &str = "nonNegativeInteger";
pub const UNSIGNED_LONG: &str = "unsignedLong";
pub const UNSIGNED_INT: &str = "unsignedInt";
pub const UNSIGNED_SHORT: &str = "unsignedShort";
pub const UNSIGNED_BYTE: &str = "unsignedByte";
pub const POSITIVE_INTEGER: &str = "positiveInteger";

// ------------------------------------------------------------------------------------------------
// Public Types ❱ constraining facets
// ------------------------------------------------------------------------------------------------

pub const ENUMERATION: &str = "enumeration";
pub const FRACTION_DIGITS: &str = "fractionDigits";
pub const LENGTH: &str = "length";
pub const MAX_EXCLUSIVE: &str = "maxExclusive";
pub const MAX_INCLUSIVE: &str = "maxInclusive";
pub const MAX_LENGTH: &str = "maxLength";
pub const MIN_EXCLUSIVE: &str = "minExclusive";
pub const MIN_INCLUSIVE: &str = "minInclusive";
pub const MIN_LENGTH: &str = "minLength";
pub const PATTERN: &str = "pattern";
pub const TOTAL_DIGITS: &str = "totalDigits";
pub const WHITE_SPACE: &str = "whiteSpace";

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn module() -> Module {
    #[allow(non_snake_case)]
    let MODULE_IRI: url::Url = url::Url::parse(MODULE_URL).unwrap();
    let mut module = Module::empty(id!(MODULE_NAME)).with_base_uri(Url::parse(MODULE_URL).unwrap());

    module
        .body_mut()
        .add_to_imports(import!(id!(rdf::MODULE_NAME), id!(rdfs::MODULE_NAME)));

    module.body_mut().extend_definitions(vec![
        // Purple
        rdf!(datatype ANY_TYPE, MODULE_IRI).into(),
        rdf!(datatype ANY_SIMPLE_TYPE, MODULE_IRI; ANY_TYPE).into(),
        // Blue
        rdf!(datatype ANY_URI, MODULE_IRI; ANY_SIMPLE_TYPE).into(),
        rdf!(datatype BASE64_BINARY, MODULE_IRI; ANY_SIMPLE_TYPE).into(),
        rdf!(datatype BOOLEAN, MODULE_IRI; ANY_SIMPLE_TYPE).into(),
        rdf!(datatype DATE, MODULE_IRI; ANY_SIMPLE_TYPE).into(),
        rdf!(datatype DATETIME, MODULE_IRI; ANY_SIMPLE_TYPE).into(),
        rdf!(datatype DECIMAL, MODULE_IRI; ANY_SIMPLE_TYPE).into(),
        rdf!(datatype DOUBLE, MODULE_IRI; ANY_SIMPLE_TYPE).into(),
        rdf!(datatype DURATION, MODULE_IRI; ANY_SIMPLE_TYPE).into(),
        rdf!(datatype FLOAT, MODULE_IRI; ANY_SIMPLE_TYPE).into(),
        rdf!(datatype GDAY, MODULE_IRI; ANY_SIMPLE_TYPE).into(),
        rdf!(datatype GMONTH, MODULE_IRI; ANY_SIMPLE_TYPE).into(),
        rdf!(datatype GMONTH_DAY, MODULE_IRI; ANY_SIMPLE_TYPE).into(),
        rdf!(datatype GYEAR, MODULE_IRI; ANY_SIMPLE_TYPE).into(),
        rdf!(datatype GYEAR_MONTH, MODULE_IRI; ANY_SIMPLE_TYPE).into(),
        rdf!(datatype HEX_BINARY, MODULE_IRI; ANY_SIMPLE_TYPE).into(),
        rdf!(datatype QNAME, MODULE_IRI; ANY_SIMPLE_TYPE).into(),
        rdf!(datatype QNOTATION, MODULE_IRI; ANY_SIMPLE_TYPE).into(),
        rdf!(datatype STRING, MODULE_IRI; ANY_SIMPLE_TYPE).into(),
        rdf!(datatype TIME, MODULE_IRI; ANY_SIMPLE_TYPE).into(),
        // Green
        rdf!(datatype NORMALIZED_STRING, MODULE_IRI; STRING).into(),
        rdf!(datatype TOKEN, MODULE_IRI; NORMALIZED_STRING).into(),
        rdf!(datatype LANGUAGE, MODULE_IRI; TOKEN).into(),
        rdf!(datatype NAME, MODULE_IRI; TOKEN).into(),
        rdf!(datatype NMTOKEN, MODULE_IRI; TOKEN).into(),
        rdf!(datatype NCNAME, MODULE_IRI; NAME).into(),
        rdf!(datatype NONPOSITIVE_INTEGER, MODULE_IRI; INTEGER).into(),
        rdf!(datatype NEGATIVE_INTEGER, MODULE_IRI; NONPOSITIVE_INTEGER).into(),
        rdf!(datatype LONG, MODULE_IRI; INTEGER).into(),
        rdf!(datatype INT, MODULE_IRI; LONG).into(),
        rdf!(datatype SHORT, MODULE_IRI; INT).into(),
        rdf!(datatype BYTE, MODULE_IRI; SHORT).into(),
        rdf!(datatype NONNEGATIVE_INTEGER, MODULE_IRI; INTEGER).into(),
        rdf!(datatype UNSIGNED_LONG, MODULE_IRI; NONNEGATIVE_INTEGER).into(),
        rdf!(datatype UNSIGNED_INT, MODULE_IRI; UNSIGNED_LONG).into(),
        rdf!(datatype UNSIGNED_SHORT, MODULE_IRI; UNSIGNED_INT).into(),
        rdf!(datatype UNSIGNED_BYTE, MODULE_IRI; UNSIGNED_SHORT).into(),
        rdf!(datatype POSITIVE_INTEGER, MODULE_IRI; NONNEGATIVE_INTEGER).into(),
        // Facets
        rdf!(property ENUMERATION, MODULE_IRI;
             (rdfs::MODULE_NAME, rdfs::DATATYPE))
        .into(),
        rdf!(property FRACTION_DIGITS, MODULE_IRI;
             (rdfs::MODULE_NAME, rdfs::DATATYPE))
        .into(),
        rdf!(property LENGTH, MODULE_IRI;
             (rdfs::MODULE_NAME, rdfs::DATATYPE))
        .into(),
        rdf!(property MAX_EXCLUSIVE, MODULE_IRI;
             (rdfs::MODULE_NAME, rdfs::DATATYPE))
        .into(),
        rdf!(property MAX_INCLUSIVE, MODULE_IRI;
             (rdfs::MODULE_NAME, rdfs::DATATYPE))
        .into(),
        rdf!(property MAX_LENGTH, MODULE_IRI;
             (rdfs::MODULE_NAME, rdfs::DATATYPE))
        .into(),
        rdf!(property MIN_EXCLUSIVE, MODULE_IRI;
             (rdfs::MODULE_NAME, rdfs::DATATYPE))
        .into(),
        rdf!(property MIN_INCLUSIVE, MODULE_IRI;
             (rdfs::MODULE_NAME, rdfs::DATATYPE))
        .into(),
        rdf!(property MIN_LENGTH, MODULE_IRI;
             (rdfs::MODULE_NAME, rdfs::DATATYPE))
        .into(),
        rdf!(property ENUMERATION, MODULE_IRI;
             (rdfs::MODULE_NAME, rdfs::DATATYPE))
        .into(),
        rdf!(property TOTAL_DIGITS, MODULE_IRI;
             (rdfs::MODULE_NAME, rdfs::DATATYPE))
        .into(),
        rdf!(property WHITE_SPACE, MODULE_IRI;
             (rdfs::MODULE_NAME, rdfs::DATATYPE))
        .into(),
    ]);

    module
}

pub fn is_constraining_facet(name: &Identifier) -> bool {
    is_constraining_facet_str(name.as_ref())
}

pub fn is_constraining_facet_str(name: &str) -> bool {
    [
        ENUMERATION,
        FRACTION_DIGITS,
        LENGTH,
        MAX_EXCLUSIVE,
        MAX_INCLUSIVE,
        MAX_LENGTH,
        MIN_EXCLUSIVE,
        MIN_INCLUSIVE,
        MIN_LENGTH,
        PATTERN,
        TOTAL_DIGITS,
        WHITE_SPACE,
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
