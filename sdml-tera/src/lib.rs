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

use context::module_to_value;
use sdml_core::{model::modules::Module, store::ModuleStore};
use std::{fs::OpenOptions, io::Write, path::Path};
use tera::{Context, Map, Tera, Value};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

#[inline]
pub fn make_engine_from(glob: &str) -> Result<Tera, error::Error> {
    let engine = Tera::new(glob)?;
    Ok(engine)
}

// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

pub fn render_module(
    engine: &Tera,
    module: &Module,
    cache: &impl ModuleStore,
    context: Option<Context>,
    template_name: &str,
) -> Result<String, error::Error> {
    let (_, value) = module_to_value(module, cache);

    let mut context = if let Some(context) = context {
        context
    } else {
        Context::default()
    };
    context.insert("module", &value);

    let result = engine.render(template_name, &context)?;
    Ok(result)
}

pub fn render_module_to<W: Write>(
    engine: &Tera,
    module: &Module,
    cache: &impl ModuleStore,
    context: Option<Context>,
    template_name: &str,
    w: &mut W,
) -> Result<(), error::Error> {
    let (_, value) = module_to_value(module, cache);

    let mut context = if let Some(context) = context {
        context
    } else {
        Context::default()
    };
    context.insert("module", &value);

    engine.render_to(template_name, &context, w)?;
    Ok(())
}

pub fn render_module_to_file<P: AsRef<Path>>(
    engine: &Tera,
    module: &Module,
    cache: &impl ModuleStore,
    context: Option<Context>,
    template_name: &str,
    path: P,
) -> Result<(), error::Error> {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path.as_ref())?;

    render_module_to(engine, module, cache, context, template_name, &mut file)?;
    Ok(())
}

// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

pub fn render_modules(
    engine: &Tera,
    modules: Vec<&Module>,
    cache: &impl ModuleStore,
    context: Option<Context>,
    template_name: &str,
) -> Result<String, error::Error> {
    let values: Map<String, Value> = modules
        .iter()
        .map(|module| module_to_value(module, cache))
        .collect();

    let mut context = if let Some(context) = context {
        context
    } else {
        Context::default()
    };
    context.insert("modules", &values);

    let result = engine.render(template_name, &context)?;
    Ok(result)
}

pub fn render_modules_to<W: Write>(
    engine: &Tera,
    modules: Vec<&Module>,
    cache: &impl ModuleStore,
    context: Option<Context>,
    template_name: &str,
    w: &mut W,
) -> Result<(), error::Error> {
    let values: Map<String, Value> = modules
        .iter()
        .map(|module| module_to_value(module, cache))
        .collect();

    let mut context = if let Some(context) = context {
        context
    } else {
        Context::default()
    };
    context.insert("modules", &values);

    engine.render_to(template_name, &context, w)?;
    Ok(())
}

pub fn render_modules_to_file<P: AsRef<Path>>(
    engine: &Tera,
    modules: Vec<&Module>,
    cache: &impl ModuleStore,
    context: Option<Context>,
    template_name: &str,
    path: P,
) -> Result<(), error::Error> {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path.as_ref())?;
    render_modules_to(engine, modules, cache, context, template_name, &mut file)?;
    Ok(())
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod context;

pub mod error;
