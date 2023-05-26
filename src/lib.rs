/*!
Simple Domain Modeling Language.

More detailed description, with

# Example

YYYYY

# Features

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
    private_in_public,
    // ---------- Deprecated
    anonymous_parameters,
    bare_trait_objects,
    ellipsis_inclusive_range_patterns,
    // ---------- Unsafe
    deref_nullptr,
    drop_bounds,
    dyn_drop,
)]

use std::borrow::Cow;
use std::fs::read_to_string;
use std::path::Path;
use tree_sitter::Parser;
use tree_sitter_sdml::language;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn parse_file<P>(path: P) -> Result<ParseTree<'static>, Error>
where
    P: AsRef<Path>,
{
    let source = read_to_string(path)?;
    parse_str_inner(Cow::Owned(source))
}

pub fn parse_str(source: &str) -> Result<ParseTree<'_>, Error> {
    parse_str_inner(Cow::Borrowed(source))
}

#[allow(clippy::needless_lifetimes)]
fn parse_str_inner<'a>(source: Cow<'a, str>) -> Result<ParseTree<'a>, Error> {
    let mut parser = Parser::new();
    parser
        .set_language(language())
        .expect("Error loading SDML grammar");
    let tree = parser.parse(source.as_ref(), None).unwrap();
    Ok(ParseTree::new(source, tree))
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[macro_use]
mod macros;

pub mod api;
use api::ParseTree;

pub mod error;
use error::Error;

pub mod convert;

pub mod draw;

pub mod fmt;

pub mod walk;
