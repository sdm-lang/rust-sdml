# Package sdml-cli

[![License-Apache_2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![License-MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/sdml_cli.svg)](https://crates.io/crates/sdml_cli)
[![Docs.rs](https://img.shields.io/docsrs/sdml-cli.svg)](https://docs.rs/sdml_cli)

This package is part of the Rust SDML project and specifically implements the
`sdml` command-line interface (CLI). The project's intent is to provide an
idiomatic implementation of the in-memory model, parser, generators, and the CLI
tool.

The following figure demonstrates this package in the broader project context.

![Package Overview](https://raw.githubusercontent.com/sdm-lang/rust-sdml/refs/heads/main/doc/overview.png)

## Installation

To install the command-line tool on MacOS or Linux use the Homebrew package
manager and the SDML Tap. Installing in this way also installs dependencies such
as GraphViz and PlantUML used for diagram generation.

```bash
❯ brew install sdm-lang/sdml/sdml
```

You can check that you have the tool installed and on the path with the
following check.

```bash
 ❯ sdml versions               
SDML CLI:        0.2.7
SDML grammar:    0.2.16
Tree-Sitter ABI: 14
```

### Install via cargo

Cargo is usually installed with the Rust toolchain using [rustup](https://rustup.rs/).

The following command should download and build the tool, and will also work to
install any updates.

```bash
❯ cargo install sdml-cli
```

Cargo will sometimes report that you have the latest version installed, to be
sure you can force it to install regardless with the `--force` option.

```bash
❯ cargo install sdml-cli --force
```

### Install from source

To install the CLI from source you need to clone the entire repository.

```bash
❯ git clone https://github.com/sdm-lang/rust-sdml.git
```

In the `rust-sdml` directory you can build/test/install using the following
commands.

```bash
❯ cargo build
❯ cargo test
❯ cargo install --path sdml-cli
```

## Global Options

Certain command-line options act on all commands, these must appear before the
command. The SDML tool has a `log-filter` and a `no-color` global option.

The set of packages making up `rust-sdml` all have extensive logging which can be
enabled when running the tool. The global argument `--log-filter` takes a log
level and displays any log event with a severity greater than, or equal to, the
filter.

```bash
❯ sdml --log-filter tracing versions
2024-02-20T19:06:53.141741Z  INFO sdml: Log level set to `LevelFilter::Tracing`
2024-02-20T19:06:53.141877Z TRACE sdml: Commands::execute self: Versions
SDML CLI:        0.2.7
SDML grammar:    0.2.16
Tree-Sitter ABI: 14
```

Some of the commands will, by default, use colored output which can be a problem
if you save a file for future processing as the control characters play havoc
with diff tools for example.

```bash
❯ sdml --no-color versions
❯ NO_COLOR=1  sdml versions
❯ CLI_COLOR=0 sdml versions
```

## Commands

Input Files

### Getting Help

```bash
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

This command (convert) allows the conversion of a module from the SDML surface
syntax into one of a number of alternate representations.

#### RDF

This uses the surface to RDF mapping defined in the SDML Language Reference. The
mapping is normative and stable.

#### JSON

This is a direct representation of the in-memory model in the Rust package
`sdml_core` in JSON. This mapping is non-normative and may change according to any
model structure change.

#### S-Expression

This is a debugging representation, and supported as the underlying tree-sitter
library uses s-expressions as a parse-tree visualization.

### Dependency Visualization

This command (dep) generates a representation of the transitive closure of
dependencies for a given module into one of a number of alternate
representations.

#### As Text Tree

Show dependencies as a text tree with the original as the root.

```bash
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

In some cases the entire set of dependencies is not necessary and the `--depth`
argument can be added to only show a number of levels of import from the root.
The depth argument instructs to command to stop after that many dependencies
away from the original module. Setting depth to 1 will only show the direct
dependencies of the original.

```bash
❯ sdml deps --depth 1 sdml
sdml
├── owl
├── rdf
├── rdfs
├── skos
└── xsd
```

#### As GraphViz Graph

Create an SVG representation of the dependency graph using GraphViz.

```bash
❯ sdml deps --output-format graph sdml > sdml-deps.svg
❯ open -a Safari sdml-deps.svg
```

![example](https://raw.githubusercontent.com/sdm-lang/rust-sdml/main/sdml-generate/doc/example_deps_graph.svg)

#### As RDF Statements

Create a set of RDF statements,as N-Triples, that represent the individual OWL
import relationships.

```bash
❯ sdml deps --depth 1 --output-format rdf sdml
<http://sdml.io/sdml-owl.ttl#> <http://www.w3.org/2002/07/owl#imports> <http://www.w3.org/2002/07/owl#> .
<http://sdml.io/sdml-owl.ttl#> <http://www.w3.org/2002/07/owl#imports> <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
<http://sdml.io/sdml-owl.ttl#> <http://www.w3.org/2002/07/owl#imports> <http://www.w3.org/2000/01/rdf-schema#> .
<http://sdml.io/sdml-owl.ttl#> <http://www.w3.org/2002/07/owl#imports> <http://www.w3.org/2004/02/skos/core#> .
<http://sdml.io/sdml-owl.ttl#> <http://www.w3.org/2002/07/owl#imports> <http://www.w3.org/2001/XMLSchema#> .
```

### Diagram Generation

This command (draw) generates diagrams of a module with different perspectives.

#### Concept Diagram

```bash
❯ sdml draw --diagram concepts --o example-concepts.svg -i example/example.sdm
❯ open -a Safari example-concepts.svg
```

![example](https://raw.githubusercontent.com/sdm-lang/rust-sdml/main/sdml-generate/doc/example-concepts.svg)

#### Entity Relationship Diagram

```bash
❯ sdml draw --diagram entity-relationship --o example-erd.svg -i example/example.sdm
❯ open -a Safari example-erd.svg
```

![example](https://raw.githubusercontent.com/sdm-lang/rust-sdml/main/sdml-generate/doc/example-erd.svg)

#### UML Class Diagram

```bash
❯ sdml draw --diagram uml-class --o example-uml.svg -i example/example.sdm
❯ open -a Safari example-uml.svg
```

![example](https://raw.githubusercontent.com/sdm-lang/rust-sdml/main/sdml-generate/doc/example-uml.svg)

### Document (Project) Generation

This command (doc-book) creates structured documentation for a collection of
modules, and includes annotations, constraints and all definition types. The
generated documentation also include diagrams and dependency graphs.

#### In Org-mode

Create an Emacs org-mode formatted file. This format allows all content to be
written into a single file with export options to HTML, LaTeX, Word, PDF and
more.

### Document (Module) Generation

This command (doc) creates structured documentation for a module, and includes
annotations, constraints and all definition types. The generated documentation
also include diagrams and dependency graphs.

#### In Org-mode

Create an Emacs org-mode formatted file. This format allows all content to be
written into a single file with export options to HTML, LaTeX, Word, PDF and
more.

#### In Markdown

Create a markdown formatted file, this file uses GitHub-flavored markdown to
allow for some better content formatting than CommonMark.

### Module Highlighting

TBD

### XRef Tag Generation

TBD

### Validation

This command (validate) provides deep validation of a module's content,
including errors, warnings, and linter-like advice. Checks are run not only on
the initial module, but it's transitively loaded dependencies.

```bash
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

Additionally, a `short-form` option will generate diagnostics using a CSV format
that is easier for tools to parse. The fields in this format are: severity, file
name, start line, start column, end line, end column, error code, and message.

```bash
❯ sdml validate --level all --short-form -i examples/errors/i0506.sdm
note,examples/errors/i0506.sdm,1,8,1,15,I0506,identifier not using preferred casing
note,examples/errors/i0506.sdm,3,13,3,26,I0506,identifier not using preferred casing
```

### Version Information

This command (versions) shows more information than the simple `--version` global
argument and is useful for debugging.

```bash
❯ sdml versions               
SDML CLI:        0.2.7
SDML grammar:    0.2.16
Tree-Sitter ABI: 14
```

### Module Viewer

This command (view) will generate source code from a module file, which at first
seems redundant. However, this view provides levels of detail that allow for an
overview of module definitions. The `--level` argument can be used to elide
content and get an overview of a module.

#### Definitions Only

Show only the definitions in the module, any definition body will be elided, for
an overview of the module contents. Elided definitions are followed by `";; ..."`.

```bash
❯ sdml view --level definitions -i examples/example.sdm
module example <https://example.com/api> is

  import [ dc xsd ]

  datatype Uuid <- sdml:string ;; ...

  entity Example ;; ...

end
```

#### Members

Show definitions in the module and show the members of product types and
variants of sum types but not their bodies if present.

```bash
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

#### Full

Show all contents of the module.

```bash
❯ sdml view --level full -i examples/example.sdm
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

-----

## Changes

### Version 0.3.2

* Style: Changed cargo file to use license key instead of license-file.

### Version 0.3.1

* Feature: added new `generate` command which uses the `sdml-tera` package for
  template-driven generators.

### Version 0.3.0

* Feature: updates to support the latest grammar, see `sdml-core`.
* Refactor: use the latest ~`Generator~` trait.

### Version 0.2.10

* Feature: added new command `doc-book` to create a more complex documentation
  output for a collection of modules.
* Build: bump version of `sdml-errors`, `sdml-core`, and `sdml-generate`.

### Version 0.2.9

* Build: update dependency from `sdml_error` to `sdml-errors`.
* Build: bump versions of `sdml-core`, `sdml-parse`, `sdml-generate`.

### Version 0.2.8

* Build: upgrade to `sdml_core` version `0.2.14` and the new `ModelStore` trait.

### Version 0.2.7

* Feature: better error handling in conjunction with the validation and
  diagnostics in `sdml-errors`.

### Version 0.2.6

* Build: update dependencies.

### Version 0.2.5

* Feature: Add new `--no-color` flag to the CLI which also uses the `NO_COLOR`
  environment variable.
* Feature: Removed indirect dependencies from `Cargo.toml`.
* Update: New generator features for colored RDF.
  
### Version 0.2.4

* Feature: Add new `source` command to call the new source generator.
* Fix: Change the description of `depth` parameter for `deps` command, 0 is the
  default which means all depths are included in the output.
* Update: Use new generator traits that require a module cache parameter.

### Version 0.2.3

* Feature: add new `stdlib` modules with standard layout.
* Feature: minor refactor of cache and loader.

### Version 0.2.2

* Feature: Update to latest grammar for version URIs and RDF definitions.
  * Add support for base URI on modules.
  * Add support for version info and URI on modules.
  * Add support for version URI on module import.
  * Parse RDF definitions for classes and properties.

### Version 0.2.1

* Feature: Remove member groups.

### Version 0.2.0

* Feature: Update to latest grammar.
  * Remove `ValueVariant` numeric values.
  * Update formal constraints.
  * Add type classes.

### Version 0.1.6

* Updated dependencies.

### Version 0.1.5

Initial stand-alone crate.

### Version 0.1.4

Previously part of a single crate [sdml](https://crates.io/crates/sdml).
