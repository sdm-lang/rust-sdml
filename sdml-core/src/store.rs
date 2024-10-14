/*!
This module provides a trait for module *stores*, and an implementation for in-memory caches.

# Example

```
use sdml_core::model::identifiers::Identifier;
use sdml_core::store::{InMemoryModuleCache, ModuleStore};
use std::str::FromStr;

let store = InMemoryModuleCache::default().with_stdlib();

let xml_schema_module = Identifier::from_str("xsd").unwrap();

assert_eq!(true, store.contains(&xml_schema_module));
```

*/

use crate::model::definitions::Definition;
use crate::model::identifiers::{Identifier, IdentifierReference, QualifiedIdentifier};
use crate::model::modules::Module;
use crate::model::HasName;
use crate::stdlib;
use std::collections::HashMap;
use url::Url;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// A trait for any type that /stores/ modules and can retrieve them by name and by URI.
///
pub trait ModuleStore {
    ///
    /// Return the number of modules in the store.
    ///
    fn len(&self) -> usize;

    ///
    /// Return `true` if there are no modules in this store, else `false`.
    ///
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    ///
    /// Returns `true` if the loader's cache contains a module with the name `name`, else `false`.
    ///
    fn contains(&self, name: &Identifier) -> bool;

    ///
    /// Returns `true` if the loader's cache contains a module with the base URI `uri`, else `false`.
    ///
    fn contains_by_uri(&self, uri: &Url) -> bool {
        let name = self.uri_to_module_name(uri).cloned();
        if let Some(name) = name {
            self.contains(&name)
        } else {
            false
        }
    }

    ///
    /// Returns a reference to the `Module` identified by `name` if the store contains it;
    /// else `None`.
    ///
    fn get(&self, name: &Identifier) -> Option<&Module>;

    ///
    /// Returns a mutable reference to the `Module` identified by `name` if the store contains it;
    /// else `None`.
    ///
    fn get_mut(&mut self, name: &Identifier) -> Option<&mut Module>;

    ///
    /// Returns a reference to the `Module` identified by `uri` if the store contains it;
    /// else `None`.
    ///
    fn get_by_uri(&self, uri: &Url) -> Option<&Module> {
        self.uri_to_module_name(uri).and_then(|name| self.get(name))
    }

    ///
    /// Returns a mutable reference to the `Module` identified by `uri` if the store contains it;
    /// else `None`.
    ///
    fn get_by_uri_mut(&mut self, uri: &Url) -> Option<&mut Module> {
        let name = self.uri_to_module_name(uri).cloned();
        if let Some(name) = name {
            self.get_mut(&name)
        } else {
            None
        }
    }

    ///
    /// Return an iterator over all modules in this store. This may be an expensive operation if
    /// modules only exist in some backing store.
    ///
    fn modules(&self) -> impl Iterator<Item = &Module>;

    ///
    /// Return an iterator over the names of the modules in this store.
    ///
    fn module_names(&self) -> impl Iterator<Item = &Identifier>;

    ///
    /// Insert `module` into the store.
    ///
    fn insert(&mut self, module: Module);

    ///
    /// Remove any module identified by `name`.
    ///
    fn remove(&mut self, name: &Identifier) -> bool;

    ///
    /// Remove any module identified by `uri`.
    ///
    fn remove_by_uri(&mut self, uri: &Url) -> bool {
        let name = self.uri_to_module_name(uri).cloned();
        if let Some(name) = name {
            self.remove(&name)
        } else {
            false
        }
    }

    ///
    /// Return the module name corresponding to the provided `url` if it exists, or else `None`.
    ///
    fn uri_to_module_name(&self, url: &Url) -> Option<&Identifier>;

    ///
    /// Return the module URI corresponding to the provided `name` if it exists, or else `None`.
    ///
    fn module_name_to_uri(&self, name: &Identifier) -> Option<&Url>;

    ///
    /// Given a qualified identifier, find the named module or return `None`, then find the named
    /// member in the found module or return `None`.
    ///
    /// # Example
    ///
    /// ```
    /// use sdml_core::model::identifiers::QualifiedIdentifier;
    /// use sdml_core::store::{InMemoryModuleCache, ModuleStore};
    /// use std::str::FromStr;
    ///
    /// let cache = InMemoryModuleCache::default().with_stdlib();
    /// let name = QualifiedIdentifier::from_str("xsd:integer").unwrap();
    /// let integer = cache.resolve(&name).unwrap();
    /// println!("{integer:?}");
    /// ```
    ///
    fn resolve(&self, definition: &QualifiedIdentifier) -> Option<&Definition> {
        if let Some(module) = self.get(definition.module()) {
            module.resolve_local(definition.member())
        } else {
            None
        }
    }

    ///
    /// If `definition` is a `QualifiedIdentifier` this is the same as `resolve`; however, if
    /// `definition` is an `Identifier` then look for definition in the module named
    /// `in_module`.
    ///
    /// # Example
    ///
    /// ```
    /// use sdml_core::model::identifiers::{Identifier, IdentifierReference};
    /// use sdml_core::store::{InMemoryModuleCache, ModuleStore};
    /// use std::str::FromStr;
    ///
    /// let cache = InMemoryModuleCache::default().with_stdlib();
    /// let default_module = Identifier::from_str("xsd").unwrap();
    /// let name = IdentifierReference::from_str("integer").unwrap();
    /// let integer = cache.resolve_or_in(&name, &default_module).unwrap();
    /// println!("{integer:?}");
    /// ```
    ///
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

///
/// An implementation of [`ModuleStore`] that has no persistence it simply acts as an in-process
/// cache.
///
#[derive(Clone, Debug, Default)]
pub struct InMemoryModuleCache {
    uri_map: HashMap<Url, Identifier>,
    modules: HashMap<Identifier, Module>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl ModuleStore for InMemoryModuleCache {
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

    fn modules(&self) -> impl Iterator<Item = &Module> {
        self.modules.values()
    }

    fn module_names(&self) -> impl Iterator<Item = &Identifier> {
        self.modules.keys()
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

    fn uri_to_module_name(&self, url: &Url) -> Option<&Identifier> {
        self.uri_map.get(url)
    }

    fn module_name_to_uri(&self, id: &Identifier) -> Option<&Url> {
        self.modules
            .get(id)
            .map(|module| module.base_uri().map(|hv| hv.value()))
            .unwrap_or_default()
    }
}

impl InMemoryModuleCache {
    ///
    /// Construct a cache with all of the standard library modules pre-inserted.
    ///
    pub fn with_stdlib(self) -> Self {
        self.with(stdlib::dc::module())
            // NYI .with(stdlib::dc::am::module())
            .with(stdlib::dc::terms::module())
            // NYI .with(stdlib::dc::types::module())
            .with(stdlib::iso_3166::module())
            .with(stdlib::iso_4217::module())
            .with(stdlib::owl::module())
            .with(stdlib::rdf::module())
            .with(stdlib::rdfs::module())
            .with(stdlib::sdml::module())
            .with(stdlib::skos::module())
            .with(stdlib::xsd::module())
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
