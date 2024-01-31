use pretty_assertions::assert_eq;
use sdml_core::cache::ModuleCache;
use sdml_core::model::identifiers::Identifier;
use sdml_core::model::modules::Module;
use sdml_generate::color::{set_colorize, UseColor};
use sdml_generate::convert::source::SourceGenerator;
use sdml_generate::GenerateToWriter;
use url::Url;

#[test]
fn test_generate_module_empty() {
    // turn this off to avoid control characters in the output.
    set_colorize(UseColor::Never);
    let module = Module::empty(Identifier::new_unchecked("example"));
    let mut generator: SourceGenerator = Default::default();
    let source = generator
        .write_to_string(&module, &ModuleCache::default())
        .unwrap();
    println!(">>{source:?}<<");
    assert_eq!(source.as_str(), "module example is end\n");
}

#[test]
fn test_generate_module_empty_with_base() {
    // turn this off to avoid control characters in the output.
    set_colorize(UseColor::Never);
    let module = Module::empty(Identifier::new_unchecked("example"))
        .with_base_uri(Url::parse("http://example.com").unwrap());
    let mut generator: SourceGenerator = Default::default();
    let source = generator
        .write_to_string(&module, &ModuleCache::default())
        .unwrap();
    println!(">>{source:?}<<");
    assert_eq!(
        source.as_str(),
        "module example <http://example.com/> is end\n"
    );
}
