# Project SDML 

![SDML Logo Text](https://raw.githubusercontent.com/sdm-lang/.github/main/profile/horizontal-text.svg)

Rust Library and Tools for the Simple Domain Modeling Language (SDML).

![MIT License](https://img.shields.io/badge/license-mit-118811.svg)
[![Rust](https://github.com/johnstonskj/rust-sdml/actions/workflows/rust.yml/badge.svg)](https://github.com/johnstonskj/rust-sdml/actions/workflows/rust.yml)
[![Security audit](https://github.com/johnstonskj/rust-sdml/actions/workflows/security-audit.yml/badge.svg)](https://github.com/johnstonskj/rust-sdml/actions/workflows/security-audit.yml)
[![Codecov](https://codecov.io/gh/johnstonskj/rust-sdml/branch/main/graph/badge.svg?token=1HGN6M4KIT)](https://codecov.io/gh/johnstonskj/rust-sdml)
[![GitHub stars](https://img.shields.io/github/stars/johnstonskj/rust-sdml.svg)](https://github.com/johnstonskj/rust-sdml/stargazers)

This project's intent is to provide an idiomatic implementation of the in-memory model, parser, generators, and the CLI
tool.

The following figure shows the usage and relationships of the packages in this workspace.

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
## 
This is  a command-line tool that provides functionality to process
[SDML](https://sdml.io) files. The SDML site has more information on the command-line options and capabilities.

# Commands

For most commands the tool accepts a pair of arguments that determine the format and location of the generated output.
These commands `--output-format`/`-f` and `--output-file`/`-o` can be used in the following manner.

``` shell
--output-format svg --output-file rentals.svg
-output-file rentals.svg
```

In the second example above the format is not specified and so the tool will use the extension of the output file as the
format specifier. If the output file is not specified the output is written to stdout. If neither argument is provided
the output is written to stdout in the default format.

Similarly most tools use the module resolution rules for loading a module and so the input *file* is actually specified
as a module name. To allow for searches in non-standard locations the argument `--base-path`/`-b` can be used to
prepend a path to the standard search path. Thus the two examples below are identical as the current directory is always
a component of the search path.

``` shell
--base-path . rentals
rentals
```

Finally, if no module name is specified the tool will read from `stdin`.

## Status

| Command   | Format              | Status       |
|-----------|---------------------|--------------|
| convert   | s-expr              | **Complete** |
| convert   | rdf                 | Not started  |
| convert   | org                 | Incomplete   |
| deps      |                     | Not started  |
| draw      | concepts            | **Complete** |
| draw      | entity-relationship | **Complete** |
| draw      | uml-class           | Not started  |
| highlight | ansi                | **Complete** |
| highlight | html                | **Complete** |
| highlight | html-standalone     | **Complete** |
| tags      |                     | Not started  |
| check     |                     | Not started  |

## License

This package is released under the Apache License, Version 2.0. See LICENSE file for details.

# Changes

After version 0.1.4 the single crate has been replaced with the four `sdml_core`, `sdml_parse`, `sdml_generate`, and `sdml_cli`.
Each will have it's own version history starting with **0.1.5**.

**Version: 0.1.4**

* Support the latest grammar
* UML Class Diagram (initial)
* Modeling Library modules

**Version: 0.1.3**

* Support the latest grammar

**Version: 0.1.2**

* Syntax highlighting supported
* Support the latest grammar

**Version: 0.1.1**

* More drawing details
* More command-line features
* Support the latest grammar

**Version: 0.1.0**

* Initial version, limited to basic drawings

# Other Links

Formatting:

* https://github.com/numtide/treefmt/wiki
* https://melpa.org/#/format-all
* https://github.com/tweag/topiary

Linting:

* https://github.com/github/super-linter
