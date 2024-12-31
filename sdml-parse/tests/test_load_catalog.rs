use sdml_core::{load::ModuleLoader, model::identifiers::Identifier, store::ModuleStore};
use sdml_parse::load::SDML_CATALOG_FILE_VARIABLE;
use serial_test::serial;
use std::str::FromStr;
use url::Url;

const MANIFEST_PATH: &str = env!("CARGO_MANIFEST_DIR");
const TEST_PATH: &str = "tests/catalog_examples";

const CATALOG_FILE: &str = "custom-catalog.json";
const MODULE_NAME: &str = "campaign";

fn set_env_variable(env_key: &str, env_value: Option<String>) {
    match env_value {
        Some(v) => std::env::set_var(env_key, v),
        None => std::env::remove_var(env_key),
    }
}

fn with_env_variable<F>(env_key: &str, env_value: Option<&str>, test: F)
where
    F: FnOnce() + std::panic::UnwindSafe,
{
    // Set the environment variable
    let old_value = std::env::var(env_key).ok();

    set_env_variable(env_key, env_value.map(String::from));

    // Run the test, catching any panic
    let result = std::panic::catch_unwind(|| {
        test();
    });

    // Clean-up / restore environment variable
    set_env_variable(env_key, old_value);

    // Propagate the panic if it occurred
    if let Err(err) = result {
        std::panic::resume_unwind(err);
    }
}

#[test]
#[serial]
fn test_load_without_catalogue() {
    with_env_variable(SDML_CATALOG_FILE_VARIABLE, None, || {
        let mut cache = ::sdml_core::store::InMemoryModuleCache::default().with_stdlib();
        let mut loader = ::sdml_parse::load::FsModuleLoader::default();
        let module_name = Identifier::from_str(MODULE_NAME).unwrap();

        loader
            .load(
                &module_name,
                loader.get_file_id(&module_name),
                &mut cache,
                true,
            )
            .expect_err("Error: Should have failed to load the module.");
    });
}

#[test]
#[serial]
fn test_load_with_catalogue() {
    let catalog_path =
        ::std::path::PathBuf::from(format!("{}/{}/{}", MANIFEST_PATH, TEST_PATH, CATALOG_FILE,));

    with_env_variable(SDML_CATALOG_FILE_VARIABLE, catalog_path.to_str(), || {
        let mut cache = ::sdml_core::store::InMemoryModuleCache::default().with_stdlib();
        let mut loader = ::sdml_parse::load::FsModuleLoader::default();
        let module_name = Identifier::from_str(MODULE_NAME).unwrap();

        loader
            .load(
                &module_name,
                loader.get_file_id(&module_name),
                &mut cache,
                true,
            )
            .expect("Error: Should have been able to load the module.");

        let module = cache
            .get(&module_name)
            .expect("Error: Module not found in cache.");

        let url = Url::from_str("https://examples.sdml.io/campaign#").ok();

        assert_eq!(module.base_uri().map(|x| { x.value().clone() }), url);
    });
}
