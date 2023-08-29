/*!
Contains only string constants for the tree-sitter node types and field names.
 */

pub const NAME_SDML: &str = "sdml";

// ------------------------------------------------------------------------------------------------
// Grammar Node kinds
// ------------------------------------------------------------------------------------------------

pub const NODE_KIND_ANNOTATION: &str = "annotation";
pub const NODE_KIND_ANNOTATION_PROPERTY: &str = "annotation_property";
pub const NODE_KIND_ANNOTATION_ONLY_BODY: &str = "annotation_only_body";
pub const NODE_KIND_ATOMIC_SENTENCE: &str = "atomic_sentence";

pub const NODE_KIND_BICONDITIONAL: &str = "biconditional";
pub const NODE_KIND_BINARY_BOOLEAN_SENTENCE: &str = "binary_boolean_sentence";
pub const NODE_KIND_BOOLEAN: &str = "boolean";
pub const NODE_KIND_BOOLEAN_SENTENCE: &str = "boolean_sentence";
pub const NODE_KIND_BUILTIN_SIMPLE_TYPE: &str = "builtin_simple_type";

pub const NODE_KIND_CARDINALITY_EXPRESSION: &str = "cardinality_expression";
pub const NODE_KIND_CONJUNCTION: &str = "conjunction";
pub const NODE_KIND_CONSTANT_DEF: &str = "constant_def";
pub const NODE_KIND_CONSTRAINT: &str = "constraint";
pub const NODE_KIND_CONSTRAINT_ENVIRONMENT: &str = "constraint_environment";
pub const NODE_KIND_CONSTRAINT_ENVIRONMENT_END: &str = "constraint_environment_end";
pub const NODE_KIND_CONSTRAINT_SENTENCE: &str = "constraint_sentence";
pub const NODE_KIND_CONTROLLED_LANGUAGE_TAG: &str = "controlled_language_tag";

pub const NODE_KIND_DATA_TYPE_DEF: &str = "data_type_def";
pub const NODE_KIND_DECIMAL: &str = "decimal";
pub const NODE_KIND_DEFINITION: &str = "definition";
pub const NODE_KIND_DISJUNCTION: &str = "disjunction";
pub const NODE_KIND_DOUBLE: &str = "double";

pub const NODE_KIND_ENTITY_BODY: &str = "entity_body";
pub const NODE_KIND_ENTITY_DEF: &str = "entity_def";
pub const NODE_KIND_ENTITY_GROUP: &str = "entity_group";
pub const NODE_KIND_ENUM_BODY: &str = "enum_body";
pub const NODE_KIND_ENUM_DEF: &str = "enum_def";
pub const NODE_KIND_ENVIRONMENT_DEFINITION: &str = "environment_definition";
pub const NODE_KIND_EQUATION: &str = "equation";
pub const NODE_KIND_EVENT_DEF: &str = "event_def";
pub const NODE_KIND_EXCLUSIVE_DISJUNCTION: &str = "exclusive_disjunction";
pub const NODE_KIND_EXISTENTIAL: &str = "existential";

pub const NODE_KIND_FORMAL_CONSTRAINT: &str = "formal_constraint";
pub const NODE_KIND_FUNCTION_CARDINALITY_EXPRESSION: &str = "function_cardinality_expression";
pub const NODE_KIND_FUNCTION_COMPOSITION: &str = "function_composition";
pub const NODE_KIND_FUNCTION_DEF: &str = "function_def";
pub const NODE_KIND_FUNCTION_PARAMETER: &str = "function_parameter";
pub const NODE_KIND_FUNCTION_SIGNATURE: &str = "function_signature";
pub const NODE_KIND_FUNCTION_TYPE_REFERENCE: &str = "function_type_reference";
pub const NODE_KIND_FUNCTIONAL_TERM: &str = "functional_term";

pub const NODE_KIND_GREATER_THAN: &str = "greater_than";
pub const NODE_KIND_GREATER_THAN_OR_EQUAL: &str = "greater_than_or_equal";

pub const NODE_KIND_IDENTIFIER: &str = "identifier";
pub const NODE_KIND_IDENTIFIER_REFERENCE: &str = "identifier_reference";
pub const NODE_KIND_IDENTITY_MEMBER: &str = "identity_member";
pub const NODE_KIND_IDENTITY_ROLE: &str = "identity_role";
pub const NODE_KIND_IMPLICATION: &str = "implication";
pub const NODE_KIND_IMPORT: &str = "import";
pub const NODE_KIND_IMPORT_STATEMENT: &str = "import_statement";
pub const NODE_KIND_INEQUATION: &str = "inequation";
pub const NODE_KIND_INFORMAL_CONSTRAINT: &str = "informal_constraint";
pub const NODE_KIND_INTEGER: &str = "integer";
pub const NODE_KIND_IRI_REFERENCE: &str = "iri_reference";

pub const NODE_KIND_LANGUAGE_TAG: &str = "language_tag";
pub const NODE_KIND_LESS_THAN: &str = "less_than";
pub const NODE_KIND_LESS_THAN_OR_EQUAL: &str = "less_than_or_equal";
pub const NODE_KIND_LINE_COMMENT: &str = "line_comment";

pub const NODE_KIND_MAPPING_TYPE: &str = "mapping_type";
pub const NODE_KIND_MAPPING_VALUE: &str = "mapping_value";
pub const NODE_KIND_MAPPING_VARIABLE: &str = "mapping_variable";
pub const NODE_KIND_MEMBER_BY_REFERENCE: &str = "member_by_reference";
pub const NODE_KIND_MEMBER_BY_VALUE: &str = "member_by_value";
pub const NODE_KIND_MEMBER_IMPORT: &str = "member_import";
pub const NODE_KIND_MEMBER_INVERSE_NAME: &str = "member_inverse_name";
pub const NODE_KIND_MODULE: &str = "module";
pub const NODE_KIND_MODULE_BODY: &str = "module_body";
pub const NODE_KIND_MODULE_IMPORT: &str = "module_import";

pub const NODE_KIND_NAMED_VARIABLE_SET: &str = "named_variable_set";
pub const NODE_KIND_NEGATION: &str = "negation";
pub const NODE_KIND_NOT_EQUAL: &str = "not_equal";

pub const NODE_KIND_PREDICATE_VALUE: &str = "predicate_value";
pub const NODE_KIND_PROPERTY_BODY: &str = "property_body";
pub const NODE_KIND_PROPERTY_DEF: &str = "property_def";
pub const NODE_KIND_PROPERTY_MEMBER: &str = "property_member";

pub const NODE_KIND_QUALIFIED_IDENTIFIER: &str = "qualified_identifier";
pub const NODE_KIND_QUANTIFIED_SENTENCE: &str = "quantified_sentence";
pub const NODE_KIND_QUANTIFIED_VARIABLE_BINDING: &str = "quantified_variable_binding";
pub const NODE_KIND_QUANTIFIER_BOUND_NAMES: &str = "quantifier_bound_names";
pub const NODE_KIND_QUOTED_STRING: &str = "quoted_string";

pub const NODE_KIND_RESERVED_SELF: &str = "reserved_self";
pub const NODE_KIND_RESERVED_SELF_TYPE: &str = "reserved_self_type";
pub const NODE_KIND_ROLE_BY_REFERENCE: &str = "role_by_reference";
pub const NODE_KIND_ROLE_BY_VALUE: &str = "role_by_value";

pub const NODE_KIND_SEQUENCE_BUILDER: &str = "sequence_builder";
pub const NODE_KIND_SEQUENCE_ITERATOR: &str = "sequence_iterator";
pub const NODE_KIND_SEQUENCE_OF_PREDICATE_VALUES: &str = "sequence_of_predicate_values";
pub const NODE_KIND_SEQUENCE_OF_VALUES: &str = "sequence_of_values";
pub const NODE_KIND_SIMPLE_SENTENCE: &str = "simple_sentence";
pub const NODE_KIND_SIMPLE_VALUE: &str = "simple_value";
pub const NODE_KIND_STRING: &str = "string";
pub const NODE_KIND_STRUCTURE_BODY: &str = "structure_body";
pub const NODE_KIND_STRUCTURE_DEF: &str = "structure_def";
pub const NODE_KIND_STRUCTURE_GROUP: &str = "structure_group";
pub const NODE_KIND_STRUCTURE_MEMBER: &str = "structure_member";

pub const NODE_KIND_TERM: &str = "term";
pub const NODE_KIND_TYPE_ITERATOR: &str = "type_iterator";
pub const NODE_KIND_TYPE_VARIANT: &str = "type_variant";

pub const NODE_KIND_UNARY_BOOLEAN_SENTENCE: &str = "unary_boolean_sentence";
pub const NODE_KIND_UNION_BODY: &str = "union_body";
pub const NODE_KIND_UNION_DEF: &str = "union_def";
pub const NODE_KIND_UNKNOWN_TYPE: &str = "unknown_type";
pub const NODE_KIND_UNIVERSAL: &str = "universal";
pub const NODE_KIND_UNSIGNED: &str = "unsigned";

pub const NODE_KIND_VALUE_CONSTRUCTOR: &str = "value_constructor";
pub const NODE_KIND_VALUE_VARIANT: &str = "value_variant";

pub const NODE_KIND_WILDCARD: &str = "wildcard";

// ------------------------------------------------------------------------------------------------
// Grammar Node field names
// ------------------------------------------------------------------------------------------------

pub const FIELD_NAME_ARGUMENT: &str = "argument";

pub const FIELD_NAME_BASE: &str = "base";
pub const FIELD_NAME_BINARY: &str = "binary";
pub const FIELD_NAME_BINDING: &str = "binding";
pub const FIELD_NAME_BODY: &str = "body";

pub const FIELD_NAME_DOMAIN: &str = "domain";

pub const FIELD_NAME_ELEMENT: &str = "element";

pub const FIELD_NAME_FUNCTION: &str = "function";

pub const FIELD_NAME_IDENTITY: &str = "identity";
pub const FIELD_NAME_INVERSE_NAME: &str = "inverse_name";

pub const FIELD_NAME_LANGUAGE: &str = "language";
pub const FIELD_NAME_LHS: &str = "lhs";

pub const FIELD_NAME_MEMBER: &str = "member";
pub const FIELD_NAME_MAX: &str = "max";
pub const FIELD_NAME_MIN: &str = "min";
pub const FIELD_NAME_MODULE: &str = NODE_KIND_MODULE;

pub const FIELD_NAME_NAME: &str = "name";

pub const FIELD_NAME_OPERATOR: &str = "operator";
pub const FIELD_NAME_ORDERING: &str = "ordering";

pub const FIELD_NAME_PARAMETER: &str = "parameter";
pub const FIELD_NAME_PREDICATE: &str = "predicate";
pub const FIELD_NAME_PROPERTY: &str = "property";

pub const FIELD_NAME_QUANTIFIER: &str = "quantifier";

pub const FIELD_NAME_RANGE: &str = "range";
pub const FIELD_NAME_RELATION: &str = "relation";
pub const FIELD_NAME_RENAME: &str = "rename";
pub const FIELD_NAME_RHS: &str = "rhs";

pub const FIELD_NAME_SIGNATURE: &str = "signature";
pub const FIELD_NAME_SOURCE: &str = "source";
pub const FIELD_NAME_SUBJECT: &str = "subject";

pub const FIELD_NAME_TARGET: &str = "target";
pub const FIELD_NAME_CARDINALITY: &str = "cardinality";

pub const FIELD_NAME_UNARY: &str = "unary";
pub const FIELD_NAME_UNIQUENESS: &str = "uniqueness";

pub const FIELD_NAME_VALUE: &str = "value";
pub const FIELD_NAME_VARIABLE: &str = "variable";

// ------------------------------------------------------------------------------------------------
// Simple Type Keywords
// ------------------------------------------------------------------------------------------------

pub const KW_ORDERING_ORDERED: &str = "ordered";
pub const KW_ORDERING_UNORDERED: &str = "unordered";

pub const KW_TYPE_UNKNOWN: &str = "unknown";

pub const KW_UNIQUENESS_UNIQUE: &str = "unique";
pub const KW_UNIQUENESS_NONUNIQUE: &str = "nonunique";

pub const KW_TYPE_EXISTS: &str = "exists";
pub const KW_TYPE_EXISTS_SYMBOL: &str = "∃";

pub const KW_TYPE_FORALL: &str = "forall";
pub const KW_TYPE_FORALL_SYMBOL: &str = "∀";

pub const KW_RELATION_NOT_EQUAL: &str = "/=";
pub const KW_RELATION_NOT_EQUAL_SYMBOL: &str = "≠";
pub const KW_RELATION_LESS_THAN: &str = "<";
pub const KW_RELATION_GREATER_THAN: &str = ">";
pub const KW_RELATION_LESS_THAN_OR_EQUAL: &str = "<=";
pub const KW_RELATION_LESS_THAN_OR_EQUAL_SYMBOL: &str = "≤";
pub const KW_RELATION_GREATER_THAN_OR_EQUAL: &str = ">=";
pub const KW_RELATION_GREATER_THAN_OR_EQUAL_SYMBOL: &str = "≥";

pub const KW_TYPE_HAS_TYPE: &str = "->";
pub const KW_TYPE_HAS_TYPE_SYMBOL: &str = "→";
pub const KW_TYPE_HAS_TYPE_SYMBOL_ALT: &str = "↦";

pub const KW_TYPE_TYPE_RESTRICTION: &str = "<-";
pub const KW_TYPE_TYPE_RESTRICTION_SYMBOL: &str = "←";

pub const KW_WILDCARD: &str = "_";
