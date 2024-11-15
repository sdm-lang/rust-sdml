use objio::{HasOptions, ObjectWriter};
use sdml_core::{model::modules::Module, store::ModuleStore};
use sdml_json::generate::{Writer, WriterOptions};

fn write_context_string(module: &Module, _: &impl ModuleStore) -> String {
    let writer = Writer::for_context()
        .with_options(WriterOptions::for_context().with_pretty_printing(true));
    format!("{}\n", writer.write_to_string(module).unwrap())
}

sdml_tests::test_setup! {
    "context",
    standard,
    write_context_string
}
