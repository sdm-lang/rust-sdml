/*!
Provides traits for resolving module names to paths, and loading modules.

*/

use crate::error::Error;
use crate::model::{identifiers::Identifier, modules::Module};
use std::cell::{RefCell, Ref, RefMut};
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
    fn prepend_to_search_path(&self, path: &Path);

    fn append_to_search_path(&self, path: &Path);

    fn name_to_path(&self, name: &Identifier) -> Result<PathBuf, Error>;
}

// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct ModuleRef(Rc<RefCell<Module>>);

// ------------------------------------------------------------------------------------------------

///
/// TBD
///
/// Note this trait does not require mutability even for clearly mutating operations.
///
pub trait ModuleLoader: Debug {
    fn load(&self, name: &Identifier) -> Result<ModuleRef, Error>;

    fn load_from_file(&self, file: PathBuf) -> Result<ModuleRef, Error>;

    fn load_from_reader(&self, reader: &mut dyn Read) -> Result<ModuleRef, Error>;

    fn adopt_raw(&self, module: Module) {
        self.adopt(module.into());
    }

    fn adopt(&self, module: ModuleRef);

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
        maybe.map(|v|v.get_mut())
    }
}
