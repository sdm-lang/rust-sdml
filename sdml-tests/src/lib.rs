// TODO: Make tests for Windows

pub const CORPUS_MANIFEST_PATH: &str = env!("CARGO_MANIFEST_DIR");
pub const CORPUS_PATH: &str = "corpus";

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
                let mut cache = ::sdml_core::store::InMemoryModuleCache::default().with_stdlib();
                let mut loader = ::sdml_parse::load::FsModuleLoader::default();
                let module = loader.load_from_file(input, &mut cache, false);
                if let Err(e) = module {
                    panic!("Load/Parse error: {}", e);
                }
                let module = cache.get(&module.unwrap()).unwrap();
                println!("Module {} loaded.", module.name());
                //println!("{:?}", module);

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
                    let expected_string = expected_string
                        .unwrap()
                        .replace("MANIFEST_PATH", $crate::CORPUS_MANIFEST_PATH);

                    pretty_assertions::assert_eq!(result_string, expected_string);
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
    ($result_type: literal, standard, $generator_fn: expr) => {
        $crate::test_setup!($result_type, $generator_fn);

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
                import_module_only,
                import_module_version,
                import_multiple_members,
                import_multiple_mixed,
                import_multiple_modules,
                import_multiple_module_version
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
                annotation_multiple_string
            )
        }

        $crate::test_suite! {
            rdf => (
                rdf_definitions
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
            datatype => (
                datatype_empty,
                datatype_from_string,
                datatype_with_restrictions
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
                enum_variants
            )
        }

        $crate::test_suite! {
            dimension => (
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
                event_with_source
            )
        }

        $crate::test_suite! {
            property => (
                property_def_some,
                property_def_used
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
    ($result_type: literal, $generator_fn: expr) => {
        const RESULT_DIR: &str = $result_type;
        const RESULT_FILE_EXT: &str = $result_type;
        const GENERATOR_FN: fn(
            &::sdml_core::model::modules::Module,
            &::sdml_core::store::InMemoryModuleCache,
        ) -> String = $generator_fn;
    };
}
