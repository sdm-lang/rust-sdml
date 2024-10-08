use pretty_assertions::assert_eq;
use sdml_core::model::identifiers::Identifier;
use sdml_core::model::modules::Module;
use sdml_core::store::InMemoryModuleCache;
use sdml_errors::diagnostics::UseColor;
use sdml_generate::color::set_colorize;
use sdml_generate::convert::source::{SourceGenerator, SourceGeneratorOptions};
use sdml_generate::Generator;
use url::Url;

pub mod common;

#[test]
fn test_generate_module_empty() {
    // turn this off to avoid control characters in the output.
    set_colorize(UseColor::Never);
    let module = Module::empty(Identifier::new_unchecked("example"));
    let mut generator: SourceGenerator = Default::default();
    let source = generator
        .generate_to_string(
            &module,
            &InMemoryModuleCache::default(),
            SourceGeneratorOptions::default(),
            None,
        )
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
        .generate_to_string(
            &module,
            &InMemoryModuleCache::default(),
            SourceGeneratorOptions::default(),
            None,
        )
        .unwrap();
    println!(">>{source:?}<<");
    assert_eq!(
        source.as_str(),
        "module example <http://example.com/> is end\n"
    );
}
