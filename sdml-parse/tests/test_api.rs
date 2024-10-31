use sdml_core::model::annotations::{AnnotationProperty, HasAnnotations};
use sdml_core::model::definitions::Definition;
use sdml_core::model::modules::ImportStatement;
use sdml_core::model::values::{SequenceMember, SimpleValue, Value};
use sdml_core::model::{HasBody, HasName, HasNameReference};
use sdml_core::store::{InMemoryModuleCache, ModuleStore};
use sdml_parse::load::FsModuleLoader;
use std::io::Cursor;
use std::str::FromStr;
use url::Url;

#[test]
fn test_parse_empty_module() {
    let mut cache = InMemoryModuleCache::default();
    let mut loader = FsModuleLoader::default();
    let module = loader.load_from_reader(&mut Cursor::new(b"module foo is end"), &mut cache, false);
    println!("{:#?}", module);
    assert!(module.is_ok());

    let module = cache.get(&module.unwrap());
    let module = module.unwrap();
    let name = module.name();
    assert_eq!(name.as_ref(), "foo");
}

#[test]
fn test_parse_module_with_imports() {
    let mut cache = InMemoryModuleCache::default();
    let mut loader = FsModuleLoader::default();
    let module = loader.load_from_reader(
        &mut Cursor::new(
            r#"module foo is

  import foo

  import foo:bar

  import [ goo goo:poo ]
end"#
                .as_bytes(),
        ),
        &mut cache,
        false,
    );
    println!("{:#?}", module);
    assert!(module.is_ok());

    let module = cache.get(&module.unwrap());
    let module = module.unwrap();
    let body = module.body();

    let imports: Vec<&ImportStatement> = body.imports().collect();
    assert_eq!(imports.len(), 3);

    let import = imports.first().unwrap();
    let imported: Vec<String> = import.imports().map(|i| i.to_string()).collect();
    assert_eq!(imported, ["foo"]);
}

#[test]
fn test_parse_module_with_annotations() {
    let mut cache = InMemoryModuleCache::default();
    let mut loader = FsModuleLoader::default();
    let module = loader.load_from_reader(
        &mut Cursor::new(
            r#"module foo is

  @xml:base = <https://example.org/>

  @dc:version = 2

  @skos:prefLang = [
    "aa"@en
    "bb"
  ]

end"#
                .as_bytes(),
        ),
        &mut cache,
        false,
    );
    println!("{:#?}", module);
    assert!(module.is_ok());

    let module = cache.get(&module.unwrap());
    let module = module.unwrap();
    let body = module.body();

    let annotations: Vec<&AnnotationProperty> = body.annotation_properties().collect();
    assert_eq!(annotations.len(), 3);

    let annotation = annotations.first().unwrap();
    assert_eq!(annotation.name_reference().to_string().as_str(), "xml:base");
    if let Value::Simple(SimpleValue::IriReference(value)) = annotation.value() {
        assert_eq!(value, &Url::from_str("https://example.org/").unwrap());
    } else {
        panic!();
    }

    let annotation = annotations.get(1).unwrap();
    assert_eq!(
        annotation.name_reference().to_string().as_str(),
        "dc:version"
    );
    if let Value::Simple(SimpleValue::Integer(value)) = annotation.value() {
        assert_eq!(*value, 2);
    } else {
        panic!();
    }

    let annotation = annotations.get(2).unwrap();
    assert_eq!(
        annotation.name_reference().to_string().as_str(),
        "skos:prefLang"
    );
    match annotation.value() {
        Value::Sequence(list) => {
            let values: Vec<&SequenceMember> = list.iter().collect();
            assert_eq!(values.len(), 2);

            if let Some(SequenceMember::Simple(SimpleValue::String(value))) = values.first() {
                let language_str: &str = value.language().unwrap().as_ref();
                assert_eq!(value.value().as_str(), "aa");
                assert_eq!(language_str, "en");
            } else {
                panic!();
            }

            if let Some(SequenceMember::Simple(SimpleValue::String(value))) = values.get(1) {
                assert_eq!(value.value().as_str(), "bb");
                assert!(value.language().is_none());
            } else {
                panic!();
            }
        }
        _ => {
            panic!();
        }
    }
}

#[test]
fn test_parse_datatype() {
    let mut cache = InMemoryModuleCache::default();
    let mut loader = FsModuleLoader::default();
    let module = loader.load_from_reader(
        &mut Cursor::new(
            r#"module foo is

  datatype Name <- xsd:string

end"#
                .as_bytes(),
        ),
        &mut cache,
        false,
    );
    println!("{:#?}", module);
    assert!(module.is_ok());

    let module = cache.get(&module.unwrap());
    let module = module.unwrap();
    let body = module.body();

    let types: Vec<&Definition> = body.definitions().collect();
    assert_eq!(types.len(), 1);

    if let Some(Definition::Datatype(definition)) = types.first() {
        assert_eq!(definition.name().as_ref(), "Name");
        assert_eq!(definition.base_type().to_string().as_str(), "xsd:string");
    } else {
        panic!();
    }
}
