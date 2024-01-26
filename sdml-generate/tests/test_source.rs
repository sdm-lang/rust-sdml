use sdml_core::cache::ModuleCache;
use sdml_core::model::identifiers::Identifier;
use sdml_core::model::modules::Module;
use sdml_generate::convert::source::SourceGenerator;
use sdml_generate::GenerateToWriter;
use pretty_assertions::assert_eq;
use url::Url;

#[test]
fn test_generate_module_empty() {
    let module = Module::empty(Identifier::new_unchecked("example"));
    let mut generator: SourceGenerator = Default::default();
    let source = generator.write_to_string(&module, &mut ModuleCache::default()).unwrap();
    assert_eq!(source.as_str(), "module example is end\n");
}

#[test]
fn test_generate_module_empty_with_base() {
    let module = Module::empty(Identifier::new_unchecked("example"))
        .with_base_uri(Url::parse("http://example.com").unwrap());
    let mut generator: SourceGenerator = Default::default();
    let source = generator.write_to_string(&module, &mut ModuleCache::default()).unwrap();
    assert_eq!(
        source.as_str(),
        "module example <http://example.com/> is end\n"
    );
}
