/*!
Contains only string constants for the Rust model names used in serialization.
 */

use super::grammar::*;

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

macro_rules! model {
    (node $name:ident is $value:expr) => {
        ::paste::paste! {
            pub const [< NODE_KIND_ $name >]: &str = stringify!($value) ;
        }
    };
    //(node $name:ident as $value:expr) => {
    //    ::paste::paste! {
    //        pub const [< NODE_KIND_ $name >]: &str = $value ;
    //    }
    //};
    (variant $name:ident is $value:expr) => {
        ::paste::paste! {
            pub const [< NODE_VARIANT_ $name >]: &str = stringify!($value) ;
        }
    };
    (variant $name:ident as $value:expr) => {
        ::paste::paste! {
            pub const [< NODE_VARIANT_ $name >]: &str = $value ;
        }
    };
    (variant $name:ident as_plural $value:expr) => {
        ::paste::paste! {
            pub const [< NODE_VARIANT_ $name >]: &str = ::const_format::concatcp!( $value, "s" ) ;
        }
    };
    (field $name:ident is $value:expr) => {
        ::paste::paste! {
            pub const [< FIELD_NAME_ $name >]: &str = stringify!($value) ;
        }
    };
    (field $name:ident as $value:expr) => {
        ::paste::paste! {
            pub const [< FIELD_NAME_ $name >]: &str = $value ;
        }
    };
    (field $name:ident as_plural $value:expr) => {
        ::paste::paste! {
            pub const [< FIELD_NAME_ $name >]: &str = ::const_format::concatcp!( $value, "s" ) ;
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Public Values ❱ Model Structure Names
// ------------------------------------------------------------------------------------------------

model!(node CONSTRAINT_BODY is constraint_body);
model!(node DIMENSION_IDENTITY is dimension_identity);
model!(node FUNCTION_TYPE is function_type);
model!(node SEQUENCE_MEMBER is sequence_member);
model!(node NAMED_VARIABLE_SET is named_variable_set);
model!(node VARIABLES is variables);

// ------------------------------------------------------------------------------------------------
// Public Values ❱ Model Field Names
// ------------------------------------------------------------------------------------------------

model!(field ANNOTATIONS as_plural NODE_KIND_ANNOTATION);
model!(field END as KW_BLOCK_END);
model!(field FUNCTIONS as_plural FIELD_NAME_FUNCTION);
model!(field IMPORTS as_plural KW_IMPORT);
model!(field IS_LIBRARY_MODULE is is_library_module);
model!(field IS_OPAQUE is is_opaque);
model!(field KIND is kind);
model!(field MEMBERS as_plural NODE_KIND_MEMBER);
model!(field METHODS as_plural FIELD_NAME_METHOD);
model!(field OPERAND is operand);
model!(field NAMES as_plural FIELD_NAME_NAME);
model!(field PARAMETERS as_plural FIELD_NAME_PARAMETER);
model!(field PARENTS as_plural KW_DIMENSION_PARENT);
model!(field SENTENCE is sentence);
model!(field SOURCE_FILE is source_file);
model!(field SPAN as NODE_KIND_SPAN);
model!(field START is start);
model!(field UNARY is unary);
model!(field VARIABLES as_plural FIELD_NAME_VARIABLE);
model!(field VARIANTS is variants);
model!(field WITH is with);

// ------------------------------------------------------------------------------------------------
// Public Values ❱ Model Variant Names
// ------------------------------------------------------------------------------------------------

model!(variant ATOMIC is atomic);
model!(variant BINARY as NODE_KIND_BINARY);
model!(variant BOOLEAN as NODE_KIND_BOOLEAN);
model!(variant COMPOSITION is composition);
model!(variant CONSTRAINT as NODE_KIND_CONSTRAINT);
model!(variant DATATYPE as KW_DATATYPE);
model!(variant DECIMAL as NODE_KIND_DECIMAL);
model!(variant DEFINITION as NODE_KIND_DEFINITION);
model!(variant DIMENSION as KW_DIMENSION);
model!(variant DOUBLE as NODE_KIND_DOUBLE);
model!(variant ENTITY as KW_ENTITY);
model!(variant ENUM as KW_ENUM);
model!(variant EQUATION as NODE_KIND_EQUATION);
model!(variant EVENT as KW_EVENT);
model!(variant FORMAL is formal);
model!(variant FUNCTION as FIELD_NAME_FUNCTION);
model!(variant IDENTIFIER is identifier);
model!(variant INEQUATION as NODE_KIND_INEQUATION);
model!(variant INFORMAL is informal);
model!(variant INTEGER as NODE_KIND_INTEGER);
model!(variant IRI as NODE_KIND_IRI);
model!(variant MAPPING is mapping);
model!(variant MEMBER as NODE_KIND_MEMBER);
model!(variant MODULE as NODE_KIND_MODULE);
model!(variant NAMED is named);
model!(variant PROPERTY as KW_PROPERTY);
model!(variant QUALIFIED_IDENTIFIER as NODE_KIND_QUALIFIED_IDENTIFIER);
model!(variant QUANTIFIED is quantified);
model!(variant RDF as KW_RDF);
model!(variant REFERENCE is reference);
model!(variant RESTRICTIONS as_plural FIELD_NAME_RESTRICTION);
model!(variant SELF as KW_SELF);
model!(variant SENTENCE is MODEL_FIELD_SENTENCE);
model!(variant SEQUENCE is sequence);
model!(variant SIMPLE is simple);
model!(variant STRING as NODE_KIND_STRING);
model!(variant STRUCTURE as KW_STRUCTURE);
model!(variant TERM is term);
model!(variant TYPE as "type");
model!(variant TYPE_CLASS is type_class);
model!(variant UNARY as FIELD_NAME_UNARY);
model!(variant UNION as KW_UNION);
model!(variant UNKNOWN is unknown);
model!(variant UNSIGNED as NODE_KIND_UNSIGNED);
model!(variant VALUE as NODE_KIND_VALUE);
model!(variant VALUE_CONSTRUCTOR is constructor);
model!(variant WILDCARD as NODE_KIND_WILDCARD);
