/*!
Provide the Rust types that implement an in-memory representation of the the SDML Grammar.

The following.

 * **Identifiers**
   * [`Identifier`], [`IdentifierReference`], [`Named`], [`QualifiedIdentifier`]

 * **Modules and Imports**
   * [`Import`], [`ImportStatement`], [`Module`], [`ModuleBody`]

 * **Annotations & Comments**
   * [`Annotation`], [`AnnotationProperty`]

 * **Constraints**
   * [`AtomicSentence`], [`BinaryOperation`], [`Binding`], [`BooleanSentence`], [`BoundSentence`],
     [`Constraint`], [`ConstraintBody`], [`ConstraintSentence`], [`ControlledLanguageString`],
     [`ControlledLanguageTag`], [`FunctionalTerm`], [`NamePath`], [`PredicateValue`],
     [`QuantifiedSentence`], [`SimpleSentence`], [`Subject`], [`Term`], [`UnaryOperation`]

 * **Type Definitions**
   * [`AnnotationOnlyBody`], [`DatatypeDef`], [`Definition`], [`EntityBody`], [`EntityDef`],
     [`EntityGroup`], [`EntityMember`], [`EnumBody`], [`EnumDef`], [`EventDef`],
     [`StructureBody`], [`StructureDef`], [`StructureGroup`], [`TypeVariant`], [`UnionBody`],
     [`UnionDef`], [`ValueVariant`]

 * **Property Definitions**
   * [`PropertyBody`], [`PropertyDef`], [`PropertyRole`]

 * **Member Definitions**
   * [`ByReferenceMember`], [`ByReferenceMemberDef`], [`ByReferenceMemberInner`], [`ByValueMember`],
     [`ByValueMemberDef`], [`ByValueMemberInner`], [`Cardinality`], [`IdentityMember`],
     [`IdentityMemberDef`], [`IdentityMemberInner`], [`TypeReference`]

 * **Values**
   * [`LanguageString`], [`LanguageTag`], [`ListMember`], [`ListOfValues`], [`SimpleValue`],
     [`Value`], [`ValueConstructor`]

*/

use std::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::Range,
};
use tree_sitter::Node;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types
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
// Modules
// ------------------------------------------------------------------------------------------------

#[macro_use]
mod macros;

mod ids;
pub use ids::{Identifier, IdentifierReference, Named, QualifiedIdentifier};

mod mods;
pub use mods::{Import, ImportStatement, Module, ModuleBody};

mod annotations;
pub use annotations::{Annotation, AnnotationProperty};

mod constraints;
pub use constraints::{
    AtomicSentence, BinaryOperation, BooleanSentence, BoundSentence, Constraint, ConstraintBody,
    ConstraintSentence, ControlledLanguageString, ControlledLanguageTag, FunctionalTerm, NamePath,
    QuantifiedSentence, SimpleSentence, Subject, Term,
};

mod values;
pub use values::{
    LanguageString, LanguageTag, ListMember, ListOfValues, SimpleValue, Value, ValueConstructor,
};

mod types;
pub use types::{
    AnnotationOnlyBody, DatatypeDef, Definition, EntityBody, EntityDef, EntityGroup, EntityMember,
    EnumBody, EnumDef, EventDef, PropertyBody, PropertyDef, PropertyRole, StructureBody,
    StructureDef, StructureGroup, TypeVariant, UnionBody, UnionDef, ValueVariant,
};

mod members;
pub use members::{
    ByReferenceMember, ByReferenceMemberDef, ByReferenceMemberInner, ByValueMember,
    ByValueMemberDef, ByValueMemberInner, Cardinality, IdentityMember, IdentityMemberDef,
    IdentityMemberInner, TypeReference,
};

pub mod walk;
