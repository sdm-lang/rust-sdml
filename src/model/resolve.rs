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
pub struct ModuleResolver {
    search_path: SearchPath,
    modules: HashMap<Identifier, LoadedModule>,
}

#[derive(Clone, Debug)]
pub struct LoadedModule {
    path: Option<PathBuf>,
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

impl From<Source> for LoadedModule {
    fn from(source: Source) -> Self {
        Self {
            path: None,
            source,
            parsed: None,
        }
    }
}

impl From<String> for LoadedModule {
    fn from(v: String) -> Self {
        Self::from(Source::from(v))
    }
}

impl LoadedModule {
    pub fn path(&self) -> Option<&PathBuf> {
        self.path.as_ref()
    }

    pub fn source(&self) -> &Source {
        &self.source
    }

    pub fn parsed_module(&self) -> Option<&Module> {
        self.parsed.as_ref()
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for ModuleResolver {
    fn default() -> Self {
        let mut search_path = SearchPath::new_or_default(SDML_RESOLVER_PATH_VARIABLE);
        search_path.prepend_cwd();
        Self {
            search_path,
            modules: Default::default(),
        }
    }
}

impl ModuleResolver {
    pub fn no_path() -> Self {
        Self {
            search_path: Default::default(),
            modules: Default::default(),
        }
    }

    pub fn prepend_to_search_path(&mut self, path: PathBuf) {
        self.search_path.prepend(path);
    }

    pub fn append_to_search_path(&mut self, path: PathBuf) {
        self.search_path.append(path);
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

    pub fn resolve_module(&mut self, name: &Identifier) -> Result<&LoadedModule, Error> {
        if self.modules.contains_key(name) {
            Ok(self.modules.get(name).unwrap())
        } else {
            let resolved = LoadedModule::from(self.resolve_module_source(name)?);
            self.modules.insert(name.clone(), resolved);
            Ok(self.modules.get(name).unwrap())
        }
    }

    pub fn insert(&mut self, name: Identifier, module: LoadedModule) -> Option<LoadedModule> {
        self.modules.insert(name, module)
    }

    pub fn get(&self, name: &Identifier) -> Option<&LoadedModule> {
        self.modules.get(name)
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
