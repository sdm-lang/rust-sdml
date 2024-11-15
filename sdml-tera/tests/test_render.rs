use sdml_core::store::InMemoryModuleCache;
use sdml_core::store::ModuleStore;
use sdml_parse::load::FsModuleLoader;
use sdml_tera::make_engine_from;
use sdml_tera::render_module;
use std::io::Cursor;

#[test]
fn test_render_very_empty_module() {
    let mut cache = InMemoryModuleCache::default();
    let mut loader = FsModuleLoader::default();
    let module_name =
        loader.load_from_reader(&mut Cursor::new(b"module foo is end"), &mut cache, false);
    assert!(module_name.is_ok());

    let module = cache.get(&module_name.unwrap());
    assert!(module.is_some());

    let engine = make_engine_from("tests/templates/**/*.md");
    assert!(engine.is_ok());

    let result = render_module(&engine.unwrap(), module.unwrap(), None, "module.md");
    assert!(result.is_ok());

    println!(">>>\n{}\n<<<", result.unwrap());
}

#[test]
fn test_render_empty_module() {
    let mut cache = InMemoryModuleCache::default();
    let mut loader = FsModuleLoader::default();
    let module_name = loader.load_from_reader(
        &mut Cursor::new(b"module foo <http://example.org/v/2#> is end"),
        &mut cache,
        false,
    );
    assert!(module_name.is_ok());

    let module = cache.get(&module_name.unwrap());
    assert!(module.is_some());

    let engine = make_engine_from("tests/templates/**/*.md");
    assert!(engine.is_ok());

    let result = render_module(&engine.unwrap(), module.unwrap(), None, "module.md");
    assert!(result.is_ok());

    println!(">>>\n{}\n<<<", result.unwrap());
}

#[test]
fn test_render_module_with_version() {
    let mut cache = InMemoryModuleCache::default();
    let mut loader = FsModuleLoader::default();
    let module_name = loader.load_from_reader(
        &mut Cursor::new(b"module foo <http://example.org/v/2#> version \"v2\" <http://example.org/v/2024-10-4#> is end"),
        &mut cache,
        false,
    );
    assert!(module_name.is_ok());

    let module = cache.get(&module_name.unwrap());
    assert!(module.is_some());

    let engine = make_engine_from("tests/templates/**/*.md");
    assert!(engine.is_ok());

    let result = render_module(&engine.unwrap(), module.unwrap(), None, "module.md");
    assert!(result.is_ok());

    println!(">>>\n{}\n<<<", result.unwrap());
}

#[test]
fn test_render_module_with_imports() {
    let mut cache = InMemoryModuleCache::default().with_stdlib();
    let mut loader = FsModuleLoader::default();
    let module_name = loader.load_from_reader(
        &mut Cursor::new(
            b"module foo <http://example.org/v/2#> is
  import sdml
  import [dc skos:prefLabel]
end",
        ),
        &mut cache,
        true,
    );
    assert!(module_name.is_ok());

    let module = cache.get(&module_name.unwrap());
    assert!(module.is_some());

    let engine = make_engine_from("tests/templates/**/*.md");
    assert!(engine.is_ok());

    let result = render_module(&engine.unwrap(), module.unwrap(), None, "module.md");
    assert!(result.is_ok());

    println!(">>>\n{}\n<<<", result.unwrap());
}

#[test]
fn test_render_module_with_annotations() {
    let mut cache = InMemoryModuleCache::default().with_stdlib();
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

    let engine = make_engine_from("tests/templates/**/*.md");
    assert!(engine.is_ok());

    let result = render_module(&engine.unwrap(), module.unwrap(), None, "module.md");
    assert!(result.is_ok());

    println!(">>>\n{}\n<<<", result.unwrap());
}
