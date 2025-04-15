/*!
One-line description.

TBD

# Example

TBD

 */

use crate::highlight::Highlighter;
use url::Url;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Copy, Clone, Debug)]
pub struct None;

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

macro_rules! hl_to_string {
    ($name:ident) => {
        #[inline(always)]
        fn $name(&self, s: &str) -> String {
            s.to_string()
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Highlighter for None {
    hl_to_string!(keyword);
    hl_to_string!(operator);
    hl_to_string!(module_defn);
    hl_to_string!(module_ref);
    hl_to_string!(type_defn);
    hl_to_string!(type_ref);
    hl_to_string!(type_ref_builtin);
    hl_to_string!(punctuation);
    hl_to_string!(punctuation_bracket);
    hl_to_string!(punctuation_separator);
    hl_to_string!(value);
    hl_to_string!(value_builtin);
    hl_to_string!(value_number);
    hl_to_string!(field_defn);
    hl_to_string!(field_ref);
    hl_to_string!(var_defn);
    hl_to_string!(var_ref);
    hl_to_string!(var_param_defn);
    hl_to_string!(var_param_ref);
    hl_to_string!(fn_defn);
    hl_to_string!(fn_call);
    hl_to_string!(fn_ref);
    hl_to_string!(annotation);
    hl_to_string!(annotation_property);
    hl_to_string!(annotation_constraint);

    #[inline(always)]
    fn value_string(&self, s: &str) -> String {
        format!("{s:?}")
    }

    #[inline(always)]
    fn value_iri(&self, s: &Url) -> String {
        format!("<{s}>")
    }
}
