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

/// ------------------------------------------------------------------------------------------------
/// Load the macros
/// ------------------------------------------------------------------------------------------------
#[macro_use]
mod macros;

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
/// This trait is implemented by types that have uniquely named members such as modules and
/// structures.
///
pub trait Namespace {
    type Member: HasName;

    ///
    /// Returns `true` of the namespace contains any members, else `false`.
    ///
    fn has_members(&self) -> bool;

    ///
    /// Returns the number of members in the namespace.
    ///
    fn member_count(&self) -> usize;

    ///
    /// Returns `true` if the namespace contains a member named `name`, else `false`.
    ///
    fn contains_member(&self, name: &Identifier) -> bool;

    ///
    /// Return the member with the name `name`, if present.
    ///
    fn member(&self, name: &Identifier) -> Option<&Self::Member>;

    ///
    /// Returns an iterator over all members in the namespace.
    ///
    fn members(&self) -> impl Iterator<Item = &Self::Member>;

    ///
    /// Returns an iterator over mutable members in the namespace.
    ///
    fn members_mut(&mut self) -> impl Iterator<Item = &mut Self::Member>;

    ///
    /// Returns an iterator over the names of namespace members.
    ///
    fn member_names(&self) -> impl Iterator<Item = &Identifier>;

    ///
    /// Add a member to the namespace. If a member already existed with the same name it
    /// will be returned.
    ///
    fn add_to_members(&mut self, value: Self::Member) -> Option<Self::Member>;

    ///
    /// Add the members of the extension to the namespace. Any existing members with
    /// the same names will be replaced.
    ///
    fn extend_members<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = Self::Member>;
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Span {
    start: SpanPosition,
    end: SpanPosition,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct SpanPosition {
    byte: usize,
    line: usize,
    column: usize,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

#[cfg(feature = "tree-sitter")]
impl From<&tree_sitter::Node<'_>> for Span {
    fn from(node: &tree_sitter::Node<'_>) -> Self {
        Self {
            start: SpanPosition::from(node.start_position(), node.start_byte()),
            end: SpanPosition::from(node.end_position(), node.end_byte()),
        }
    }
}

#[cfg(feature = "tree-sitter")]
impl From<tree_sitter::Node<'_>> for Span {
    fn from(node: tree_sitter::Node<'_>) -> Self {
        Self::from(&node)
    }
}

impl From<&Span> for sdml_errors::Span {
    fn from(value: &Span) -> Self {
        value.byte_range()
    }
}

impl From<Span> for sdml_errors::Span {
    fn from(value: Span) -> Self {
        value.byte_range()
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}..{}", self.start.byte, self.end.byte)
    }
}

impl Span {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    /// Create a new span from the `start` byte and `end` byte indices.
    #[inline(always)]
    pub fn new(start: SpanPosition, end: SpanPosition) -> Self {
        assert!(start.byte <= end.byte);
        assert!(start.line <= end.line);
        assert!(start.column <= end.column || end.line > start.line);

        Self { start, end }
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    /// Return the starting byte index of this span.
    #[inline(always)]
    pub fn start(&self) -> SpanPosition {
        self.start
    }

    /// Return the ending byte index of this span.
    #[inline(always)]
    pub fn end(&self) -> SpanPosition {
        self.end
    }

    /// Return this span as a `start..end` range.
    #[inline(always)]
    pub fn byte_range(&self) -> Range<usize> {
        self.start.byte..self.end.byte
    }
}

impl SpanPosition {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    /// Create a new span position from the `byte`, `line`, and `column` indices.
    #[inline(always)]
    pub fn new(byte: usize, line: usize, column: usize) -> Self {
        Self { byte, line, column }
    }

    /// Create a new span position from the `byte` and tree-sitter point.
    #[cfg(feature = "tree-sitter")]
    pub fn from(node_point: tree_sitter::Point, byte: usize) -> Self {
        Self::new(byte, node_point.row + 1, node_point.column + 1)
    }

    pub const fn byte(&self) -> usize {
        self.byte
    }

    pub const fn line(&self) -> usize {
        self.line
    }

    pub const fn column(&self) -> usize {
        self.column
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod annotations;

pub mod check;

pub mod constraints;

pub mod definitions;

pub mod identifiers;

pub mod members;

pub mod modules;

pub mod values;

pub mod walk;
