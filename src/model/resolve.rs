/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::error::{module_file_not_found, Error};
use crate::model::Identifier;
use search_path::SearchPath;
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
    pub fn no_path() -> Self {
        Self {
            search_path: Default::default(),
        }
    }

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
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
