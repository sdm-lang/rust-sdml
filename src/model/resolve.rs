/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::error::{module_file_not_found, Error};
use crate::model::{Identifier, Module};
use ariadne::Source;
use search_path::SearchPath;
use std::collections::HashMap;
use std::path::PathBuf;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct Resolver {
    search_path: SearchPath,
    modules: HashMap<Identifier, ResolvedModule>,
}

#[derive(Clone, Debug)]
pub struct ResolvedModule {
    source: Source,
    parsed: Option<Module>,
}

pub const SDML_RESOLVER_PATH_VARIABLE: &str = "SDML_PATH";

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

impl From<Source> for ResolvedModule {
    fn from(source: Source) -> Self {
        Self {
            source,
            parsed: None,
        }
    }
}

impl From<String> for ResolvedModule {
    fn from(v: String) -> Self {
        Self::from(Source::from(v))
    }
}

impl ResolvedModule {
    pub fn source(&self) -> &Source {
        &self.source
    }

    pub fn parsed_module(&self) -> Option<&Module> {
        self.parsed.as_ref()
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for Resolver {
    fn default() -> Self {
        let mut search_path = SearchPath::new_or_default(SDML_RESOLVER_PATH_VARIABLE);
        search_path.prepend_cwd();
        Self {
            search_path,
            modules: Default::default(),
        }
    }
}

impl Resolver {
    pub fn no_path() -> Self {
        Self {
            search_path: Default::default(),
            modules: Default::default(),
        }
    }

    pub fn prepend(self, path: PathBuf) -> Self {
        let mut self_mut = self;
        self_mut.search_path.prepend(path);
        self_mut
    }

    pub fn append(self, path: PathBuf) -> Self {
        let mut self_mut = self;
        self_mut.search_path.append(path);
        self_mut
    }

    pub fn resolve_module_path(&self, name: &Identifier) -> Result<PathBuf, Error> {
        self.search_path
            .find(format!("{}.sdm", name).as_ref())
            .or_else(|| {
                self.search_path
                    .find(format!("{}/{}.sdm", name, name).as_ref())
                    .or_else(|| {
                        self.search_path
                            .find(format!("{}.sdml", name).as_ref())
                            .or_else(|| {
                                self.search_path
                                    .find(format!("{}/{}.sdml", name, name).as_ref())
                            })
                    })
            })
            .ok_or_else(|| module_file_not_found(name.clone()))
    }

    pub fn resolve_module_source(&self, name: &Identifier) -> Result<String, Error> {
        Ok(std::fs::read_to_string(self.resolve_module_path(name)?)?)
    }

    pub fn resolve_module(&mut self, name: &Identifier) -> Result<&ResolvedModule, Error> {
        if self.modules.contains_key(name) {
            Ok(self.modules.get(name).unwrap())
        } else {
            let resolved = ResolvedModule::from(self.resolve_module_source(name)?);
            self.modules.insert(name.clone(), resolved);
            Ok(self.modules.get(name).unwrap())
        }
    }

    pub fn insert(&mut self, name: Identifier, source: Source) -> Option<ResolvedModule> {
        self.modules.insert(
            name,
            ResolvedModule {
                source,
                parsed: None,
            },
        )
    }

    pub fn insert_with_module(
        &mut self,
        name: Identifier,
        source: Source,
        parsed: Module,
    ) -> Option<ResolvedModule> {
        self.modules.insert(
            name,
            ResolvedModule {
                source,
                parsed: Some(parsed),
            },
        )
    }

    pub fn get(&self, name: &Identifier) -> Option<&ResolvedModule> {
        self.modules.get(name)
    }

    pub fn source(&self, name: &Identifier) -> Option<&Source> {
        self.get(name).map(|resolved| resolved.source())
    }

    pub fn module(&self, name: &Identifier) -> Option<Option<&Module>> {
        self.get(name).map(|resolved| resolved.parsed_module())
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
