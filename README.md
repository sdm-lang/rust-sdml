# Crate sdml

Rust CLI for Simple Domain Modeling Language (SDML).

![MIT License](https://img.shields.io/badge/license-mit-118811.svg)
[![Rust](https://github.com/johnstonskj/rust-sdml/actions/workflows/rust.yml/badge.svg)](https://github.com/johnstonskj/rust-sdml/actions/workflows/rust.yml)
[![Security audit](https://github.com/johnstonskj/rust-sdml/actions/workflows/security-audit.yml/badge.svg)](https://github.com/johnstonskj/rust-sdml/actions/workflows/security-audit.yml)
[![Codecov](https://codecov.io/gh/johnstonskj/rust-sdml/branch/main/graph/badge.svg?token=1HGN6M4KIT)](https://codecov.io/gh/johnstonskj/rust-sdml)
[![GitHub stars](https://img.shields.io/github/stars/johnstonskj/rust-sdml.svg)](https://github.com/johnstonskj/rust-sdml/stargazers)

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

# Changes

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
