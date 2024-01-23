/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use super::definitions::Definition;
use super::identifiers::IdentifierReference;
use crate::cache::ModuleCache;
use crate::error::Error;
use crate::model::modules::Module;
use crate::model::HasBody;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// TODO: what about those that need a model loader?

// TODO: interactive vs. API calls?

pub trait Validate {
    fn is_complete(&self, top: &Module, cache: &ModuleCache) -> Result<bool, Error>;

    // Fail on first error
    fn is_valid(
        &self,
        check_constraints: bool,
        top: &Module,
        cache: &ModuleCache,
    ) -> Result<bool, Error>;

    // Find all errors
    fn validate(
        &self,
        check_constraints: bool,
        top: &Module,
        cache: &ModuleCache,
        errors: &mut Vec<Error>,
    ) -> Result<(), Error> {
        if let Err(e) = self.is_valid(check_constraints, top, cache) {
            errors.push(e);
        }
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn find_definition<'a>(
    name: &IdentifierReference,
    top: &'a Module,
    cache: &'a ModuleCache,
) -> Option<&'a Definition> {
    match name {
        IdentifierReference::Identifier(ref name) => top.body().get_definition(name),
        IdentifierReference::QualifiedIdentifier(ref name) => {
            if let Some(module) = cache.get(name.module()) {
                module.body().get_definition(name.member())
            } else {
                None
            }
        }
    }
}

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
