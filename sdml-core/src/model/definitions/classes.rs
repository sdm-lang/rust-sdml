/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::load::ModuleLoader;
use crate::model::annotations::{
    Annotation, AnnotationBuilder, AnnotationProperty, HasAnnotations,
};
use crate::model::check::{MaybeIncomplete, Validate};
use crate::model::constraints::{ConstraintSentence, FunctionCardinality, FunctionSignature};
use crate::model::identifiers::{Identifier, IdentifierReference};
use crate::model::modules::Module;
use crate::model::values::Value;
use crate::model::{HasName, HasOptionalBody, HasSourceSpan, References, Span};
use crate::store::ModuleStore;
use std::collections::{BTreeMap, BTreeSet};

use sdml_errors::diagnostics::functions::IdentifierCaseConvention;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Definitions ❱ Type Classes
// ------------------------------------------------------------------------------------------------

/// Corresponds to the grammar rule `type_class_def`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct TypeClassDef {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    name: Identifier,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    variables: Vec<TypeVariable>, // assert 1..
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    body: Option<TypeClassBody>,
}

/// Corresponds to the grammar rule `type_variable`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct TypeVariable {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    name: Identifier,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    cardinality: Option<FunctionCardinality>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    restrictions: Vec<TypeClassReference>,
}

/// Corresponds to the grammar rule `type_class_reference`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct TypeClassReference {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    name: IdentifierReference,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    arguments: Vec<TypeClassArgument>, // 0..
}

/// Corresponds to the grammar rule `type_class_arguments`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum TypeClassArgument {
    Wildcard,
    Reference(Box<TypeClassReference>),
}

/// Corresponds to the grammar rule `type_class_body`.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct TypeClassBody {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    annotations: Vec<Annotation>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "BTreeMap::is_empty"))]
    methods: BTreeMap<Identifier, MethodDef>,
}

/// Corresponds to the grammar rule `method_def`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct MethodDef {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    name: Identifier,
    signature: FunctionSignature,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    body: Option<ConstraintSentence>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    annotations: Vec<Annotation>,
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Definitions ❱ TypeClassDef
// ------------------------------------------------------------------------------------------------

impl HasName for TypeClassDef {
    fn name(&self) -> &Identifier {
        &self.name
    }

    fn set_name(&mut self, name: Identifier) {
        self.name = name;
    }
}

impl HasOptionalBody for TypeClassDef {
    type Body = TypeClassBody;

    fn body(&self) -> Option<&Self::Body> {
        self.body.as_ref()
    }

    fn body_mut(&mut self) -> Option<&mut Self::Body> {
        self.body.as_mut()
    }

    fn set_body(&mut self, body: Self::Body) {
        self.body = Some(body);
    }

    fn unset_body(&mut self) {
        self.body = None;
    }
}

impl HasSourceSpan for TypeClassDef {
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

impl AnnotationBuilder for TypeClassDef {
    fn with_predicate<I, V>(self, predicate: I, value: V) -> Self
    where
        Self: Sized,
        I: Into<IdentifierReference>,
        V: Into<Value>,
    {
        let mut self_mut = self;
        if let Some(ref mut inner) = self_mut.body {
            inner.add_to_annotations(AnnotationProperty::new(predicate.into(), value.into()));
        }
        self_mut
    }
}

impl MaybeIncomplete for TypeClassDef {
    fn is_incomplete(&self, _: &Module, _: &impl ModuleStore) -> bool {
        self.body.is_none()
    }
}

impl References for TypeClassDef {
    fn referenced_types<'a>(&'a self, _names: &mut BTreeSet<&'a IdentifierReference>) {}

    fn referenced_annotations<'a>(&'a self, _names: &mut BTreeSet<&'a IdentifierReference>) {}
}

impl Validate for TypeClassDef {
    fn validate(
        &self,
        top: &crate::model::modules::Module,
        _cache: &impl ModuleStore,
        loader: &impl ModuleLoader,
        _check_constraints: bool,
    ) {
        self.name()
            .validate(top, loader, Some(IdentifierCaseConvention::TypeDefinition));
        todo!()
    }
}

impl TypeClassDef {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn new<I>(name: Identifier, variables: I) -> Self
    where
        I: IntoIterator<Item = TypeVariable>,
    {
        Self {
            span: None,
            name,
            variables: Vec::from_iter(variables),
            body: None,
        }
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    pub fn has_variables(&self) -> bool {
        !self.variables.is_empty()
    }

    pub fn variable_count(&self) -> usize {
        self.variables.len()
    }

    pub fn variables(&self) -> impl Iterator<Item = &TypeVariable> {
        self.variables.iter()
    }

    pub fn variables_mut(&mut self) -> impl Iterator<Item = &mut TypeVariable> {
        self.variables.iter_mut()
    }

    pub fn add_to_variables<I>(&mut self, value: I)
    where
        I: Into<TypeVariable>,
    {
        self.variables.push(value.into())
    }

    pub fn extend_variables<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = TypeVariable>,
    {
        self.variables.extend(extension)
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Definitions ❱ TypeVariable
// ------------------------------------------------------------------------------------------------

impl HasName for TypeVariable {
    fn name(&self) -> &Identifier {
        &self.name
    }

    fn set_name(&mut self, name: Identifier) {
        self.name = name;
    }
}

impl HasSourceSpan for TypeVariable {
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

impl TypeVariable {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub const fn new(name: Identifier) -> Self {
        Self {
            span: None,
            name,
            cardinality: None,
            restrictions: Vec::new(),
        }
    }

    pub fn with_cardinality(self, cardinality: FunctionCardinality) -> Self {
        Self {
            cardinality: Some(cardinality),
            ..self
        }
    }

    pub fn with_restrictions<I>(self, restrictions: I) -> Self
    where
        I: IntoIterator<Item = TypeClassReference>,
    {
        Self {
            restrictions: Vec::from_iter(restrictions),
            ..self
        }
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    pub const fn has_cardinality(&self) -> bool {
        self.cardinality.is_some()
    }

    pub const fn cardinality(&self) -> Option<&FunctionCardinality> {
        self.cardinality.as_ref()
    }

    pub fn set_cardinality(&mut self, cardinality: FunctionCardinality) {
        self.cardinality = Some(cardinality);
    }

    pub fn unset_cardinality(&mut self) {
        self.cardinality = None;
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_restrictions(&self) -> bool {
        !self.restrictions.is_empty()
    }

    pub fn restrictions_len(&self) -> usize {
        self.restrictions.len()
    }

    pub fn restrictions(&self) -> impl Iterator<Item = &TypeClassReference> {
        self.restrictions.iter()
    }

    pub fn restrictions_mut(&mut self) -> impl Iterator<Item = &mut TypeClassReference> {
        self.restrictions.iter_mut()
    }

    pub fn add_to_restrictions<I>(&mut self, value: I)
    where
        I: Into<TypeClassReference>,
    {
        self.restrictions.push(value.into())
    }

    pub fn extend_restrictions<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = TypeClassReference>,
    {
        self.restrictions.extend(extension)
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Definitions ❱ TypeClassReference
// ------------------------------------------------------------------------------------------------

impl HasSourceSpan for TypeClassReference {
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

impl TypeClassReference {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub const fn new(name: IdentifierReference) -> Self {
        Self {
            span: None,
            name,
            arguments: Vec::new(),
        }
    }

    pub fn with_arguments<I>(self, arguments: I) -> Self
    where
        I: IntoIterator<Item = TypeClassArgument>,
    {
        Self {
            arguments: Vec::from_iter(arguments),
            ..self
        }
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    pub const fn name(&self) -> &IdentifierReference {
        &self.name
    }

    pub fn set_name(&mut self, name: IdentifierReference) {
        self.name = name;
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_arguments(&self) -> bool {
        !self.arguments.is_empty()
    }

    pub fn arguments_len(&self) -> usize {
        self.arguments.len()
    }

    pub fn arguments(&self) -> impl Iterator<Item = &TypeClassArgument> {
        self.arguments.iter()
    }

    pub fn arguments_mut(&mut self) -> impl Iterator<Item = &mut TypeClassArgument> {
        self.arguments.iter_mut()
    }

    pub fn add_to_arguments<I>(&mut self, value: I)
    where
        I: Into<TypeClassArgument>,
    {
        self.arguments.push(value.into())
    }

    pub fn extend_arguments<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = TypeClassArgument>,
    {
        self.arguments.extend(extension)
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Definitions ❱ TypeClassArgument
// ------------------------------------------------------------------------------------------------

impl TypeClassArgument {
    // --------------------------------------------------------------------------------------------
    // Variants
    // --------------------------------------------------------------------------------------------

    pub const fn is_wildcard(&self) -> bool {
        matches!(self, Self::Wildcard)
    }

    pub const fn is_reference(&self) -> bool {
        matches!(self, Self::Reference(_))
    }

    pub const fn as_reference(&self) -> Option<&TypeClassReference> {
        match self {
            Self::Reference(v) => Some(v),
            _ => None,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Definitions ❱ TypeClassBody
// ------------------------------------------------------------------------------------------------

impl HasSourceSpan for TypeClassBody {
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

impl HasAnnotations for TypeClassBody {
    fn has_annotations(&self) -> bool {
        !self.annotations.is_empty()
    }

    fn annotation_count(&self) -> usize {
        self.annotations.len()
    }

    fn annotations(&self) -> impl Iterator<Item = &Annotation> {
        self.annotations.iter()
    }

    fn annotations_mut(&mut self) -> impl Iterator<Item = &mut Annotation> {
        self.annotations.iter_mut()
    }

    fn add_to_annotations<I>(&mut self, value: I)
    where
        I: Into<Annotation>,
    {
        self.annotations.push(value.into())
    }

    fn extend_annotations<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = Annotation>,
    {
        self.annotations.extend(extension.into_iter())
    }
}

impl TypeClassBody {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn with_methods<I>(self, methods: I) -> Self
    where
        I: IntoIterator<Item = MethodDef>,
    {
        Self {
            methods: methods
                .into_iter()
                .map(|elem| (elem.name().clone(), elem))
                .collect(),
            ..self
        }
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    pub fn has_methods(&self) -> bool {
        !self.methods.is_empty()
    }

    pub fn method_count(&self) -> usize {
        self.methods.len()
    }

    pub fn contains_method(&self, name: &Identifier) -> bool {
        self.methods.contains_key(name)
    }

    pub fn method(&self, name: &Identifier) -> Option<&MethodDef> {
        self.methods.get(name)
    }

    pub fn method_mut(&mut self, name: &Identifier) -> Option<&mut MethodDef> {
        self.methods.get_mut(name)
    }

    pub fn methods(&self) -> impl Iterator<Item = &MethodDef> {
        self.methods.values()
    }

    pub fn methods_mut(&mut self) -> impl Iterator<Item = &mut MethodDef> {
        self.methods.values_mut()
    }

    pub fn method_names(&self) -> impl Iterator<Item = &Identifier> {
        self.methods.keys()
    }

    pub fn add_to_methods(&mut self, value: MethodDef) -> Option<MethodDef> {
        self.methods.insert(value.name().clone(), value)
    }

    pub fn extend_methods<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = MethodDef>,
    {
        self.methods.extend(
            extension
                .into_iter()
                .map(|elem| (elem.name().clone(), elem)),
        )
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Definitions ❱ MethodDef
// ------------------------------------------------------------------------------------------------

impl HasAnnotations for MethodDef {
    fn has_annotations(&self) -> bool {
        !self.annotations.is_empty()
    }

    fn annotation_count(&self) -> usize {
        self.annotations.len()
    }

    fn annotations(&self) -> impl Iterator<Item = &Annotation> {
        self.annotations.iter()
    }

    fn annotations_mut(&mut self) -> impl Iterator<Item = &mut Annotation> {
        self.annotations.iter_mut()
    }

    fn add_to_annotations<I>(&mut self, value: I)
    where
        I: Into<Annotation>,
    {
        self.annotations.push(value.into())
    }

    fn extend_annotations<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = Annotation>,
    {
        self.annotations.extend(extension.into_iter())
    }
}

impl HasName for MethodDef {
    fn name(&self) -> &Identifier {
        &self.name
    }

    fn set_name(&mut self, name: Identifier) {
        self.name = name;
    }
}

impl HasOptionalBody for MethodDef {
    type Body = ConstraintSentence;

    fn body(&self) -> Option<&Self::Body> {
        self.body.as_ref()
    }

    fn body_mut(&mut self) -> Option<&mut Self::Body> {
        self.body.as_mut()
    }

    fn set_body(&mut self, body: Self::Body) {
        self.body = Some(body);
    }

    fn unset_body(&mut self) {
        self.body = None;
    }
}

impl HasSourceSpan for MethodDef {
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

impl MethodDef {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub const fn new(name: Identifier, signature: FunctionSignature) -> Self {
        Self {
            span: None,
            name,
            signature,
            body: None,
            annotations: Vec::new(),
        }
    }

    pub fn with_body(self, body: ConstraintSentence) -> Self {
        Self {
            body: Some(body),
            ..self
        }
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    pub const fn signature(&self) -> &FunctionSignature {
        &self.signature
    }

    pub fn set_signature(&mut self, signature: FunctionSignature) {
        self.signature = signature;
    }
}
