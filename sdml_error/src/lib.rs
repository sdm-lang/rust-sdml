/*!
One-line description.

More detailed description, with

# Example

End of file during parsingSymbol’s value as variable is void: rustEnd of file during parsing

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
    // ---------- Deprecated
    anonymous_parameters,
    bare_trait_objects,
    ellipsis_inclusive_range_patterns,
    // ---------- Unsafe
    deref_nullptr,
    drop_bounds,
    dyn_drop,
)]

use codespan_reporting::files::SimpleFiles;
use std::fmt::Display;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// An opaque identifier used to index the source associated with a loaded module.
///
pub type FileId = usize;

///
/// A type that holds the source code loaded prior to parsing.
///
#[derive(Clone, Debug)]
pub struct Source(String);

pub type SourceFiles = SimpleFiles<String, Source>;

///
/// A span, in bytes, start..end for some context.
///
pub type Span = std::ops::Range<usize>;

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for Source {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl AsRef<str> for Source {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<[u8]> for Source {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl Source {
    fn as_str(&self) -> &str {
        self.0.as_str()
    }

    fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[macro_use]
mod i18n;

pub mod codes;

pub mod errors;
pub use errors::Error;

pub mod diagnostics;
pub use diagnostics::{Diagnostic, Reporter};
