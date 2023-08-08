# Crate sdlm_core

Rust in-Memory model of the Simple Domain Modeling Language (SDML).

[![crates.io](https://img.shields.io/crates/v/sdml_core.svg)](https://crates.io/crates/sdml_core)
[![docs.rs](https://docs.rs/sdml_core/badge.svg)](https://docs.rs/sdml_core)

## Changes

**Version 0.1.9**

* Run Cargo format and clippy

**Version 0.1.8**

* Remove most macros from the model.
* Made the name for constraints required, not `Option`.

**Version 0.1.7**

* Minor fixes

**Version 0.1.6**

* Updated parser with grammar changes in tree-sitter-sdml v0.1.29

**Version 0.1.6**

* Updated parser with grammar changes in tree-sitter-sdml v0.1.26

**Version 0.1.5**

* Created a `stdlib` module and moved all the SDML and relevant RDF files into it.
* Updated model to the same level as `tree-sitter-sdml` version `0.1.21`.
* Updated `tree-sitter-sdml` dependency with updated constraints.
  * Renamed `TypeDefinition` to `Definition` to address the fact that property definitions aren't types.
  * Renamed `EnumVariant` to `ValueVariant` to align with `TypeVariant` on unions. This required change to walker methods.

**Version 0.1.4**

Previously part of a single crate [sdml](https://crates.io/crates/sdml).
