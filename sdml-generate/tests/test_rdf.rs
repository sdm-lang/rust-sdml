use paste::paste;
use sdml_core::{
    model::{modules::Module, HasName},
    store::{InMemoryModuleCache, ModuleStore},
};
use sdml_errors::diagnostics::UseColor;
use sdml_generate::convert::rdf::{RdfModelGenerator, RdfModelOptions};
use sdml_generate::{color::set_colorize, Generator};

#[macro_use]
mod common;

// ------------------------------------------------------------------------------------------------
// Modules ❱ Imports
// ------------------------------------------------------------------------------------------------

generator! {
    generate_turtle,
    RdfModelGenerator::default(),
    RdfModelOptions::default().as_ntriples(),
    || {
        // turn this off to avoid control characters in the output.
        set_colorize(UseColor::Never);
    }
}

// ------------------------------------------------------------------------------------------------
// Modules ❱ Imports
// ------------------------------------------------------------------------------------------------

test_examples! {
    rdf_turtle, "turtle", generate_turtle => (
        // Annotations -----------------
        // [NYI] Mannotation_multiple_binary,
        annotation_multiple_double,
        annotation_multiple_integer,
        annotation_multiple_iri,
        annotation_multiple_language_string,
        annotation_multiple_separate,
        annotation_multiple_string,
        // [NYI] annotation_single_binary,
        annotation_single_boolean,
        annotation_single_constructor,
        annotation_single_decimal,
        annotation_single_double,
        annotation_single_integer,
        annotation_single_iri,
        annotation_single_language_string,
        annotation_single_mapping,
        annotation_single_string,

        // Constraints -----------------
        // [NYI] constraint_informal,
        // [NYI] constraint_informal_language,
        // [NYI] constraint_informal_language_controlled,

        // Datatypes -------------------
        datatype_empty,
        datatype_from_string,
        datatype_with_restrictions,

        // Entities --------------------
        entity_empty,
        // [NYI] entity_with_constraints,
        entity_with_diff_members,
        entity_with_members,
        entity_with_unknowns,

        // Events ----------------------
        event_empty,

        // Imports ---------------------
        import_member_only,
        import_module_only,
        import_module_version,
        import_multiple_members,
        import_multiple_mixed,
        import_multiple_modules,
        import_multiple_module_version,

        // Modules ---------------------
        module_annotations,
        module_empty,
        module_empty_with_base,
        module_empty_with_comments,
        module_empty_with_version,

        // Properties ------------------
        // [incomplete] property_def_empty,
        // [incomplete] property_def_some,
        // [incomplete] property_def_used,

        // RDF -------------------------
        rdf_definitions,

        // Structures ------------------
        structure_empty,
        structure_mapping_type,
        structure_simple_types,
        structure_with_features,

        // Type Classes
        // [incomplete] type_class_empty,
        // [incomplete] type_class_methods,
        // [incomplete] type_class_subtype,

        // Unions ----------------------
        union_empty,
        union_rename_variant,
        union_variants,

        // Regression Tests
        type_constructor_fix9
    )
}
