use sdml_core::{model::modules::Module, store::ModuleStore};
use sdml_json::write::{write_module_with_options_to_string, WriteOptions};

fn write_context_string(module: &Module, cache: &impl ModuleStore) -> String {
    format!(
        "{}\n",
        write_module_with_options_to_string(
            module,
            cache,
            WriteOptions::for_context().with_pretty_printing(true)
        )
        .unwrap()
    )
}

sdml_tests::test_setup! {
    all "context" => write_context_string
}
