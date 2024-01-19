/*!

*/

use crate::parse::parse_str;
use codespan_reporting::files::SimpleFiles;
use sdml_core::cache::ModuleCache;
use sdml_core::error::{module_file_not_found, Error};
use sdml_core::model::identifiers::Identifier;
use sdml_core::model::modules::Module;
use sdml_core::model::HasName;
use sdml_core::stdlib;
use search_path::SearchPath;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use tracing::{error, info, trace, warn};
use url::Url;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The resolver implements the logic to map module identifiers to file system paths using the
/// environment variable `SDML_PATH` to contain a search path.
///
#[derive(Clone, Debug)]
pub struct ModuleResolver {
    catalog: Option<ModuleCatalog>,
    search_path: SearchPath,
}

/// The name of the SDML environment variable that may be used to hold a load path.
pub const SDML_RESOLVER_PATH_VARIABLE: &str = "SDML_PATH";

/// The recommended file extension for SDML resources.
pub const SDML_FILE_EXTENSION: &str = "sdm";

/// The alternate file extension for SDML resources.
pub const SDML_FILE_EXTENSION_LONG: &str = "sdml";

/// The name used for resolver catalog files.
pub const SDML_CATALOG_FILE_NAME: &str = "sdml-catalog.json";

// ------------------------------------------------------------------------------------------------

///
/// A wrapper around the source code loaded prior to parsing.
///
#[derive(Clone, Debug)]
pub struct Source(String);

///
/// The loader is used to manage the process of creating an in-memory model from file-system resources.
///
/// A Module Loader is therefore responsible for:
///
/// 1. finding the resource that contains a module definition,
/// 2. parsing the source into an in-memory representation,
/// 3. caching the loaded module, and it's source, for future use.
///
#[derive(Clone, Debug)]
pub struct ModuleLoader {
    resolver: ModuleResolver,
    module_file_ids: HashMap<Identifier, usize>,
    module_files: SimpleFiles<String, Source>,
}

// ------------------------------------------------------------------------------------------------

///
/// This type represents the content of a resolver file.
///
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct ModuleCatalog {
    base: Url,
    #[serde(skip)]
    loaded_from: PathBuf,
    entries: HashMap<String, CatalogEntry>,
}

///
/// An entry in a resolver catalog file is either an item or group of items.
///
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CatalogEntry {
    Group(Group),
    Item(Item),
}

///
/// A resolver group allows the common configuration of multiple items.
///
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Group {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    relative_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    relative_path: Option<PathBuf>,
    entries: HashMap<String, Item>,
}

///
/// A specific resolver item.
///
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Item {
    relative_url: String,
    relative_path: PathBuf,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Given a `module` instance, load all it's dependencies (imported modules). If `recursive` is
/// `true` the same operation is performed on all loaded dependencies.
///
/// This requires a module `cache` to hold the loaded dependencies and a `loader` to perform the
/// actual parsing.
///
pub fn load_module_dependencies(
    module: &Identifier,
    recursive: bool,
    cache: &mut ModuleCache,
    loader: &mut ModuleLoader,
) -> Result<(), Error> {
    let module = cache.get(module).unwrap();
    trace!(
        "load_module_dependencies({}, {recursive}, _, _)",
        module.name()
    );
    let dependencies = module
        .imported_modules()
        .into_iter()
        .cloned()
        .collect::<Vec<Identifier>>();
    load_module_dependencies_named(&dependencies, recursive, cache, loader)
}


///
/// Given a `module` instance, load the list of `dependencies`. If `recursive` is
/// `true` the same operation is performed on all loaded dependencies.
///
/// This requires a module `cache` to hold the loaded dependencies and a `loader` to perform the
/// actual parsing.
///
pub fn load_module_dependencies_named(
    dependencies: &[Identifier],
    recursive: bool,
    cache: &mut ModuleCache,
    loader: &mut ModuleLoader,
) -> Result<(), Error> {
    trace!("load_module_dependencies_named({dependencies:?}, {recursive}, _, _)");

    for dependency in dependencies {
        if !cache.contains(dependency) {
            let dependency_module = loader.load(dependency)?;
            let dependencies = dependency_module
                .imported_modules()
                .into_iter()
                .cloned()
                .collect::<Vec<Identifier>>();
            cache.insert(dependency_module);
            if recursive {
                load_module_dependencies_named(&dependencies, recursive, cache, loader)?;
            }
        }
    }

    Ok(())
}

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

macro_rules! trace_entry {
    ($type_name: literal, $fn_name: literal) => {
        const FULL_NAME: &str = concat!($type_name, "::", $fn_name);
        let tracing_span = ::tracing::trace_span!(FULL_NAME);
        let _enter_span = tracing_span.enter();
        ::tracing::trace!("{FULL_NAME}()");
    };
    ($type_name: literal, $fn_name: literal => $format: literal, $( $value: expr ),+ ) => {
        const FULL_NAME: &str = concat!($type_name, "::", $fn_name);
        let tracing_span = ::tracing::trace_span!(FULL_NAME);
        let _enter_span = tracing_span.enter();
        let arguments = format!($format, $( $value ),+);
        ::tracing::trace!("{FULL_NAME}({arguments})");
    };
}

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
        Self(value)
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
        trace_entry!("ModuleResolver", "default");

        // 1. Use the standard environment variable as a search path
        let mut search_path = SearchPath::new_or_default(SDML_RESOLVER_PATH_VARIABLE);

        // 2. Add the current directory to the search path
        search_path.prepend_cwd();

        // 3. Load any catalog file found in the search path
        let catalog = ModuleCatalog::load_from_current(true);

        let _self = Self {
            catalog,
            search_path,
        };

        trace!("=> {:?}", _self);
        _self
    }
}

impl ModuleResolver {
    /// Add the provided path to the beginning of the search list.
    pub fn prepend_to_search_path(&mut self, path: &Path) {
        self.search_path.append(PathBuf::from(path));
    }

    /// Add the provided path to the end of the search list.
    pub fn append_to_search_path(&mut self, path: &Path) {
        self.search_path.append(PathBuf::from(path));
    }

    /// Return a file system path for the resource that /should/ contain the named module.
    pub fn name_to_path(&self, name: &Identifier) -> Result<PathBuf, Error> {
        trace_entry!("ModuleResolver", "name_to_path" => "{}", name);
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

impl Default for ModuleLoader {
    fn default() -> Self {
        Self {
            resolver: Default::default(),
            module_file_ids: Default::default(),
            module_files: SimpleFiles::new(),
        }
    }
}

impl ModuleLoader {
    pub fn with_resolver(self, resolver: ModuleResolver) -> Self {
        Self { resolver, ..self }
    }

    /// Load the named module using the loader's current resolver.
    pub fn load(&mut self, name: &Identifier) -> Result<Module, Error> {
        trace_entry!("ModuleLoader", "load" => "{}", name);
        if let Some(module) = stdlib::library_module(name) {
            Ok(module)
        } else {
            let file = self.resolver.name_to_path(name)?;
            self.load_from_file(file)
        }
    }

    /// Load a module from the source in `file`.
    pub fn load_from_file(&mut self, file: PathBuf) -> Result<Module, Error> {
        trace_entry!("ModuleLoader", "load_from_file" => "{:?}", file);
        let mut reader = File::open(&file)?;
        let catalog = self.resolver.catalog.clone();
        let mut module = self.load_inner(&mut reader, Some(file.clone()))?;
        if !module.has_base_uri() {
            if let Some(catalog) = catalog {
                let name = module.name().to_string();
                if let Some(url) = catalog.resolve_uri(&name) {
                    module.set_base_uri(url);
                }
            } else {
                let file = file.canonicalize()?;
                match Url::from_file_path(file) {
                    Ok(base) => module.set_base_uri(base),
                    Err(_) => warn!("Could not construct a base URI"),
                }
            }
        }
        Ok(module)
    }

    /// Load a module reading the source from `reader`.
    pub fn load_from_reader(&mut self, reader: &mut dyn Read) -> Result<Module, Error> {
        trace_entry!("ModuleLoader", "load_from_reader");
        self.load_inner(reader, None)
    }

    pub fn get_source(&self, name: &Identifier) -> Option<Source> {
        if let Some(file_id) = self.module_file_ids.get(name) {
            match self.files().get(*file_id) {
                Ok(file) => Some(file.source().clone()),
                Err(err) => {
                    error!("Could not retrieve module: {file_id:?}, error: {err}");
                    None
                }
            }
        } else {
            None
        }
    }

    fn load_inner(
        &mut self,
        reader: &mut dyn Read,
        file: Option<PathBuf>,
    ) -> Result<Module, Error> {
        trace!("ModuleLoader::load_inner(..., {file:?})");
        let mut source = String::new();
        reader.read_to_string(&mut source)?;
        let file_name: String = file
            .map(|p| p.to_string_lossy().into_owned())
            .unwrap_or_default();
        let file_id = self.module_files.add(file_name, source.into());

        let (module, counters) = parse_str(file_id, self)?.into_inner();

        let name = module.name().clone();
        counters.display(&name)?;

        let _ = self.module_file_ids.insert(name, file_id);
        Ok(module)
    }

    #[inline(always)]
    pub fn resolver(&self) -> &ModuleResolver {
        &self.resolver
    }

    #[inline(always)]
    pub(crate) fn files(&self) -> &SimpleFiles<String, Source> {
        &self.module_files
    }
}

// ------------------------------------------------------------------------------------------------

impl ModuleCatalog {
    ///
    /// Load a resolver catalog file from the current directory.
    ///
    /// If the parameter `look_in_parents` is `true` this will check parent directories.
    ///
    pub fn load_from_current(look_in_parents: bool) -> Option<Self> {
        trace!("ModuleCatalog::load_from_current({look_in_parents})");
        let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        Self::load_from(&cwd, look_in_parents)
    }

    ///
    /// Load a resolver catalog file from the `path`.
    ///
    /// If the parameter `look_in_parents` is `true` this will check parent directories.
    ///
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

    ///
    /// Load from the `file` path, this has been found by one of the methods above and so it should
    /// exist.
    ///
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
            r#"{
  "base": "https://example.org/rentals/",
  "entries": {
    "vehicle": {
      "item": {
        "relative_url": "vehicle#",
        "relative_path": "vehicle-v1.sdm"
      }
    }
  }
}"#,
        )
        .unwrap();
    }

    #[test]
    fn test_parse_catalog_with_group() {
        let _: ModuleCatalog = serde_json::from_str(
            r#"{
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
}"#,
        )
        .unwrap();
    }
}
