use sdml;
use sdml::api::{NodeWrapper, SimpleValue, TypeDefinition, Value};
use std::str::FromStr;
use url::Url;

#[test]
fn test_parse_empty_module() {
    let parse_tree = sdml::parse_str("module foo is end");
    assert!(parse_tree.is_ok());

    let parse_tree = parse_tree.unwrap();
    let module = parse_tree.module();
    let name = module.name();
    let name_text = name.text();
    assert!(name_text.is_ok());

    let name_text = name_text.unwrap();
    assert_eq!(name_text, "foo");
}

#[test]
fn test_parse_module_with_imports() {
    let parse_tree = sdml::parse_str(
        r#"module foo is

  import foo

  import foo:bar

  import [ goo goo:poo ]
end"#,
    );
    assert!(parse_tree.is_ok());

    let parse_tree = parse_tree.unwrap();
    let module = parse_tree.module();
    let body = module.body();

    let imports = body.imports();
    assert_eq!(imports.len(), 3);

    let import = imports.get(0).unwrap();
    let imported: Vec<&str> = import.imported().iter().map(|i|i.text().unwrap()).collect();
    assert_eq!(imported, ["foo"]);
}

#[test]
fn test_parse_module_with_annotations() {
    let parse_tree = sdml::parse_str(
        r#"module foo is

  @xml:base = <https://example.org/>

  @dc:version = 2

  @skos:prefLang = [
    "aa"@en
    "bb"
  ]

end"#,
    );
    assert!(parse_tree.is_ok());

    let parse_tree = parse_tree.unwrap();
    let module = parse_tree.module();
    let body = module.body();

    let imports = body.annotations();
    assert_eq!(imports.len(), 3);

    let annotation = imports.get(0).unwrap();
    assert_eq!(annotation.name().as_ref(), "xml:base");
    if let Value::IriReference(value) = annotation.value() {
        assert_eq!(
            value.value(),
            Url::from_str("https://example.org/").unwrap()
        );
    } else {
        panic!();
    }

    let annotation = imports.get(1).unwrap();
    assert_eq!(annotation.name().as_ref(), "dc:version");
    if let Value::Integer(value) = annotation.value() {
        assert_eq!(value.value(), 2);
    } else {
        panic!();
    }

    let annotation = imports.get(2).unwrap();
    assert_eq!(annotation.name().as_ref(), "skos:prefLang");
    if let Value::ListOfValues(list) = annotation.value() {
        let values = list.values();
        assert_eq!(values.len(), 2);

        if let Some(SimpleValue::String(value)) = values.get(0) {
            assert_eq!(value.string().value(), "aa");
            assert_eq!(value.language().unwrap().value(), "en");
        } else {
            panic!();
        }

        if let Some(SimpleValue::String(value)) = values.get(1) {
            assert_eq!(value.string().value(), "bb");
            assert!(value.language().is_none());
        } else {
            panic!();
        }
    } else {
        panic!();
    }
}

#[test]
fn test_parse_datatype() {
    let parse_tree = sdml::parse_str(
        r#"module foo is

  datatype Name <- xsd:string

end"#,
    );
    assert!(parse_tree.is_ok());

    let parse_tree = parse_tree.unwrap();
    let module = parse_tree.module();
    let body = module.body();

    let types = body.definitions();
    assert_eq!(types.len(), 1);

    if let Some(TypeDefinition::Datatype(definition)) = types.get(0) {
        assert_eq!(definition.name().as_ref(), "Name");
        assert_eq!(definition.base_type().as_ref(), "xsd:string");
    } else {
        panic!();
    }
}
