use sdml_deps::write::{rdf::RdfDependencyWriter, DependencyWriterOptions};
use sdml_tests::{generator_fn, test_setup, test_suite};

generator_fn! {
    default DependencyWriterOptions,
    default RdfDependencyWriter
}

test_setup! {
    all "deps_rdf" => generate_to_string ; true, true
}

test_suite! {
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
