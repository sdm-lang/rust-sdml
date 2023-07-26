/*!
One-line description.

More detailed description, with

# Example

YYYYY

 */

use crate::parse::parse_str;
use codespan_reporting::files::SimpleFiles;
use sdml_core::error::{module_file_not_found, Error};
use sdml_core::load::{ModuleLoader as LoaderTrait, ModuleResolver as ResolverTrait};
use sdml_core::model::{Identifier, Module};
use sdml_core::{get, get_and_mutate, get_and_mutate_map_of, is_as_variant};
use search_path::SearchPath;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use tracing::{error, info, trace, warn};
use url::Url;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct ModuleResolver {
    catalog: Option<Rc<ModuleCatalog>>,
    search_path: SearchPath,
}

pub const SDML_RESOLVER_PATH_VARIABLE: &str = "SDML_PATH";

pub const SDML_FILE_EXTENSION: &str = "sdm";
pub const SDML_FILE_EXTENSION_LONG: &str = "sdml";

pub const SDML_CATALOG_FILE_NAME: &str = "sdml-catalog.json";

// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct ModuleLoader {
    resolver: ModuleResolver,
    modules: HashMap<Identifier, (Module, usize)>,
    module_files: SimpleFiles<String, String>,
}

// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct ModuleCatalog {
    base: Url,
    #[serde(skip)]
    loaded_from: PathBuf,
    entries: HashMap<String, CatalogEntry>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CatalogEntry {
    Group(Group),
    Item(Item),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Group {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    relative_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    relative_path: Option<PathBuf>,
    entries: HashMap<String, Item>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Item {
    relative_url: String,
    relative_path: PathBuf,
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
        let catalog = ModuleCatalog::load_from_current(true);
        Self {
            catalog: catalog.map(|c| Rc::new(c)),
            search_path,
        }
    }
}

impl ResolverTrait for ModuleResolver {
    fn prepend_to_search_path(&mut self, path: &Path) {
        self.search_path.prepend(PathBuf::from(path));
    }

    fn append_to_search_path(&mut self, path: &Path) {
        self.search_path.append(PathBuf::from(path));
    }

    fn name_to_path(&self, name: &Identifier) -> Result<PathBuf, Error> {
        trace!("ModuleResolver::name_to_path({name:?})");
        if let Some(catalog) = &self.catalog {
            let name: String = name.to_string();
            if let Some(path) = catalog.resolve_local_path(&name) {
                trace!("Found module in catalog, path: {path:?}");
                return Ok(path);
            }
        }
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

impl LoaderTrait for ModuleLoader {
    fn load(&mut self, name: &Identifier) -> Result<&Module, Error> {
        let exists = self.modules.contains_key(name);
        if exists {
            Ok(self.get(name).unwrap())
        } else {
            let file = self.resolver.name_to_path(name)?;
            self.load_from_file(file)
        }
    }

    fn load_from_file(&mut self, file: PathBuf) -> Result<&Module, Error> {
        let mut reader = File::open(&file)?;
        let catalog = self.resolver.catalog.clone();
        let module = self.load_inner(&mut reader, Some(file.clone()))?;
        if !module.has_base() {
            if let Some(catalog) = catalog {
                let name = module.name().to_string();
                if let Some(url) = catalog.resolve_uri(&name) {
                    module.set_base(url);
                }
            } else {
                module.set_base(Url::from_file_path(file).unwrap())
            }
        }
        Ok(module)
    }

    fn load_from_reader(&mut self, reader: &mut dyn Read) -> Result<&Module, Error> {
        Ok(self.load_inner(reader, None)?)
    }

    fn contains(&self, name: &Identifier) -> bool {
        self.modules.contains_key(name)
    }

    fn get(&self, name: &Identifier) -> Option<&Module> {
        self.modules.get(name).map(|m| &m.0)
    }

    fn get_source(&self, name: &Identifier) -> Option<&String> {
        if let Some(module) = self.modules.get(name) {
            Some(self.module_files.get(module.1).unwrap().source())
        } else {
            None
        }
    }

    fn resolver(&self) -> &dyn ResolverTrait {
        &self.resolver
    }
}

impl ModuleLoader {
    fn load_inner(
        &mut self,
        reader: &mut dyn Read,
        file: Option<PathBuf>,
    ) -> Result<&mut Module, Error> {
        let mut source = String::new();
        reader.read_to_string(&mut source)?;
        let file_name: String = file
            .map(|p| p.to_string_lossy().into_owned())
            .unwrap_or_default();
        // Insert into the codespan cache
        let file_id = self.module_files.add(file_name, source);

        let (module, counters) = parse_str(file_id, self)?.into_inner();

        let name = module.name().clone();
        counters.display(&name)?;

        // save codespan file ID with module
        let _ = self.modules.insert(name.clone(), (module, file_id));
        Ok(self.get_mut(&name).unwrap())
    }

    fn get_mut(&mut self, name: &Identifier) -> Option<&mut Module> {
        self.modules.get_mut(name).map(|m| &mut m.0)
    }

    pub(crate) fn files(&self) -> &SimpleFiles<String, String> {
        &self.module_files
    }
}

// ------------------------------------------------------------------------------------------------

impl ModuleCatalog {
    pub fn load_from_current(look_in_parents: bool) -> Option<Self> {
        let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        Self::load_from(&cwd, look_in_parents)
    }

    pub fn load_from(path: &Path, look_in_parents: bool) -> Option<Self> {
        trace!("ModuleCatalog::load_from({path:?}, {look_in_parents})");
        if path.is_file() {
            Self::load_from_file(path)
        } else if path.is_dir() {
            let file = path.join(SDML_CATALOG_FILE_NAME);
            if file.is_file() {
                Self::load_from_file(&file)
            } else if look_in_parents {
                if let Some(parent_path) = path.parent() {
                    Self::load_from(parent_path, look_in_parents)
                } else {
                    warn!("No catalog file found in file-system parent path");
                    None
                }
            } else {
                warn!("No catalog found in provided directory");
                None
            }
        } else {
            warn!("The provided path was not a file or directory");
            None
        }
    }

    fn load_from_file(file: &Path) -> Option<Self> {
        match std::fs::read_to_string(file) {
            Ok(source) => match serde_json::from_str::<ModuleCatalog>(&source) {
                Ok(mut catalog) => {
                    catalog.loaded_from = file.parent().unwrap().to_path_buf();
                    info!("Loaded catalog, file: {file:?}");
                    Some(catalog)
                }
                Err(e) => {
                    error!("Error parsing catalog, file: {file:?}, error: {e}");
                    None
                }
            },
            Err(e) => {
                error!("Error reading catalog, file: {file:?}, error: {e}");
                None
            }
        }
    }

    get_and_mutate!(pub base => Url);
    get!(pub loaded_from => PathBuf);
    get_and_mutate_map_of!(pub entries => HashMap, String, CatalogEntry);

    pub fn groups(&self) -> impl Iterator<Item = (&String, &Group)> {
        self.entries.iter().filter_map(|(k, e)| {
            if let Some(group) = e.as_group() {
                Some((k, group))
            } else {
                None
            }
        })
    }

    pub fn items(&self) -> impl Iterator<Item = (&String, &Item)> {
        self.entries.iter().filter_map(|(k, e)| {
            if let Some(item) = e.as_item() {
                Some((k, item))
            } else {
                None
            }
        })
    }

    pub fn resolve_uri(&self, module: &String) -> Option<Url> {
        if let Some(CatalogEntry::Item(item)) = self.get_from_entries(module) {
            Some(self.base.join(item.relative_url().as_str()).unwrap())
        } else {
            self.groups()
                .find(|(_, g)| g.entries_contains_key(module))
                .map(|(_, g)| g.resolve_uri(&self.base, module))
                .unwrap_or_default()
        }
    }

    pub fn resolve_local_path(&self, module: &String) -> Option<PathBuf> {
        if let Some(CatalogEntry::Item(item)) = self.get_from_entries(module) {
            Some(self.loaded_from.join(item.relative_path()))
        } else {
            self.groups()
                .find(|(_, g)| g.entries_contains_key(module))
                .map(|(_, g)| g.resolve_local_path(&self.loaded_from, module))
                .unwrap_or_default()
        }
    }
}

impl CatalogEntry {
    is_as_variant!(pub group => Group, Group);
    is_as_variant!(pub item => Item, Item);
}

impl Group {
    get_and_mutate!(pub relative_name => option String);
    get_and_mutate!(pub relative_path => option PathBuf);
    get_and_mutate_map_of!(pub entries => HashMap, String, Item);

    pub fn resolve_uri(&self, base: &Url, module: &String) -> Option<Url> {
        let base = if let Some(relative_url) = &self.relative_name {
            base.join(relative_url.as_str()).unwrap()
        } else {
            base.clone()
        };
        if let Some(item) = self.get_from_entries(module) {
            Some(base.join(item.relative_url().as_str()).unwrap())
        } else {
            None
        }
    }

    pub fn resolve_local_path(&self, base: &Path, module: &String) -> Option<PathBuf> {
        let base = if let Some(group_base) = &self.relative_path {
            base.join(group_base)
        } else {
            base.to_path_buf()
        };
        if let Some(item) = self.get_from_entries(module) {
            Some(base.join(item.relative_url().as_str()))
        } else {
            None
        }
    }
}

impl Item {
    get_and_mutate!(pub relative_path => PathBuf);
    get_and_mutate!(pub relative_url => String);
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_catalog() {
        let catalog = ModuleCatalog {
            base: Url::parse("https://example.org/schema/").unwrap(),
            loaded_from: PathBuf::from("."),
            entries: vec![(
                String::from("rentals"),
                CatalogEntry::Item(Item {
                    relative_url: String::from("rentals/v1/"),
                    relative_path: PathBuf::from("examples/rentals.sdm"),
                }),
            )]
            .into_iter()
            .collect(),
        };
        println!("{}", serde_json::to_string_pretty(&catalog).unwrap());
    }

    #[test]
    fn test_parse_catalog() {
        let _: ModuleCatalog = serde_json::from_str(
            r##"{
  "base": "https://example.org/rentals/",
  "entries": {
    "vehicle": {
      "item": {
        "relative_url": "vehicle#",
        "relative_path": "vehicle-v1.sdm"
      }
    }
  }
}"##,
        )
        .unwrap();
    }

    #[test]
    fn test_parse_catalog_with_group() {
        let _: ModuleCatalog = serde_json::from_str(
            r##"{
  "base": "https://example.org/rentals/",
  "entries": {
    "rentals": {
      "group": {
        "relative_name": "entities/",
        "relative_path": "/entities-v1",
        "entries": {
            "item": {
              "relative_url": "vehicle#",
              "relative_path": "vehicle-v1.sdm"
          }
        }
      }
    }
  }
}"##,
        )
        .unwrap();
    }
}
