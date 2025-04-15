/*!
Provides the core in-memory implementation of the *Simple Domain Modeling Language* (SDML).

This package also includes the traits used to describe module loading as well as artifact
generators.

# Features

## repr-write

Includes a trait [`RepresentationWriter`] for implementers, and clients, of model representation forms requiring more than simply Serde support.

## serde

Support for serde derived serialization and de-serialization for all the model types.

## stdlib

Includes support for the standard library definitions.

## stdlib-ext-config

Includes support for external configuration of standard library content.

## terms

Includes support for term checking as a part of the standard validation process.

## tree-sitter

*/

#![warn(
    unknown_lints,
    // ---------- Stylistic
    absolute_paths_not_starting_with_crate,
    elided_lifetimes_in_paths,
    explicit_outlives_requirements,
    macro_use_extern_crate,
    nonstandard_style, /* group */
    noop_method_call,
    rust_2018_idioms,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    // ---------- Future
    future_incompatible, /* group */
    rust_2021_compatibility, /* group */
    // ---------- Public
    missing_debug_implementations,
    // missing_docs,
    unreachable_pub,
    // ---------- Unsafe
    unsafe_code,
    unsafe_op_in_unsafe_fn,
    // ---------- Unused
    unused, /* group */
)]
#![deny(
    // ---------- Public
    exported_private_dependencies,
    // ---------- Deprecated
    anonymous_parameters,
    bare_trait_objects,
    ellipsis_inclusive_range_patterns,
    // ---------- Unsafe
    deref_nullptr,
    drop_bounds,
    dyn_drop,
)]

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub use sdml_errors::errors as error;

pub mod config;

pub mod load;

pub mod model;

#[cfg(feature = "repr-write")]
pub mod repr;

#[cfg(feature = "stdlib")]
pub mod stdlib;

pub mod store;

pub mod syntax;
