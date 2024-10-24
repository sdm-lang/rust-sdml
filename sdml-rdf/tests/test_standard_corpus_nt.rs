use objio::ObjectWriter;
use rdftk_io::nt::NTripleWriter;
use sdml_core::{model::modules::Module, store::InMemoryModuleCache};
use sdml_rdf::generate::module_to_graph;

sdml_tests::test_setup! {
    "nt",
    standard,
    module_to_turtle
}

fn module_to_turtle(module: &Module, cache: &InMemoryModuleCache) -> String {
    let graph = module_to_graph(module, cache).unwrap();
    let writer = NTripleWriter::default();
    writer.write_to_string(&graph).unwrap()
}
