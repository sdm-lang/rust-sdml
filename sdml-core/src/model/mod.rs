/*!
Rust types that model the SDML Grammar.

More detailed description, with

# Example

YYYYY

*/

use std::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::Range,
};
use tree_sitter::Node;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Span(Range<usize>);

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

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
    #[inline(always)]
    pub fn new(start: usize, end: usize) -> Self {
        assert!(start <= end);
        Self(start..end)
    }

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
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[macro_use]
mod macros;

mod ids;
pub use ids::{Identifier, IdentifierReference, Named, QualifiedIdentifier};

mod mods;
pub use mods::{Import, ImportStatement, Module, ModuleBody};

mod annotations;
pub use annotations::{Annotation, AnnotationProperty, Comment, Constraint};

mod constraints;
pub use constraints::{
    AtomicSentence, BinaryOperation, Binding, BooleanSentence, BoundSentence, ConstraintBody,
    ConstraintSentence, FunctionalTerm, PredicateValue, QuantifiedSentence, SimpleSentence, Term,
    UnaryOperation,
};

mod values;
pub use values::{
    LanguageString, LanguageTag, ListMember, ListOfValues, SimpleValue, Value, ValueConstructor,
};

mod types;
pub use types::{
    AnnotationOnlyBody, DatatypeDef, EntityBody, EntityDef, EntityGroup, EntityMember, EnumBody,
    EnumDef, EnumVariant, EventDef, PropertyBody, PropertyDef, PropertyRole, StructureBody,
    StructureDef, StructureGroup, TypeDefinition, TypeVariant, UnionBody, UnionDef,
};

mod members;
pub use members::{
    ByReferenceMember, ByReferenceMemberDef, ByReferenceMemberInner, ByValueMember,
    ByValueMemberDef, ByValueMemberInner, Cardinality, IdentityMember, IdentityMemberDef,
    IdentityMemberInner, TypeReference,
};

pub mod walk;
