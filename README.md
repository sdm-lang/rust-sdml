# Crate sdml

Rust CLI for Simple Domain Modeling Language (SDML).

![MIT License](https://img.shields.io/badge/license-mit-118811.svg)
[![Rust](https://github.com/johnstonskj/rust-sdml/actions/workflows/rust.yml/badge.svg)](https://github.com/johnstonskj/rust-sdml/actions/workflows/rust.yml)
[![Security audit](https://github.com/johnstonskj/rust-sdml/actions/workflows/security-audit.yml/badge.svg)](https://github.com/johnstonskj/rust-sdml/actions/workflows/security-audit.yml)
[![Codecov](https://codecov.io/gh/johnstonskj/rust-sdml/branch/main/graph/badge.svg?token=1HGN6M4KIT)](https://codecov.io/gh/johnstonskj/rust-sdml)
[![GitHub stars](https://img.shields.io/github/stars/johnstonskj/rust-sdml.svg)](https://github.com/johnstonskj/rust-sdml/stargazers)

This is  a command-line tool that provides functionality to process
[SDML](https://github.com/johnstonskj/tree-sitter-sdml) files. 

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

## Convert

Conversion to an Emacs [org-Mode](https://orgmode.org/) file provides a documentation format with the most complete options.

``` shell
sdml convert --output-format org-mode rentals
```

This command converts the SDML surface syntax into it's underlying RDF representation.

``` shell
sdml convert --output-format org-mode rentals
```

This command convert the SDML surface syntax into an expanded version of the s-expressions used in tree-sitter queries. 

``` shell
sdml convert --output-format org-mode rentals
```

## Highlight

For the console this uses ANSI escape sequences to format the text. 

``` shell
sdml highlight --output-format ansi rentals
```

To generate formatted and highlighted HTML the tool accepts two different format specifiers, `html` for simply a block
of HTML that can be inserted into another document, or `html-standalone` to generate a full document around the
highlighted code block.

``` shell
sdml highlight --output-format html rentals
sdml highlight --output-format html-standalone rentals
```

## Draw

To draw a high-level Concepts diagram, use the diagram specifier `concepts`.

``` shell
sdml draw --diagram concepts \
          --output-format svg --output-file rentals.svg \
          --base-path . rentals
```

For more detail an Entity-Relationship diagram can be generated with the diagram specifier `entity-relationship`.

``` shell
sdml draw --diagram entity-relationship \
          --output-format svg --output-file rentals.svg \
          --base-path . rentals
```

For the mose detail a UML Class diagram can be generated with the diagram specifier `uml-class`.

``` shell
sdml draw --diagram uml-class \
          --output-format svg --output-file rentals.svg \
          --base-path . rentals
```

# Other Links

Formatting:

* https://github.com/numtide/treefmt/wiki
* https://melpa.org/#/format-all
* https://github.com/tweag/topiary

Linting:

* https://github.com/github/super-linter

# Changes

**Version: 0.1.2**

* Syntax highlighting supported
* Support the latest grammar

**Version: 0.1.1**

* More drawing details
* More command-line features
* Support the latest grammar

**Version: 0.1.0**

* Initial version, limited to basic drawings
