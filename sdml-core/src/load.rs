/*!
One-line description.

More detailed description, with

# Example

End of file during parsingSymbol’s value as variable is void: rustEnd of file during parsing

 */

use crate::{cache::ModuleStore, model::identifiers::Identifier};
use sdml_errors::{
    diagnostics::{reporter::ReportCounters, SeverityFilter},
    Diagnostic, FileId, Source,
};
use url::Url;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub trait ModuleResolver: Default {
    fn name_to_resource(
        &self,
        name: &Identifier,
        from: Option<FileId>,
    ) -> Result<Url, sdml_errors::Error>;
}

pub trait ModuleLoader: Default {
    fn load(
        &mut self,
        name: &Identifier,
        from: Option<FileId>,
        cache: &mut impl ModuleStore,
        recursive: bool,
    ) -> Result<Identifier, sdml_errors::Error>;

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

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
