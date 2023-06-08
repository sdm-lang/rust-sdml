use sdml::model::parse::parse_file;
use std::fs::read_to_string;

macro_rules! test_example {
    ($fnname: ident, $exname: literal) => {
        #[test]
        fn $fnname() {
            let input = format!("tests/examples/{}.sdm", $exname);

            let module = parse_file(input);
            if let Err(e) = module {
                panic!("parse error: {}", e);
            }
            let module = module.unwrap();

            let module_as_string = format!("{:#?}\n", module);

            let expected = format!("tests/examples/{}.ron", $exname);
            let expected_string = read_to_string(expected);
            if let Err(e) = expected_string {
                panic!("io error: {}", e);
            }

            pretty_assertions::assert_eq!(module_as_string, expected_string.unwrap());
        }
    };
}

test_example!(test_module_annotations, "module_annotations");
test_example!(test_module_empty, "module_empty");
test_example!(
    test_module_empty_with_comments,
    "module_empty_with_comments"
);
test_example!(test_module_with_underscore, "module_with_underscore");

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
test_example!(test_annotation_single_string, "annotation_single_string");

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

test_example!(test_import_member_only, "import_member_only");
test_example!(test_import_module_only, "import_module_only");
test_example!(test_import_multiple_mixed, "import_multiple_mixed");
test_example!(test_import_multiple_modules, "import_multiple_modules");

test_example!(test_datatype_empty, "datatype_empty");

test_example!(test_entity_empty, "entity_empty");
test_example!(test_entity_with_diff_members, "entity_with_diff_members");
test_example!(test_entity_with_groups, "entity_with_groups");
test_example!(test_entity_with_members, "entity_with_members");
test_example!(test_entity_with_unknowns, "entity_with_unknowns");

test_example!(test_enum_empty, "enum_empty");
test_example!(test_enum_variants, "enum_variants");

test_example!(test_event_empty, "event_empty");

test_example!(test_structure_empty, "structure_empty");

test_example!(test_union_empty, "union_empty");
test_example!(test_union_rename_variant, "union_rename_variant");
test_example!(test_union_variants, "union_variants");
