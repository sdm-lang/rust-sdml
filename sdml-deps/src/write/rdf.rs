/*!
One-line description.

```bash
$ cargo run deps -f rdf sdml
<http://sdml.io/sdml-owl.ttl#> <http://www.w3.org/2002/07/owl#imports> <http://www.w3.org/2002/07/owl#> .
<http://sdml.io/sdml-owl.ttl#> <http://www.w3.org/2002/07/owl#imports> <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
<http://sdml.io/sdml-owl.ttl#> <http://www.w3.org/2002/07/owl#imports> <http://www.w3.org/2000/01/rdf-schema#> .
<http://sdml.io/sdml-owl.ttl#> <http://www.w3.org/2002/07/owl#imports> <http://www.w3.org/2004/02/skos/core#> .
<http://sdml.io/sdml-owl.ttl#> <http://www.w3.org/2002/07/owl#imports> <http://www.w3.org/2001/XMLSchema#> .
<http://www.w3.org/2002/07/owl#> <http://www.w3.org/2002/07/owl#imports> <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
<http://www.w3.org/2002/07/owl#> <http://www.w3.org/2002/07/owl#imports> <http://www.w3.org/2000/01/rdf-schema#> .
<http://www.w3.org/2002/07/owl#> <http://www.w3.org/2002/07/owl#imports> <http://www.w3.org/2001/XMLSchema#> .
<http://www.w3.org/1999/02/22-rdf-syntax-ns#> <http://www.w3.org/2002/07/owl#imports> <http://www.w3.org/2000/01/rdf-schema#> .
<http://www.w3.org/2000/01/rdf-schema#> <http://www.w3.org/2002/07/owl#imports> <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
<http://www.w3.org/2001/XMLSchema#> <http://www.w3.org/2002/07/owl#imports> <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
<http://www.w3.org/2001/XMLSchema#> <http://www.w3.org/2002/07/owl#imports> <http://www.w3.org/2000/01/rdf-schema#> .
<http://www.w3.org/2004/02/skos/core#> <http://www.w3.org/2002/07/owl#imports> <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
<http://www.w3.org/2004/02/skos/core#> <http://www.w3.org/2002/07/owl#imports> <http://www.w3.org/2000/01/rdf-schema#> .
```

TBD

# Example

TBD

 */

use crate::{write::DependencyWriterOptions, DependencyNode};
use sdml_core::{
    error::Error,
    model::modules::{HeaderValue, Module},
    repr::RepresentationWriter,
    stdlib::owl,
    store::InMemoryModuleCache,
};
use std::io::Write;
use url::Url;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct RdfDependencyWriter;

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl RepresentationWriter for RdfDependencyWriter {
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

        let mut list = Vec::default();
        self.tree_to_rdf_list(&tree, &mut list);

        for (subj, obj) in list {
            w.write_all(
                format!(
                    "<{}> <{}> <{}> .",
                    subj.value().as_ref(),
                    owl::IMPORTS,
                    obj.value().as_ref(),
                )
                .as_bytes(),
            )?;
        }

        Ok(())
    }
}

impl RdfDependencyWriter {
    #[allow(clippy::only_used_in_recursion)]
    fn tree_to_rdf_list<'a>(
        &self,
        node: &'a DependencyNode<'_>,
        list: &mut Vec<(&'a HeaderValue<Url>, &'a HeaderValue<Url>)>,
    ) {
        println!(">>> {node:#?}");
        if let Some(node_base_uri) = node.base_uri() {
            println!(">>> has node.base_uri <{node_base_uri}>");
            if let Some(children) = &node.children() {
                println!(">>> has node.children");
                for child in children.iter() {
                    println!(">>> child: {child:#?}");
                    if let Some(child_base_uri) = child.base_uri() {
                        println!(">>> has child.base_uri <{child_base_uri}>");
                        if let Some(child_version_uri) = child.version_uri() {
                            print!(">>> has child.version_uri <{child_version_uri}>");
                            list.push((node_base_uri, child_version_uri));
                        } else {
                            println!(">>> no child.version_uri");
                            list.push((node_base_uri, child_base_uri));
                        }
                    } else {
                        println!("<<< no child.base_uri");
                    }
                }
                println!(">>> process child trees");
                for child in children.iter() {
                    self.tree_to_rdf_list(child, list);
                }
            }
        } else {
            println!("<<< no node.base_uri");
        }
    }
}
