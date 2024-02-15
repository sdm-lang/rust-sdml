/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use super::definitions::Definition;
use super::identifiers::IdentifierReference;
use super::members::TypeReference;
use super::values::Value;
use crate::cache::ModuleCache;
use crate::error::Error;
use crate::load::ModuleLoader;
use crate::model::modules::Module;
use crate::model::HasBody;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub trait MaybeIncomplete {
    fn is_incomplete(&self, top: &Module, cache: &ModuleCache) -> bool;
}

pub trait Validate {
    fn validate(
        &self,
        top: &Module,
        cache: &ModuleCache,
        loader: &impl ModuleLoader,
        check_constraints: bool,
    );
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn find_definition<'a>(
    name: &IdentifierReference,
    current: &'a Module,
    cache: &'a ModuleCache,
) -> Option<&'a Definition> {
    match name {
        IdentifierReference::Identifier(ref name) => current.body().get_definition(name),
        IdentifierReference::QualifiedIdentifier(ref name) => {
            if let Some(module) = cache.get(name.module()) {
                module.body().get_definition(name.member())
            } else {
                None
            }
        }
    }
}

pub fn validate_value(
    _a_value: &Value,
    a_type: &TypeReference,
    current: &Module,
    cache: &ModuleCache,
    _check_constraints: bool,
    _errors: &mut Vec<Error>,
) {
    match a_type {
        TypeReference::Unknown => {
            panic!("no value allowed for unknown");
        }
        TypeReference::Type(id_ref) => {
            if let Some(_defn) = find_definition(id_ref, current, cache) {
                // todo: check it's an actual type
                todo!()
            } else {
                panic!("not a valid type reference");
            }
        }
        TypeReference::FeatureSet(_id_ref) => todo!(),
        TypeReference::MappingType(_map_type) => todo!(),
    }
}
