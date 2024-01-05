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
    pub fn with_stdlib(self) -> Self {
        let mut self_mut = self;
        self_mut.insert(stdlib::dc());
        self_mut.insert(stdlib::rdf());
        self_mut.insert(stdlib::rdfs());
        self_mut.insert(stdlib::sdml());
        self_mut.insert(stdlib::skos());
        self_mut.insert(stdlib::xsd());
        self_mut
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
            self.uri_map.insert(base_uri.clone(), module.name().clone());
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
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
