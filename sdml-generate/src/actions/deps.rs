/*!
Generate a text-based dependency tree, or GraphViz-based dependency graph, starting from the supplied module.

*/

use crate::color;
use crate::color::rdf::format_url;
use crate::color::rdf::Separator;
use crate::GenerateToWriter;
use nu_ansi_term::Style;
use sdml_core::error::Error;
use sdml_core::model::identifiers::Identifier;
use sdml_core::model::modules::HeaderValue;
use sdml_core::model::modules::Module;
use sdml_core::model::HasName;
use sdml_core::{cache::ModuleCache, stdlib::is_library_module};
use std::collections::HashSet;
use std::io::Write;
use text_trees::{FormatCharacters, TreeFormatting, TreeNode};
use url::Url;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct DependencyViewGenerator {
    depth: usize,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum DependencyViewRepresentation {
    ///
    /// This representation is most intended for command-line tools, it displays the output in a
    /// hierarchical tree format.
    ///
    /// ```bash
    /// $ cargo run deps sdml
    /// sdml
    /// ├── owl
    /// │   ├── rdf
    /// │   │   └── rdfs
    /// │   │       └── rdf
    /// │   ├── rdfs
    /// │   └── xsd
    /// │       ├── rdf
    /// │       └── rdfs
    /// ├── rdf
    /// ├── rdfs
    /// ├── skos
    /// │   ├── rdf
    /// │   └── rdfs
    /// └── xsd
    /// ```
    ///
    TextTree,
    ///
    /// This representation gives a
    /// - subject module: bold
    /// - library: italic
    ///
    /// ```bash
    /// $ cargo run deps -f graph sdml
    /// digraph G {
    ///   bgcolor="transparent";
    ///   rankdir="TB";
    ///   fontname="Helvetica,Arial,sans-serif";
    ///   node [fontname="Helvetica,Arial,sans-serif"; fontsize=10];
    ///   edge [fontname="Helvetica,Arial,sans-serif"; fontsize=9; fontcolor="dimgrey";
    ///         labelfontcolor="blue"; labeldistance=2.0];
    ///
    ///   sdml [label=<<B><I>sdml</I></B>>];
    ///   rdf [label=<<I>rdf</I>>];
    ///   skos [label=<<I>skos</I>>];
    ///   owl [label=<<I>owl</I>>];
    ///   rdfs [label=<<I>rdfs</I>>];
    ///   xsd [label=<<I>xsd</I>>];
    ///
    ///   sdml -> owl;
    ///   owl -> rdf;
    ///   rdf -> rdfs;
    ///   rdfs -> rdf;
    ///   owl -> rdfs;
    ///   owl -> xsd;
    ///   xsd -> rdf;
    ///   xsd -> rdfs;
    ///   sdml -> rdf;
    ///   sdml -> rdfs;
    ///   sdml -> skos;
    ///   skos -> rdf;
    ///   skos -> rdfs;
    ///   sdml -> xsd;
    /// }
    /// ```
    ///
    DotGraph,
    ///
    /// ```bash
    /// $ cargo run deps -f rdf sdml
    /// <http://sdml.io/sdml-owl.ttl#> <http://www.w3.org/2002/07/owl#imports> <http://www.w3.org/2002/07/owl#> .
    /// <http://sdml.io/sdml-owl.ttl#> <http://www.w3.org/2002/07/owl#imports> <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
    /// <http://sdml.io/sdml-owl.ttl#> <http://www.w3.org/2002/07/owl#imports> <http://www.w3.org/2000/01/rdf-schema#> .
    /// <http://sdml.io/sdml-owl.ttl#> <http://www.w3.org/2002/07/owl#imports> <http://www.w3.org/2004/02/skos/core#> .
    /// <http://sdml.io/sdml-owl.ttl#> <http://www.w3.org/2002/07/owl#imports> <http://www.w3.org/2001/XMLSchema#> .
    /// <http://www.w3.org/2002/07/owl#> <http://www.w3.org/2002/07/owl#imports> <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
    /// <http://www.w3.org/2002/07/owl#> <http://www.w3.org/2002/07/owl#imports> <http://www.w3.org/2000/01/rdf-schema#> .
    /// <http://www.w3.org/2002/07/owl#> <http://www.w3.org/2002/07/owl#imports> <http://www.w3.org/2001/XMLSchema#> .
    /// <http://www.w3.org/1999/02/22-rdf-syntax-ns#> <http://www.w3.org/2002/07/owl#imports> <http://www.w3.org/2000/01/rdf-schema#> .
    /// <http://www.w3.org/2000/01/rdf-schema#> <http://www.w3.org/2002/07/owl#imports> <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
    /// <http://www.w3.org/2001/XMLSchema#> <http://www.w3.org/2002/07/owl#imports> <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
    /// <http://www.w3.org/2001/XMLSchema#> <http://www.w3.org/2002/07/owl#imports> <http://www.w3.org/2000/01/rdf-schema#> .
    /// <http://www.w3.org/2004/02/skos/core#> <http://www.w3.org/2002/07/owl#imports> <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
    /// <http://www.w3.org/2004/02/skos/core#> <http://www.w3.org/2002/07/owl#imports> <http://www.w3.org/2000/01/rdf-schema#> .
    /// ```
    ///
    RdfImports,
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

#[derive(Debug)]
struct Node<'a> {
    name: &'a Identifier,
    base_uri: Option<&'a HeaderValue<Url>>,
    version_uri: Option<&'a HeaderValue<Url>>,
    children: Option<Vec<Node<'a>>>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for DependencyViewRepresentation {
    fn default() -> Self {
        Self::TextTree
    }
}

// ------------------------------------------------------------------------------------------------

impl GenerateToWriter<DependencyViewRepresentation> for DependencyViewGenerator {
    fn write_in_format<W>(
        &mut self,
        module: &Module,
        cache: &ModuleCache,
        writer: &mut W,
        format: DependencyViewRepresentation,
    ) -> Result<(), Error>
    where
        W: Write + Sized,
    {
        match format {
            DependencyViewRepresentation::TextTree => {
                self.write_text_tree(module, cache, self.depth, writer)
            }
            DependencyViewRepresentation::DotGraph => {
                self.write_dot_graph(module, cache, self.depth, writer)
            }
            DependencyViewRepresentation::RdfImports => {
                self.write_rdf_imports(module, cache, self.depth, writer)
            }
        }
    }
}

impl DependencyViewGenerator {
    pub fn new(depth: usize) -> Self {
        Self { depth }
    }

    // --------------------------------------------------------------------------------------------
    // Generate ❱ Text Trees
    // --------------------------------------------------------------------------------------------

    fn write_text_tree<W>(
        &self,
        module: &Module,
        cache: &ModuleCache,
        depth: usize,
        writer: &mut W,
    ) -> Result<(), Error>
    where
        W: Write + Sized,
    {
        let depth = if depth == 0 { usize::MAX } else { depth };

        let mut seen = Default::default();
        let tree = Node::from_module(module, None, &mut seen, cache, depth);

        // Convert from internal tree to TextTree
        let new_tree = tree.make_text_tree(true);

        // Write out text tree using it's write API
        new_tree.write_with_format(
            writer,
            &TreeFormatting::dir_tree(FormatCharacters::box_chars()),
        )?;

        Ok(())
    }

    // --------------------------------------------------------------------------------------------
    // Generate ❱ Graphviz Dot File
    // --------------------------------------------------------------------------------------------

    fn write_dot_graph<W>(
        &self,
        module: &Module,
        cache: &ModuleCache,
        depth: usize,
        writer: &mut W,
    ) -> Result<(), Error>
    where
        W: Write + Sized,
    {
        let depth = if depth == 0 { usize::MAX } else { depth };

        let mut seen = Default::default();
        let tree = Node::from_module(module, None, &mut seen, cache, depth);

        writer.write_all(
            r#"digraph G {
  bgcolor="transparent";
  rankdir="TB";
  fontname="Helvetica,Arial,sans-serif";
  node [
    shape="tab";
    fontname="Helvetica,Arial,sans-serif"; fontsize=11
  ];
  edge [
    style="dashed"; arrowhead="open";
    fontname="Helvetica,Arial,sans-serif"; fontsize=9; fontcolor="dimgrey";
    labelfontcolor="blue"; labeldistance=2.0
  ];

"#
            .as_bytes(),
        )?;

        if !seen.contains(module.name()) {
            writer.write_all(
                self.write_gv_node(module.name(), true, is_library_module(module.name()))
                    .as_bytes(),
            )?;
        }

        for module_name in seen {
            writer.write_all(
                self.write_gv_node(
                    module_name,
                    module_name == module.name(),
                    is_library_module(module_name),
                )
                .as_bytes(),
            )?;
        }

        writer.write_all(b"\n")?;

        self.write_graph_node(&tree, writer)?;

        writer.write_all(b"}\n")?;

        Ok(())
    }

    #[allow(clippy::only_used_in_recursion)]
    fn write_graph_node<W>(&self, node: &Node<'_>, writer: &mut W) -> Result<(), Error>
    where
        W: Write + Sized,
    {
        if let Some(children) = &node.children {
            for child in children {
                if let Some(version_uri) = child.version_uri {
                    writer.write_all(
                        format!(
                            "  {} -> {} [label=\"{}\"];\n",
                            node.name, child.name, version_uri
                        )
                        .as_bytes(),
                    )?;
                } else {
                    writer.write_all(format!("  {} -> {};\n", node.name, child.name).as_bytes())?;
                }
                self.write_graph_node(child, writer)?;
            }
        }

        Ok(())
    }

    fn write_gv_node(&self, name: &Identifier, is_subject: bool, is_library: bool) -> String {
        const MODULE_STEREOTYPE: &str = "<FONT POINT-SIZE=\"9\">«module»</FONT><BR/>";
        match (is_subject, is_library) {
            (true, true) => format!(
                "  {} [label=<{}<B><I>{}</I></B>>];\n",
                name, MODULE_STEREOTYPE, name
            ),
            (true, false) => format!(
                "  {} [label=<{}<B>{}</B>>];\n",
                name, MODULE_STEREOTYPE, name
            ),
            (false, true) => format!(
                "  {} [label=<{}<I>{}</I>>];\n",
                name, MODULE_STEREOTYPE, name
            ),
            (false, false) => format!("  {}[label=<{}{}>];\n", name, MODULE_STEREOTYPE, name),
        }
    }

    // --------------------------------------------------------------------------------------------
    // Generate ❱ RDF Triples
    // --------------------------------------------------------------------------------------------

    fn write_rdf_imports<W>(
        &self,
        module: &Module,
        cache: &ModuleCache,
        depth: usize,
        writer: &mut W,
    ) -> Result<(), Error>
    where
        W: Write + Sized,
    {
        const OWL_IMPORTS: &str = "http://www.w3.org/2002/07/owl#imports";
        let depth = if depth == 0 { usize::MAX } else { depth };

        let mut seen = Default::default();
        let tree = Node::from_module(module, None, &mut seen, cache, depth);

        let mut list = Vec::default();
        self.tree_to_rdf_list(&tree, &mut list);

        for (subj, obj) in list {
            writer.write_all(
                format!(
                    "{} {} {}{}",
                    format_url(subj.as_ref()),
                    format_url(OWL_IMPORTS),
                    format_url(obj.as_ref()),
                    Separator::Statement,
                )
                .as_bytes(),
            )?;
        }

        Ok(())
    }

    #[allow(clippy::only_used_in_recursion)]
    fn tree_to_rdf_list<'a>(
        &self,
        node: &'a Node<'_>,
        list: &mut Vec<(&'a HeaderValue<Url>, &'a HeaderValue<Url>)>,
    ) {
        if node.base_uri.is_some() {
            if let Some(children) = &node.children {
                for child in children {
                    if let Some(child_base_uri) = child.base_uri {
                        if let Some(child_version_uri) = child.version_uri {
                            list.push((node.base_uri.unwrap(), child_version_uri));
                        } else {
                            list.push((node.base_uri.unwrap(), child_base_uri));
                        }
                    }
                }
                for child in children {
                    if child.base_uri.is_some() {
                        self.tree_to_rdf_list(child, list);
                    }
                }
            }
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl<'a> Node<'a> {
    fn from_module(
        module: &'a Module,
        version_uri: Option<&'a HeaderValue<Url>>,
        seen: &mut HashSet<&'a Identifier>,
        cache: &'a ModuleCache,
        depth: usize,
    ) -> Self {
        let mut children: Vec<Node<'a>> = Default::default();
        let import_map = module.imported_module_versions();
        let mut modules = import_map.keys().collect::<Vec<_>>();
        modules.sort();
        for imported in modules {
            #[allow(clippy::map_clone)]
            let imported_version_uri = import_map.get(imported).map(|v| *v).unwrap_or_default();
            if depth == 1 || seen.contains(imported) {
                if let Some(cached) = cache.get(imported) {
                    children.push(Self::from_name(
                        imported,
                        cached.base_uri(),
                        imported_version_uri,
                    ));
                } else {
                    children.push(Self::from_name_only(imported, imported_version_uri));
                }
            } else {
                seen.insert(imported);
                if let Some(cached) = cache.get(imported) {
                    children.push(Self::from_module(
                        cached,
                        imported_version_uri,
                        seen,
                        cache,
                        depth - 1,
                    ));
                } else {
                    children.push(Self::from_name_only(imported, imported_version_uri));
                }
            }
        }

        Self {
            name: module.name(),
            base_uri: module.base_uri(),
            version_uri,
            children: Some(children),
        }
    }

    fn from_name_only(module: &'a Identifier, version_uri: Option<&'a HeaderValue<Url>>) -> Self {
        Self::from_name(module, None, version_uri)
    }

    fn from_name(
        module: &'a Identifier,
        base_uri: Option<&'a HeaderValue<Url>>,
        version_uri: Option<&'a HeaderValue<Url>>,
    ) -> Self {
        Self {
            name: module,
            base_uri,
            version_uri,
            children: None,
        }
    }

    fn make_text_tree(&'a self, is_root: bool) -> TreeNode<String> {
        let children = if let Some(children) = &self.children {
            children
                .iter()
                .map(|node| node.make_text_tree(false))
                .collect::<Vec<TreeNode<_>>>()
        } else {
            Default::default()
        };
        TreeNode::with_child_nodes(self.make_node_string(is_root), children.into_iter())
    }

    fn make_node_string(&self, is_root: bool) -> String {
        let node_string = format!(
            "{}{}",
            self.name,
            if let Some(version_uri) = self.version_uri {
                format!("@<{version_uri}>")
            } else {
                String::new()
            }
        );
        if color::colorize().colorize() {
            let mut style = Style::new();

            if is_root {
                style = style.bold();
            }
            if is_library_module(self.name) {
                style = style.dimmed().italic();
            }

            style.paint(node_string).to_string()
        } else {
            node_string
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
