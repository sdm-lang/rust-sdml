//use pretty_assertions::assert_eq;
use rdftk_core::model::graph::Graph;
use sdml_core::{
    model::identifiers::Identifier,
    store::{InMemoryModuleCache, ModuleStore},
};
use sdml_parse::load::FsModuleLoader;
use sdml_rdf::write::module_to_graph;
use std::io::Cursor;

fn print_graph(graph: &Graph) {
    for stmt in graph.statements() {
        println!(
            "{} <{}> {} .",
            stmt.subject(),
            stmt.predicate(),
            stmt.object()
        );
    }
}

#[test]
fn test_parse_empty_module() {
    let mut cache = InMemoryModuleCache::with_stdlib();
    let mut loader = FsModuleLoader::default();
    let module_name = loader.load_from_reader(
        &mut Cursor::new(b"module foo <http://example.org/v/2#> is end"),
        &mut cache,
        false,
    );
    println!("{module_name:#?}");
    assert!(module_name.is_ok());

    let module = cache.get(&module_name.unwrap());
    assert!(module.is_some());

    let result = module_to_graph(module.unwrap(), &cache, &Default::default());
    assert!(result.is_ok());

    let graph = result.unwrap();
    print_graph(&graph);
}

#[test]
fn test_parse_module_with_version() {
    let mut cache = InMemoryModuleCache::with_stdlib();
    let mut loader = FsModuleLoader::default();
    let module_name = loader.load_from_reader(
        &mut Cursor::new(b"module foo <http://example.org/v/2#> version \"v2\" <http://example.org/v/2024-10-4#> is end"),
        &mut cache,
        false,
    );
    println!("{module_name:#?}");
    assert!(module_name.is_ok());

    let module = cache.get(&module_name.unwrap());
    assert!(module.is_some());

    let result = module_to_graph(module.unwrap(), &cache, &Default::default());
    assert!(result.is_ok());

    let graph = result.unwrap();
    print_graph(&graph);
}

#[test]
fn test_parse_module_with_imports() {
    let mut cache = InMemoryModuleCache::with_stdlib();
    let mut loader = FsModuleLoader::default();
    let module_name = loader.load_from_reader(
        &mut Cursor::new(
            b"module foo <http://example.org/v/2#> is
  import sdml
  import [dc skos]
end",
        ),
        &mut cache,
        true,
    );
    println!("{module_name:#?}");
    assert!(module_name.is_ok());

    println!(
        "LOADED {:?}",
        cache.module_names().collect::<Vec<&Identifier>>()
    );

    let module = cache.get(&module_name.unwrap());
    assert!(module.is_some());

    let result = module_to_graph(module.unwrap(), &cache, &Default::default());
    assert!(result.is_ok());

    let graph = result.unwrap();
    print_graph(&graph);
}

#[test]
fn test_parse_module_with_annotations() {
    let mut cache = InMemoryModuleCache::with_stdlib();
    let mut loader = FsModuleLoader::default();
    let module_name = loader.load_from_reader(
        &mut Cursor::new(
            b"module foo <http://example.org/v/2#> is
  import dc
  @dc:title = \"The Foo module.\"
end",
        ),
        &mut cache,
        true,
    );
    println!("{module_name:#?}");
    assert!(module_name.is_ok());

    let module = cache.get(&module_name.unwrap());
    assert!(module.is_some());

    let result = module_to_graph(module.unwrap(), &cache, &Default::default());
    assert!(result.is_ok());

    let graph = result.unwrap();
    print_graph(&graph);
}
