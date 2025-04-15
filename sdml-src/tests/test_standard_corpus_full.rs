use sdml_core::{model::modules::Module, repr::RepresentationWriter, store::InMemoryModuleCache};
use sdml_src::write::{Level, Options, Writer};

sdml_tests::test_setup! {
    all "source_full" =>  module_to_source
}

fn module_to_source(module: &Module, cache: &InMemoryModuleCache) -> String {
    let options = Options::default()
        .with_level(Level::Full)
        .with_emit_base_iri(false);
    let writer = Writer;
    writer
        .write_to_string_with(module, Some(cache), &options)
        .unwrap()
}
