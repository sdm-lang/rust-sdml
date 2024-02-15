/*!
This module provides a set of *actions* that can be performed on an SDML file.

- `deps` -- Show a module's dependencies as either a tree, graph, or RDF statements.
- `highlight` -- Highlight a file in terminal colors or as formatted HTML.
- `tags` -- Generate a CTags file for a module and it's dependencies.
- `verify` -- A detailed verifier/linter.
*/

pub mod deps;

pub mod highlight;

pub mod tags;
