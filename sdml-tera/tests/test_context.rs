use sdml_core::store::InMemoryModuleCache;
use sdml_core::store::ModuleStore;
use sdml_json::generate::module_to_value;
use sdml_json::generate::JsonFlavor;
use sdml_parse::load::FsModuleLoader;
use std::io::Cursor;

#[test]
fn test_context_from_very_empty_module() {
    let mut cache = InMemoryModuleCache::default();
    let mut loader = FsModuleLoader::default();
    let module_name =
        loader.load_from_reader(&mut Cursor::new(b"module foo is end"), &mut cache, false);
    assert!(module_name.is_ok());

    let module = cache.get(&module_name.unwrap());
    assert!(module.is_some());

    let value = module_to_value(module.unwrap(), JsonFlavor::Context);
    let result = serde_json::to_writer_pretty(std::io::stdout(), &value);
    assert!(result.is_ok());
}

#[test]
fn test_context_from_empty_module() {
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

    let value = module_to_value(module.unwrap(), JsonFlavor::Context);
    let result = serde_json::to_writer_pretty(std::io::stdout(), &value);
    assert!(result.is_ok());
}

#[test]
fn test_context_from_module_with_version() {
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

    let value = module_to_value(module.unwrap(), JsonFlavor::Context);
    let result = serde_json::to_writer_pretty(std::io::stdout(), &value);
    assert!(result.is_ok());
}

#[test]
fn test_context_from_module_with_imports() {
    let mut cache = InMemoryModuleCache::default().with_stdlib();
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
    assert!(module_name.is_ok());

    let module = cache.get(&module_name.unwrap());
    assert!(module.is_some());

    let value = module_to_value(module.unwrap(), JsonFlavor::Context);
    let result = serde_json::to_writer_pretty(std::io::stdout(), &value);
    assert!(result.is_ok());
}

#[test]
fn test_context_from_module_with_annotations() {
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

    let value = module_to_value(module.unwrap(), JsonFlavor::Context);
    let result = serde_json::to_writer_pretty(std::io::stdout(), &value);
    assert!(result.is_ok());
}
