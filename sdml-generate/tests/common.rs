pub const MANIFEST_PATH: &str = env!("CARGO_MANIFEST_DIR");
pub const TEST_INPUT_PATH: &str = "tests/examples";

#[macro_export]
macro_rules! test_examples {
    ($suite_name: ident, $result_ext: literal, $transform: expr => ( $($test_name: ident),+ ) ) => {
        paste! {
            #[cfg(test)]
            mod  [< $suite_name:lower _tests >] {
                use super::*;

                $(
                    test_example!($test_name, $result_ext, $transform);
                )+
            }
        }
    };
}

#[macro_export]
macro_rules! generator {
    ($fn_name: ident, $generator: expr, $options: expr, $setup_fn: expr) => {
        fn $fn_name(module: &Module, cache: &InMemoryModuleCache) -> String {
            let _ = $setup_fn();
            let mut generator = $generator;
            let options = $options;
            generator
                .generate_to_string(module, cache, options, None)
                .unwrap()
        }
    };
    ($fn_name: ident, $generator: expr, $options: expr) => {
        fn $fn_name(module: &Module, cache: &InMemoryModuleCache) -> String {
            let mut generator = $generator;
            let options = $options;
            generator
                .generate_to_string(module, cache, options, None)
                .unwrap()
        }
    };
    ($generator: expr, $options: expr, $setup_fn: expr) => {
        generator! {
            generate_to_string, $generator, $options, $setup_fn
        }
    };
    ($generator: expr, $options: expr) => {
        generator! {
            generate_to_string, $generator, $options
        }
    };
}

const UPDATE_EXAMPLES_OUTPUT_ENV: &str = "UPDATE_EXAMPLES_OUTPUT";

pub fn verify_example_output(result_string: &str, expected_path: &std::path::PathBuf) {
    match std::env::var(UPDATE_EXAMPLES_OUTPUT_ENV) {
        Ok(val) if val == "1" => {
            println!("Updating results in file {:?}", expected_path);

            if let Err(e) = ::std::fs::write(expected_path, result_string) {
                panic!("IO error writing expected: {}", e);
            }
        }
        _ => {
            println!("Comparing to result in file {:?}", expected_path);
            let expected_string = ::std::fs::read_to_string(expected_path);
            if let Err(e) = expected_string {
                panic!("IO error reading expected: {}", e);
            }

            pretty_assertions::assert_eq!(result_string, expected_string.unwrap());
        }
    }
}

#[macro_export]
macro_rules! test_example {
    ($test_name: ident, $result_ext: literal, $transform: expr) => {
        paste! {
            #[test]
            #[cfg_attr(windows, ignore)]
            fn [< test_ $test_name:lower _  $result_ext>]() {
                let test_name = stringify!($test_name);
                let input = ::std::path::PathBuf::from(
                    format!(
                        "{}/{}/{}.sdm",
                        $crate::common::MANIFEST_PATH,
                        $crate::common::TEST_INPUT_PATH,
                        test_name
                    ));
                let expected = std::path::PathBuf::from(
                    format!(
                        "{}/{}/{}/{}.{}",
                        $crate::common::MANIFEST_PATH,
                        $crate::common::TEST_INPUT_PATH,
                        $result_ext,
                        test_name,
                        $result_ext
                    ));

                println!("Reading test example from {:?}", input);
                let mut cache = ::sdml_core::store::InMemoryModuleCache::default();
                let mut loader = ::sdml_parse::load::FsModuleLoader::default();
                let module = loader.load_from_file(input, &mut cache, false);
                if let Err(e) = module {
                    panic!("Load/Parse error: {}", e);
                }
                let module = cache.get(&module.unwrap()).unwrap();
                println!("Module {} loaded.", module.name());

                let result_string = $transform(module, &cache);

                $crate::common::verify_example_output(&result_string, &expected);
            }
        }
    };
}
