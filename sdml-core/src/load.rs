/*!
Provides traits for resolving module names to paths, and loading modules.

*/

use crate::error::Error;
use crate::model::{identifiers::Identifier, modules::Module};
use std::cell::{Ref, RefCell, RefMut};
use std::fmt::Debug;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::rc::Rc;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The resolver implements the logic to map module identifiers to file system paths using the
/// environment variable `SDML_PATH` to contain a search path.
///
/// Note this trait does not require mutability even for clearly mutating operations.
///
pub trait ModuleResolver: Debug {
    /// Add the provided path to the beginning of the search list.
    fn prepend_to_search_path(&self, path: &Path);

    /// Add the provided path to the end of the search list.
    fn append_to_search_path(&self, path: &Path);

    /// Return a file system path for the resource that /should/ contain the named module.
    fn name_to_path(&self, name: &Identifier) -> Result<PathBuf, Error>;
}

// ------------------------------------------------------------------------------------------------

///
/// This is the reference type actually held by the loader.
#[derive(Clone, Debug)]
pub struct ModuleRef(Rc<RefCell<Module>>);

// ------------------------------------------------------------------------------------------------

///
/// The loader is used to manage the process of creating an in-memory model from file-system resources.
///
/// A Module Loader is therefore responsible for:
///
/// 1. finding the resource that contains a module definition,
/// 2. parsing the source into an in-memory representation,
/// 3. caching the loaded module, and it's source, for future use.
///
/// Note this trait does not require mutability even for clearly mutating operations.
///
pub trait ModuleLoader: Debug {
    /// Load the named module using the loader's current resolver.
    fn load(&self, name: &Identifier) -> Result<ModuleRef, Error>;

    /// Load a module from the source in `file`.
    fn load_from_file(&self, file: PathBuf) -> Result<ModuleRef, Error>;

    /// Load a module reading the source from `reader`.
    fn load_from_reader(&self, reader: &mut dyn Read) -> Result<ModuleRef, Error>;

    /// Add a module to this loader's cache.
    fn adopt(&self, module: ModuleRef);

    /// Add a module to this loader's cache, wrapping it in a reference first.
    fn adopt_raw(&self, module: Module) {
        self.adopt(module.into());
    }

    /// Returns `true` if the loader's cache contains a module with the name `name`, else `false`.
    fn contains(&self, name: &Identifier) -> bool;

    fn get(&self, name: &Identifier) -> Option<ModuleRef>;

    fn get_source(&self, name: &Identifier) -> Option<Box<dyn AsRef<str>>>;

    fn resolver(&self) -> Rc<dyn ModuleResolver>;
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl From<Module> for ModuleRef {
    fn from(value: Module) -> Self {
        Self::from(Rc::new(RefCell::new(value)))
    }
}

impl From<Rc<RefCell<Module>>> for ModuleRef {
    fn from(value: Rc<RefCell<Module>>) -> Self {
        Self(value)
    }
}

impl From<ModuleRef> for Rc<RefCell<Module>> {
    fn from(value: ModuleRef) -> Self {
        value.0
    }
}

impl ModuleRef {
    pub fn borrow(&self) -> Ref<'_, Module> {
        self.0.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<'_, Module> {
        self.0.borrow_mut()
    }

    pub fn get_mut(&mut self) -> Option<&mut Module> {
        let maybe: Option<&mut RefCell<Module>> = Rc::get_mut(&mut self.0);
        maybe.map(|v| v.get_mut())
    }
}
