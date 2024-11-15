use objio::{HasOptions, ObjectWriter};
use sdml_core::{model::modules::Module, store::ModuleStore};
use sdml_json::generate::{Writer, WriterOptions};

fn write_context_string(module: &Module, _: &impl ModuleStore) -> String {
    let writer = Writer::for_model()
        .with_options(WriterOptions::for_model().with_pretty_printing(true));
    format!("{}\n", writer.write_to_string(module).unwrap())
}

sdml_tests::test_setup! {
    "json",
    standard,
    write_context_string
}
