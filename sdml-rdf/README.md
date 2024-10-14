# Package sdml-parse

Rust Parser for the Simple Domain Modeling Language (SDML).

![https://crates.io/crates/sdml_parse](https://img.shields.io/crates/v/sdml_parse.svg)
![https://docs.rs/sdml_parse](https://img.shields.io/docsrs/sdml-parse.svg)

This package is part of the Rust SDML project and specifically implements the bi-directional mapping between the SDML
data model (in package sdml-core) and the RDF semantics..

The following figure demonstrates this package in the broader project context.

```text
                           ╭───────╮
                           │  CLI  │
                ╔═════╦══  │ crate │  ═══╦══════╗
┌╌╌╌╌╌╌╌╌┐      ║     ║    ╰───────╯     ║      ║
┆        ┆      ║     V                  V      ║
┆ source ┆      ║  ╭───────╮       ╭──────────╮ ║
┆  file  ┆  ══> ║  │ parse │  ══>  │ generate │ ║
┆        ┆   ╭──║──│ crate │───────│   crate  │─║──╮
└╌╌╌╌╌╌╌╌┘   │  ║  ╰───────╯       ╰──────────╯ ║  │
             │  v    core & errors crates       v  │
┌╌╌╌╌╌╌╌╌┐   │ ╭───────╮              ╭──────────╮ │
┆        ┆   ╰─│  rdf  │──────────────│   tera   │─╯
┆  RDF   ┆  ══>    │ crate │      ⋀       │  crate   │
┆  file  ┆         ╰───────╯      ║       ╰──────────╯
┆        ┆
└╌╌╌╌╌╌╌╌┘
```

## Changes

### Version 0.1.0

Initial release.
