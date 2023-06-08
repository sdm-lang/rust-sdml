// ------------------------------------------------------------------------------------------------
// Grammar Node kinds
// ------------------------------------------------------------------------------------------------

pub(crate) const NODE_KIND_ANNOTATION: &str = "annotation";
pub(crate) const NODE_KIND_ANNOTATION_ONLY_BODY: &str = "annotation_only_body";

pub(crate) const NODE_KIND_BOOLEAN: &str = "boolean";
pub(crate) const NODE_KIND_CARDINALITY_EXPRESSION: &str = "cardinality_expression";

pub(crate) const NODE_KIND_DATA_TYPE_DEF: &str = "data_type_def";
pub(crate) const NODE_KIND_DECIMAL: &str = "decimal";
pub(crate) const NODE_KIND_DOUBLE: &str = "double";

pub(crate) const NODE_KIND_ENTITY_BODY: &str = "entity_body";
pub(crate) const NODE_KIND_ENTITY_DEF: &str = "entity_def";
pub(crate) const NODE_KIND_ENTITY_GROUP: &str = "entity_group";
pub(crate) const NODE_KIND_ENUM_BODY: &str = "enum_body";
pub(crate) const NODE_KIND_ENUM_DEF: &str = "enum_def";
pub(crate) const NODE_KIND_ENUM_VARIANT: &str = "enum_variant";
pub(crate) const NODE_KIND_EVENT_DEF: &str = "event_def";

pub(crate) const NODE_KIND_IDENTIFIER: &str = "identifier";
pub(crate) const NODE_KIND_IDENTIFIER_REFERENCE: &str = "identifier_reference";
pub(crate) const NODE_KIND_IDENTITY_MEMBER: &str = "identity_member";
pub(crate) const NODE_KIND_IMPORT: &str = "import";
pub(crate) const NODE_KIND_IMPORT_STATEMENT: &str = "import_statement";
pub(crate) const NODE_KIND_INTEGER: &str = "integer";
pub(crate) const NODE_KIND_IRI_REFERENCE: &str = "iri_reference";

pub(crate) const NODE_KIND_LANGUAGE_TAG: &str = "language_tag";
pub(crate) const NODE_KIND_LINE_COMMENT: &str = "line_comment";
pub(crate) const NODE_KIND_LIST_OF_VALUES: &str = "list_of_values";

pub(crate) const NODE_KIND_MEMBER_BY_VALUE: &str = "member_by_value";
pub(crate) const NODE_KIND_MEMBER_BY_REFERENCE: &str = "member_by_reference";
pub(crate) const NODE_KIND_MEMBER_IMPORT: &str = "member_import";
pub(crate) const NODE_KIND_MODULE: &str = "module";
pub(crate) const NODE_KIND_MODULE_BODY: &str = "module_body";
pub(crate) const NODE_KIND_MODULE_IMPORT: &str = "module_import";

pub(crate) const NODE_KIND_QUALIFIED_IDENTIFIER: &str = "qualified_identifier";
pub(crate) const NODE_KIND_QUOTED_STRING: &str = "quoted_string";

pub(crate) const NODE_KIND_SIMPLE_VALUE: &str = "simple_value";
pub(crate) const NODE_KIND_STRING: &str = "string";
pub(crate) const NODE_KIND_STRUCTURE_BODY: &str = "structure_body";
pub(crate) const NODE_KIND_STRUCTURE_DEF: &str = "structure_def";
pub(crate) const NODE_KIND_STRUCTURE_GROUP: &str = "structure_group";

pub(crate) const NODE_KIND_TYPE_DEF: &str = "type_def";
pub(crate) const NODE_KIND_TYPE_VARIANT: &str = "type_variant";

pub(crate) const NODE_KIND_UNION_BODY: &str = "union_body";
pub(crate) const NODE_KIND_UNION_DEF: &str = "union_def";
pub(crate) const NODE_KIND_UNKNOWN_TYPE: &str = "unknown_type";
pub(crate) const NODE_KIND_UNSIGNED: &str = "unsigned";

pub(crate) const NODE_KIND_VALUE_CONSTRUCTOR: &str = "value_constructor";

// ------------------------------------------------------------------------------------------------
// Grammar Node field names
// ------------------------------------------------------------------------------------------------

pub(crate) const FIELD_NAME_BASE: &str = "base";
pub(crate) const FIELD_NAME_BODY: &str = "body";

pub(crate) const FIELD_NAME_IDENTITY: &str = "identity";

pub(crate) const FIELD_NAME_LANGUAGE: &str = "language";

pub(crate) const FIELD_NAME_MEMBER: &str = "member";
pub(crate) const FIELD_NAME_MAX: &str = "max";
pub(crate) const FIELD_NAME_MIN: &str = "min";
pub(crate) const FIELD_NAME_MODULE: &str = "module";

pub(crate) const FIELD_NAME_NAME: &str = "name";

pub(crate) const FIELD_NAME_RENAME: &str = "rename";

pub(crate) const FIELD_NAME_SOURCE: &str = "source";
pub(crate) const FIELD_NAME_SOURCE_CARDINALITY: &str = "source_cardinality";

pub(crate) const FIELD_NAME_TARGET: &str = "target";
pub(crate) const FIELD_NAME_TARGET_CARDINALITY: &str = "target_cardinality";

pub(crate) const FIELD_NAME_VALUE: &str = "value";
