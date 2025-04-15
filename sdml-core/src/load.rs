/*!
This module provides the traits for loading modules from some source.

The goal of the traits [`ModuleResolver`] and [`ModuleLoader`] is to provide the ability to load
a module into memory from some resource.

# Example

```
use sdml_core::model::identifiers::Identifier;
use sdml_core::load::ModuleLoader;
use sdml_core::store::ModuleStore;

fn module_found(
    module: &Identifier,
    loader: &mut impl ModuleLoader,
    cache: &mut impl ModuleStore
) -> bool {
    loader.load(module, None, cache, false).is_ok()
}
```

 */

use crate::{
    model::{identifiers::Identifier, modules::ModulePath},
    store::ModuleStore,
};
use sdml_errors::{
    diagnostics::{reporter::ReportCounters, SeverityFilter},
    Diagnostic, FileId, Source,
};
use tracing::warn;
use url::Url;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// A resolver implementation is responsible for determining the resource identifier (URL) for
/// a module named `name`.
///
/// The additional parameter `imported_by` identifies the module source making the request.
///
pub trait ModuleResolver: Default {
    ///
    /// Return a URL given the module name `name`.
    ///
    fn name_to_resource(
        &self,
        name: &Identifier,
        imported_by: Option<FileId>,
    ) -> Result<Url, sdml_errors::Error>;

    fn path_name_to_resource(
        &self,
        name: &ModulePath,
        imported_by: Option<FileId>,
    ) -> Result<Url, sdml_errors::Error> {
        warn!("path_name_to_resource({name:?}, {imported_by:?}) not implemented");
        todo!();
    }
}

///
/// A loader instance is responsible for resolving a module into a resource URL and parsing it into
/// memory. Note that the loader does not return the module instance itself but rather the module's
/// name parsed from the resource, the module itself is inserted into the `cache`.
///
pub trait ModuleLoader: Default {
    ///
    /// Resolve `name` into a resource identifier (URL) and parse into memory. The loader will check
    /// the `store` first to see if the module is already loaded, and will add the module into the
    /// store after parsing. The value of `recursive` tells the loader whether to also load
    /// the module's dependencies as well.
    ///
    fn load(
        &mut self,
        name: &Identifier,
        imported_by: Option<FileId>,
        store: &mut impl ModuleStore,
        recursive: bool,
    ) -> Result<Identifier, sdml_errors::Error>;

    ///
    /// Returns the instance of [`ModuleResolver`] used by this loader.
    ///
    fn resolver(&self) -> &impl ModuleResolver;

    fn get_file_id(&self, name: &Identifier) -> Option<FileId>;

    fn get_source_by_name(&self, name: &Identifier) -> Option<Source> {
        self.get_file_id(name).and_then(|id| self.get_source(id))
    }

    fn has_source(&self, file_id: FileId) -> bool {
        self.get_source(file_id).is_some()
    }

    fn get_source(&self, file_id: FileId) -> Option<Source>;

    fn report(&self, diagnostic: &Diagnostic) -> Result<(), sdml_errors::Error>;
    fn reporter_done(
        &self,
        top_module_name: Option<String>,
    ) -> Result<ReportCounters, sdml_errors::Error>;

    fn set_severity_filter(&mut self, filter: SeverityFilter);
}
