/*!
Provide the Rust types that implement an in-memory representation of the the SDML Grammar.

The following.

 * **Identifiers**
   * [`Identifier`], [`IdentifierReference`], [`QualifiedIdentifier`]

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
     [`IdentityMemberDef`], [`IdentityMemberInner`], [`MappingType`], [`Ordering`],
     [`PseudoSequenceType`], [`TypeReference`], [`Uniqueness`],
     [`DEFAULT_BY_REFERENCE_CARDINALITY`], [`DEFAULT_BY_VALUE_CARDINALITY`]

 * **Values**
   * [`LanguageString`], [`LanguageTag`], [`ListMember`], [`ListOfValues`], [`MappingValue`],
     [`SimpleValue`], [`Value`], [`ValueConstructor`]

*/

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
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The source location information from the tree-sitter [`Node`] type.
///
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Span(Range<usize>);

pub trait ModelElement {
    fn has_ts_span(&self) -> bool {
        self.ts_span().is_some()
    }
    fn ts_span(&self) -> Option<&Span>;
    fn set_ts_span(&mut self, span: Span);
    fn unset_ts_span(&mut self);

    fn name(&self) -> &Identifier;
    fn set_name(&mut self, name: Identifier);

    fn is_complete(&self) -> bool;

    fn referenced_types(&self) -> HashSet<&IdentifierReference>;
    fn referenced_annotations(&self) -> HashSet<&IdentifierReference>;
}

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

mod identifiers;
pub use identifiers::{Identifier, IdentifierReference, QualifiedIdentifier};

mod modules;
pub use modules::{Module, ModuleBody};

mod imports;
pub use imports::{Import, ImportStatement};

mod annotations;
pub use annotations::{Annotation, AnnotationProperty};

mod constraints;
pub use constraints::{
    AnyOr, AtomicSentence, BinaryExpressionOperation, BooleanExpression, BooleanSentence,
    BoundExpression, BoundSentence, Constraint, ConstraintBody, ConstraintSentence,
    ControlledLanguageString, ControlledLanguageTag, EnvironmentDef, EnvironmentDefBody,
    Expression, FormalConstraint, FunctionDef, FunctionParameter, FunctionSignature, FunctionType,
    FunctionTypeReference, FunctionalTerm, IteratorTarget, NamePath, PredicateListMember,
    PredicateValue, PredicateValueList, QuantifiedExpression, QuantifiedSentence,
    QuantifierBinding, QuantifierNamedBinding, SequenceComprehension, SequenceIterator,
    SimpleSentence, Subject, Term, TypeIterator,
};

mod values;
pub use values::{
    LanguageString, LanguageTag, ListMember, ListOfValues, MappingValue, SimpleValue, Value,
    ValueConstructor,
};

mod definitions;
pub use definitions::{
    AnnotationOnlyBody, DatatypeDef, Definition, EntityBody, EntityDef, EntityGroup, EntityMember,
    EnumBody, EnumDef, EventDef, PropertyBody, PropertyDef, PropertyRole, StructureBody,
    StructureDef, StructureGroup, TypeVariant, UnionBody, UnionDef, ValueVariant,
};

mod members;
pub use members::{
    ByReferenceMember, ByReferenceMemberDef, ByReferenceMemberInner, ByValueMember,
    ByValueMemberDef, ByValueMemberInner, Cardinality, IdentityMember, IdentityMemberDef,
    IdentityMemberInner, MappingType, Ordering, PseudoSequenceType, TypeReference, Uniqueness,
    DEFAULT_BY_REFERENCE_CARDINALITY, DEFAULT_BY_VALUE_CARDINALITY, TYPE_BAG_CARDINALITY,
    TYPE_LIST_CARDINALITY, TYPE_MAYBE_CARDINALITY, TYPE_ORDERED_SET_CARDINALITY,
    TYPE_SET_CARDINALITY,
};

pub mod walk;
