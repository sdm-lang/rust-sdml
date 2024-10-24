use objio::{HasOptions, ObjectWriter};
use rdftk_io::turtle::{TurtleWriter, TurtleWriterOptions};
use sdml_core::{model::modules::Module, store::InMemoryModuleCache};
use sdml_rdf::generate::module_to_graph;

sdml_tests::test_setup! {
    "ttl",
    standard,
    module_to_turtle
}

fn module_to_turtle(module: &Module, cache: &InMemoryModuleCache) -> String {
    let graph = module_to_graph(module, cache).unwrap();
    let options = TurtleWriterOptions::default().with_predicate_padding(true);
    let writer = TurtleWriter::default().with_options(options);
    writer.write_to_string(&graph).unwrap()
}
