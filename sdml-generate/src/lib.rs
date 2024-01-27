/*!
This package provides a set of generators, or transformations, from the in-memory model to either other representations
as well as actions that can be performed on modules.

This package also provides a pair of traits used to define *generators*, types that convert one or more modules into
other artifacts.

# Example

The following shows common usage of the `GenerateToWriter` trait in this case to write a text-tree representation of a
modules transitive dependencies.

```rust
use sdml_core::cache::ModuleCache;
use sdml_core::model::modules::Module;
use sdml_generate::GenerateToWriter;
use sdml_generate::actions::deps::{
    DependencyViewRepresentation, DependencyViewGenerator,
};
use std::io::stdout;
# use sdml_core::model::identifiers::Identifier;
# fn load_module() -> (Module, ModuleCache) { (Module::empty(Identifier::new_unchecked("example")), ModuleCache::default()) }

let (module, cache) = load_module();

let view = DependencyViewRepresentation::TextTree;
let mut generator = DependencyViewGenerator::default();
generator.write_in_format(&module, &cache, &mut stdout(), view)
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

use sdml_core::{cache::ModuleCache, error::Error, model::modules::Module, model::HasName};
use std::{fmt::Debug, fs::File, io::Cursor, io::Write, path::Path};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

macro_rules! trace_entry {
    ($type_name: literal, $fn_name: literal => $format: literal, $( $value: expr ),+ ) => {
        const FULL_NAME: &str = concat!($type_name, "::", $fn_name);
        let tracing_span = ::tracing::trace_span!(FULL_NAME);
        let _enter_span = tracing_span.enter();
        let arguments = format!($format, $( $value ),+);
        ::tracing::trace!("{FULL_NAME}({arguments})");
    };
}

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This trait denotes a generator that writes to a file path.
///
/// This trait is a subset of the trait [`GenerateToWriter`], it is not however a sub-, or super-,
/// type as the need for this trait is for generators that are not able to process intermediate
/// results.
///
/// The type parameter `F` is used to describe any format information required by the generator.
///
pub trait GenerateToFile<F: Default + Debug>: Debug {
    ///
    /// Generate from the given module into the provided file path. This method uses the
    /// default value of the format type `F`.
    ///
    fn write_to_file(
        &mut self,
        module: &Module,
        cache: &ModuleCache,
        path: &Path,
    ) -> Result<(), Error> {
        trace_entry!(
            "GenerateToFile",
            "write_to_file" =>
                "module: {}, cache: {}, path: {:?}",
            module.name(),
            cache_to_string(cache),
            path
        );
        self.write_to_file_in_format(module, cache, path, F::default())
    }

    ///
    /// Generate from the given module, in the requested format, into the provided file path.
    ///
    fn write_to_file_in_format(
        &mut self,
        module: &Module,
        cache: &ModuleCache,
        path: &Path,
        format: F,
    ) -> Result<(), Error>;
}

///
/// This trait denotes a generator that writes to an implementation of [Write].
///
/// This trait is a superset of the trait [`GenerateToFile`],  see that trait's documentation for
/// more information.
///
/// The type parameter `F` is used to describe any format information required by the generator.
///
pub trait GenerateToWriter<F: Default + Debug>: Debug {

    // --------------------------------------------------------------------------------------------
    // Write to ❱ implementation of `Write`
    // --------------------------------------------------------------------------------------------

    ///
    /// Generate from the given module into the provided writer. This method uses the
    /// default value of the format type `F`.
    ///
    fn write<W>(
        &mut self,
        module: &Module,
        cache: &ModuleCache,
        writer: &mut W,
    ) -> Result<(), Error>
    where
        W: Write + Sized,
    {
        trace_entry!(
            "GenerateToWriter",
            "write" =>
                "module: {}, cache: {}, ...",
            module.name(),
            cache_to_string(cache)
        );
        self.write_in_format(module, cache, writer, F::default())
    }

    ///
    /// Generate from the given module, in the requested format, into the provided writer.
    ///
    fn write_in_format<W>(
        &mut self,
        module: &Module,
        cache: &ModuleCache,
        writer: &mut W,
        format: F,
    ) -> Result<(), Error>
    where
        W: Write + Sized;

    // --------------------------------------------------------------------------------------------
    // Write to ❱ String
    // --------------------------------------------------------------------------------------------

    ///
    /// Generate from the given module into a string. This method uses the
    /// default value of the format type `F`.
    ///
    fn write_to_string(&mut self, module: &Module, cache: &ModuleCache) -> Result<String, Error> {
        trace_entry!(
            "GenerateToWriter",
            "write_to_string" =>
                "module: {}, cache: {}",
            module.name(),
            cache_to_string(cache)
        );
        self.write_to_string_in_format(module, cache, F::default())
    }

    ///
    /// Generate from the given module, in the requested format, into a string.
    ///
    fn write_to_string_in_format(
        &mut self,
        module: &Module,
        cache: &ModuleCache,
        format: F,
    ) -> Result<String, Error> {
        trace_entry!(
            "GenerateToWriter",
            "write_to_string_in_format" =>
                "module: {}, cache: {}, format: {:?}",
            module.name(),
            cache_to_string(cache),
            format
        );
        let mut buffer = Cursor::new(Vec::new());
        self.write(module, cache, &mut buffer)?;
        Ok(String::from_utf8(buffer.into_inner())?)
    }

    // --------------------------------------------------------------------------------------------
    // Write to ❱ File
    // --------------------------------------------------------------------------------------------

    ///
    /// Generate from the given module into the provided file path. This method uses the
    /// default value of the format type `F`.
    ///
    fn write_to_file(
        &mut self,
        module: &Module,
        cache: &ModuleCache,
        path: &Path,
    ) -> Result<(), Error> {
        trace_entry!(
            "GenerateToWriter",
            "write_to_file" =>
                "module: {}, cache: {}, path: {:?}",
            module.name(),
            cache_to_string(cache),
            path
        );
        self.write_to_file_in_format(module, cache, path, F::default())
    }

    ///
    /// Generate from the given module, in the requested format, into the provided file path.
    ///
    fn write_to_file_in_format(
        &mut self,
        module: &Module,
        cache: &ModuleCache,
        path: &Path,
        format: F,
    ) -> Result<(), Error> {
        trace_entry!(
            "GenerateToWriter",
            "write_to_file_in_format" =>
                "module: {}, cache: {}, path: {:?}, format: {:?}",
            module.name(),
            cache_to_string(cache),
            path,
            format
        );
        let mut file = File::create(path)?;
        self.write_in_format(module, cache, &mut file, format)?;
        Ok(())
    }
}

///
/// A type that may be used when no format options are required by a generator implementation.
///
#[derive(Clone, Copy, Debug, Default)]
pub struct NoFormatOptions {}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn cache_to_string(cache: &ModuleCache) -> String {
    format!(
        "[{}]",
        cache
            .iter()
            .map(|module| module.name().to_string())
            .collect::<Vec<_>>()
            .join(", ")
    )
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[macro_use]
mod macros;

mod exec;

pub mod actions;

pub mod convert;

pub mod draw;
