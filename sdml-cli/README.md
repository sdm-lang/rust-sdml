# Crate sdlm_cli

Rust CLI for the Simple Domain Modeling Language (SDML).

[![crates.io](https://img.shields.io/crates/v/sdml_cli.svg)](https://crates.io/crates/sdml_cli)
[![docs.rs](https://docs.rs/sdml_cli/badge.svg)](https://docs.rs/sdml_cli)

This package is part of the Rust SDML project and specifically implements the `sdml` command-line interface (CLI).
The project's intent is to provide an idiomatic implementation of the in-memory model, parser, generators, and the CLI tool.

The following figure demonstrates this package in the broader project context.

```
                            ╭───────╮
                            │  CLI  │
                       ╔══  │ crate │  ══╗
                       ║    ╰───────╯    ║
┌╌╌╌╌╌╌╌╌┐             V                 V
┆        ┆       ╭──────────╮       ╭──────────╮       Formatted Source
┆ source ┆  ══>  │  parse   │  ══>  │ generate │  ══>  RDF Representation 
┆  file  ┆    ╭──│  crate   │───────│   crate  │──╮    Documentation
┆        ┆    │  ╰──────────╯       ╰──────────╯  │    Diagrams
└╌╌╌╌╌╌╌╌┘    │             core crate            │──╮
              ╰───────────────────────────────────╯  │
 ┌───────┐             ⋀          error crate        │
 │ other │             ║  ╌╌╌╌╌╌╌╌╌╌╌╌╌╌─────────────╯
 │ tools │  ═══════════╝
 └───────┘
```


## Installation

Installation of command-line interface is via the `cargo` command. Cargo is usually installed with the Rust toolchain
using [rustup](https://rustup.rs/).

The following command should download and build the tool, and will also work to install any updates.

```
❯ cargo install sdml-cli
```

Cargo will sometimes report that you have the latest version installed, to be sure you can force it to install
regardless with the `--force` option.

```
❯ cargo install sdml-cli --force
```

You can check that you have the tool installed and on the path with the following check.

```
❯ sdml versions               
SDML CLI:        0.2.7
SDML grammar:    0.2.16
Tree-Sitter ABI: 14
```

## Global Options

Certain command-line options act on all commands, these must appear before the command. The SDML tool has a log-filter
and a no-color global option.

The set of packages making up `rust-sdml` all have extensive logging which can be enabled when running the tool. The
global argument `--log-filter` takes a log level and displays any log event with a severity greater than, or equal to,
the filter.

```
❯ sdml --log-filter tracing versions
2024-02-20T19:06:53.141741Z  INFO sdml: Log level set to `LevelFilter::Tracing`
2024-02-20T19:06:53.141877Z TRACE sdml: Commands::execute self: Versions
SDML CLI:        0.2.7
SDML grammar:    0.2.16
Tree-Sitter ABI: 14
```

Some of the commands will, by default, use colored output which can be a problem if you save a file for future
processing as the control characters play havoc with diff tools for example. 

```
❯ sdml --no-color versions
❯ NO_COLOR=1  sdml versions
❯ CLI_COLOR=0 sdml versions
```

## Commands

Input Files

### Getting Help

```
❯ sdml --help
Rust CLI for Simple Domain Modeling Language (SDML)

Usage: sdml [OPTIONS] <COMMAND>

Commands:
  convert    Convert module into alternate representations
  draw       Draw diagrams from a module
  deps       Show module dependencies
  doc        Document a module
  highlight  Syntax highlight a module source
  tags       Extract tags from a module
  validate   Validate a module
  versions   Show tool and library versions
  view       View formatted module source code
  help       Print this message or the help of the given subcommand(s)

Options:
      --log-filter <LOG_FILTER>
          Level of logging to enable
          
          [default: none]

          Possible values:
          - none:        Turn off all logging
          - errors:      Enable error logging only
          - warnings:    Enable warnings and above
          - information: Enable information and above
          - debugging:   Enable debugging and above
          - tracing:     Enable tracing (ALL) and above

      --no-color
          Turn off color for code emitters
          
          [env: NO_COLOR=]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

### Representation Conversion

Output Format: json, json-pretty, rdf, s-expressions (Lispy)

### Diagram Generation

TBD

```
❯ sdml draw --diagram concepts --output-format svg -i example/example.sdm
❯ open -a Safari example.svg
```


```
❯ sdml draw --diagram entity-relationship --output-format source -i example/example.sdm
```


```
❯ sdml draw --diagram uml-class --output-format svg -i example/example.sdm
❯ open -a Safari example.svg
```

### Dependency Visualization

This command (`deps`) allows you to view the transitive dependencies of a specific module. The `--output-format` option may be one of
`graph`, `rdf`, or `tree`; the default is `tree`.

```
❯ sdml deps sdml
sdml
├── owl
│   ├── rdf
│   │   └── rdfs
│   │       └── rdf
│   ├── rdfs
│   └── xsd
│       ├── rdf
│       └── rdfs
├── rdf
├── rdfs
├── skos
│   ├── rdf
│   └── rdfs
└── xsd
```

In some cases the entire set of dependencies is not necessary and the `--depth` argument can be added to only show a
number of levels of import from the root. 

```
❯ sdml deps --depth 1 sdml
sdml
├── owl
├── rdf
├── rdfs
├── skos
└── xsd
```

The `rdf` output format dumps raw N-Triples with OWL import statements for each module import.

```
❯ sdml deps --depth 1 --output-format rdf sdml
<http://sdml.io/sdml-owl.ttl#> <http://www.w3.org/2002/07/owl#imports> <http://www.w3.org/2002/07/owl#> .
<http://sdml.io/sdml-owl.ttl#> <http://www.w3.org/2002/07/owl#imports> <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
<http://sdml.io/sdml-owl.ttl#> <http://www.w3.org/2002/07/owl#imports> <http://www.w3.org/2000/01/rdf-schema#> .
<http://sdml.io/sdml-owl.ttl#> <http://www.w3.org/2002/07/owl#imports> <http://www.w3.org/2004/02/skos/core#> .
<http://sdml.io/sdml-owl.ttl#> <http://www.w3.org/2002/07/owl#imports> <http://www.w3.org/2001/XMLSchema#> .
```

The `graph` output format outputs an SVG representation of the dependency graph.

```
❯ sdml deps --output-format graph sdml > sdml-deps.svg
❯ open -a Safari sdml-deps.svg
```

![example dependency graph](https://raw.githubusercontent.com/sdm-lang/rust-sdml/main/sdml-generate/doc/example_deps_graph.svg)

### Document Generation

The documentation command (`doc`) generates documentation in `--output-format` Markdown (`markdown`) or Emacs Org-mode (`org-mode`).
The generated documentation will cover all elements in the module only, although an appendix with the module's
dependency graph is included. Additional appendices have the original source as well as the RDF representation of the
module.

### Module Highlighting

TBD

### XRef Tag Generation

TBD

### Validation

The validation command (`validate`) runs not only error checks on a module, but it's transitively loaded dependencies.
By default the command only shows diagnostics with severity `bug` and `error`, but `warning`, `notes`, and `help` can be
output with the `--level` argument. This argument also takes the values `none` and `all`.


```
❯ sdml validate --level all -i examples/errors/i0506.sdm
note[I0506]: identifier not using preferred casing
  ┌─ examples/errors/i0506.sdm:1:8
  │
1 │ module Example <https://example.com/api> is
  │        ^^^^^^^ this identifier
  │
  = expected snake case (snake_case)
  = help: for more details, see <https://sdml.io/errors/#I0506>

note[I0506]: identifier not using preferred casing
  ┌─ examples/errors/i0506.sdm:3:13
  │
3 │   structure access_record is
  │             ^^^^^^^^^^^^^ this identifier
  │
  = expected upper camel case (UpperCamelCase)
  = help: for more details, see <https://sdml.io/errors/#I0506>
```

### Version Information

This command shows more information than the simple `--version` global argument and is useful for debugging.

```
❯ sdml versions               
SDML CLI:        0.2.7
SDML grammar:    0.2.16
Tree-Sitter ABI: 14
```

### Module Viewer

The module viewer (`view`) command may not seem exciting at first, it displays a highlighted copy of a file:

```
❯ sdml view -i examples/example.sdm 
module example <https://example.com/api> is

  import [ dc xsd ]

  datatype Uuid <- sdml:string is
    @xsd:pattern = "[0-9a-f]{8}-([0-9a-f]{4}-){3}[0-9a-f]{12}"
  end

  entity Example is
    version -> Uuid
    name -> sdml:string is
      @dc:description = "the name of this thing"@en
    end
  end

end
```

The `--level` argument can be used to elide content and get an overview of a module. The `definitions` value will only
show top-level definitions and any that had bodies previously will be followed by the string `";; ..."`.

```
❯ sdml view --level definitions -i examples/example.sdm
module example <https://example.com/api> is

  import [ dc xsd ]

  datatype Uuid <- sdml:string ;; ...

  entity Example ;; ...

end
```

To see a little more, the `members` value will similarly show the members of product types and variants of sum types but
not their bodies if present.

```
❯ sdml view --level members -i examples/example.sdm
module example <https://example.com/api> is

  import [ dc xsd ]

  datatype Uuid <- sdml:string ;; ...

  entity Example is
    version -> Uuid
    name -> sdml:string ;; ...
  end

end
```

## Changes

**Version 0.2.7**

* Feature: better error handling in conjunction with the validation and diagnostics in `sdml_error`.

**Version 0.2.6**

* Build: update dependencies.

**Version 0.2.5**

* Feature: Add new `--no-color` flag to the CLI which also uses the `NO_COLOR` environment variable.
* Feature: Removed indirect dependencies from Cargo.toml.
* Update: New generator features for colored RDF.

**Version 0.2.4**

* Feature: Add new `source` command to call the new source generator.
* Fix: Change the description of `depth` parameter for `deps` command, 0 is the default which means all depths are
  included in the output.
* Update: Use new generator traits that require a module cache parameter.

**Version 0.2.3**

* Feature: add new stdlib modules with standard layout.
* Feature: minor refactor of cache and loader.

**Version 0.2.2**

* Feature: Update to latest grammar for version URIs and RDF definitions.
  * Add support for base URI on modules
  * Add support for version info and URI on modules
  * Add support for version URI on module import
  * Parse RDF definitions for classes and properties.

**Version 0.2.1**

* Feature: Remove member groups.

**Version 0.2.0**

* Feature: Update to latest grammar.
  * Remove Value Variant numeric values
  * Update formal constraints
  * Add type classes

**Version 0.1.6**

* Updated dependencies

**Version 0.1.5**

Initial stand-alone crate.

**Version 0.1.4**

Previously part of a single crate [sdml](https://crates.io/crates/sdml).
