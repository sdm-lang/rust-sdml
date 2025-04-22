/*!
Generate a text-based dependency tree, or GraphViz-based dependency graph, starting from the supplied module.

*/

use sdml_core::{
    model::{
        identifiers::Identifier,
        modules::{HeaderValue, Module},
        HasName,
    },
    store::ModuleStore,
};
use std::collections::HashSet;
use url::Url;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct DependencyNode<'a> {
    name: &'a Identifier,
    base_uri: Option<&'a HeaderValue<Url>>,
    version_uri: Option<&'a HeaderValue<Url>>,
    children: Option<Vec<DependencyNode<'a>>>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl<'a> DependencyNode<'a> {
    pub fn from_module(
        module: &'a Module,
        version_uri: Option<&'a HeaderValue<Url>>,
        seen: &mut HashSet<&'a Identifier>,
        cache: Option<&'a impl ModuleStore>,
        depth: usize,
    ) -> Self {
        let mut children: Vec<Self> = Default::default();
        let import_map = module.imported_module_versions();
        let mut modules = import_map.keys().collect::<Vec<_>>();
        modules.sort();
        for imported in modules {
            #[allow(clippy::map_clone)]
            let imported_version_uri = import_map.get(imported).map(|v| *v).unwrap_or_default();
            match (
                depth,
                seen.contains(imported),
                cache.map(|c| c.get(imported)).unwrap_or_default(),
            ) {
                (1, true, Some(some_cached)) => {
                    println!("*** from_module(...) 1, true, cached");
                    children.push(Self::from_name(
                        imported,
                        some_cached.base_uri(),
                        imported_version_uri,
                    ));
                }
                (_, false, Some(some_cached)) => {
                    println!("*** from_module(...) _, false, cached");
                    seen.insert(some_cached.name());
                    children.push(Self::from_module(
                        some_cached,
                        imported_version_uri,
                        seen,
                        cache,
                        depth - 1,
                    ));
                }
                _ => {
                    println!("*** from_module(...) _, _, _");
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

    pub fn from_name_only(
        module: &'a Identifier,
        version_uri: Option<&'a HeaderValue<Url>>,
    ) -> Self {
        Self::from_name(module, None, version_uri)
    }

    pub fn from_name(
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

    fn name(&self) -> &'a Identifier {
        self.name
    }

    fn base_uri(&self) -> Option<&'a HeaderValue<Url>> {
        self.base_uri.and_then(|v| Some(v))
    }

    fn version_uri(&self) -> Option<&'a HeaderValue<Url>> {
        self.version_uri.and_then(|v| Some(v))
    }

    fn children(&self) -> Option<&Vec<DependencyNode<'a>>> {
        self.children.as_ref()
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod write;
