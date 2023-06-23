/*!
One-line description.

More detailed description, with

# Example

YYYYY

 */

use crate::error::Error;
use crate::model::parse::parse_str;
use crate::model::resolve::ModuleResolver;
use crate::model::{Identifier, Module};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, Default)]
pub struct ModuleLoader {
    resolver: ModuleResolver,
    modules: HashMap<Identifier, LoadedModule>,
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

#[derive(Clone, Debug)]
struct LoadedModule {
    path: Option<PathBuf>,
    original: String,
    module: Module,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl ModuleLoader {
    pub fn new(resolver: ModuleResolver) -> Self {
        Self {
            resolver,
            modules: Default::default(),
        }
    }

    pub fn load(&mut self, name: &Identifier) -> Result<&Module, Error> {
        let path = self.resolver.name_to_path(name)?;
        let mut file = File::open(&path)?;
        self.load_inner(&mut file, Some(path))
    }

    pub fn load_from_reader<R>(&mut self, reader: &mut R) -> Result<&Module, Error>
    where
        R: Read,
    {
        self.load_inner(reader, None)
    }

    fn load_inner<R>(&mut self, reader: &mut R, path: Option<PathBuf>) -> Result<&Module, Error>
    where
        R: Read,
    {
        let mut original = String::new();
        reader.read_to_string(&mut original)?;
        let module = parse_str(&original, path.as_ref().map(|p|p.to_string_lossy().into_owned()), true)?;
        let name = module.name().clone();
        let loaded = LoadedModule {
            path,
            original,
            module,
        };
        let _ = self.insert(name.clone(), loaded);
        Ok(self.get_loaded_module(&name).unwrap())
    }

    fn insert(&mut self, name: Identifier, module: LoadedModule) -> Option<LoadedModule> {
        self.modules.insert(name, module)
    }

    pub fn is_module_loaded(&self, name: &Identifier) -> bool {
        self.modules.contains_key(name)
    }

    pub fn get_loaded_module_path(&self, name: &Identifier) -> Option<&PathBuf> {
        self.modules.get(name).map(|m| m.path()).unwrap_or_default()
    }

    pub fn get_loaded_module_source(&self, name: &Identifier) -> Option<&String> {
        self.modules.get(name).map(|m| m.original_source())
    }

    pub fn get_loaded_module(&self, name: &Identifier) -> Option<&Module> {
        self.modules.get(name).map(|m| m.module())
    }

    pub fn resolver(&self) -> &ModuleResolver {
        &self.resolver
    }
}

// ------------------------------------------------------------------------------------------------

impl LoadedModule {
    fn path(&self) -> Option<&PathBuf> {
        self.path.as_ref()
    }

    fn original_source(&self) -> &String {
        &self.original
    }

    fn module(&self) -> &Module {
        &self.module
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
