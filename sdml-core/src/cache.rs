/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::model::identifiers::Identifier;
use crate::model::modules::Module;
use crate::model::HasName;
use crate::stdlib;
use std::collections::HashMap;
use url::Url;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, Default)]
pub struct ModuleCache {
    uri_map: HashMap<Url, Identifier>,
    modules: HashMap<Identifier, Module>,
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

impl ModuleCache {
    ///
    /// Construct a cache with all of the standard library modules pre-inserted.
    ///
    pub fn with_stdlib(self) -> Self {
        let mut self_mut = self;
        self_mut.insert(stdlib::dc::module());
        self_mut.insert(stdlib::dc::terms::module());
        self_mut.insert(stdlib::iso_3166::module());
        self_mut.insert(stdlib::iso_4217::module());
        self_mut.insert(stdlib::owl::module());
        self_mut.insert(stdlib::rdf::module());
        self_mut.insert(stdlib::rdfs::module());
        self_mut.insert(stdlib::sdml::module());
        self_mut.insert(stdlib::skos::module());
        self_mut.insert(stdlib::xsd::module());
        self_mut
    }

    ///
    /// Builder-like function to add a module to a newly constructed cache.
    ///
    pub fn with(self, module: Module) -> Self {
        let mut self_mut = self;
        self_mut.insert(module);
        self_mut
    }

    pub fn len(&self) -> usize {
        self.modules.len()
    }

    pub fn is_empty(&self) -> bool {
        self.modules.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Module> {
        self.modules.values()
    }

    /// Returns `true` if the loader's cache contains a module with the name `name`, else `false`.
    pub fn contains(&self, name: &Identifier) -> bool {
        self.modules.contains_key(name)
    }

    pub fn contains_by_uri(&self, uri: &Url) -> bool {
        self.uri_map.contains_key(uri)
    }

    pub fn insert(&mut self, module: Module) {
        if let Some(base_uri) = module.base_uri() {
            self.uri_map
                .insert(base_uri.value().clone(), module.name().clone());
        }
        self.modules.insert(module.name().clone(), module);
    }

    pub fn get(&self, name: &Identifier) -> Option<&Module> {
        self.modules.get(name)
    }

    pub fn get_mut(&mut self, name: &Identifier) -> Option<&mut Module> {
        self.modules.get_mut(name)
    }

    pub fn get_by_uri(&self, uri: &Url) -> Option<&Module> {
        match self.uri_map.get(uri) {
            Some(name) => self.get(name),
            _ => None,
        }
    }

    pub fn get_by_uri_mut(&mut self, uri: &Url) -> Option<&mut Module> {
        let name = self.uri_map.get_mut(uri).map(|n| n.clone());
        match name {
            Some(name) => self.get_mut(&name),
            _ => None,
        }
    }

    pub fn identifier_for_url(&self, url: &Url) -> Option<&Identifier> {
        self.uri_map.get(url)
    }

    pub fn url_for_identifier(&self, id: &Identifier) -> Option<&Url> {
        self.modules
            .get(id)
            .map(|module| module.base_uri().map(|hv| hv.value()))
            .unwrap_or_default()
    }

    pub fn url_to_identifier_map(&self) -> impl Iterator<Item = (&Url, &Identifier)> {
        self.uri_map.iter()
    }

    pub fn identifier_to_url_map(&self) -> impl Iterator<Item = (&Identifier, &Url)> {
        self.uri_map.iter().map(|(url, id)| (id, url))
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
