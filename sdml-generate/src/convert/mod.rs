/*!
This module provides the generators for *module-to-file* conversion which includes *documentation*, *RDF*, and
*s-expressions*.

- `doc` -- Generate documentation for a module in either org-mode or markdown.
- `rdf` -- Generate the RDF representation of a module.
- `s-sexpr` -- Generate an s-expression representation of a module.
- `source` -- Generate source code for a module, including elided forms.
*/

pub mod doc;

pub mod rdf;

pub mod sexpr;

pub mod source;
