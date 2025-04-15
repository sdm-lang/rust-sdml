use objio::ObjectWriter;
use rdftk_core::model::graph::PrefixMapping;
use rdftk_io::turtle::{TurtleWriter, TurtleWriterOptions};
use sdml_core::{model::modules::Module, store::InMemoryModuleCache};
use sdml_rdf::write::{module_to_graph, Options};

sdml_tests::test_setup! {
    all "ttl" => module_to_turtle
}

fn module_to_turtle(module: &Module, cache: &InMemoryModuleCache) -> String {
    let options = Options::default()
        .with_include_source_location(true)
        .with_mappings(PrefixMapping::default());
    let graph = module_to_graph(module, cache, &options).unwrap();
    let options = TurtleWriterOptions::default().with_predicate_padding(true);
    let writer = TurtleWriter::default().with_options(options);
    writer.write_to_string(&graph).unwrap()
}
