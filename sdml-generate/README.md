# Crate sdlm_generate

Rust Library of artifact generators from the Simple Domain Modeling Language (SDML).

[![crates.io](https://img.shields.io/crates/v/sdml_generate.svg)](https://crates.io/crates/sdml_generate)
[![docs.rs](https://docs.rs/sdml_generate/badge.svg)](https://docs.rs/sdml_generate)

This package is part of the Rust SDML project and specifically defines the model-to-*other* generators for SDML modules.
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

## Changes

**Version 0.2.4**

* Feature: add new stdlib modules with standard layout.
* Feature: minor refactor of cache and loader.

**Version 0.2.3**

* Feature: Update to latest grammar for version URIs and RDF definitions.
  * Add support for base URI on modules
  * Add support for version info and URI on modules
  * Add support for version URI on module import
  * Parse RDF definitions for classes and properties.

**Version 0.2.2**

* Feature: Remove member groups.

**Version 0.2.1**

* Fix: replace "-" with "__" as qualified identifier replacement

**Version 0.2.0**

* Feature: Update to latest grammar.
  * Remove Value Variant numeric values
  * Update formal constraints
  * Add type classes

**Version 0.1.8**

* Feature: Add mapping type to the *s-expr* and *UML* generators.
* Build: Update to latest `tree-sitter-sdml` to pick up changes in highlighting.
* Build: Update to latest `sdml-core` to pick up changes in `Cardinality::to_uml_string`.

**Version 0.1.7**

* Build: Update with recent model changes and fixes.
* Fix: Clean-up the UML output.

**Version 0.1.6**

* Feature: Updated org-mode and UML generators with a number of model changes from sdml-core.

**Version 0.1.5**

* Feature: Updated with a number of model changes from sdml-core.

**Version 0.1.4**

Previously part of a single crate [sdml](https://crates.io/crates/sdml).
