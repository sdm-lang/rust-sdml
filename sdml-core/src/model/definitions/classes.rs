/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::cache::ModuleStore;
use crate::load::ModuleLoader;
use crate::model::annotations::Annotation;
use crate::model::check::Validate;
use crate::model::constraints::{ConstraintSentence, FunctionCardinality, FunctionSignature};
use crate::model::identifiers::{Identifier, IdentifierReference};
use crate::model::{HasName, References, Span};

use sdml_errors::diagnostics::functions::IdentifierCaseConvention;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types
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
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    methods: Vec<MethodDef>,
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
// Implementations
// ------------------------------------------------------------------------------------------------

impl_has_name_for!(TypeClassDef);

impl_has_optional_body_for!(TypeClassDef, TypeClassBody);

impl_has_source_span_for!(TypeClassDef);

impl_annotation_builder!(TypeClassDef, optional body);

impl_maybe_incomplete_for!(TypeClassDef; exists body);

impl References for TypeClassDef {
    fn referenced_types<'a>(
        &'a self,
        _names: &mut std::collections::HashSet<&'a IdentifierReference>,
    ) {
    }

    fn referenced_annotations<'a>(
        &'a self,
        _names: &mut std::collections::HashSet<&'a IdentifierReference>,
    ) {
    }
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
    // TypeClassDef :: Constructors
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
    // TypeClassDef :: Fields
    // --------------------------------------------------------------------------------------------

    get_and_set_vec!(
        pub
        has has_variables,
        variables_len,
        variables,
        variables_mut,
        add_to_variables,
        extend_variables
            => variables, TypeVariable
    );
}

// ------------------------------------------------------------------------------------------------

impl_has_name_for!(TypeVariable);

impl_has_source_span_for!(TypeVariable);

impl TypeVariable {
    // --------------------------------------------------------------------------------------------
    // TypeVariable :: Constructors
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
    // TypeVariable :: Fields
    // --------------------------------------------------------------------------------------------

    get_and_set!(pub cardinality, set_cardinality, unset_cardinality => optional has_cardinality, FunctionCardinality);

    get_and_set_vec!(
        pub
        has has_restrictions,
        restrictions_len,
        restrictions,
        restrictions_mut,
        add_to_restrictions,
        extend_restrictions
            => restrictions, TypeClassReference
    );
}

// ------------------------------------------------------------------------------------------------

impl_has_source_span_for!(TypeClassReference);

impl TypeClassReference {
    // --------------------------------------------------------------------------------------------
    // TypeClassReference :: Constructors
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
    // TypeClassReference :: Fields
    // --------------------------------------------------------------------------------------------

    get_and_set!(pub name, set_name => IdentifierReference);

    get_and_set_vec!(
        pub
        has has_arguments,
        arguments_len,
        arguments,
        arguments_mut,
        add_to_arguments,
        extend_arguments
            => arguments, TypeClassArgument
    );
}

// ------------------------------------------------------------------------------------------------

impl TypeClassArgument {
    // --------------------------------------------------------------------------------------------
    // TypeClassArgument :: Variants
    // --------------------------------------------------------------------------------------------

    is_variant!(Wildcard  => is_wildcard);

    is_as_variant!(Reference (TypeClassReference) => is_reference, as_reference);
}

// ------------------------------------------------------------------------------------------------

impl_has_source_span_for!(TypeClassBody);

impl_has_annotations_for!(TypeClassBody);

impl TypeClassBody {
    // --------------------------------------------------------------------------------------------
    // TypeClassBody :: Constructors
    // --------------------------------------------------------------------------------------------

    pub fn with_methods<I>(self, methods: I) -> Self
    where
        I: IntoIterator<Item = MethodDef>,
    {
        Self {
            methods: Vec::from_iter(methods),
            ..self
        }
    }

    // --------------------------------------------------------------------------------------------
    // TypeClassBody :: Fields
    // --------------------------------------------------------------------------------------------

    get_and_set_vec!(
        pub
        has has_methods,
        methods_len,
        methods,
        methods_mut,
        add_to_methods,
        extend_methods
            => methods, MethodDef
    );
}

// ------------------------------------------------------------------------------------------------

impl_has_annotations_for!(MethodDef);

impl_has_name_for!(MethodDef);

impl_has_optional_body_for!(MethodDef, ConstraintSentence);

impl_has_source_span_for!(MethodDef);

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

    get_and_set!(pub signature, set_signature => FunctionSignature);
}
