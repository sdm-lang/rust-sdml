use std::io::Cursor;

use paste::paste;
use sdml_core::{
    cache::{ModuleCache, ModuleStore},
    model::{modules::Module, HasName},
};
use sdml_errors::diagnostics::UseColor;
use sdml_generate::{color::set_colorize, GenerateToWriter};
use sdml_parse::load::FsModuleLoader;

const MANIFEST_PATH: &str = env!("CARGO_MANIFEST_DIR");
const TEST_PATH: &str = "tests/examples";

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

macro_rules! test_example {
    ($test_name: ident, $result_ext: literal, $transform: expr) => {
        paste! {
            #[test]
            #[cfg_attr(windows, ignore)]
            fn [< test_ $test_name:lower >]() {
                let test_name = stringify!($test_name);
                let input = ::std::path::PathBuf::from(
                    format!(
                        "{}/{}/{}.sdm",
                        MANIFEST_PATH,
                        TEST_PATH,
                        test_name
                    ));
                let expected = std::path::PathBuf::from(
                    format!(
                        "{}/{}/{}.{}",
                        MANIFEST_PATH,
                        TEST_PATH,
                        test_name,
                        $result_ext
                    ));

                println!("Reading test example from {:?}", input);
                let mut cache = ::sdml_core::cache::ModuleCache::default();
                let mut loader = ::sdml_parse::load::FsModuleLoader::default();
                let module = loader.load_from_file(input, &mut cache, false);
                if let Err(e) = module {
                    panic!("Load/Parse error: {}", e);
                }
                let module = cache.get(&module.unwrap()).unwrap();
                 println!("Module {} loaded.", module.name());

                let result_string = $transform(module, &cache, &loader);

                println!("Comparing to result in file {:?}", expected);
                let expected_string = ::std::fs::read_to_string(expected);
                if let Err(e) = expected_string {
                    panic!("IO error reading expected: {}", e);
                }

                pretty_assertions::assert_eq!(result_string, expected_string.unwrap());
            }
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Modules ❱ Imports
// ------------------------------------------------------------------------------------------------

fn generate_dependency_tree(module: &Module, cache: &ModuleCache, _: &FsModuleLoader) -> String {
    // turn this off to avoid control characters in the output.
    set_colorize(UseColor::Never);
    let mut buffer = Cursor::new(Vec::new());
    let view = sdml_generate::actions::deps::DependencyViewRepresentation::TextTree;
    let mut generator =
        sdml_generate::actions::deps::DependencyViewGenerator::default().with_format_options(view);
    generator.write(module, cache, &mut buffer).unwrap();
    String::from_utf8(buffer.into_inner()).unwrap()
}

// ------------------------------------------------------------------------------------------------
// Modules ❱ Imports
// ------------------------------------------------------------------------------------------------

test_examples! {
    import_dep_tree, "deps", generate_dependency_tree => (
        import_member_only,
        import_module_only,
        import_module_version,
        import_multiple_members,
        import_multiple_mixed,
        import_multiple_modules,
        import_multiple_module_version
    )
}
