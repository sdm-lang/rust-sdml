/*!
This representation is most intended for command-line tools, it displays the output in a
hierarchical tree format.

```bash
$ cargo run deps sdml
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

TBD

# Example

TBD

 */

use crate::{write::DependencyWriterOptions, DependencyNode};
use nu_ansi_term::Style;
use sdml_core::{
    config::is_library_module, error::Error, model::modules::Module, repr::RepresentationWriter,
    store::InMemoryModuleCache,
};
use std::io::Write;
use text_trees::{FormatCharacters, TreeFormatting, TreeNode};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct TextDependencyWriter;

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl RepresentationWriter for TextDependencyWriter {
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

        // Convert from internal tree to TextTree
        let new_tree = self.make_text_tree(&tree, true, options.use_color());

        // Write out text tree using it's write API
        new_tree.write_with_format(w, &TreeFormatting::dir_tree(FormatCharacters::box_chars()))?;

        Ok(())
    }
}

impl TextDependencyWriter {
    fn make_text_tree(
        &self,
        node: &DependencyNode,
        is_root: bool,
        use_color: bool,
    ) -> TreeNode<String> {
        let children = if let Some(children) = &node.children() {
            children
                .iter()
                .map(|node| self.make_text_tree(node, false, use_color))
                .collect::<Vec<TreeNode<_>>>()
        } else {
            Default::default()
        };
        TreeNode::with_child_nodes(
            self.make_node_string(node, is_root, use_color),
            children.into_iter(),
        )
    }

    fn make_node_string(&self, node: &DependencyNode, is_root: bool, use_color: bool) -> String {
        let node_string = format!(
            "{}{}",
            node.name(),
            if let Some(version_uri) = node.version_uri() {
                format!("@<{version_uri}>")
            } else {
                String::new()
            }
        );
        if use_color {
            let mut style = Style::new();

            if is_root {
                style = style.bold();
            }
            if is_library_module(node.name) {
                style = style.dimmed().italic();
            }

            style.paint(node_string).to_string()
        } else {
            node_string
        }
    }
}
