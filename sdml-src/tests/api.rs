use pretty_assertions::assert_eq;
use sdml_core::model::identifiers::Identifier;
use sdml_core::model::modules::Module;
use sdml_core::repr::RepresentationWriter;
use sdml_src::write::{Options, Writer};
use url::Url;

#[test]
fn test_generate_module_empty() {
    let module = Module::new(Identifier::new_unchecked("example"));
    let writer = Writer;
    let source = writer.write_to_string(&module, None).unwrap();
    println!(">>{source:?}<<");
    assert_eq!(source.as_str(), "module example is end\n");
}

#[test]
fn test_generate_module_empty_with_base() {
    let module = Module::new(Identifier::new_unchecked("example"))
        .with_base_uri(Url::parse("http://example.com").unwrap());
    let writer = Writer;
    let source = writer
        .write_to_string_with(&module, None, &Options::default())
        .unwrap();
    println!(">>{source:?}<<");
    assert_eq!(
        source.as_str(),
        "module example <http://example.com/> is end\n"
    );
}
