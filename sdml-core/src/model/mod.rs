/*!
Provide the Rust types that implement an in-memory representation of the SDML Grammar.
*/

use crate::model::identifiers::{Identifier, IdentifierReference};
use std::{
    collections::HashSet,
    fmt::{Debug, Display},
    hash::Hash,
    ops::Range,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Traits
// ------------------------------------------------------------------------------------------------

///
/// This trait is implemented by types that have a distinct *body* type.
///
pub trait HasBody {
    ///
    /// This type is the particular body for the enclosing type.
    ///
    type Body;

    ///
    /// Get the body of the enclosing type.
    ///
    fn body(&self) -> &Self::Body;

    ///
    /// Get a mutable reference to the body of the enclosing type.
    ///
    fn body_mut(&mut self) -> &mut Self::Body;

    ///
    /// Set the body of the enclosing type.
    ///
    fn set_body(&mut self, body: Self::Body);
}

///
/// This trait is implemented by types that have a unique name.
///
pub trait HasName {
    ///
    /// Get the name of the enclosing type.
    ///
    fn name(&self) -> &Identifier;

    ///
    /// Set the name of the enclosing type.
    ///
    fn set_name(&mut self, name: Identifier);
}

///
/// This trait is implemented by types whose name is derived from a reference.
///
pub trait HasNameReference {
    ///
    /// Get the name reference for the enclosing type.
    ///
    fn name_reference(&self) -> &IdentifierReference;

    ///
    /// Set the name reference for the enclosing type.
    ///
    fn set_name_reference(&mut self, name: IdentifierReference);
}

///
/// This trait is implemented by types that have a distinct, but optional, *body* type.
///
pub trait HasOptionalBody {
    ///
    /// This type is the particular body for the enclosing type.
    ///
    type Body;

    fn has_body(&self) -> bool {
        self.body().is_some()
    }
    fn body(&self) -> Option<&Self::Body>;
    fn body_mut(&mut self) -> Option<&mut Self::Body>;
    fn set_body(&mut self, body: Self::Body);
    fn unset_body(&mut self);
}

///
/// This trait is implemented by types that include a source location from which they were parsed.
///
pub trait HasSourceSpan {
    fn with_source_span(self, ts_span: Span) -> Self;
    fn has_source_span(&self) -> bool {
        self.source_span().is_some()
    }
    fn source_span(&self) -> Option<&Span>;
    fn set_source_span(&mut self, span: Span);
    fn unset_source_span(&mut self);
}

///
/// This trait is implemented by types to allow for query of references.
///
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
/// The source location information from the tree-sitter `Node` type. The location is stored as
/// a start and end position, where the positions are byte indices.
///
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Span(Range<usize>);

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

#[cfg(feature = "tree-sitter")]
impl From<&tree_sitter::Node<'_>> for Span {
    fn from(node: &tree_sitter::Node<'_>) -> Self {
        Self(node.byte_range())
    }
}

#[cfg(feature = "tree-sitter")]
impl From<tree_sitter::Node<'_>> for Span {
    fn from(node: tree_sitter::Node<'_>) -> Self {
        Self::from(&node)
    }
}

impl From<Span> for sdml_errors::Span {
    fn from(value: Span) -> Self {
        value.byte_range()
    }
}

impl From<&Span> for sdml_errors::Span {
    fn from(value: &Span) -> Self {
        sdml_errors::Span::from(value.clone())
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

    /// Create a new span from the `start` byte and `end` byte indices.
    #[inline(always)]
    pub fn new(start: usize, end: usize) -> Self {
        assert!(start <= end);
        Self(start..end)
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    /// Return the starting byte index of this span.
    #[inline(always)]
    pub fn start(&self) -> usize {
        self.0.start
    }

    /// Return the ending byte index of this span.
    #[inline(always)]
    pub fn end(&self) -> usize {
        self.0.end
    }

    /// Return this span as a `start..end` range.
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

pub mod comments;

pub mod annotations;

pub mod check;

pub mod constraints;

pub mod definitions;

pub mod identifiers;

pub mod members;

pub mod modules;

pub mod values;

pub mod walk;
