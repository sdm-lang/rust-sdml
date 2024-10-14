/*!
Provides integration that allows document generation from SDML modules using the
[Tera](https://keats.github.io/tera/docs/) template engine.

This package provides a set of *rendering* functions as well as a set of *context* functions. By
default all render functions will create new context value using the [`module_to_value`] function
to convert a `Module` into a context object. However, you may provide your own context to add custom
values.

The documentation for the [`context`] module describes the simplifications made in the creation of
the context object(s).

# Example

We wish to produce an output such as the following, a bulleted outline of a module.

```markdown
# Module `campaign` Outline

* **campaign** (Module)
  * **Name** <- *xsd:string* (Datatype)
  * **CampaignId** <- *xsd:string* (Datatype)
  * **State** (Enum)
    * Running
    * Paused
    * error
  * **Tag** (Structure)
    * key -> *xsd:NMTOKEN*
    * value -> *rdf:langString*
  * **Ad** (Entity)
  * **AdGroup** (Entity)
  * **Campaign** (Entity)
    * identity campaignId -> *CampaignId*
    * name -> *unknown*
    * tag -> *Tag*
    * target -> *TargetCriteria*
  * **AudienceTarget** (Entity)
  * **GeographicTarget** (Entity)
  * **TargetCriteria** (Union)
    * Audience (Audience)
    * Geographic (Geographic)
```

To do this we create a file named `outline.md` with the following content.

```markdown
{% macro member(item) %}
{%- if item.__type == "reference" -%}
*{{ item.type_ref }}*
{% elif item.__type == "definition" -%}
{{ item.name }} -> *{{ item.type_ref }}*
{% endif -%}
{% endmacro member %}

# Module `{{ module.name }}` Outline

* **{{ module.name }}** (Module)
{% for def in module.definitions %}  * **{{ def.name }}**
{%- if def.__type == "datatype" %} <- *{{ def.base_type }}*
{%- endif %} ({{ def.__type | capitalize | replace(from="-", to=" ") }})
{% if def.__type == "entity" -%}
{%- if def.identity %}    * identity {{ self::member(item=def.identity) }}
{%- endif -%}
{%- if def.members -%}
{% for member in def.members %}    * {{ self::member(item=member) }}
{%- endfor -%}
{%- endif -%}
{%- elif def.__type == "enum" -%}
{% for var in def.variants %}    * {{ var.name }}
{% endfor -%}
{% elif def.__type == "event" -%}
{%- if def.members -%}
{% for member in def.members %}    * {{ self::member(item=member) }}
{%- endfor -%}
{%- endif -%}
{% elif def.__type == "structure" -%}
{%- if def.identity %}  * identity {{ self::member(item=def.identity) }}
{%- endif -%}
{%- if def.members -%}
{% for member in def.members %}    * {{ self::member(item=member) }}
{%- endfor -%}
{%- endif -%}
{%- elif def.__type == "union" -%}
{% for var in def.variants %}    * {% if var.rename %}{{ var.rename }} ({{ var.name }})
{%- else %}{{ var.name }}
{%- endif %}
{% endfor -%}
{% endif -%}
{% endfor %}
```

Once we have finished testing using the `sdml-tera` tool we can write the following code to render
any module with the template above.

```rust
use sdml_core::model::modules::Module;
use sdml_core::store::ModuleStore;
use sdml_tera::make_engine_from;
use sdml_tera::render_module;

fn print_module(module: &Module, cache: &impl ModuleStore) {

    let engine = make_engine_from("tests/templates/**/*.md")
        .expect("Could not parse template files");


    let rendered = render_module(&engine, module, cache, None, "outline.md")
        .expect("Issue in template rendering");

    println!("{}", rendered);
}
```

# Features

This crate also has a binary that allows you to test the development of templates. The tool takes a
glob expression for Tera to load templates and a specific template name to use for a specific test.
The input/output allows for file read/write and stdin/stdout, or for input you can specify a module
name for the standard resolver to find for you.

```bash
❯ sdml-tera --help
Simple Domain Modeling Language (SDML) Tera Integration

Usage: sdml-tera [OPTIONS] --template-name <TEMPLATE_NAME> [MODULE]

Arguments:
  [MODULE]  SDML module, loaded using the standard resolver

Options:
  -o, --output <OUTPUT>                File name to write to, or '-' to write to stdout [default: -]
  -i, --input <INPUT>                  Input SDML file name to read from, or '-' to read from stdin [default: -]
  -g, --template-glob <TEMPLATE_GLOB>  [default: templates/**/*.md]
  -n, --template-name <TEMPLATE_NAME>
  -h, --help                           Print help
  -V, --version                        Print version
```

The error messages produced by the tool are also verbose to help as much as possible to diagnose
issues as you develop templates. For example, the following shows the output when a syntax error is
found in a template.

```bash
An error occurred creating the Tera engine; most likely this is a syntax error in one of your templates.
Error: A template error occurred; source:
* Failed to parse "/Users/simonjo/Projects/sdm-lang/rust-sdml/sdml-tera/tests/templates/module.md"
  --> 35:25
   |
35 | event {{ definition.name$ }} source {{ definition.source }}
   |                         ^---
   |
   = expected `or`, `and`, `not`, `<=`, `>=`, `<`, `>`, `==`, `!=`, `+`, `-`, `*`, `/`, `%`, a filter, or a variable end (`}}`)
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

use context::module_to_value;
use sdml_core::{model::modules::Module, store::ModuleStore};
use sdml_errors::Error;
use std::{fs::OpenOptions, io::Write, path::Path};
use tera::{Context, Map, Tera, Value};

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions > Engine
// ------------------------------------------------------------------------------------------------

///
/// A local wrapper around the Tera engine creation.
///
/// This function is introduced mainly to allow the integration with the SDML core error structure.
///
#[inline]
pub fn make_engine_from(glob: &str) -> Result<Tera, Error> {
    let engine = Tera::new(glob)?;
    Ok(engine)
}

// ------------------------------------------------------------------------------------------------
// Public Functions > Render Single Module
// ------------------------------------------------------------------------------------------------

///
/// Render `module`, with the template in the file `template_name`, and using `engine`.
///
/// If `context` is not specified a new blank object is created, and in either case a representation
/// of the module is added to the context object under the key `"module"`.
///
/// ```json
/// {
///     "module": {}
/// }
/// ```
///
/// In the case of this function the result is returned as a `String`.
///
pub fn render_module(
    engine: &Tera,
    module: &Module,
    cache: &impl ModuleStore,
    context: Option<Context>,
    template_name: &str,
) -> Result<String, Error> {
    let context = make_context_from(module, cache, context);
    let result = engine.render(template_name, &context)?;
    Ok(result)
}

///
/// Render `module`, with the template in the file `template_name`, and using `engine`.
///
/// If `context` is not specified a new blank object is created, and in either case a representation
/// of the module is added to the context object under the key `"module"`.
///
/// ```json
/// {
///     "module": {}
/// }
/// ```
///
/// In the case of this function the template is rendered to the write implementation `w`.
///
pub fn render_module_to<W: Write>(
    engine: &Tera,
    module: &Module,
    cache: &impl ModuleStore,
    context: Option<Context>,
    template_name: &str,
    w: &mut W,
) -> Result<(), Error> {
    let context = make_context_from(module, cache, context);
    engine.render_to(template_name, &context, w)?;
    Ok(())
}

///
/// Render `module`, with the template in the file `template_name`, and using `engine`.
///
/// If `context` is not specified a new blank object is created, and in either case a representation
/// of the module is added to the context object under the key `"module"`.
///
/// ```json
/// {
///     "module": {}
/// }
/// ```
///
/// In the case of this function the template is rendered to the file named by `path`.
///
pub fn render_module_to_file<P: AsRef<Path>>(
    engine: &Tera,
    module: &Module,
    cache: &impl ModuleStore,
    context: Option<Context>,
    template_name: &str,
    path: P,
) -> Result<(), Error> {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path.as_ref())?;

    render_module_to(engine, module, cache, context, template_name, &mut file)?;
    Ok(())
}

// ------------------------------------------------------------------------------------------------
// Public Functions > Render Set of Modules
// ------------------------------------------------------------------------------------------------

///
/// Render the set of `modules`, with the template in the file `template_name`, and using `engine`.
///
/// If `context` is not specified a new blank object is created, and in either case a map is created
/// under the key `"modules"` as a map from module name to module representation.
///
/// ```json
/// {
///     "modules": {
///         "Identifier": {},
///     }
/// }
/// ```
///
/// In the case of this function the result is returned as a `String`.
///
pub fn render_modules(
    engine: &Tera,
    modules: Vec<&Module>,
    cache: &impl ModuleStore,
    context: Option<Context>,
    template_name: &str,
) -> Result<String, Error> {
    let context = make_context_from_all(modules, cache, context);
    let result = engine.render(template_name, &context)?;
    Ok(result)
}

///
/// Render the set of `modules`, with the template in the file `template_name`, and using `engine`.
/// If `context` is not specified a new blank object is created, and in either case a map is created
/// under the key `"modules"` as a map from module name to module representation.
///
/// ```json
/// {
///     "modules": {
///         "Identifier": {},
///     }
/// }
/// ```
///
/// In the case of this function the template is rendered to the write implementation `w`.
///
pub fn render_modules_to<W: Write>(
    engine: &Tera,
    modules: Vec<&Module>,
    cache: &impl ModuleStore,
    context: Option<Context>,
    template_name: &str,
    w: &mut W,
) -> Result<(), Error> {
    let context = make_context_from_all(modules, cache, context);
    engine.render_to(template_name, &context, w)?;
    Ok(())
}

///
/// Render the set of `modules`, with the template in the file `template_name`, and using `engine`.
///
/// If `context` is not specified a new blank object is created, and in either case a map is created
/// under the key `"modules"` as a map from module name to module representation.
///
/// ```json
/// {
///     "modules": {
///         "Identifier": {},
///     }
/// }
/// ```
///
/// In the case of this function the template is rendered to the file named by `path`.
///
pub fn render_modules_to_file<P: AsRef<Path>>(
    engine: &Tera,
    modules: Vec<&Module>,
    cache: &impl ModuleStore,
    context: Option<Context>,
    template_name: &str,
    path: P,
) -> Result<(), Error> {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path.as_ref())?;
    render_modules_to(engine, modules, cache, context, template_name, &mut file)?;
    Ok(())
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn make_context_from(
    module: &Module,
    cache: &impl ModuleStore,
    context: Option<Context>,
) -> Context {
    let (_, value) = module_to_value(module, cache);

    let mut context = context.unwrap_or_default();
    context.insert("module", &value);
    context
}

fn make_context_from_all(
    modules: Vec<&Module>,
    cache: &impl ModuleStore,
    context: Option<Context>,
) -> Context {
    let values: Map<String, Value> = modules
        .iter()
        .map(|module| module_to_value(module, cache))
        .collect();

    let mut context = context.unwrap_or_default();
    context.insert("modules", &values);
    context
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod context;
