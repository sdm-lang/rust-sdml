/*!
Provide the Rust types that implement an in-memory representation of the the SDML Grammar.

*/

use crate::model::identifiers::{Identifier, IdentifierReference};
use std::{
    collections::HashSet,
    fmt::{Debug, Display},
    hash::Hash,
    ops::Range,
};
use tree_sitter::Node;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Traits
// ------------------------------------------------------------------------------------------------

pub trait HasBody<T> {
    fn body(&self) -> &T;
    fn set_body(&mut self, body: T);
}

pub trait HasName {
    fn name(&self) -> &Identifier;
    fn set_name(&mut self, name: Identifier);
}

pub trait HasNameReference {
    fn name_reference(&self) -> &IdentifierReference;
    fn set_name_reference(&mut self, name: IdentifierReference);
}

pub trait HasOptionalBody<T> {
    fn has_body(&self) -> bool {
        self.body().is_some()
    }
    fn body(&self) -> Option<&T>;
    fn set_body(&mut self, body: T);
    fn unset_body(&mut self);
}

pub trait HasSourceSpan {
    fn with_source_span(self, ts_span: Span) -> Self;
    fn has_source_span(&self) -> bool {
        self.source_span().is_some()
    }
    fn source_span(&self) -> Option<&Span>;
    fn set_source_span(&mut self, span: Span);
    fn unset_source_span(&mut self);
}

pub trait References {
    #[allow(unused_variables)]
    fn referenced_types<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {}

    #[allow(unused_variables)]
    fn referenced_annotations<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {}
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Structures
// ------------------------------------------------------------------------------------------------

///
/// The source location information from the tree-sitter [`Node`] type.
///
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Span(Range<usize>);

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl From<&Node<'_>> for Span {
    fn from(node: &Node<'_>) -> Self {
        Self(node.byte_range())
    }
}

impl From<Node<'_>> for Span {
    fn from(node: Node<'_>) -> Self {
        Self::from(&node)
    }
}

impl Debug for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Span")
            .field("start", &self.0.start)
            .field("end", &self.0.end)
            .finish()
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}..{}", self.0.start, self.0.end)
    }
}

impl Span {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    #[inline(always)]
    pub fn new(start: usize, end: usize) -> Self {
        assert!(start <= end);
        Self(start..end)
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    #[inline(always)]
    pub fn start(&self) -> usize {
        self.0.start
    }

    #[inline(always)]
    pub fn end(&self) -> usize {
        self.0.end
    }

    #[inline(always)]
    pub fn byte_range(&self) -> Range<usize> {
        self.0.clone()
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[macro_use]
mod macros;

pub mod annotations;

pub mod check;

pub mod constraints;

pub mod definitions;

pub mod identifiers;

pub mod members;

pub mod modules;

pub mod values;

pub mod walk;
