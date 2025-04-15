// TODO: Make tests for Windows

pub const CORPUS_MANIFEST_PATH: &str = env!("CARGO_MANIFEST_DIR");
pub const CORPUS_PATH: &str = "corpus";

#[macro_export]
macro_rules! writer_to_string {
    ($closure:expr) => {{
        let mut buffer = ::std::io::Cursor::new(Vec::new());
        $closure(&mut buffer).unwrap();
        String::from_utf8(buffer.into_inner()).unwrap()
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! write_file_constants {
    ($result_type: literal, $generator_fn: expr, $replace_manifest_path: expr, $sort_results: expr) => {
        const RESULT_DIR: &str = $result_type;
        const RESULT_FILE_EXT: &str = $result_type;
        const GENERATOR_FN: fn(
            &::sdml_core::model::modules::Module,
            &::sdml_core::store::InMemoryModuleCache,
        ) -> String = $generator_fn;
        const REPLACE_MANIFEST_PATH: bool = $replace_manifest_path;
        const SORT_RESULT_LINES: bool = $sort_results;
    };
}

#[macro_export]
macro_rules! sort_string_as_lines {
    ($s: expr) => {{
        let mut new_s = $s.split("\n").collect::<Vec<&str>>();
        new_s.sort_unstable();
        new_s.join("\n")
    }};
}

#[macro_export]
macro_rules! test_case {
    ($test_name: ident) => {
        ::paste::paste! {
            #[test]
            #[cfg_attr(windows, ignore)]
            fn [< case_ $test_name:lower >]() {
                use sdml_core::model::HasName;
                use sdml_core::store::ModuleStore;
                let test_name = stringify!($test_name);
                let input = ::std::path::PathBuf::from(
                    format!(
                        "{}/{}/{}.sdm",
                        $crate::CORPUS_MANIFEST_PATH,
                        $crate::CORPUS_PATH,
                        test_name
                    ));

                println!("Reading test example from {:?}", input);
                let mut cache = ::sdml_core::store::InMemoryModuleCache::with_stdlib();
                let mut loader = ::sdml_parse::load::FsModuleLoader::default();
                let module = loader.load_from_file(input, &mut cache, false);
                if let Err(e) = module {
                    panic!("Load/Parse error: {}", e);
                }
                let module = cache.get(&module.unwrap()).unwrap();
                println!("Module {} loaded.", module.name());

                let expected = ::std::path::PathBuf::from(
                    format!(
                        "{}/{}/{}/{}.{}",
                        $crate::CORPUS_MANIFEST_PATH,
                        $crate::CORPUS_PATH,
                        RESULT_DIR,
                        test_name,
                        RESULT_FILE_EXT
                    ));
                let result_string: String = GENERATOR_FN(module, &cache);
                if let Ok(true) = ::std::fs::exists(&expected) {

                    println!("Comparing to result in file {:?}", expected);
                    let expected_string = ::std::fs::read_to_string(expected);
                    if let Err(e) = expected_string {
                        panic!("IO error reading expected: {}", e);
                    }

                    let expected_string = if REPLACE_MANIFEST_PATH {
                        expected_string
                            .unwrap()
                            .replace("MANIFEST_PATH", $crate::CORPUS_MANIFEST_PATH)
                    } else {
                        expected_string.unwrap()
                    };

                    if SORT_RESULT_LINES {

                        pretty_assertions::assert_eq!(
                            $crate::sort_string_as_lines!(&result_string),
                            $crate::sort_string_as_lines!(&expected_string),
                        );
                    } else {
                        pretty_assertions::assert_eq!(result_string, expected_string);
                    }
                } else {
                    println!("Skipping test `{test_name}`, no result file in directory {RESULT_DIR}:?");
                    println!("Generated result:\n{result_string}");
                }
            }
        }
    };
}

#[macro_export]
macro_rules! test_suite {
    ($suite_name: ident => ( $($test_name: ident),+ ) ) => {
        ::paste::paste! {
            #[cfg(test)]
            mod  [< $suite_name:lower _suite >] {
                use super::*;

                $(
                    $crate::test_case!($test_name);
                )+
            }
        }
    };
    ($($test_name: ident),+) => {
        $(
            $crate::test_case!($test_name);
        )+
    };
}

#[macro_export]
macro_rules! test_setup {
    (all $result_type: literal => $generator_fn: expr) => {
        $crate::test_setup!(all $result_type => $generator_fn ; true, false);
    };
    (all $result_type: literal => $generator_fn: expr ; $replace_manifest_path: expr, $sort_results: expr) => {
        $crate::write_file_constants!(
            $result_type,
            $generator_fn,
            $replace_manifest_path,
            $sort_results
        );

        $crate::test_suite! {
            module => (
                module_annotations,
                module_empty,
                module_empty_with_base,
                module_empty_with_comments,
                module_empty_with_version,
                module_with_underscore
            )
        }

        $crate::test_suite! {
            import => (
                import_member_only,
                import_member_rename,
                import_member_from,
                import_module_only,
                import_module_rename,
                import_module_version,
                import_module_from,
                import_multiple_members,
                import_multiple_mixed,
                import_multiple_modules,
                import_multiple_module_version,
                import_multiple_from
            )
        }

        $crate::test_suite! {
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
                annotation_multiple_string,
                // test sequence constraints:
                annotation_multiple_ordered,
                annotation_multiple_unique,
                annotation_multiple_as_list,
                annotation_multiple_as_set,
                // test the various escaping forms:
                annotation_escaped_strings
            )
        }

        $crate::test_suite! {
            formal_constraint => (
                // 1. Constraint-sentence:
                // 1.1.  Simple-sentence:
                // 1.1.1. Atomic-sentence:
                constraint_formal_atomic_sentence,
                constraint_formal_function_composition,
                // 1.1.2. Equation:
                constraint_formal_equation,
                // 1.1.3. Inequation:
                constraint_formal_greater_than,
                constraint_formal_greater_than_or_equal,
                constraint_formal_inequation,
                constraint_formal_less_than,
                constraint_formal_less_than_or_equal,
                // 1.2. Boolean-sentence:
                // 1.2.1. Unary-boolean-sentence:
                constraint_formal_unary_negation,
                // 1.2.2. Binary-boolean-sentence:
                constraint_formal_binary_conjunction,
                constraint_formal_binary_disjunction,
                constraint_formal_binary_exclusive_disjunction,
                constraint_formal_binary_biconditional,
                constraint_formal_binary_implication,
                // 1.3. Quantified-sentence:
                // Optional Environment:
                constraint_formal_with_environment
            )
        }

        $crate::test_suite! {
            informal_constraint => (
                constraint_informal,
                constraint_informal_language,
                constraint_informal_language_controlled
            )
        }

        $crate::test_suite! {
            member_cardinality => (
                cardinality_exactly_one,
                cardinality_one_to_many,
                cardinality_two_to_eight,
                cardinality_zero_to_many,
                cardinality_zero_to_many_as_list,
                cardinality_zero_to_many_as_set,
                cardinality_zero_to_many_nonunique,
                cardinality_zero_to_many_ordered,
                cardinality_zero_to_many_unique,
                cardinality_zero_to_many_unordered,
                cardinality_zero_to_one
            )
        }

        $crate::test_suite! {
            datatype => (
                datatype_empty,
                datatype_from_string,
                datatype_with_restrictions,
                datatype_with_fixed_restriction,
                datatype_with_pattern_restriction
            )
        }

        $crate::test_suite! {
            entity => (
                entity_empty,
                entity_with_constraints,
                entity_with_diff_members,
                entity_with_members,
                entity_with_unknowns
            )
        }

        $crate::test_suite! {
            r#enum => (
                enum_empty,
                enum_variants,
                enum_pattern_numbered_variants
            )
        }

        $crate::test_suite! {
            dimension => (
                dimension_empty,
                dimension_with_identity,
                dimension_with_identity_and_members,
                dimension_with_identity_and_parents,
                dimension_with_source,
                dimension_with_source_and_members,
                dimension_with_source_and_parents
            )
        }

        $crate::test_suite! {
            event => (
                event_empty,
                event_with_members,
                event_with_source,
                event_with_source_and_members
            )
        }

        $crate::test_suite! {
            property => (
                property_def_empty,
                property_def_some,
                property_def_used
            )
        }

        $crate::test_suite! {
            rdf => (
                rdf_definitions
            )
        }

        $crate::test_suite! {
            structure => (
                structure_empty,
                structure_mapping_type,
                structure_simple_types
            )
        }

        $crate::test_suite! {
            type_class => (
                type_class_empty,
                type_class_methods,
                type_class_subtype
            )
        }

        $crate::test_suite! {
            union => (
                union_empty,
                union_rename_variant,
                union_variants
            )
        }
    };
}
