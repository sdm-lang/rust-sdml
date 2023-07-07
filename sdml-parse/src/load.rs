/*!
One-line description.

More detailed description, with

# Example

YYYYY

 */

use crate::parse::parse_str;
use codespan_reporting::files::SimpleFiles;
use sdml_core::error::{module_file_not_found, Error};
use sdml_core::model::{Identifier, Module};
use search_path::SearchPath;
use std::collections::HashMap;
use std::fs::File;
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
}

pub const SDML_RESOLVER_PATH_VARIABLE: &str = "SDML_PATH";

pub const SDML_FILE_EXTENSION: &str = "sdm";
pub const SDML_FILE_EXTENSION_LONG: &str = "sdml";

// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct ModuleLoader {
    resolver: ModuleResolver,
    modules: HashMap<Identifier, (Module, usize)>,
    module_files: SimpleFiles<String, String>,
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

impl Default for ModuleResolver {
    fn default() -> Self {
        let mut search_path = SearchPath::new_or_default(SDML_RESOLVER_PATH_VARIABLE);
        search_path.prepend_cwd();
        Self { search_path }
    }
}

impl ModuleResolver {
    pub fn prepend_to_search_path<P>(&mut self, path: P)
    where
        P: AsRef<Path>,
    {
        self.search_path.prepend(PathBuf::from(path.as_ref()));
    }

    pub fn append_to_search_path<P>(&mut self, path: P)
    where
        P: AsRef<Path>,
    {
        self.search_path.append(PathBuf::from(path.as_ref()));
    }

    pub fn name_to_path(&self, name: &Identifier) -> Result<PathBuf, Error> {
        self.search_path
            .find(format!("{}.{}", name, SDML_FILE_EXTENSION).as_ref())
            .or_else(|| {
                self.search_path
                    .find(format!("{}/{}.{}", name, name, SDML_FILE_EXTENSION).as_ref())
                    .or_else(|| {
                        self.search_path
                            .find(format!("{}.{}", name, SDML_FILE_EXTENSION_LONG).as_ref())
                            .or_else(|| {
                                self.search_path.find(
                                    format!("{}/{}.{}", name, name, SDML_FILE_EXTENSION_LONG)
                                        .as_ref(),
                                )
                            })
                    })
            })
            .ok_or_else(|| module_file_not_found(name.clone()))
    }
}

// ------------------------------------------------------------------------------------------------

impl From<ModuleResolver> for ModuleLoader {
    fn from(resolver: ModuleResolver) -> Self {
        Self {
            resolver,
            modules: Default::default(),
            module_files: SimpleFiles::new(),
        }
    }
}

impl Default for ModuleLoader {
    fn default() -> Self {
        Self {
            resolver: Default::default(),
            modules: Default::default(),
            module_files: SimpleFiles::new(),
        }
    }
}

impl ModuleLoader {
    pub fn load(&mut self, name: &Identifier) -> Result<&Module, Error> {
        let exists = self.modules.contains_key(name);
        if exists {
            Ok(self.get(name).unwrap())
        } else {
            let file = self.resolver.name_to_path(name)?;
            self.load_from_file(file)
        }
    }

    pub fn load_from_file(&mut self, file: PathBuf) -> Result<&Module, Error> {
        let mut reader = File::open(&file)?;
        self.load_inner(&mut reader, Some(file))
    }

    pub fn load_from_reader<R>(&mut self, reader: &mut R) -> Result<&Module, Error>
    where
        R: Read,
    {
        self.load_inner(reader, None)
    }

    fn load_inner<R>(&mut self, reader: &mut R, file: Option<PathBuf>) -> Result<&Module, Error>
    where
        R: Read,
    {
        let mut source = String::new();
        reader.read_to_string(&mut source)?;
        let file_name: String = file
            .map(|p| p.to_string_lossy().into_owned())
            .unwrap_or_default();
        let file_id = self.module_files.add(file_name, source);

        let (module, counters) = parse_str(file_id, self)?.into_inner();

        let name = module.name().clone();
        counters.display(&name)?;

        let _ = self.modules.insert(name.clone(), (module, file_id));
        Ok(self.get(&name).unwrap())
    }

    pub fn contains(&self, name: &Identifier) -> bool {
        self.modules.contains_key(name)
    }

    pub fn get(&self, name: &Identifier) -> Option<&Module> {
        self.modules.get(name).map(|m| &m.0)
    }

    pub fn resolver(&self) -> &ModuleResolver {
        &self.resolver
    }

    pub(crate) fn files(&self) -> &SimpleFiles<String, String> {
        &self.module_files
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
