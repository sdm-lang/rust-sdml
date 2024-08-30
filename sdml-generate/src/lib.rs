/*!
This package provides a set of generators, or transformations, from the in-memory model to either other representations
as well as actions that can be performed on modules.

This package also provides a pair of traits used to define *generators*, types that convert one or more modules into
other artifacts.

# Example

The following shows common usage of the `GenerateToWriter` trait in this case to write a text-tree representation of a
modules transitive dependencies.

```rust
use sdml_core::store::InMemoryModuleCache;
use sdml_core::model::modules::Module;
use sdml_generate::Generator;
use sdml_generate::actions::deps::{
    DependencyViewGenerator, DependencyViewOptions,
};
use std::io::stdout;
# use sdml_core::model::identifiers::Identifier;
# fn load_module() -> (Module, InMemoryModuleCache) { (Module::empty(Identifier::new_unchecked("example")), InMemoryModuleCache::default()) }

let (module, cache) = load_module();

let mut generator = DependencyViewGenerator::default();
let options = DependencyViewOptions::default().as_text_tree();
generator.generate_with_options(&module, &cache, options, None, &mut stdout())
         .expect("write to stdout failed");
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

use sdml_core::{error::Error, model::modules::Module, store::ModuleStore};
use std::{fmt::Debug, fs::OpenOptions, io::Cursor, io::Write, path::PathBuf};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This trait denotes a type that generates content from a module.
///
/// The type `Options` denotes some type that contains any settings that affect the behavior of
/// the generator. If no settings are required `Options` may be set to `()`. Given that options
/// are provided at the method level it is recommended that generators are constructed using
/// `Default::default()`.
///
pub trait Generator: Default {
    type Options: Default + Debug;

    // --------------------------------------------------------------------------------------------
    // Write to ❱ implementation of `Write`
    // --------------------------------------------------------------------------------------------

    ///
    /// Generate from the given module into the provided writer. Note that this calls
    /// `generate_with_options` using `Self::Options::default()`.
    ///
    fn generate<W>(
        &mut self,
        module: &Module,
        cache: &impl ModuleStore,
        path: Option<PathBuf>,
        writer: &mut W,
    ) -> Result<(), Error>
    where
        W: Write + Sized,
    {
        self.generate_with_options(module, cache, Default::default(), path, writer)
    }

    ///
    /// Generate from the given module into a string.
    ///
    fn generate_to_string(
        &mut self,
        module: &Module,
        cache: &impl ModuleStore,
        options: Self::Options,
        path: Option<PathBuf>,
    ) -> Result<String, Error> {
        let mut buffer = Cursor::new(Vec::new());
        self.generate_with_options(module, cache, options, path, &mut buffer)?;
        Ok(String::from_utf8(buffer.into_inner())?)
    }

    ///
    /// Generate from the given module into a file.
    ///
    /// Note: The referenced file will be created if it does not exist, and replaced if it does.
    ///
    fn generate_to_file(
        &mut self,
        module: &Module,
        cache: &impl ModuleStore,
        options: Self::Options,
        path: &PathBuf,
    ) -> Result<(), Error> {
        let mut file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(path)?;
        self.generate_with_options(module, cache, options, Some(path.clone()), &mut file)
    }

    ///
    /// Generate from the given module into the provided writer.
    ///
    fn generate_with_options<W>(
        &mut self,
        module: &Module,
        cache: &impl ModuleStore,
        options: Self::Options,
        path: Option<PathBuf>,
        writer: &mut W,
    ) -> Result<(), Error>
    where
        W: Write + Sized;
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[macro_use]
mod macros;

mod errors;

mod exec;

pub mod color;

pub mod actions;

pub mod convert;

pub mod draw;
