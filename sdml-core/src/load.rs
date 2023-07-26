/*!
Provides traits for resolving module names to paths, and loading modules.

*/

use crate::error::Error;
use crate::model::{Identifier, Module};
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
pub trait ModuleResolver: Debug {
    fn prepend_to_search_path(&mut self, path: &Path);

    fn append_to_search_path(&mut self, path: &Path);

    fn name_to_path(&self, name: &Identifier) -> Result<PathBuf, Error>;
}

// ------------------------------------------------------------------------------------------------

///
/// TBD
///
pub trait ModuleLoader: Debug {
    fn load(&mut self, name: &Identifier) -> Result<&Module, Error>;

    fn load_from_file(&mut self, file: PathBuf) -> Result<&Module, Error>;

    fn load_from_reader(&mut self, reader: &mut dyn Read) -> Result<&Module, Error>;

    fn contains(&self, name: &Identifier) -> bool;

    fn get(&self, name: &Identifier) -> Option<&Module>;

    fn get_source(&self, name: &Identifier) -> Option<&String>;

    fn resolver(&self) -> &dyn ModuleResolver;
}

// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct ModuleLoaderRef<T>(Rc<RefCell<T>>)
where
    T: ModuleLoader + Clone;

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl<T> From<T> for ModuleLoaderRef<T>
where
    T: ModuleLoader + Clone,
{
    fn from(value: T) -> Self {
        Self(Rc::new(RefCell::new(value)))
    }
}

impl<T> AsRef<RefCell<T>> for ModuleLoaderRef<T>
where
    T: ModuleLoader + Clone,
{
    fn as_ref(&self) -> &RefCell<T> {
        self.0.as_ref()
    }
}

impl<T> ModuleLoaderRef<T>
where
    T: ModuleLoader + Clone,
{
    pub fn borrow(&self) -> Ref<'_, T> {
        self.0.borrow()
    }

    pub fn borrow_mut(&mut self) -> RefMut<'_, T> {
        self.0.borrow_mut()
    }

    pub fn into_inner(self) -> Option<T> {
        Rc::into_inner(self.0).map(|ref_cell| ref_cell.into_inner())
    }
}
