/*!
One-line description.

More detailed description, with

# Example

YYYYY

 */

use crate::parse::parse_str;
use codespan_reporting::files::SimpleFiles;
use sdml_core::error::{module_file_not_found, Error};
use sdml_core::load::{ModuleLoader as LoaderTrait, ModuleResolver as ResolverTrait, ModuleRef};
use sdml_core::model::identifiers::Identifier;
use sdml_core::model::HasName;
use search_path::SearchPath;
use serde::{Deserialize, Serialize};
use std::cell::{RefCell, Ref};
use std::collections::HashMap;
use std::fmt::Display;
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
    catalog: Option<Rc<RefCell<ModuleCatalog>>>,
    search_path: Rc<RefCell<SearchPath>>,
}

pub const SDML_RESOLVER_PATH_VARIABLE: &str = "SDML_PATH";

pub const SDML_FILE_EXTENSION: &str = "sdm";
pub const SDML_FILE_EXTENSION_LONG: &str = "sdml";

pub const SDML_CATALOG_FILE_NAME: &str = "sdml-catalog.json";

// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct Source(Rc<String>);

#[derive(Clone, Debug)]
pub struct ModuleLoader {
    resolver: Rc<ModuleResolver>,
    modules: Rc<RefCell<HashMap<Identifier, (ModuleRef, usize)>>>,
    module_files: Rc<RefCell<SimpleFiles<String, Source>>>,
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
    relative_url: Option<String>,
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

impl Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for Source {
    fn from(value: String) -> Self {
        Self(Rc::new(value))
    }
}

impl AsRef<str> for Source {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<[u8]> for Source {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl Source {
    fn as_str(&self) -> &str {
        self.0.as_str()
    }
    fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for ModuleResolver {
    fn default() -> Self {
        let mut search_path = SearchPath::new_or_default(SDML_RESOLVER_PATH_VARIABLE);
        search_path.prepend_cwd();
        let catalog = ModuleCatalog::load_from_current(true);
        Self {
            catalog: catalog.map(|v|Rc::new(RefCell::new(v))),
            search_path: Rc::new(RefCell::new(search_path)),
        }
    }
}

impl ResolverTrait for ModuleResolver {
    fn prepend_to_search_path(&self, path: &Path) {
        self.search_path.borrow_mut().append(PathBuf::from(path));
    }

    fn append_to_search_path(&self, path: &Path) {
        self.search_path.borrow_mut().append(PathBuf::from(path));
    }

    fn name_to_path(&self, name: &Identifier) -> Result<PathBuf, Error> {
        trace!("ModuleResolver::name_to_path({name:?})");
        if let Some(catalog) = &self.catalog {
            let name: String = name.to_string();
            if let Some(path) = catalog.borrow().resolve_local_path(&name) {
                trace!("Found module in catalog, path: {path:?}");
                return Ok(path);
            }
        }
        self.search_path
            .borrow()
            .find(format!("{}.{}", name, SDML_FILE_EXTENSION).as_ref())
            .or_else(|| {
                self.search_path
                    .borrow()
                    .find(format!("{}/{}.{}", name, name, SDML_FILE_EXTENSION).as_ref())
                    .or_else(|| {
                        self.search_path
                            .borrow()
                            .find(format!("{}.{}", name, SDML_FILE_EXTENSION_LONG).as_ref())
                            .or_else(|| {
                                self.search_path
                                    .borrow()
                                    .find(
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
        Self::from(Rc::new(resolver))
    }
}

impl From<Rc<ModuleResolver>> for ModuleLoader {
    fn from(resolver: Rc<ModuleResolver>) -> Self {
        Self {
            resolver,
            modules: Rc::new(RefCell::new(Default::default())),
            module_files: Rc::new(RefCell::new(SimpleFiles::new())),
        }
    }
}

impl Default for ModuleLoader {
    fn default() -> Self {
        Self {
            resolver: Rc::new(Default::default()),
            modules: Rc::new(RefCell::new(Default::default())),
            module_files: Rc::new(RefCell::new(SimpleFiles::new())),
        }
    }
}

impl LoaderTrait for ModuleLoader {
    fn load(&self, name: &Identifier) -> Result<ModuleRef, Error> {
        let exists = self.modules.borrow().contains_key(name);
        if exists {
            Ok(self.get(name).unwrap())
        } else {
            let file = self.resolver.name_to_path(name)?;
            self.load_from_file(file)
        }
    }

    fn load_from_file(&self, file: PathBuf) -> Result<ModuleRef, Error> {
        let mut reader = File::open(&file)?;
        let catalog = self.resolver.catalog.clone();
        let module = self.load_inner(&mut reader, Some(file.clone()))?;
        if !module.borrow().has_base() {
            if let Some(catalog) = catalog {
                let name = module.borrow().name().to_string();
                if let Some(url) = catalog.borrow().resolve_uri(&name) {
                    module.borrow_mut().set_base(url);
                }
            } else {
                let file = file.canonicalize()?;
                match Url::from_file_path(file) {
                    Ok(base) => module.borrow_mut().set_base(base),
                    Err(_) => warn!("Could not construct a base URI"),
                }
            }
        }
        Ok(module)
    }

    fn load_from_reader(&self, reader: &mut dyn Read) -> Result<ModuleRef, Error> {
        Ok(self.load_inner(reader, None)?)
    }

    fn adopt(&self, module: ModuleRef) {
        let name = module.borrow().name().clone();
        let _ = self.modules.borrow_mut().insert(name, (module, 0));
    }

    fn contains(&self, name: &Identifier) -> bool {
        self.modules.borrow().contains_key(name)
    }

    fn get(&self, name: &Identifier) -> Option<ModuleRef> {
        self.modules.borrow().get(name).map(|m| m.0.clone())
    }

    fn get_source(&self, name: &Identifier) -> Option<Box<dyn AsRef<str>>> {
        if let Some(module) = self.modules.borrow().get(name) {
            if module.1 == 0 {
                None
            } else {
                match self.module_files.borrow().get(module.1 - 1) {
                    Ok(file) => Some(Box::new(file.source().clone())),
                    Err(err) => {
                        error!("Could not retrieve module: {module:?}, error: {err}");
                        None
                    },
                }
            }
        } else {
            None
        }
    }

    fn resolver(&self) -> Rc<dyn ResolverTrait> {
        self.resolver.clone()
    }
}

impl ModuleLoader {
    fn load_inner(
        &self,
        reader: &mut dyn Read,
        file: Option<PathBuf>,
    ) -> Result<ModuleRef, Error> {
        let mut source = String::new();
        reader.read_to_string(&mut source)?;
        let file_name: String = file
            .map(|p| p.to_string_lossy().into_owned())
            .unwrap_or_default();
        let file_id = self.module_files.borrow_mut().add(file_name, source.into());

        let (module, counters) = parse_str(file_id, self)?.into_inner();

        let name = module.name().clone();
        counters.display(&name)?;

        // save codespan file ID with module
        let _ = self.modules.borrow_mut().insert(name.clone(), (module.into(), file_id));
        Ok(self.get(&name).unwrap())
    }

    pub(crate) fn files(&self) -> Ref<'_, SimpleFiles<String, Source>> {
        self.module_files.borrow()
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

    // --------------------------------------------------------------------------------------------

    pub fn base(&self) -> &Url {
        &self.base
    }

    pub fn set_base(&mut self, base: Url) {
        self.base = base;
    }

    // --------------------------------------------------------------------------------------------

    pub fn loaded_from(&self) -> &PathBuf {
        &self.loaded_from
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_entries(&self) -> bool {
        !self.entries.is_empty()
    }

    pub fn get_entry(&self, key: &String) -> Option<&CatalogEntry> {
        self.entries.get(key)
    }

    pub fn entries_contains_key(&self, key: &String) -> bool {
        self.entries.contains_key(key)
    }

    pub fn entries(&self) -> impl Iterator<Item = (&String, &CatalogEntry)> {
        self.entries.iter()
    }

    pub fn entry_keys(&self) -> impl Iterator<Item = &String> {
        self.entries.keys()
    }

    pub fn entry_values(&self) -> impl Iterator<Item = &CatalogEntry> {
        self.entries.values()
    }

    // --------------------------------------------------------------------------------------------

    pub fn groups(&self) -> impl Iterator<Item = (&String, &Group)> {
        self.entries()
            .filter_map(|(k, e)| e.as_group().map(|group| (k, group)))
    }

    // --------------------------------------------------------------------------------------------

    pub fn items(&self) -> impl Iterator<Item = (&String, &Item)> {
        self.entries()
            .filter_map(|(k, e)| e.as_item().map(|item| (k, item)))
    }

    // --------------------------------------------------------------------------------------------

    pub fn resolve_uri(&self, module: &String) -> Option<Url> {
        if let Some(CatalogEntry::Item(item)) = self.get_entry(module) {
            Some(self.base.join(item.relative_url().as_str()).unwrap())
        } else {
            self.groups()
                .find(|(_, g)| g.entries_contains_key(module))
                .map(|(_, g)| g.resolve_uri(&self.base, module))
                .unwrap_or_default()
        }
    }

    pub fn resolve_local_path(&self, module: &String) -> Option<PathBuf> {
        if let Some(CatalogEntry::Item(item)) = self.get_entry(module) {
            Some(self.loaded_from.join(item.relative_path()))
        } else {
            self.groups()
                .find(|(_, g)| g.entries_contains_key(module))
                .map(|(_, g)| g.resolve_local_path(&self.loaded_from, module))
                .unwrap_or_default()
        }
    }
}

impl From<Group> for CatalogEntry {
    fn from(value: Group) -> Self {
        Self::Group(value)
    }
}

impl From<Item> for CatalogEntry {
    fn from(value: Item) -> Self {
        Self::Item(value)
    }
}

impl CatalogEntry {
    pub fn is_group(&self) -> bool {
        matches!(self, Self::Group(_))
    }
    pub fn as_group(&self) -> Option<&Group> {
        match self {
            Self::Group(v) => Some(v),
            _ => None,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn is_item(&self) -> bool {
        matches!(self, Self::Item(_))
    }
    pub fn as_item(&self) -> Option<&Item> {
        match self {
            Self::Item(v) => Some(v),
            _ => None,
        }
    }
}

impl Group {
    pub fn relative_path(&self) -> Option<&PathBuf> {
        self.relative_path.as_ref()
    }
    pub fn set_relative_path(&mut self, relative_path: PathBuf) {
        self.relative_path = Some(relative_path);
    }
    pub fn unset_relative_path(&mut self) {
        self.relative_path = None;
    }

    // --------------------------------------------------------------------------------------------

    pub fn relative_url(&self) -> Option<&String> {
        self.relative_url.as_ref()
    }
    pub fn set_relative_url(&mut self, relative_url: String) {
        self.relative_url = Some(relative_url);
    }
    pub fn unset_relative_url(&mut self) {
        self.relative_url = None;
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_entries(&self) -> bool {
        !self.entries.is_empty()
    }

    pub fn get_entry(&self, key: &String) -> Option<&Item> {
        self.entries.get(key)
    }

    pub fn entries_contains_key(&self, key: &String) -> bool {
        self.entries.contains_key(key)
    }

    pub fn entries(&self) -> impl Iterator<Item = (&String, &Item)> {
        self.entries.iter()
    }

    pub fn entry_keys(&self) -> impl Iterator<Item = &String> {
        self.entries.keys()
    }

    pub fn entry_values(&self) -> impl Iterator<Item = &Item> {
        self.entries.values()
    }

    // --------------------------------------------------------------------------------------------

    pub fn resolve_uri(&self, base: &Url, module: &String) -> Option<Url> {
        let base = if let Some(relative_url) = &self.relative_url {
            base.join(relative_url.as_str()).unwrap()
        } else {
            base.clone()
        };
        self.get_entry(module)
            .map(|item| base.join(item.relative_url().as_str()).unwrap())
    }

    pub fn resolve_local_path(&self, base: &Path, module: &String) -> Option<PathBuf> {
        let base = if let Some(group_base) = &self.relative_path {
            base.join(group_base)
        } else {
            base.to_path_buf()
        };
        self.get_entry(module)
            .map(|item| base.join(item.relative_url().as_str()))
    }
}

impl Item {
    pub fn relative_path(&self) -> &PathBuf {
        &self.relative_path
    }
    pub fn set_relative_path(&mut self, relative_path: PathBuf) {
        self.relative_path = relative_path;
    }

    // --------------------------------------------------------------------------------------------

    pub fn relative_url(&self) -> &String {
        &self.relative_url
    }
    pub fn set_relative_url(&mut self, relative_url: String) {
        self.relative_url = relative_url;
    }
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
