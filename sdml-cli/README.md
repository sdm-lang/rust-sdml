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

## Commands

Input Files

Logging

Color

### Getting Help

```sh
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

### Dependency Visualization

Output Format: graph, rdf, tree

Depth: ...

```sh
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

```sh
❯ sdml deps --depth 1 sdml
sdml
├── owl
├── rdf
├── rdfs
├── skos
└── xsd
```

```sh
❯ sdml deps --depth 1 --output-format rdf sdml
<http://sdml.io/sdml-owl.ttl#> <http://www.w3.org/2002/07/owl#imports> <http://www.w3.org/2002/07/owl#> .
<http://sdml.io/sdml-owl.ttl#> <http://www.w3.org/2002/07/owl#imports> <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
<http://sdml.io/sdml-owl.ttl#> <http://www.w3.org/2002/07/owl#imports> <http://www.w3.org/2000/01/rdf-schema#> .
<http://sdml.io/sdml-owl.ttl#> <http://www.w3.org/2002/07/owl#imports> <http://www.w3.org/2004/02/skos/core#> .
<http://sdml.io/sdml-owl.ttl#> <http://www.w3.org/2002/07/owl#imports> <http://www.w3.org/2001/XMLSchema#> .
```

```sh
❯ sdml deps --output-format graph sdml > sdml-deps.svg
❯ open -a Safari sdml-deps.svg
```

![example dependency graph](https://raw.githubusercontent.com/sdm-lang/rust-sdml/main/sdml-generate/doc/example_deps_graph.svg)

### Document Generation

Output Format: markdown, org-mode

### Validation

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
