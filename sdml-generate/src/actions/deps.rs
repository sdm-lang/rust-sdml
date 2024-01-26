/*!
Generate a text-based dependency tree, or GraphViz-based dependency graph, starting from the supplied module.

*/

use sdml_core::cache::ModuleCache;
use sdml_core::error::Error;
use sdml_core::model::identifiers::Identifier;
use sdml_core::model::modules::Module;
use sdml_core::model::HasName;
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

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn write_dependency_tree<W: Write>(
    module: &Module,
    cache: &ModuleCache,
    depth: usize,
    w: &mut W,
) -> Result<(), Error> {
    let depth = if depth == 0 { usize::MAX } else { depth };

    let mut seen = Default::default();
    let tree = Node::from_module(module, &mut seen, cache, depth);

    let new_tree = make_tree_node(tree);

    new_tree.write_with_format(w, &TreeFormatting::dir_tree(FormatCharacters::box_chars()))?;

    Ok(())
}

pub fn write_dependency_graph<W: Write>(
    module: &Module,
    cache: &ModuleCache,
    depth: usize,
    w: &mut W,
) -> Result<(), Error> {
    let depth = if depth == 0 { usize::MAX } else { depth };

    let mut seen = Default::default();
    let tree = Node::from_module(module, &mut seen, cache, depth);

    w.write_all(
        r#"digraph G {
  bgcolor="transparent";
  rankdir="TB";
  fontname="Helvetica,Arial,sans-serif";
  node [fontname="Helvetica,Arial,sans-serif"; fontsize=10];
  edge [fontname="Helvetica,Arial,sans-serif"; fontsize=9; fontcolor="dimgrey";
        labelfontcolor="blue"; labeldistance=2.0];

"#
        .as_bytes(),
    )?;

    if !seen.contains(module.name()) {
        w.write_all(format!("  {};\n", module.name()).as_bytes())?;
    }

    for module in seen {
        w.write_all(format!("  {};\n", module).as_bytes())?;
    }

    write_graph_node(&tree, w)?;

    w.write_all(b"}\n")?;

    Ok(())
}

pub fn write_dependency_rdf<W: Write>(
    module: &Module,
    cache: &ModuleCache,
    depth: usize,
    w: &mut W,
) -> Result<(), Error> {
    let depth = if depth == 0 { usize::MAX } else { depth };

    let mut seen = Default::default();
    let tree = Node::from_module(module, &mut seen, cache, depth);

    let mut list = Vec::default();
    tree_to_list(&tree, &mut list);

    for (left, right) in list {
        w.write_all(
            format!(
                "<{}> <http://www.w3.org/2002/07/owl#imports> <{}> .\n",
                left, right,
            )
            .as_bytes(),
        )?;
    }

    Ok(())
}

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
struct Node<'a> {
    name: &'a Identifier,
    base: Option<&'a Url>,
    children: Option<Vec<Node<'a>>>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl<'a> Node<'a> {
    fn from_module(
        module: &'a Module,
        seen: &mut HashSet<&'a Identifier>,
        cache: &'a ModuleCache,
        depth: usize,
    ) -> Self {
        let mut children: Vec<Node<'a>> = Default::default();
        let mut modules = module.imported_modules().into_iter().collect::<Vec<_>>();
        modules.sort();
        for imported in modules {
            if depth == 1 || seen.contains(imported) {
                if let Some(cached) = cache.get(imported) {
                    children.push(Self::from_name(imported, cached.base_uri()));
                } else {
                    children.push(Self::from_name_only(imported));
                }
            } else {
                seen.insert(imported);
                if let Some(cached) = cache.get(imported) {
                    children.push(Self::from_module(cached, seen, cache, depth - 1));
                } else {
                    children.push(Self::from_name_only(imported));
                }
            }
        }

        Self {
            name: module.name(),
            base: module.base_uri(),
            children: Some(children),
        }
    }

    fn from_name_only(module: &'a Identifier) -> Self {
        Self::from_name(module, None)
    }

    fn from_name(module: &'a Identifier, base: Option<&'a Url>) -> Self {
        Self {
            name: module,
            base,
            children: None,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn write_graph_node<W: Write>(node: &Node<'_>, w: &mut W) -> Result<(), Error> {
    if let Some(children) = &node.children {
        for child in children {
            w.write_all(format!("  {} -> {};\n", node.name, child.name).as_bytes())?;
            write_graph_node(child, w)?;
        }
    }

    Ok(())
}

fn make_tree_node<'a>(node: Node<'a>) -> TreeNode<&'a Identifier> {
    let children = if let Some(children) = node.children {
        children
            .into_iter()
            .map(|n| make_tree_node(n))
            .collect::<Vec<TreeNode<&'a Identifier>>>()
    } else {
        Default::default()
    };
    TreeNode::with_child_nodes(node.name, children.into_iter())
}

fn tree_to_list<'a>(node: &'a Node<'_>, list: &mut Vec<(&'a Url, &'a Url)>) {
    if node.base.is_some() {
        if let Some(children) = &node.children {
            for child in children {
                if child.base.is_some() {
                    list.push((node.base.unwrap(), child.base.unwrap()));
                    tree_to_list(child, list);
                }
            }
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
