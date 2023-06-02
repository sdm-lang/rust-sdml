use sdml::model::parse::parse_file;
use std::fs::read_to_string;

macro_rules! test_example {
    ($fnname: ident, $exname: literal) => {
        #[test]
        fn $fnname() {
            let input = format!("tests/examples/{}.sdm", $exname);

            let module = parse_file(input);
            if let Err(e) = module {
                panic!("parse error: {}", e);
            }
            let module = module.unwrap();

            let module_as_string = format!("{:#?}\n", module);

            let expected = format!("tests/examples/{}.ron", $exname);
            let expected_string = read_to_string(expected);
            if let Err(e) = expected_string {
                panic!("io error: {}", e);
            }

            pretty_assertions::assert_eq!(module_as_string, expected_string.unwrap());
        }
    };
}

test_example!(test_empty_module, "empty_module");

test_example!(test_module_annotations, "module_annotations");

test_example!(test_simple_datatype, "simple_datatype");
