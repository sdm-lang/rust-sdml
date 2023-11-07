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

///
/// This trait is implemented by types that have a distinct /body/ type.
///
pub trait HasBody<T> {
    /// Get the body of the enclosing type.
    fn body(&self) -> &T;
    /// Set the body of the enclosing type.
    fn set_body(&mut self, body: T);
}

///
/// This trait is implemented by types that have a unique name.
///
pub trait HasName {
    /// Get the name of the enclosing type.
    fn name(&self) -> &Identifier;
    /// Set the name of the enclosing type.
    fn set_name(&mut self, name: Identifier);
}

pub trait HasNameReference {
    /// Get the name reference for the enclosing type.
    fn name_reference(&self) -> &IdentifierReference;
    /// Set the name reference for the enclosing type.
    fn set_name_reference(&mut self, name: IdentifierReference);
}

pub trait Namespace<T>
where
    T: HasName,
{
    /// Returns `true` of the namespace contains any members, else `false`.
    fn has_members(&self) -> bool;

    /// Returns the number of members in the namespace.
    fn member_count(&self) -> usize;

    /// Returns `true` if the namespace contains a member named `name`, else `false`.
    fn contains_member(&self, name: &Identifier) -> bool;

    /// Return the member with the name `name`, if present.
    fn member(&self, name: &Identifier) -> Option<&T>;

    /// Returns an iterator over all members in the namespace.
    fn members(&self) -> Box<dyn Iterator<Item = &T> + '_>;

    /// Returns an iterator over mutable members in the namespace.
    fn members_mut(&mut self) -> Box<dyn Iterator<Item = &mut T> + '_>;

    /// Returns an iterator over the names of namespace members.
    fn member_names(&self) -> Box<dyn Iterator<Item = &Identifier> + '_>;

    /// Add a member to the namespace. If a member already existed with the same name it
    /// will be returned.
    fn add_to_members(&mut self, value: T) -> Option<T>;

    /// Add the members of the extension to the namespace. Any existing members with
    /// the same names will be replaced.
    fn extend_members<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = T>;
}

///
/// This trait is implemented by types that have a distinct, but optional, *body* type.
///
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
/// The source location information from the tree-sitter [`Node`] type. The location is stored as
/// a start and end position, where the positions are byte indices.
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

pub mod annotations;

pub mod check;

pub mod constraints;

pub mod definitions;

pub mod identifiers;

pub mod members;

pub mod modules;

pub mod values;

pub mod walk;
