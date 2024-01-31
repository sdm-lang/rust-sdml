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
┌╌╌╌╌╌╌╌╌┐          V                 V
┆        ┆       ╭───────╮       ╭──────────╮       Formatted Source
┆ source ┆  ══>  │ parse │  ══>  │ generate │  ══>  RDF Representation 
┆  file  ┆   ╭───│ crate │───────│   crate  │───╮   Documentation
┆        ┆   │   ╰───────╯       ╰──────────╯   │   Diagrams
└╌╌╌╌╌╌╌╌┘   │           core crate             │
             ╰──────────────────────────────────╯
 ┌───────┐                  ⋀
 │ other │                  ║
 │ tools │  ════════════════╝
 └───────┘
```

## 
## Changes

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
