use sdml_core::load::ModuleLoader as LoaderTrait;
use sdml_parse::load::ModuleLoader;
use std::fs::read_to_string;
use std::path::PathBuf;
use paste::paste;

// TODO: Make tests for Windows

const MANIFEST_PATH: &str = env!("CARGO_MANIFEST_DIR");
const TEST_PATH: &str = "tests/examples";

macro_rules! test_examples {
    ($suite_name: ident => ( $($test_name: ident),+ ) ) => {
        paste! {
            #[cfg(test)]
            mod  [< $suite_name:lower _tests >] {
                use super::*;

                $(
                    test_example!($test_name);
                )+
            }
        }
    };
    ($($test_name: ident),+) => {
        $(
            test_example!($test_name);
        )+
    };
}
macro_rules! test_example {
    ($test_name: ident) => {
        paste! {
            #[test]
            #[cfg_attr(windows, ignore)]
            fn [< test_ $test_name:lower >]() {
                let test_name = stringify!($test_name);
                let input = PathBuf::from(
                    format!(
                        "{}/{}/{}.sdm",
                        MANIFEST_PATH,
                        TEST_PATH,
                        test_name
                    ));
                let expected = PathBuf::from(
                    format!(
                        "{}/{}/{}.ron",
                        MANIFEST_PATH,
                        TEST_PATH,
                        test_name
                    ));

                println!("Reading test example from {:?}", input);
                let loader = ModuleLoader::default();
                let module = loader.load_from_file(input);
                if let Err(e) = module {
                    panic!("parse error: {}", e);
                }
                println!("1");
                let module = module.unwrap();

                let module_as_string = format!("{:#?}\n", module);

                println!("Comparing to result in {:?}", expected);
                let expected_string = read_to_string(expected);
                if let Err(e) = expected_string {
                    panic!("io error: {}", e);
                }
                println!("2");

                pretty_assertions::assert_eq!(module_as_string, expected_string.unwrap());
            }
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

test_examples!{
    module => (
        module_annotations,
        module_empty,
        module_empty_with_base,
        module_empty_with_comments,
        module_with_underscore
    )
}

// ------------------------------------------------------------------------------------------------
// Modules ❱ Imports
// ------------------------------------------------------------------------------------------------

test_examples!{
    import => (
        import_member_only,
        import_module_only,
        import_multiple_mixed,
        import_multiple_modules
    )
}

// ------------------------------------------------------------------------------------------------
// Annotation Properties
// ------------------------------------------------------------------------------------------------

test_examples!{
    annotation_property => (
        annotation_single_binary,
        annotation_single_boolean,
        annotation_single_constructor,
        annotation_single_decimal,
        annotation_single_double,
        annotation_single_integer,
        annotation_single_iri,
        annotation_single_language_string,
        annotation_single_mapping,
        annotation_single_string,
        annotation_multiple_decimal,
        annotation_multiple_double,
        annotation_multiple_integer,
        annotation_multiple_iri,
        annotation_multiple_language_string,
        annotation_multiple_separate,
        annotation_multiple_string
    )
}

// ------------------------------------------------------------------------------------------------
// Annotation Constraints
// ------------------------------------------------------------------------------------------------

test_examples!{
    informal_constraint => (
        constraint_informal,
        constraint_informal_language,
        constraint_informal_language_controlled
    )
}

// ------------------------------------------------------------------------------------------------
// Types ❱ Datatype
// ------------------------------------------------------------------------------------------------

test_examples!{
    datatype => (
        datatype_empty,
        datatype_from_string,
        datatype_with_restrictions
    )
}

// ------------------------------------------------------------------------------------------------
// Types ❱ Entities
// ------------------------------------------------------------------------------------------------

test_examples!{
    entity => (
        entity_empty,
        entity_with_constraints,
        entity_with_diff_members,
        entity_with_groups,
        entity_with_members,
        entity_with_unknowns
    )
}

// ------------------------------------------------------------------------------------------------
// Types ❱ Enums
// ------------------------------------------------------------------------------------------------

test_examples!{
    r#enum => (
        enum_empty,
        enum_variants
    )
}

// ------------------------------------------------------------------------------------------------
// Types ❱ Events
// ------------------------------------------------------------------------------------------------

test_examples!{
    event => (
        event_empty
    )
}

// ------------------------------------------------------------------------------------------------
// Types ❱ Properties
// ------------------------------------------------------------------------------------------------

test_examples!{
    property => (
        property_def_empty,
        property_def_some,
        property_def_used
    )
}

// ------------------------------------------------------------------------------------------------
// Types ❱ Structures
// ------------------------------------------------------------------------------------------------

test_examples!{
    structure => (
        structure_empty,
        structure_mapping_type,
        structure_simple_types,
        structure_with_features
    )
}

// ------------------------------------------------------------------------------------------------
// Types ❱ Type Classes
// ------------------------------------------------------------------------------------------------

test_examples!{
    type_class => (
        type_class_empty,
        type_class_methods,
        type_class_subtype
    )
}

// ------------------------------------------------------------------------------------------------
// Types ❱ Unions
// ------------------------------------------------------------------------------------------------

test_examples!{
    union => (
        union_empty,
        union_rename_variant,
        union_variants
    )
}
