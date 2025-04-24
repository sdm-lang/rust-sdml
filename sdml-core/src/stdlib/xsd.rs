/**
This Rust module contains the SDML model of the SDML library module `xsd` for XML Schema.
 */
use crate::model::{
    annotations::{AnnotationOnlyBody, HasAnnotations},
    modules::Module,
    {HasOptionalBody},
};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub const MODULE_PATH: &str = "::org::w3";
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
pub const DATETIME_STAMP: &str = "dateTimeStamp";
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

pub const DAYTIME_DURATION: &str = "dayTimeDuration";
pub const YEARMONTH_DURATION: &str = "yearMonthDuration";

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

module_function!(|| {
    let module_uri: url::Url = url::Url::parse(MODULE_URL).unwrap();

    module!(
        id!(unchecked xsd), module_uri ; call |module: Module|
        module.with_imports([import_statement!(
            id!(unchecked dct),
        )])
            .with_annotations([
                annotation!(id!(unchecked dc_terms:title), rdf_str!("XML Schema Part 2: Datatypes Second Edition"@en)),
                annotation!(id!(unchecked rdfs:seeAlso), url!("https://www.w3.org/TR/xmlschema-2/")),
            ])
            .with_definitions([
                // ---------------------------------------------------------------------------------
                // Datatypes, Purple
                // ---------------------------------------------------------------------------------
                rdf!(
                    id!(unchecked anySimpleType) ;
                    datatype ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked xsd)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("anySimpleType"@en)),
                    ])).into(),
                // ---------------------------------------------------------------------------------
                // Datatypes, Blue
                // ---------------------------------------------------------------------------------
                datatype!(
                    id!(unchecked anyURI), idref!(unchecked anySimpleType) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked xsd)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("anyURI"@en)),
                    ])).into(),
                datatype!(
                    id!(unchecked base64Binary), idref!(unchecked anySimpleType) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked xsd)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("base64Binary"@en)),
                    ])).into(),
                datatype!(
                    id!(unchecked boolean), idref!(unchecked anySimpleType) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked xsd)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("boolean"@en)),
                    ])).into(),
                datatype!(
                    id!(unchecked date), idref!(unchecked anySimpleType) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked xsd)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("date"@en)),
                    ])).into(),
                datatype!(
                    id!(unchecked dateTime), idref!(unchecked anySimpleType) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked xsd)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("dateTime"@en)),
                    ])).into(),
                datatype!(
                    id!(unchecked decimal), idref!(unchecked anySimpleType) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked xsd)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("decimal"@en)),
                    ])).into(),
                datatype!(
                    id!(unchecked double), idref!(unchecked anySimpleType) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked xsd)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("double"@en)),
                    ])).into(),
                datatype!(
                    id!(unchecked duration), idref!(unchecked anySimpleType) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked xsd)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("duration"@en)),
                    ])).into(),
                datatype!(
                    id!(unchecked float), idref!(unchecked anySimpleType) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked xsd)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("float"@en)),
                    ])).into(),
                datatype!(
                    id!(unchecked gDay), idref!(unchecked anySimpleType) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked xsd)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("gDay"@en)),
                    ])).into(),
                datatype!(
                    id!(unchecked gMonth), idref!(unchecked anySimpleType) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked xsd)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("gMonth"@en)),
                    ])).into(),
                datatype!(
                    id!(unchecked gMonthDay), idref!(unchecked anySimpleType) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked xsd)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("gMonthDay"@en)),
                    ])).into(),
                datatype!(
                    id!(unchecked gYear), idref!(unchecked anySimpleType) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked xsd)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("gYear"@en)),
                    ])).into(),
                datatype!(
                    id!(unchecked gYearMonth), idref!(unchecked anySimpleType) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked xsd)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("gYearMonth"@en)),
                    ])).into(),
                datatype!(
                    id!(unchecked hexBinary), idref!(unchecked anySimpleType) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked xsd)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("hexBinary"@en)),
                    ])).into(),
                datatype!(
                    id!(unchecked QName), idref!(unchecked anySimpleType) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked xsd)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("QName"@en)),
                    ])).into(),
                datatype!(
                    id!(unchecked QNotation), idref!(unchecked anySimpleType) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked xsd)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("QNotation"@en)),
                    ])).into(),
                datatype!(
                    id!(unchecked string), idref!(unchecked anySimpleType) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked xsd)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("string"@en)),
                    ])).into(),
                datatype!(
                    id!(unchecked time), idref!(unchecked anySimpleType) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked xsd)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("time"@en)),
                    ])).into(),
                // ---------------------------------------------------------------------------------
                // Datatypes, Green
                // ---------------------------------------------------------------------------------
                datatype!(
                    id!(unchecked integer), idref!(unchecked decimal) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked xsd)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("integer"@en)),
                    ])).into(),
                datatype!(
                    id!(unchecked normalizedString), idref!(unchecked string) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked xsd)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("normalizedString"@en)),
                    ])).into(),
                datatype!(
                    id!(unchecked token), idref!(unchecked normalizedString) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked xsd)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("token"@en)),
                    ])).into(),
                datatype!(
                    id!(unchecked language), idref!(unchecked token) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked xsd)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("language"@en)),
                    ])).into(),
                datatype!(
                    id!(unchecked Name), idref!(unchecked token) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked xsd)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Name"@en)),
                    ])).into(),
                datatype!(
                    id!(unchecked NMTOKEN), idref!(unchecked token) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked xsd)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("NMTOKEN"@en)),
                    ])).into(),
                datatype!(
                    id!(unchecked NCNAME), idref!(unchecked Name) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked xsd)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("NCNAME"@en)),
                    ])).into(),
                datatype!(
                    id!(unchecked nonPositiveInteger), idref!(unchecked integer) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked xsd)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("nonPositiveInteger"@en)),
                    ])).into(),
                datatype!(
                    id!(unchecked negativeInteger), idref!(unchecked nonPositiveInteger) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked xsd)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("negativeInteger"@en)),
                    ])).into(),
                datatype!(
                    id!(unchecked long), idref!(unchecked integer) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked xsd)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("long"@en)),
                    ])).into(),
                datatype!(
                    id!(unchecked int), idref!(unchecked long) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked xsd)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("int"@en)),
                    ])).into(),
                datatype!(
                    id!(unchecked short), idref!(unchecked int) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked xsd)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("short"@en)),
                    ])).into(),
                datatype!(
                    id!(unchecked byte), idref!(unchecked short) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked xsd)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("byte"@en)),
                    ])).into(),
                datatype!(
                    id!(unchecked nonNegativeInteger), idref!(unchecked integer) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked xsd)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("nonNegativeInteger"@en)),
                    ])).into(),
                datatype!(
                    id!(unchecked unsignedLong), idref!(unchecked nonNegativeInteger) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked xsd)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("unsignedLong"@en)),
                    ])).into(),
                datatype!(
                    id!(unchecked unsignedInt), idref!(unchecked unsignedLong) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked xsd)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("unsignedInt"@en)),
                    ])).into(),
                datatype!(
                    id!(unchecked unsignedShort), idref!(unchecked unsignedInt) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked xsd)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("unsignedShort"@en)),
                    ])).into(),
                datatype!(
                    id!(unchecked unsignedByte), idref!(unchecked unsignedShort) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked xsd)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("unsignedByte"@en)),
                    ])).into(),
                datatype!(
                    id!(unchecked positiveInteger), idref!(unchecked nonNegativeInteger) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked xsd)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("positiveInteger"@en)),
                    ])).into(),
                // ---------------------------------------------------------------------------------
                // Facets
                // ---------------------------------------------------------------------------------
                rdf!(id!(unchecked enumeration) ;
                    property id!(unchecked Property) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked xsd)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!(enumeration@en)),
                    ])).into(),
                rdf!(id!(unchecked fractionDigits) ;
                    property id!(unchecked Property) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked xsd)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!(fractionDigits@en)),
                    ])).into(),
                rdf!(id!(unchecked length) ;
                    property id!(unchecked Property) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked xsd)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!(length@en)),
                    ])).into(),
                rdf!(id!(unchecked maxExclusive) ;
                    property id!(unchecked Property) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked xsd)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!(maxExclusive@en)),
                    ])).into(),
                rdf!(id!(unchecked maxInclusive) ;
                    property id!(unchecked Property) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked xsd)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!(maxInclusive@en)),
                    ])).into(),
                rdf!(id!(unchecked maxLength) ;
                    property id!(unchecked Property) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked xsd)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!(maxLength@en)),
                    ])).into(),
                rdf!(id!(unchecked minExclusive) ;
                    property id!(unchecked Property) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked xsd)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!(minExclusive@en)),
                    ])).into(),
                rdf!(id!(unchecked minInclusive) ;
                    property id!(unchecked Property) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked xsd)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!(minInclusive@en)),
                    ])).into(),
                rdf!(id!(unchecked minLength) ;
                    property id!(unchecked Property) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked xsd)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!(minLength@en)),
                    ])).into(),
                rdf!(id!(unchecked pattern) ;
                    property id!(unchecked Property) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked xsd)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!(pattern@en)),
                    ])).into(),
                rdf!(id!(unchecked totalDigits) ;
                    property id!(unchecked Property) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked xsd)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!(totalDigits@en)),
                    ])).into(),
                rdf!(id!(unchecked whiteSpace) ;
                    property id!(unchecked Property) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked xsd)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!(whiteSpace@en)),
                    ])).into(),
            ])
    )
});
