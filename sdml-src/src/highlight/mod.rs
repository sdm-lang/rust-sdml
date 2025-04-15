/*!
One-line description.

TBD

# Example

TBD

 */

use url::Url;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[allow(dead_code)]
pub(super) trait Highlighter {
    fn keyword(&self, s: &str) -> String;

    fn operator(&self, s: &str) -> String;

    fn module_defn(&self, s: &str) -> String;
    fn module_ref(&self, s: &str) -> String;

    fn type_defn(&self, s: &str) -> String;
    fn type_ref(&self, s: &str) -> String;
    fn type_ref_builtin(&self, s: &str) -> String;

    fn field_defn(&self, s: &str) -> String;
    fn field_ref(&self, s: &str) -> String;

    fn var_defn(&self, s: &str) -> String;
    fn var_ref(&self, s: &str) -> String;

    fn var_param_defn(&self, s: &str) -> String;
    fn var_param_ref(&self, s: &str) -> String;

    fn fn_defn(&self, s: &str) -> String;
    fn fn_call(&self, s: &str) -> String;
    fn fn_ref(&self, s: &str) -> String;

    fn punctuation(&self, s: &str) -> String;
    fn punctuation_bracket(&self, s: &str) -> String;
    fn punctuation_separator(&self, s: &str) -> String;

    fn value(&self, s: &str) -> String;
    fn value_builtin(&self, s: &str) -> String;
    fn value_number(&self, s: &str) -> String;
    fn value_string(&self, s: &str) -> String;
    fn value_iri(&self, s: &Url) -> String;

    fn annotation(&self, s: &str) -> String;
    fn annotation_property(&self, s: &str) -> String;
    fn annotation_constraint(&self, s: &str) -> String;
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod ansi;

pub mod html;

pub mod none;
