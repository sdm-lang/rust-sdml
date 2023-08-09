use sdml_core::load::ModuleLoader as LoaderTrait;
use sdml_parse::load::ModuleLoader;
use std::fs::read_to_string;
use std::path::PathBuf;

// TODO: Make tests for Windows

macro_rules! test_example {
    ($fnname: ident, $exname: literal) => {
        #[test]
        #[cfg_attr(windows, ignore)]
        fn $fnname() {
            let input = PathBuf::from(format!("tests/examples/{}.sdm", $exname));

            let mut loader = ModuleLoader::default();
            let module = loader.load_from_file(input);
            if let Err(e) = module {
                panic!("parse error: {}", e);
            }
            println!("1");
            let module = module.unwrap();

            let module_as_string = format!("{:#?}\n", module);

            let expected = format!("tests/examples/{}.ron", $exname);
            let expected_string = read_to_string(expected);
            if let Err(e) = expected_string {
                panic!("io error: {}", e);
            }
            println!("2");

            pretty_assertions::assert_eq!(module_as_string, expected_string.unwrap());
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

test_example!(test_module_annotations, "module_annotations");
test_example!(test_module_empty, "module_empty");
test_example!(
    test_module_empty_with_comments,
    "module_empty_with_comments"
);
test_example!(test_module_with_underscore, "module_with_underscore");

// ------------------------------------------------------------------------------------------------
// Modules ❱ Imports
// ------------------------------------------------------------------------------------------------

test_example!(test_import_member_only, "import_member_only");
test_example!(test_import_module_only, "import_module_only");
test_example!(test_import_multiple_mixed, "import_multiple_mixed");
test_example!(test_import_multiple_modules, "import_multiple_modules");

// ------------------------------------------------------------------------------------------------
// Annotations ❱ Single
// ------------------------------------------------------------------------------------------------

test_example!(test_annotation_single_boolean, "annotation_single_boolean");
test_example!(
    test_annotation_single_constructor,
    "annotation_single_constructor"
);
test_example!(test_annotation_single_decimal, "annotation_single_decimal");
test_example!(test_annotation_single_double, "annotation_single_double");
test_example!(test_annotation_single_integer, "annotation_single_integer");
test_example!(test_annotation_single_iri, "annotation_single_iri");
test_example!(
    test_annotation_single_language_string,
    "annotation_single_language_string"
);
test_example!(test_annotation_single_mapping, "annotation_single_mapping");
test_example!(test_annotation_single_string, "annotation_single_string");

// ------------------------------------------------------------------------------------------------
// Annotations ❱ Multiple
// ------------------------------------------------------------------------------------------------

test_example!(
    test_annotation_multiple_decimal,
    "annotation_multiple_decimal"
);
test_example!(
    test_annotation_multiple_double,
    "annotation_multiple_double"
);
test_example!(
    test_annotation_multiple_integer,
    "annotation_multiple_integer"
);
test_example!(test_annotation_multiple_iri, "annotation_multiple_iri");
test_example!(
    test_annotation_multiple_language_string,
    "annotation_multiple_language_string"
);
test_example!(
    test_annotation_multiple_string,
    "annotation_multiple_string"
);

test_example!(
    test_annotation_multiple_separate,
    "annotation_multiple_separate"
);

// ------------------------------------------------------------------------------------------------
// Types ❱ Datatype
// ------------------------------------------------------------------------------------------------

test_example!(test_datatype_empty, "datatype_empty");
test_example!(test_datatype_from_string, "datatype_from_string");

// ------------------------------------------------------------------------------------------------
// Types ❱ Entities
// ------------------------------------------------------------------------------------------------

test_example!(test_entity_empty, "entity_empty");
test_example!(test_entity_with_diff_members, "entity_with_diff_members");
test_example!(test_entity_with_groups, "entity_with_groups");
test_example!(test_entity_with_members, "entity_with_members");
test_example!(test_entity_with_unknowns, "entity_with_unknowns");

// ------------------------------------------------------------------------------------------------
// Types ❱ Enums
// ------------------------------------------------------------------------------------------------

test_example!(test_enum_empty, "enum_empty");
test_example!(test_enum_variants, "enum_variants");

// ------------------------------------------------------------------------------------------------
// Types ❱ Events
// ------------------------------------------------------------------------------------------------

test_example!(test_event_empty, "event_empty");

// ------------------------------------------------------------------------------------------------
// Types ❱ Structures
// ------------------------------------------------------------------------------------------------

test_example!(test_structure_empty, "structure_empty");
test_example!(test_structure_mapping_type, "structure_mapping_type");

// ------------------------------------------------------------------------------------------------
// Types ❱ Unions
// ------------------------------------------------------------------------------------------------

test_example!(test_union_empty, "union_empty");
test_example!(test_union_rename_variant, "union_rename_variant");
test_example!(test_union_variants, "union_variants");
