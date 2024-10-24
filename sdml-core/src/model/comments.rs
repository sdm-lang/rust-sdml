/*!
Provide the Rust types that implement *comments* from the SDML Grammar.
 */

use std::fmt::Display;

use crate::model::Span;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// Corresponds to the grammar rule `line_comment`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Comment {
    span: Option<Span>,
    content: String,
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

impl From<String> for Comment {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&str> for Comment {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl AsRef<str> for Comment {
    fn as_ref(&self) -> &str {
        self.content.as_str()
    }
}

impl Display for Comment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, ";{}", self.content)
    }
}

impl_has_source_span_for!(Comment);

impl Comment {
    pub fn new<S>(content: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            span: Default::default(),
            content: content.into(),
        }
    }

    get_and_set!(pub content, set_content => into String);
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
