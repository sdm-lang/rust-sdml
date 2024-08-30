/*!
This library provides a parser for the Simple Domain Modeling Language (SDML) and produces an in-memory representation
using the crate [sdml-core](https://crates.io/crates/sdml-core).

The `ModuleLoader` trait from, `sdml-core`, provides the interface for finding, parsing, and loading modules and the
[`load::FsModuleLoader`] implementation is provided in this crate for file-system based module definitions.

# Example

The following example demonstrates the `FsModuleLoader` to resolve a module name to a file and parse it.


```rust,no_run
use sdml_core::model::identifiers::Identifier;
use sdml_core::store::{InMemoryModuleCache, ModuleStore};
use sdml_core::load::ModuleLoader;
use sdml_parse::load::FsModuleLoader;
use std::str::FromStr;

let mut cache = InMemoryModuleCache::default().with_stdlib();
let mut loader = FsModuleLoader::default();

let name = Identifier::from_str("example").unwrap();

let module_name = loader.load(&name, None, &mut cache, true);
assert!(module_name.is_ok());

let module = cache.get(&module_name.unwrap()).unwrap();
```

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

mod parse;

pub use sdml_core::error;

pub mod load;
