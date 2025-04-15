/*!
Constants for the syntactic elements of the language.
 */

pub const NAME_SDML: &str = "sdml";

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod grammar;
pub use grammar::*;

pub mod model;
pub use model::*;
