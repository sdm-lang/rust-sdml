use paste::paste;
use sdml_core::{
    model::{modules::Module, HasName},
    store::{InMemoryModuleCache, ModuleStore},
};
use sdml_errors::diagnostics::UseColor;
use sdml_generate::{
    actions::deps::{DependencyViewGenerator, DependencyViewOptions},
    color::set_colorize,
    Generator,
};

#[macro_use]
mod common;

// ------------------------------------------------------------------------------------------------
// Modules ❱ Imports
// ------------------------------------------------------------------------------------------------

generator! {
    DependencyViewGenerator::default(),
    DependencyViewOptions::default().as_text_tree(),
    || {
        // turn this off to avoid control characters in the output.
        set_colorize(UseColor::Never);
    }
}

// ------------------------------------------------------------------------------------------------
// Modules ❱ Imports
// ------------------------------------------------------------------------------------------------

test_examples! {
    dependency_view, "dep_tree", generate_to_string => (
        import_member_only,
        import_module_only,
        import_module_version,
        import_multiple_members,
        import_multiple_mixed,
        import_multiple_modules,
        import_multiple_module_version
    )
}
