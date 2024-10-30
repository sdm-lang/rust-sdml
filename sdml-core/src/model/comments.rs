/*!
Provide the Rust types that implement *comments* from the SDML Grammar.
 */

use crate::model::{HasSourceSpan, Span};
use std::fmt::Display;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

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
// Implementations ❱ Comment
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

impl HasSourceSpan for Comment {
    fn with_source_span(self, span: Span) -> Self {
        let mut self_mut = self;
        self_mut.span = Some(span);
        self_mut
    }

    fn source_span(&self) -> Option<&Span> {
        self.span.as_ref()
    }

    fn set_source_span(&mut self, span: Span) {
        self.span = Some(span);
    }

    fn unset_source_span(&mut self) {
        self.span = None;
    }
}

impl Comment {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn new<S>(content: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            span: Default::default(),
            content: content.into(),
        }
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    pub const fn content(&self) -> &String {
        &self.content
    }

    pub fn set_content<T>(&mut self, content: T)
    where
        T: Into<String>,
    {
        self.content = content.into();
    }
}
