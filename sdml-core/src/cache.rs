/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::model::definitions::Definition;
use crate::model::identifiers::{Identifier, IdentifierReference, QualifiedIdentifier};
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

///
/// A trait for any type that /stores/ modules and can retrieve them by name and by URI.
///
pub trait ModuleStore {
    /// Return the number of modules in the store.
    fn len(&self) -> usize;
    /// Return `true` if there are no modules in this store, else `false`.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns `true` if the loader's cache contains a module with the name `name`, else `false`.
    fn contains(&self, name: &Identifier) -> bool;
    /// Returns `true` if the loader's cache contains a module with the base URI `uri`, else `false`.
    fn contains_by_uri(&self, uri: &Url) -> bool;

    fn get(&self, name: &Identifier) -> Option<&Module>;
    fn get_mut(&mut self, name: &Identifier) -> Option<&mut Module>;
    fn get_by_uri(&self, uri: &Url) -> Option<&Module>;
    fn get_by_uri_mut(&mut self, uri: &Url) -> Option<&mut Module>;

    fn iter(&self) -> impl Iterator<Item = &Module>;

    fn insert(&mut self, module: Module);
    fn remove(&mut self, name: &Identifier) -> bool;
    fn remove_by_uri(&mut self, uri: &Url) -> bool;

    fn identifier_for_url(&self, url: &Url) -> Option<&Identifier>;
    fn url_for_identifier(&self, id: &Identifier) -> Option<&Url>;
    fn url_to_identifier_map(&self) -> impl Iterator<Item = (&Url, &Identifier)>;
    fn identifier_to_url_map(&self) -> impl Iterator<Item = (&Identifier, &Url)>;

    fn resolve(&self, definition: &QualifiedIdentifier) -> Option<&Definition> {
        if let Some(module) = self.get(definition.module()) {
            module.resolve_local(definition.member())
        } else {
            None
        }
    }

    fn resolve_or_in(
        &self,
        definition: &IdentifierReference,
        in_module: &Identifier,
    ) -> Option<&Definition> {
        match definition {
            IdentifierReference::Identifier(v) => self.resolve(&v.with_module(in_module.clone())),
            IdentifierReference::QualifiedIdentifier(v) => self.resolve(v),
        }
    }
}

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

impl ModuleStore for ModuleCache {
    fn len(&self) -> usize {
        self.modules.len()
    }

    fn contains(&self, name: &Identifier) -> bool {
        self.modules.contains_key(name)
    }

    fn contains_by_uri(&self, uri: &Url) -> bool {
        self.uri_map.contains_key(uri)
    }

    fn get(&self, name: &Identifier) -> Option<&Module> {
        self.modules.get(name)
    }

    fn get_mut(&mut self, name: &Identifier) -> Option<&mut Module> {
        self.modules.get_mut(name)
    }

    fn get_by_uri(&self, uri: &Url) -> Option<&Module> {
        match self.uri_map.get(uri) {
            Some(name) => self.get(name),
            _ => None,
        }
    }

    fn get_by_uri_mut(&mut self, uri: &Url) -> Option<&mut Module> {
        let name = self.uri_map.get_mut(uri).map(|n| n.clone());
        match name {
            Some(name) => self.get_mut(&name),
            _ => None,
        }
    }

    fn insert(&mut self, module: Module) {
        if let Some(base_uri) = module.base_uri() {
            self.uri_map
                .insert(base_uri.value().clone(), module.name().clone());
        }
        self.modules.insert(module.name().clone(), module);
    }

    fn remove(&mut self, name: &Identifier) -> bool {
        if self.modules.remove(name).is_some() {
            self.uri_map.retain(|_, v| v == name);
            true
        } else {
            false
        }
    }

    fn remove_by_uri(&mut self, uri: &Url) -> bool {
        if let Some(name) = self.uri_map.remove(uri) {
            self.modules.remove(&name);
            true
        } else {
            false
        }
    }

    fn iter(&self) -> impl Iterator<Item = &Module> {
        self.modules.values()
    }

    fn identifier_for_url(&self, url: &Url) -> Option<&Identifier> {
        self.uri_map.get(url)
    }

    fn url_for_identifier(&self, id: &Identifier) -> Option<&Url> {
        self.modules
            .get(id)
            .map(|module| module.base_uri().map(|hv| hv.value()))
            .unwrap_or_default()
    }

    fn url_to_identifier_map(&self) -> impl Iterator<Item = (&Url, &Identifier)> {
        self.uri_map.iter()
    }

    fn identifier_to_url_map(&self) -> impl Iterator<Item = (&Identifier, &Url)> {
        self.uri_map.iter().map(|(url, id)| (id, url))
    }
}

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
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
