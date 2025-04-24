/*!
Contains only string constants for the tree-sitter node types and field names.
 */

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

macro_rules! grammar {
    (node $name:ident is $value:expr) => {
        ::paste::paste! {
            #[doc = "Constant for the tree-sitter node name `" $value "`."]
            pub const [< NODE_KIND_ $name >]: &str = stringify!($value) ;
        }
    };
    (field $name:ident is $value:expr) => {
        ::paste::paste! {
            #[doc = "Constant for the tree-sitter field name `" $value "`."]
            pub const [< FIELD_NAME_ $name >]: &str = stringify!($value) ;
        }
    };
    (field $name:ident as $value:expr) => {
        ::paste::paste! {
            #[doc = "Constant for the tree-sitter field name `" $name:lower "` [" $value "]."]
            pub const [< FIELD_NAME_ $name >]: &str = $value ;
        }
    };
    (kw $name:ident is $value:expr) => {
        ::paste::paste! {
            #[doc = "Constant for the SDML keyword `" $value "`."]
            pub const [< KW_ $name >]: &str = stringify!($value) ;
        }
    };
    (kw $name:ident as $value:expr) => {
        ::paste::paste! {
            #[doc = "Constant for the SDML keyword `" $value "`."]
            pub const [< KW_ $name >]: &str = $value ;
        }
    };
    (op $name:ident is $value:expr) => {
        ::paste::paste! {
            #[doc = "Constant for the SDML operator/relation `" $value "`."]
            pub const [< OP_ $name >]: &str = stringify!($value) ;
        }
    };
    (op $name:ident as $value:expr) => {
        ::paste::paste! {
            #[doc = "Constant for the SDML operator/relation `" $value "`."]
            pub const [< OP_ $name >]: &str = $value ;
        }
    };
    (value $name:ident as $value:expr) => {
        ::paste::paste! {
            pub const [< VALUE_ $name >]: &str = $value ;
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Public Values ❱ Grammar Node kinds
// ------------------------------------------------------------------------------------------------

grammar!(node ANNOTATION is annotation);
grammar!(node ANNOTATION_ONLY_BODY is annotation_only_body);
grammar!(node ANNOTATION_PROPERTY is annotation_property);
grammar!(node ATOMIC_SENTENCE is atomic_sentence);
grammar!(node BINARY is binary);
grammar!(node BINARY_BOOLEAN_SENTENCE is binary_boolean_sentence);
grammar!(node BOOLEAN is boolean);
grammar!(node BOOLEAN_SENTENCE is boolean_sentence);
grammar!(node BUILTIN_SIMPLE_TYPE is builtin_simple_type);
grammar!(node CARDINALITY_EXPRESSION is cardinality_expression);
grammar!(node CONSTRAINT is constraint);
grammar!(node CONSTRAINT_ENVIRONMENT is constraint_environment);
grammar!(node CONSTRAINT_SENTENCE is constraint_sentence);
grammar!(node CONTROLLED_LANGUAGE_TAG is controlled_language_tag);
grammar!(node DATATYPE_DEF_RESTRICTION is datatype_def_restriction);
grammar!(node DATA_TYPE_DEF is data_type_def);
grammar!(node DECIMAL is decimal);
grammar!(node DEFINITION is definition);
grammar!(node FROM_DEFINITION_CLAUSE is from_definition_clause);
grammar!(node DIGIT_RESTRICTION_FACET is digit_restriction_facet);
grammar!(node DIMENSION_BODY is dimension_body);
grammar!(node DIMENSION_DEF is dimension_def);
grammar!(node DIMENSION_PARENT is dimension_parent);
grammar!(node DOUBLE is double);
grammar!(node ENTITY_BODY is entity_body);
grammar!(node ENTITY_DEF is entity_def);
grammar!(node ENTITY_IDENTITY is entity_identity);
grammar!(node ENUM_BODY is enum_body);
grammar!(node ENUM_DEF is enum_def);
grammar!(node EQUATION is equation);
grammar!(node EVENT_BODY is event_body);
grammar!(node EVENT_DEF is event_def);
grammar!(node FORMAL_CONSTRAINT is formal_constraint);
grammar!(node FROM_CLAUSE is from_clause);
grammar!(node FUNCTIONAL_TERM is functional_term);
grammar!(node FUNCTION_BODY is function_body);
grammar!(node FUNCTION_CARDINALITY_EXPRESSION is function_cardinality_expression);
grammar!(node FUNCTION_COMPOSITION is function_composition);
grammar!(node FUNCTION_DEF is function_def);
grammar!(node FUNCTION_PARAMETER is function_parameter);
grammar!(node FUNCTION_SIGNATURE is function_signature);
grammar!(node FUNCTION_TYPE_REFERENCE is function_type_reference);
grammar!(node IDENTIFIER is identifier);
grammar!(node IDENTIFIER_REFERENCE is identifier_reference);
grammar!(node IMPORT is import);
grammar!(node IMPORT_STATEMENT is import_statement);
grammar!(node INEQUATION is inequation);
grammar!(node INFORMAL_CONSTRAINT is informal_constraint);
grammar!(node INTEGER is integer);
grammar!(node IRI is iri);
grammar!(node LANGUAGE_TAG is language_tag);
grammar!(node LENGTH_RESTRICTION_FACET is length_restriction_facet);
grammar!(node LINE_COMMENT is line_comment);
grammar!(node LOGICAL_OP_BICONDITIONAL is logical_op_biconditional);
grammar!(node LOGICAL_OP_CONJUNCTION is logical_op_conjunction);
grammar!(node LOGICAL_OP_DISJUNCTION is logical_op_disjunction);
grammar!(node LOGICAL_OP_EXCLUSIVE_DISJUNCTION is logical_op_exclusive_disjunction);
grammar!(node LOGICAL_OP_IMPLICATION is logical_op_implication);
grammar!(node LOGICAL_OP_NEGATION is logical_op_negation);
grammar!(node LOGICAL_QUANTIFIER_EXISTENTIAL is logical_quantifier_existential);
grammar!(node LOGICAL_QUANTIFIER_UNIVERSAL is logical_quantifier_universal);
grammar!(node MAPPING_TYPE is mapping_type);
grammar!(node MAPPING_VALUE is mapping_value);
grammar!(node MEMBER is member);
grammar!(node MEMBER_DEF is member_def);
grammar!(node MEMBER_IMPORT is member_import);
grammar!(node METHOD_DEF is method_def);
grammar!(node MODULE is module);
grammar!(node MODULE_BODY is module_body);
grammar!(node MODULE_IMPORT is module_import);
grammar!(node MODULE_PATH_ABSOLUTE is module_path_absolute);
grammar!(node MODULE_PATH_RELATIVE is module_path_relative);
grammar!(node MODULE_PATH_ROOT is module_path_root_only);
grammar!(node OPAQUE is opaque);
grammar!(node OP_EQUALITY is op_equality);
grammar!(node OP_GREATER_THAN is op_greater_than);
grammar!(node OP_GREATER_THAN_OR_EQUAL is op_greater_than_or_equal);
grammar!(node OP_INEQUALITY is op_inequality);
grammar!(node OP_LESS_THAN is op_less_than);
grammar!(node OP_LESS_THAN_OR_EQUAL is op_less_than_or_equal);
grammar!(node PATTERN_RESTRICTION_FACET is pattern_restriction_facet);
grammar!(node PREDICATE_SEQUENCE_MEMBER is predicate_sequence_member);
grammar!(node PREDICATE_VALUE is predicate_value);
grammar!(node PROPERTY_DEF is property_def);
grammar!(node PROPERTY_REF is property_ref);
grammar!(node QUALIFIED_IDENTIFIER is qualified_identifier);
grammar!(node QUANTIFIED_SENTENCE is quantified_sentence);
grammar!(node QUANTIFIED_VARIABLE is quantified_variable);
grammar!(node QUANTIFIED_VARIABLE_BINDING is quantified_variable_binding);
grammar!(node QUANTIFIER is quantifier);
grammar!(node QUOTED_STRING is quoted_string);
grammar!(node RDF_DEF is rdf_def);
grammar!(node RDF_TYPES is rdf_types);
grammar!(node RESERVED_SELF is reserved_self);
grammar!(node SEQUENCE_BUILDER is sequence_builder);
grammar!(node SEQUENCE_BUILDER_BODY is sequence_builder_body);
grammar!(node SEQUENCE_OF_PREDICATE_VALUES is sequence_of_predicate_values);
grammar!(node SEQUENCE_OF_VALUES is sequence_of_values);
grammar!(node SEQUENCE_ORDERING is sequence_ordering);
grammar!(node SEQUENCE_UNIQUENESS is sequence_uniqueness);
grammar!(node SET_OP_BUILDER is set_op_builder);
grammar!(node SET_OP_COMPLEMENT is set_op_complement);
grammar!(node SET_OP_INTERSECTION is set_op_intersection);
grammar!(node SET_OP_MEMBERSHIP is set_op_membership);
grammar!(node SET_OP_SUBSET is set_op_subset);
grammar!(node SET_OP_SUBSET_OR_EQUAL is set_op_subset);
grammar!(node SET_OP_UNION is set_op_union);
grammar!(node SIMPLE_SENTENCE is simple_sentence);
grammar!(node SIMPLE_VALUE is simple_value);
grammar!(node SOURCE_ENTITY is source_entity);
grammar!(node SPAN is span);
grammar!(node STRING is string);
grammar!(node STRUCTURE_BODY is structure_body);
grammar!(node STRUCTURE_DEF is structure_def);
grammar!(node SUBJECT is subject);
grammar!(node TERM is term);
grammar!(node TYPE_CLASS_ARGUMENT is type_class_argument);
grammar!(node TYPE_CLASS_BODY is type_class_body);
grammar!(node TYPE_CLASS_DEF is type_class_def);
grammar!(node TYPE_CLASS_REFERENCE is type_class_reference);
grammar!(node TYPE_OP_COMBINER is type_op_combiner);
grammar!(node TYPE_REFERENCE is type_reference);
grammar!(node TYPE_VARIABLE is type_variable);
grammar!(node TYPE_VARIANT is type_variant);
grammar!(node TZ_RESTRICTION_FACET is tz_restriction_facet);
grammar!(node UNARY_BOOLEAN_SENTENCE is unary_boolean_sentence);
grammar!(node UNION_BODY is union_body);
grammar!(node UNION_DEF is union_def);
grammar!(node UNKNOWN_TYPE is unknown_type);
grammar!(node UNSIGNED is unsigned);
grammar!(node VALUE is value);
grammar!(node VALUE_CONSTRUCTOR is value_constructor);
grammar!(node VALUE_RESTRICTION_FACET is value_restriction_facet);
grammar!(node VALUE_VARIANT is value_variant);
grammar!(node VARIABLE is variable);
grammar!(node WILDCARD is wildcard);

// ------------------------------------------------------------------------------------------------
// Public Values ❱ Grammar Node field names
// ------------------------------------------------------------------------------------------------

grammar!(field ARGUMENT is argument);
grammar!(field ARGUMENTS is arguments);
grammar!(field BASE is base);
grammar!(field BINDING is binding);
grammar!(field BODY is body);
grammar!(field BYTE is byte);
grammar!(field CARDINALITY is cardinality);
grammar!(field DOMAIN is domain);
grammar!(field ELEMENT is element);
grammar!(field ENTITY is entity);
grammar!(field ENVIRONMENT is environment);
grammar!(field FACET is facet);
grammar!(field FROM is from);
grammar!(field FUNCTION is function);
grammar!(field IDENTITY is identity);
grammar!(field IS_FIXED is is_fixed);
grammar!(field LANGUAGE is language);
grammar!(field LHS is lhs);
grammar!(field MAX is max);
grammar!(field MEMBER as NODE_KIND_MEMBER);
grammar!(field METHOD is method);
grammar!(field MIN is min);
grammar!(field MODULE as NODE_KIND_MODULE);
grammar!(field NAME is name);
grammar!(field OPERATOR is operator);
grammar!(field ORDERING is ordering);
grammar!(field PARAMETER is parameter);
grammar!(field PARENT is parent);
grammar!(field PREDICATE is predicate);
grammar!(field PROPERTY is property);
grammar!(field QUANTIFIER is quantifier);
grammar!(field RANGE is range);
grammar!(field RELATION is relation);
grammar!(field RENAME is rename);
grammar!(field RESTRICTION is restriction);
grammar!(field RHS is rhs);
grammar!(field SEGMENT is segment);
grammar!(field SIGNATURE is signature);
grammar!(field SOURCE is source);
grammar!(field SUBJECT is subject);
grammar!(field TARGET is target);
grammar!(field TYPE as "type");
grammar!(field TYPES is types);
grammar!(field UNIQUENESS is uniqueness);
grammar!(field VALUE is value);
grammar!(field VARIABLE is variable);
grammar!(field VERSION_INFO is version_info);
grammar!(field VERSION_URI is version_uri);
grammar!(field WILDCARD is wildcard);

// ------------------------------------------------------------------------------------------------
// Public Values ❱ Grammar Keywords
// ------------------------------------------------------------------------------------------------

grammar!(kw A is a);
grammar!(kw ASSERT is assert);
grammar!(kw BLOCK_END is end);
grammar!(kw BLOCK_IS is is);
grammar!(kw BLOCK_OF is of);
grammar!(kw CLASS is class);
grammar!(kw DATATYPE is datatype);
grammar!(kw DATATYPE_FIXED is fixed);
grammar!(kw DATATYPE_OPAQUE as NODE_KIND_OPAQUE);
grammar!(kw DIMENSION is dimension);
grammar!(kw DIMENSION_PARENT as FIELD_NAME_PARENT);
grammar!(kw ENTITY is entity);
grammar!(kw ENTITY_IDENTITY as FIELD_NAME_IDENTITY);
grammar!(kw ENUM as "enum");
grammar!(kw EVENT is event);
grammar!(kw FN_DEF is def);
grammar!(kw FROM is from);
grammar!(kw IMPORT as NODE_KIND_IMPORT);
grammar!(kw IMPORT_FROM is from);
grammar!(kw MODULE as NODE_KIND_MODULE);
grammar!(kw MODULE_VERSION is version);
grammar!(kw ORDERING_ORDERED is ordered);
grammar!(kw ORDERING_UNORDERED is unordered);
grammar!(kw PROPERTY as FIELD_NAME_PROPERTY);
grammar!(kw RDF is rdf);
grammar!(kw REF as "ref");
grammar!(kw RENAME_AS as "as");
grammar!(kw SELF is self);
grammar!(kw SOURCE as FIELD_NAME_SOURCE);
grammar!(kw STRUCTURE is structure);
grammar!(kw TYPE as "type");
grammar!(kw TYPE_UNKNOWN is unknown);
grammar!(kw UNION is union);
grammar!(kw UNIQUENESS_NONUNIQUE is nonunique);
grammar!(kw UNIQUENESS_UNIQUE is unique);
grammar!(kw WITH is with);

grammar!(kw WILDCARD as "_");

// ------------------------------------------------------------------------------------------------
// Public Values ❱ Grammar Keywords ❱ Values
// ------------------------------------------------------------------------------------------------

grammar!(value BOOLEAN_FALSITY as "false");
grammar!(value BOOLEAN_FALSITY_SYMBOL as "⊥");
grammar!(value BOOLEAN_TRUTH as "true");
grammar!(value BOOLEAN_TRUTH_SYMBOL as "⊤");
grammar!(value EMPTY_SET as "∅");

// ------------------------------------------------------------------------------------------------
// Public Values ❱ Grammar Keywords ❱ Restriction Facets
// ------------------------------------------------------------------------------------------------

grammar!(kw FACET_EXPLICIT_TIMEZONE is explicitTimezone);
grammar!(kw FACET_FRACTION_DIGITS is fractionDigits);
grammar!(kw FACET_LENGTH is length);
grammar!(kw FACET_MAX_EXCLUSIVE is maxExclusive);
grammar!(kw FACET_MAX_INCLUSIVE is maxInclusive);
grammar!(kw FACET_MAX_LENGTH is maxLength);
grammar!(kw FACET_MIN_EXCLUSIVE is minExclusive);
grammar!(kw FACET_MIN_INCLUSIVE is minInclusive);
grammar!(kw FACET_MIN_LENGTH is minLength);
grammar!(kw FACET_PATTERN is pattern);
grammar!(kw FACET_TOTAL_DIGITS is totalDigits);
grammar!(kw FACET_TIMEZONE_OPTIONAL is optional);
grammar!(kw FACET_TIMEZONE_PROHIBITED is prohibited);
grammar!(kw FACET_TIMEZONE_REQUIRED is required);

// ------------------------------------------------------------------------------------------------
// Public Values ❱ Grammar Keywords ❱ Operators & Relations
// ------------------------------------------------------------------------------------------------

grammar!(op ASSIGNMENT as "=");
grammar!(op ASSIGNMENT_BY_DEFINITION as ":=");
grammar!(op ASSIGNMENT_BY_DEFINITION_SYMBOL as "≔");
grammar!(op FN_COMPOSITION as ".");
grammar!(op FN_COMPOSITION_SYMBOL as "·");
grammar!(op LOGICAL_BICONDITIONAL is iff);
grammar!(op LOGICAL_BICONDITIONAL_ALT as "<==>");
grammar!(op LOGICAL_BICONDITIONAL_SYMBOL as "⇔");
grammar!(op LOGICAL_CONJUNCTION is and);
grammar!(op LOGICAL_CONJUNCTION_SYMBOL as "∧");
grammar!(op LOGICAL_DISJUNCTION is or);
grammar!(op LOGICAL_DISJUNCTION_SYMBOL as "∨");
grammar!(op LOGICAL_EXCLUSIVE_DISJUNCTION is xor);
grammar!(op LOGICAL_EXCLUSIVE_DISJUNCTION_SYMBOL as "⊻");
grammar!(op LOGICAL_IMPLICATION is implies);
grammar!(op LOGICAL_IMPLICATION_ALT as "==>");
grammar!(op LOGICAL_IMPLICATION_SYMBOL as "⇒");
grammar!(op LOGICAL_NEGATION is not);
grammar!(op LOGICAL_NEGATION_SYMBOL as "¬");
grammar!(op LOGICAL_QUANTIFIER_EXISTS is exists);
grammar!(op LOGICAL_QUANTIFIER_EXISTS_SYMBOL as "∃");
grammar!(op LOGICAL_QUANTIFIER_FORALL is forall);
grammar!(op LOGICAL_QUANTIFIER_FORALL_SYMBOL as "∀");
grammar!(op RELATION_EQUAL as "=");
grammar!(op RELATION_GREATER_THAN as ">");
grammar!(op RELATION_GREATER_THAN_OR_EQUAL as ">=");
grammar!(op RELATION_GREATER_THAN_OR_EQUAL_SYMBOL as "≥");
grammar!(op RELATION_LESS_THAN as "<");
grammar!(op RELATION_LESS_THAN_OR_EQUAL as "<=");
grammar!(op RELATION_LESS_THAN_OR_EQUAL_SYMBOL as "≤");
grammar!(op RELATION_NOT_EQUAL as "/=");
grammar!(op RELATION_NOT_EQUAL_SYMBOL as "≠");
grammar!(op SET_COMPLEMENT is complement);
grammar!(op SET_COMPLEMENT_SYMBOL as "∖");
grammar!(op SET_INTERSECTION is intersection);
grammar!(op SET_INTERSECTION_SYMBOL as "∩");
grammar!(op SET_MEMBERSHIP as "in");
grammar!(op SET_MEMBERSHIP_SYMBOL as "∈");
grammar!(op SET_SUBSET is subset);
grammar!(op SET_SUBSET_OR_EQUAL is subseteq);
grammar!(op SET_SUBSET_OR_EQUAL_SYMBOL as "⊆");
grammar!(op SET_SUBSET_SYMBOL as "⊂");
grammar!(op SET_UNION as "union");
grammar!(op SET_UNION_SYMBOL as "∪");
grammar!(op TYPE_ASSERTION as "->");
grammar!(op TYPE_ASSERTION_SYMBOL as "→");
grammar!(op TYPE_CARDINALITY_RANGE as "..");
grammar!(op TYPE_RESTRICTION as "<-");
grammar!(op TYPE_RESTRICTION_SYMBOL as "←");
grammar!(op TYPE_COMBINE as "+");
grammar!(op TYPE_COMBINE_SYMBOL as "⊕");

// ------------------------------------------------------------------------------------------------
// Public Values ❱ Grammar Punctuation
// ------------------------------------------------------------------------------------------------

pub const PC_BRACE_LEFT: &str = "{";
pub const PC_BRACE_RIGHT: &str = "}";
pub const PC_BRACKET_LEFT: &str = "[";
pub const PC_BRACKET_RIGHT: &str = "]";
pub const PC_PAREN_LEFT: &str = "(";
pub const PC_PAREN_RIGHT: &str = ")";

pub const PC_BINARY_END: &str = PC_BRACKET_RIGHT;
pub const PC_BINARY_START: &str = "#[";
pub const PC_CARDINALITY_END: &str = PC_BRACE_RIGHT;
pub const PC_CARDINALITY_START: &str = PC_BRACE_LEFT;
pub const PC_CONSTRAINT_EXRESSION_END: &str = PC_BRACKET_RIGHT;
pub const PC_CONSTRAINT_EXRESSION_START: &str = PC_PAREN_LEFT;
pub const PC_FUNCTION_PARARGS_END: &str = PC_PAREN_RIGHT;
pub const PC_FUNCTION_PARARGS_START: &str = PC_PAREN_LEFT;
pub const PC_LINE_COMMENT_START: &str = ";";
pub const PC_MAPPING_TYPE_VALUE_END: &str = PC_PAREN_RIGHT;
pub const PC_MAPPING_TYPE_VALUE_START: &str = PC_PAREN_LEFT;
pub const PC_METHOD_PARARGS_END: &str = PC_PAREN_RIGHT;
pub const PC_METHOD_PARARGS_START: &str = PC_PAREN_LEFT;
pub const PC_QUANTIFIED_SENTENCE_SEPARATOR: &str = ",";
pub const PC_RESTRICTION_END: &str = PC_BRACE_RIGHT;
pub const PC_RESTRICTION_START: &str = PC_BRACE_LEFT;
pub const PC_SEQUENCE_BUILDER_END: &str = PC_BRACE_RIGHT;
pub const PC_SEQUENCE_BUILDER_SEPARATOR: &str = "|";
pub const PC_SEQUENCE_BUILDER_START: &str = PC_BRACE_LEFT;
pub const PC_SEQUENCE_END: &str = PC_BRACKET_RIGHT;
pub const PC_SEQUENCE_START: &str = PC_BRACKET_LEFT;
pub const PC_STRING_START: &str = PC_STRING_END;
pub const PC_TYPE_CLASS_PARARGS_END: &str = PC_PAREN_RIGHT;
pub const PC_TYPE_CLASS_PARARGS_START: &str = PC_PAREN_LEFT;

// ------------------------------------------------------------------------------------------------
// Public Values ❱ Grammar Syntax Elements (non-separable tokens)
// ------------------------------------------------------------------------------------------------

pub const PC_ANNOTATION_PREFIX: &str = "@";
pub const PC_IRI_END: &str = ">";
pub const PC_IRI_START: &str = "<";
pub const PC_LANGUAGE_PREFIX: &str = "@";
pub const PC_MODULE_PATH_SEPARATOR: &str = "::";
pub const PC_QUALIFIED_IDENTIFIER_SEPARATOR: &str = ":";
pub const PC_STRING_END: &str = "\"";

// ------------------------------------------------------------------------------------------------
// Public Values ❱ Reserved Grammar Keywords
// ------------------------------------------------------------------------------------------------

pub const RESERVED_KEYWORDS: [&str; 35] = [
    KW_ASSERT,
    KW_BLOCK_END,
    KW_BLOCK_IS,
    KW_BLOCK_OF,
    KW_CLASS,
    KW_DATATYPE,
    KW_DATATYPE_FIXED,
    KW_DATATYPE_OPAQUE,
    KW_DIMENSION,
    KW_DIMENSION_PARENT,
    KW_ENTITY,
    KW_ENTITY_IDENTITY,
    KW_ENUM,
    KW_EVENT,
    KW_FN_DEF,
    KW_IMPORT,
    KW_IMPORT_FROM,
    KW_MODULE,
    KW_MODULE_VERSION,
    KW_ORDERING_ORDERED,
    KW_ORDERING_UNORDERED,
    KW_PROPERTY,
    KW_RDF,
    KW_REF,
    KW_RENAME_AS,
    KW_SELF,
    KW_SOURCE,
    KW_STRUCTURE,
    KW_TYPE_UNKNOWN,
    KW_UNION,
    KW_UNIQUENESS_NONUNIQUE,
    KW_UNIQUENESS_UNIQUE,
    KW_WITH,
    VALUE_BOOLEAN_FALSITY,
    VALUE_BOOLEAN_TRUTH,
];

// is reserved in a constraint context
pub const RESERVED_CONSTRAINT_KEYWORDS: [&str; 9] = [
    OP_LOGICAL_BICONDITIONAL,
    OP_LOGICAL_CONJUNCTION,
    OP_LOGICAL_DISJUNCTION,
    OP_LOGICAL_EXCLUSIVE_DISJUNCTION,
    OP_LOGICAL_IMPLICATION,
    OP_LOGICAL_NEGATION,
    OP_LOGICAL_QUANTIFIER_EXISTS,
    OP_LOGICAL_QUANTIFIER_FORALL,
    OP_SET_MEMBERSHIP,
];
