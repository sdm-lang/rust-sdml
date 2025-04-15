use objio::ObjectWriter;
use rdftk_core::model::graph::PrefixMapping;
use rdftk_io::nt::NTripleWriter;
use sdml_core::{model::modules::Module, store::InMemoryModuleCache};
use sdml_rdf::write::{module_to_graph, Options};

sdml_tests::test_setup! {
    all "nt" => module_to_ntriples; true, true
}

fn module_to_ntriples(module: &Module, cache: &InMemoryModuleCache) -> String {
    let options = Options::default()
        .with_include_source_location(true)
        .with_mappings(PrefixMapping::default());
    let graph = module_to_graph(module, cache, &options).unwrap();
    let writer = NTripleWriter::default();
    writer.write_to_string(&graph).unwrap()
}
