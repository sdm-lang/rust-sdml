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
use std::io::Read;
use std::path::{Path, PathBuf};

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
    original: String,
    report_source: Option<ariadne::Source>,
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

impl LoadedModule {
    pub fn new<S>(source: S) -> Result<Self, Error>
    where
        S: Into<String>,
    {
        Ok(Self {
            path: Default::default(),
            original: source.into(),
            report_source: Default::default(),
            parsed: Default::default(),
        })
    }

    pub fn new_from_path<P>(path: P) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        let source = std::fs::read_to_string(path.as_ref())?;
        let mut new_self = Self::new(source)?;
        new_self.path = Some(PathBuf::from(path.as_ref()));
        Ok(new_self)
    }

    pub fn new_from_stdin() -> Result<Self, Error> {
        let stdin = std::io::stdin();
        let mut handle = stdin.lock();
        Self::new_from_handle(&mut handle)
    }

    fn new_from_handle<R>(handle: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        let mut source = String::new();
        handle.read_to_string(&mut source)?;
        Self::new(source)
    }

    pub fn path(&self) -> Option<&PathBuf> {
        self.path.as_ref()
    }

    pub fn original_source(&self) -> &String {
        &self.original
    }

    pub fn reporter_source(&mut self) -> &Source {
        if self.report_source.is_none() {
            self.report_source = Some(Source::from(&self.original));
        }
        self.report_source.as_ref().unwrap()
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
            let resolved = LoadedModule::new(self.resolve_module_source(name)?)?;
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
