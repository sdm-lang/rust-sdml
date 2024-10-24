use sdml_errors::diagnostics::UseColor;
use sdml_generate::{
    actions::deps::{DependencyViewGenerator, DependencyViewOptions},
    color::set_colorize,
};
use sdml_generate::Generator;

#[macro_use]
mod common;

generator! {
    DependencyViewGenerator::default(),
    DependencyViewOptions::default().as_text_tree(),
    || {
        // turn this off to avoid control characters in the output.
        set_colorize(UseColor::Never);
    }
}

sdml_tests::test_setup! {
    "dep_tree",
    generate_to_string
}

sdml_tests::test_suite! {
    dependency_view => (
        import_member_only,
        import_module_only,
        import_module_version,
        import_multiple_members,
        import_multiple_mixed,
        import_multiple_modules,
        import_multiple_module_version
    )
}
