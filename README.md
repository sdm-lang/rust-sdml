# Rust SDML

![SDML Logo](https://raw.githubusercontent.com/sdm-lang/.github/main/profile/horizontal-text.svg)

Rust Library and Tools for the Simple Domain Modeling Language (SDML).

[![License-Apache_2.0]([https://img.shields.io/badge/License-Apache_2.0-blue.svg])](https://opensource.org/licenses/Apache-2.0)
[![Rust Workflow](https://github.com/johnstonskj/rust-sdml/actions/workflows/rust.yml/badge.svg)](https://github.com/johnstonskj/rust-sdml/actions/workflows/rust.yml)
[![Security Audit Workflow](https://github.com/johnstonskj/rust-sdml/actions/workflows/security-audit.yml/badge.svg)](https://github.com/johnstonskj/rust-sdml/actions/workflows/security-audit.yml)
[![Coverage Status](https://codecov.io/gh/johnstonskj/rust-sdml/branch/main/graph/badge.svg?token=1HGN6M4KIT)](https://codecov.io/gh/johnstonskj/rust-sdml)
[![Stargazer Count](https://img.shields.io/github/stars/johnstonskj/rust-sdml.svg)](https://github.com/johnstonskj/rust-sdml/stargazers)

This project's intent is to provide an idiomatic implementation of the in-memory
model, parser, generators, and the CLI tool. The following figure shows the
usage and relationships of the packages in this workspace.

To install the command-line tool on MacOS or Linux use the Homebrew package
manager and the SDML Tap.

``` bash
❯ brew install sdm-lang/sdml/sdml
```

# SDML Crates

## Crate sdml-cli

This [package](./sdml-cli/README.md) contains the entry-point for the
command-line tool.

[![crates.io](https://img.shields.io/crates/v/sdml_cli.svg)](https://crates.io/crates/sdml_cli)

## Crate core

This [package](./sdml-core/README.md) contains the in-memory model of an sdml
module, the major component of the project itself. It also has traits
(`ModuleLoader`, `ModuleResolver`, `ModuleStore`) that are required by packages
that follow.
                                                            
## Crate sdml-document

TBD

[![crates.io](https://img.shields.io/crates/v/sdml_document.svg)](https://crates.io/crates/sdml_document)
[![docs.rs](https://docs.rs/sdml_document/badge.svg)](https://docs.rs/sdml_document)

## Crate sdml-draw

TBD

[![crates.io](https://img.shields.io/crates/v/sdml_draw.svg)](https://crates.io/crates/sdml_draw)
[![docs.rs](https://docs.rs/sdml_draw/badge.svg)](https://docs.rs/sdml_draw)

## Crate sdml-errors

This [package](./sdml-errors/README.md) contains the common `Error` type as well
as a diagnostic set for reporting language parse and model issues.

[![crates.io](https://img.shields.io/crates/v/sdml_errors.svg)](https://crates.io/crates/sdml_errors)
[![docs.rs](https://docs.rs/sdml_errors/badge.svg)](https://docs.rs/sdml_errors)

## Crate sdml-generate

This [package](./sdml-generate/README.md) contains a set of tools for generating
alternative representations of an in-memory module as well as related actions
for the CLI.

[![crates.io](https://img.shields.io/crates/v/sdml_generate.svg)](https://crates.io/crates/sdml_generate)
[![docs.rs](https://docs.rs/sdml_generate/badge.svg)](https://docs.rs/sdml_generate)

## Crate sdml-json

TBD

[![crates.io](https://img.shields.io/crates/v/sdml_json.svg)](https://crates.io/crates/sdml_json)
[![docs.rs](https://docs.rs/sdml_json/badge.svg)](https://docs.rs/sdml_json)

## Crate sdml-parse

This [package](./sdml-parse/README.md) contains the bridge from the
tree-sitter SDML library to the in-memory model in `sdml_corez.

[![crates.io](https://img.shields.io/crates/v/sdml_parse.svg)](https://crates.io/crates/sdml_parse)
[![docs.rs](https://docs.rs/sdml_parse/badge.svg)](https://docs.rs/sdml_parse)

## Crate sdml-src

TBD

[![crates.io](https://img.shields.io/crates/v/sdml_src.svg)](https://crates.io/crates/sdml_src)
[![docs.rs](https://docs.rs/sdml_src/badge.svg)](https://docs.rs/sdml_src)

## Crate sdml-tera

TBD

[![crates.io](https://img.shields.io/crates/v/sdml_tera.svg)](https://crates.io/crates/sdml_tera)
[![docs.rs](https://docs.rs/sdml_tera/badge.svg)](https://docs.rs/sdml_tera)

# License

This repository, and all contents, are released under the Apache License,
Version 2.0.

For information on contributing, see [How to
Contribute](./doc/contributing.org), and the [Code of Conduct](./doc/code_of_conduct.org).

# Changes

After version 0.1.4 the single crate has been replaced with the four
`sdml-core`, `sdml-errors`, `sdml-parse`, `sdml-generate`, and `sdml-cli`. Each
will have it's own version history starting with *0.1.5*.

## Version: 0.1.4

* Support the latest grammar
* UML Class Diagram (initial)
* Modeling Library modules

## Version: 0.1.3

* Support the latest grammar

## Version: 0.1.2

* Syntax highlighting supported
* Support the latest grammar

## Version: 0.1.1

* More drawing details
* More command-line features
* Support the latest grammar

## Version: 0.1.0

* Initial version, limited to basic drawings

# Other Links

## Formatting

* [numtide/treefmt](https://github.com/numtide/treefmt/wiki)
* [tweag/topiary](https://github.com/tweag/topiary)
* [melpa/format-all](https://melpa.org/#/format-all)

## Linting

* [github/super-linter](https://github.com/github/super-linter)
