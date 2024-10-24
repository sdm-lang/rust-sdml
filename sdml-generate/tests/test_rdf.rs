use sdml_errors::diagnostics::UseColor;
use sdml_generate::{color::set_colorize, convert::rdf::{RdfModelGenerator, RdfModelOptions}};
use sdml_generate::Generator;

#[macro_use]
mod common;

generator! {
    generate_to_string,
    RdfModelGenerator::default(),
    RdfModelOptions::default().as_ntriples(),
    || {
        // turn this off to avoid control characters in the output.
        set_colorize(UseColor::Never);
    }
}

sdml_tests::test_setup! {
    "turtle",
    standard,
    generate_to_string
}
