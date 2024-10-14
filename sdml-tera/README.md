# Package sdml-tera

Rust integration between the SDML core model and the Tera template engine.

[![Crates.io](https://img.shields.io/crates/v/sdml_tera.svg)](https://crates.io/crates/sdml_tera)
[![Docs.rs](https://img.shields.io/docsrs/sdml-tera.svg)](https://docs.rs/sdml_tera)

This package is part of the Rust SDML project and specifically implements an
integration between the SDML core model and the Tera template engine to allow
template-based generation from one or more SDML modules. The project's intent is
to provide an idiomatic implementation of the in-memory model, parser,
generators, and the CLI tool.

It is planned for a more complete version of the CLI functionality to be
integrated into the overall SDML CLI at some point.

The following figure demonstrates this package in the broader project context.

![Package Overview](https://raw.githubusercontent.com/sdm-lang/rust-sdml/refs/heads/main/doc/overview-tera.png)

# Example

We wish to produce an output such as the following, a bulleted outline of a
module.

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
{%- if def.identity -%}
    * identity {{ self::member(item=def.identity) }}
{%- endif -%}
{%- if def.members -%}
{% for member in def.members -%}
    * {{ self::member(item=member) }}
{%- endfor -%}
{%- endif -%}
{%- elif def.__type == "enum" -%}
{% for var in def.variants %}    * {{ var.name }}
{% endfor -%}
{% elif def.__type == "event" -%}
{%- if def.members -%}
{% for member in def.members -%}
    * {{ self::member(item=member) }}
{%- endfor -%}
{%- endif -%}
{% elif def.__type == "structure" -%}
{%- if def.identity -%}
  * identity {{ self::member(item=def.identity) }}
{%- endif -%}
{%- if def.members -%}
{% for member in def.members -%}
    * {{ self::member(item=member) }}
{%- endfor -%}
{%- endif -%}
{%- elif def.__type == "union" -%}
{% for var in def.variants -%}
    * {% if var.rename %}{{ var.rename }} ({{ var.name }})
{%- else %}{{ var.name }}
{%- endif %}
{% endfor -%}
{% endif -%}
{% endfor %}
```

Once we have finished testing using the `sdml-tera` tool we can write the
following code to render any module with the template above.

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

# Changes

## Version 0.1.0

Initial release.
