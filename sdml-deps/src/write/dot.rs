/*!
This representation gives a

- subject module: bold
- library: italic

```bash
$ cargo run deps -f graph sdml
digraph G {
  bgcolor="transparent";
  rankdir="TB";
  fontname="Helvetica,Arial,sans-serif";
  node [fontname="Helvetica,Arial,sans-serif"; fontsize=10];
  edge [fontname="Helvetica,Arial,sans-serif"; fontsize=9; fontcolor="dimgrey";
        labelfontcolor="blue"; labeldistance=2.0];

  sdml [label=<<B><I>sdml</I></B>>];
  rdf [label=<<I>rdf</I>>];
  skos [label=<<I>skos</I>>];
  owl [label=<<I>owl</I>>];
  rdfs [label=<<I>rdfs</I>>];
  xsd [label=<<I>xsd</I>>];

  sdml -> owl;
  owl -> rdf;
  rdf -> rdfs;
  rdfs -> rdf;
  owl -> rdfs;
  owl -> xsd;
  xsd -> rdf;
  xsd -> rdfs;
  sdml -> rdf;
  sdml -> rdfs;
  sdml -> skos;
  skos -> rdf;
  skos -> rdfs;
  sdml -> xsd;
}
```

TBD

# Example

TBD

 */

use crate::{write::DependencyWriterOptions, DependencyNode};
use sdml_core::{
    config::is_library_module,
    error::Error,
    model::{identifiers::Identifier, modules::Module, HasName},
    repr::RepresentationWriter,
    store::InMemoryModuleCache,
};
use std::io::Write;
use url::Url;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct DotDependencyWriter;

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

const DOT_FILE_HEADER: &str = include_str!("header.dot");
const MODULE_STEREOTYPE: &str = "<FONT POINT-SIZE=\"9\">«module»</FONT><BR/>";

impl RepresentationWriter for DotDependencyWriter {
    type Object = Module;
    type Cache = InMemoryModuleCache;
    type Options = DependencyWriterOptions;

    fn write_with<W>(
        &self,
        w: &mut W,
        module: &Self::Object,
        cache: Option<&Self::Cache>,
        options: &Self::Options,
    ) -> Result<(), Error>
    where
        W: Write,
    {
        let depth = if options.max_depth() == 0 {
            usize::MAX
        } else {
            options.max_depth()
        };

        let mut seen = Default::default();
        let tree = DependencyNode::from_module(module, None, &mut seen, cache, depth);

        let mut nodes = Vec::default();
        let mut edges = Vec::default();

        if !seen.contains(module.name()) {
            nodes.push(self.node_to_string(module.name(), true, is_library_module(module.name())));
        }

        for module_name in seen {
            nodes.push(self.node_to_string(
                module_name,
                module_name == module.name(),
                is_library_module(module_name),
            ));
        }

        self.handle_all_edges(&tree, &mut nodes, &mut edges);

        w.write_all(DOT_FILE_HEADER.as_bytes())?;

        nodes.sort_unstable();
        for node in nodes {
            w.write_all(node.as_bytes())?;
        }

        w.write_all(b"\n")?;

        edges.sort_unstable();
        for edge in edges {
            w.write_all(edge.as_bytes())?;
        }

        w.write_all(b"}\n")?;

        Ok(())
    }
}

impl DotDependencyWriter {
    fn handle_node(
        &self,
        name: &Identifier,
        is_subject: bool,
        is_library: bool,
        nodes: &mut Vec<String>,
    ) {
        let formatted_name = match (is_subject, is_library) {
            (true, true) => {
                format!("<B><I>{name}</I></B>")
            }
            (true, false) => format!("<B>{name}</B>"),
            (false, true) => format!("<I>{name}</I>",),
            (false, false) => name.to_string(),
        };
        nodes.push(format!(
            "  {name} [label=<{MODULE_STEREOTYPE}{formatted_name}>];\n"
        ));
    }

    #[allow(clippy::only_used_in_recursion)]
    fn handle_all_edges(
        &self,
        node: &DependencyNode<'_>,
        nodes: &mut Vec<String>,
        edges: &mut Vec<String>,
    ) {
        if let Some(children) = &node.children {
            for child in children {
                edges.push(self.edge_to_string(
                    node.name,
                    child.name,
                    child.version_uri.map(|v| v.value()),
                ));
                self.handle_all_edges(child, nodes, edges);
            }
        }
    }

    fn node_to_string(&self, name: &Identifier, is_subject: bool, is_library: bool) -> String {
        let formatted_name = match (is_subject, is_library) {
            (true, true) => {
                format!("<B><I>{name}</I></B>")
            }
            (true, false) => format!("<B>{name}</B>"),
            (false, true) => format!("<I>{name}</I>",),
            (false, false) => name.to_string(),
        };
        format!("  {name} [label=<{MODULE_STEREOTYPE}{formatted_name}>];\n")
    }

    fn edge_to_string(
        &self,
        lhs: &Identifier,
        rhs: &Identifier,
        version_uri: Option<&Url>,
    ) -> String {
        let label = if let Some(version_uri) = version_uri {
            format!("[label=\"{version_uri}\"] ")
        } else {
            String::new()
        };
        format!("  {lhs} -> {rhs} {label};\n")
    }
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
