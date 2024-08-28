/*!
This module provides a generator that creates the s-expression representation of a module given its
in-memory representation.

*/

use crate::Generator;
use sdml_core::cache::ModuleStore;
use sdml_core::error::Error;
use sdml_core::model::annotations::{
    Annotation, AnnotationOnlyBody, AnnotationProperty, HasAnnotations,
};
use sdml_core::model::constraints::Constraint;
use sdml_core::model::definitions::{
    DatatypeDef, Definition, EntityBody, EntityDef, EnumBody, EnumDef, EventDef, HasMembers,
    HasVariants, PropertyDef, RdfDef, StructureBody, StructureDef, TypeClassDef, TypeVariant,
    UnionBody, UnionDef, ValueVariant,
};
use sdml_core::model::identifiers::{Identifier, IdentifierReference, QualifiedIdentifier};
use sdml_core::model::members::{
    Cardinality, MappingType, Member, MemberDef, MemberKind, TypeReference, DEFAULT_CARDINALITY,
};
use sdml_core::model::modules::{Import, ImportStatement, Module, ModuleBody};
use sdml_core::model::values::{
    LanguageTag, MappingValue, SequenceMember, SequenceOfValues, SimpleValue, Value,
    ValueConstructor,
};
use sdml_core::model::{HasBody, HasName, HasNameReference, HasOptionalBody, HasSourceSpan, Span};
use sdml_core::syntax::{
    FIELD_NAME_BASE, FIELD_NAME_BODY, FIELD_NAME_CARDINALITY, FIELD_NAME_DOMAIN,
    FIELD_NAME_IDENTITY, FIELD_NAME_LANGUAGE, FIELD_NAME_MEMBER, FIELD_NAME_MODULE,
    FIELD_NAME_NAME, FIELD_NAME_ORDERING, FIELD_NAME_RANGE, FIELD_NAME_RENAME, FIELD_NAME_SOURCE,
    FIELD_NAME_TARGET, FIELD_NAME_UNIQUENESS, FIELD_NAME_VALUE, FIELD_NAME_VERSION_INFO,
    FIELD_NAME_VERSION_URI, NODE_KIND_ANNOTATION_ONLY_BODY, NODE_KIND_ANNOTATION_PROPERTY,
    NODE_KIND_BINARY, NODE_KIND_BOOLEAN, NODE_KIND_CARDINALITY_EXPRESSION, NODE_KIND_DATA_TYPE_DEF,
    NODE_KIND_DECIMAL, NODE_KIND_DOUBLE, NODE_KIND_ENTITY_BODY, NODE_KIND_ENTITY_DEF,
    NODE_KIND_ENTITY_IDENTITY, NODE_KIND_ENUM_BODY, NODE_KIND_ENUM_DEF, NODE_KIND_EVENT_DEF,
    NODE_KIND_IDENTIFIER, NODE_KIND_IDENTIFIER_REFERENCE, NODE_KIND_IMPORT_STATEMENT,
    NODE_KIND_INTEGER, NODE_KIND_IRI, NODE_KIND_LANGUAGE_TAG, NODE_KIND_MAPPING_TYPE,
    NODE_KIND_MAPPING_VALUE, NODE_KIND_MEMBER, NODE_KIND_MEMBER_DEF, NODE_KIND_MEMBER_IMPORT,
    NODE_KIND_MODULE, NODE_KIND_MODULE_BODY, NODE_KIND_MODULE_IMPORT, NODE_KIND_PROPERTY_DEF,
    NODE_KIND_PROPERTY_REF, NODE_KIND_QUALIFIED_IDENTIFIER, NODE_KIND_RDF_DEF,
    NODE_KIND_SEQUENCE_OF_VALUES, NODE_KIND_SPAN, NODE_KIND_STRING, NODE_KIND_STRUCTURE_BODY,
    NODE_KIND_STRUCTURE_DEF, NODE_KIND_TYPE_REFERENCE, NODE_KIND_TYPE_VARIANT,
    NODE_KIND_UNION_BODY, NODE_KIND_UNION_DEF, NODE_KIND_UNSIGNED, NODE_KIND_VALUE_CONSTRUCTOR,
    NODE_KIND_VALUE_VARIANT,
};
use std::io::Write;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct SExpressionGenerator {
    options: SExpressionOptions,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SExpressionOptions {
    line_width: usize,
    pair_kw_args: bool,
    wrap_in_define: bool,
    style: SExpressionStyle,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SExpressionStyle {
    #[default]
    TreeSitter,
    Racket,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

const LPAREN: &[u8] = b"(";
const RPAREN: &[u8] = b")";
const PARENS: &[u8] = b"()";
const SPACE: &[u8] = b" ";
const NEWLN: &[u8] = b"\n";

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum SValue {
    Single(String),
    List(Vec<SValue>),
    Funcall(String, Vec<SValue>),
}

trait SList {
    fn push_keyword<K>(&mut self, kw: K, style: SExpressionStyle)
    where
        K: Into<String>;
    fn push_keyword_arg<K, V>(&mut self, kw: K, value: V, style: SExpressionStyle)
    where
        K: Into<String>,
        V: Into<SValue>;
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Generator for SExpressionGenerator {
    type Options = SExpressionOptions;

    fn generate_with_options<W>(
        &mut self,
        module: &Module,
        _: &impl ModuleStore,
        options: Self::Options,
        _: Option<std::path::PathBuf>,
        writer: &mut W,
    ) -> Result<(), Error>
    where
        W: Write + Sized,
    {
        self.options = options;
        let value = self.module_to_svalue(module);
        value.pretty_print(self.options.line_width, self.options.style, writer)?;
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------

impl SExpressionGenerator {
    fn annotations_to_svalue(&self, me: &impl HasAnnotations) -> SValue {
        let mut list: Vec<SValue> = Default::default();

        for annotation in me.annotations() {
            list.push(match &annotation {
                Annotation::Property(v) => self.annotation_property_to_svalue(v),
                Annotation::Constraint(v) => self.annotation_constraint_to_svalue(v),
            });
        }

        SValue::List(list)
    }

    fn identifier_to_svalue(&self, me: &Identifier) -> SValue {
        let args = if let Some(span) = me.source_span() {
            vec![self.span_to_svalue(span), SValue::quoted(me)]
        } else {
            vec![SValue::quoted(me)]
        };
        SValue::function_call(NODE_KIND_IDENTIFIER, args)
    }

    fn qualified_identifier_to_svalue(&self, me: &QualifiedIdentifier) -> SValue {
        let mut args = Vec::new();

        if let Some(span) = me.source_span() {
            args.push(self.span_to_svalue(span));
        }

        args.push_keyword_arg(
            FIELD_NAME_MODULE,
            SValue::quoted(me.module()),
            self.options.style,
        );

        args.push_keyword_arg(
            FIELD_NAME_MEMBER,
            SValue::quoted(me.member()),
            self.options.style,
        );

        SValue::function_call(NODE_KIND_QUALIFIED_IDENTIFIER, args)
    }

    fn identifier_reference_to_svalue(&self, me: &IdentifierReference) -> SValue {
        SValue::function_call(
            NODE_KIND_IDENTIFIER_REFERENCE,
            vec![match me {
                IdentifierReference::Identifier(v) => self.identifier_to_svalue(v),
                IdentifierReference::QualifiedIdentifier(v) => {
                    self.qualified_identifier_to_svalue(v)
                }
            }],
        )
    }

    fn module_to_svalue(&self, me: &Module) -> SValue {
        let mut args = Vec::new();

        if let Some(span) = me.source_span() {
            args.push(self.span_to_svalue(span));
        }

        args.push_keyword_arg(
            FIELD_NAME_NAME,
            self.identifier_to_svalue(me.name()),
            self.options.style,
        );

        if let Some(base_uri) = me.base_uri() {
            args.push_keyword_arg(
                FIELD_NAME_BASE,
                SValue::string(base_uri.to_string()),
                self.options.style,
            );
        }

        if let Some(version_info) = me.version_info() {
            args.push_keyword_arg(
                FIELD_NAME_VERSION_INFO,
                SValue::string(version_info.value().to_string()),
                self.options.style,
            );
        }

        if let Some(version_uri) = me.version_uri() {
            args.push_keyword_arg(
                FIELD_NAME_VERSION_URI,
                SValue::string(version_uri.to_string()),
                self.options.style,
            );
        }

        args.push_keyword_arg(
            FIELD_NAME_BODY,
            self.module_body_to_svalue(me.body()),
            self.options.style,
        );

        let module = SValue::function_call(NODE_KIND_MODULE, args);
        if self.options.wrap_in_define {
            let name = format!("module/{}", me.name());
            SValue::function_call("define", vec![SValue::quoted(name), module])
        } else {
            module
        }
    }

    fn module_body_to_svalue(&self, me: &ModuleBody) -> SValue {
        let mut args = Vec::new();

        if let Some(span) = me.source_span() {
            args.push(self.span_to_svalue(span));
        }

        for import in me.imports() {
            args.push(self.import_statement_to_svalue(import));
        }

        if me.has_annotations() {
            args.push(self.annotations_to_svalue(me));
        }

        for definition in me.definitions() {
            args.push(self.definition_to_svalue(definition));
        }

        SValue::function_call(NODE_KIND_MODULE_BODY, args)
    }

    fn import_statement_to_svalue(&self, me: &ImportStatement) -> SValue {
        let mut args = Vec::new();

        if let Some(span) = me.source_span() {
            args.push(self.span_to_svalue(span));
        }

        for import in me.imports() {
            args.push(self.import_to_svalue(import));
        }

        SValue::function_call(NODE_KIND_IMPORT_STATEMENT, args)
    }

    fn import_to_svalue(&self, me: &Import) -> SValue {
        match me {
            Import::Module(v) => {
                let mut args = Vec::new();

                if let Some(span) = me.source_span() {
                    args.push(self.span_to_svalue(span));
                }

                args.push_keyword_arg(
                    FIELD_NAME_NAME,
                    self.identifier_to_svalue(v.name()),
                    self.options.style,
                );

                if let Some(version_uri) = v.version_uri() {
                    args.push_keyword_arg(
                        FIELD_NAME_VERSION_URI,
                        SValue::string(version_uri.to_string()),
                        self.options.style,
                    );
                }

                SValue::function_call(NODE_KIND_MODULE_IMPORT, args)
            }
            Import::Member(v) => {
                let mut args = Vec::new();

                if let Some(span) = me.source_span() {
                    args.push(self.span_to_svalue(span));
                }

                args.push_keyword_arg(
                    FIELD_NAME_NAME,
                    self.qualified_identifier_to_svalue(v),
                    self.options.style,
                );

                SValue::function_call(NODE_KIND_MEMBER_IMPORT, args)
            }
        }
    }

    fn annotation_property_to_svalue(&self, me: &AnnotationProperty) -> SValue {
        let mut args = Vec::new();

        if let Some(span) = me.source_span() {
            args.push(self.span_to_svalue(span));
        }

        args.push_keyword_arg(
            FIELD_NAME_NAME,
            self.identifier_reference_to_svalue(me.name_reference()),
            self.options.style,
        );

        args.push_keyword_arg(
            FIELD_NAME_VALUE,
            self.value_to_svalue(me.value()),
            self.options.style,
        );

        SValue::function_call(NODE_KIND_ANNOTATION_PROPERTY, args)
    }

    //fn write_constraint<W: Write>(&mut self, me: &Constraint, w: &mut W) -> Result<(), Error> {
    //    self.start_node_indented(NODE_KIND_CONSTRAINT, w)?;
    //    self.indent();
    //
    //    self.write_source_span(me, w)?;
    //
    //    self.newln_and_indentation(w)?;
    //    self.field_name(FIELD_NAME_NAME, w)?;
    //    self.write_identifier(me.name(), w)?;
    //
    //    self.newln_and_indentation(w)?;
    //    self.field_name(FIELD_NAME_BODY, w)?;
    //    match me.body() {
    //        ConstraintBody::Informal(v) => {
    //            self.start_node(NODE_KIND_INFORMAL_CONSTRAINT, w)?;
    //            self.indent();
    //
    //            self.newln_and_indentation(w)?;
    //            self.node_and_value(NODE_KIND_QUOTED_STRING, format!("{:?}", v.value()), w)?;
    //
    //            if let Some(language) = v.language() {
    //                self.newln_and_indentation(w)?;
    //                self.field_name(FIELD_NAME_LANGUAGE, w)?;
    //                self.start_node(NODE_KIND_CONTROLLED_LANGUAGE_TAG, w)?;
    //                self.value_with_prefix(language, "'", w)?;
    //                self.close_paren(w)?;
    //            }
    //
    //            self.outdent();
    //            self.close_paren(w)?
    //        }
    //        ConstraintBody::Formal(v) => {
    //            self.write_formal_constraint(v, w)?;
    //        }
    //    }
    //
    //    self.close_paren(w)?;
    //    self.outdent();
    //    Ok(())
    //}

    fn annotation_constraint_to_svalue(&self, _me: &Constraint) -> SValue {
        todo!()
    }

    //fn write_formal_constraint<W: Write>(
    //    &mut self,
    //    me: &FormalConstraint,
    //    w: &mut W,
    //) -> Result<(), Error> {
    //    self.start_node(NODE_KIND_FORMAL_CONSTRAINT, w)?;
    //    self.indent();
    //
    //    self.write_source_span(me, w)?;
    //
    //    if me.has_definitions() {
    //        self.newln_and_indentation(w)?;
    //        self.start_node(NODE_KIND_CONSTRAINT_ENVIRONMENT, w)?;
    //        self.indent();
    //
    //        for defn in me.definitions() {
    //            self.newln(w)?;
    //            self.write_environment_definition(defn, w)?;
    //        }
    //
    //        self.outdent();
    //        self.close_paren(w)?
    //    }
    //
    //    self.newln_and_indentation(w)?;
    //    self.field_name(FIELD_NAME_BODY, w)?;
    //    self.write_constraint_sentence(me.body(), w)?;
    //
    //    self.close_paren(w)?;
    //    self.outdent();
    //    Ok(())
    //}

    //fn write_environment_definition<W: Write>(
    //    &mut self,
    //    me: &EnvironmentDef,
    //    w: &mut W,
    //) -> Result<(), Error> {
    //    self.start_node_indented(NODE_KIND_ENVIRONMENT_DEF, w)?;
    //    self.indent();
    //
    //    self.write_source_span(me, w)?;
    //
    //    self.newln_and_indentation(w)?;
    //    self.field_name(FIELD_NAME_NAME, w)?;
    //    self.write_identifier(me.name(), w)?;
    //
    //    self.newln_and_indentation(w)?;
    //    self.field_name(FIELD_NAME_BODY, w)?;
    //
    //    match me.body() {
    //        EnvironmentDefBody::Function(v) => self.write_function_def(v, w)?,
    //        EnvironmentDefBody::Value(v) => self.write_predicate_value(v, w)?,
    //        EnvironmentDefBody::Sentence(v) => self.write_constraint_sentence(v, w)?,
    //    }
    //
    //    self.close_paren(w)?;
    //    self.outdent();
    //    Ok(())
    //}

    //fn write_function_def<W: Write>(&mut self, me: &FunctionDef, w: &mut W) -> Result<(), Error> {
    //    self.start_node(NODE_KIND_FUNCTION_DEF, w)?;
    //    self.indent();
    //
    //    self.write_source_span(me, w)?;
    //
    //    self.newln_and_indentation(w)?;
    //    self.field_name(FIELD_NAME_SIGNATURE, w)?;
    //    self.write_function_signature(me.signature(), w)?;
    //
    //    self.newln_and_indentation(w)?;
    //    self.field_name(FIELD_NAME_BODY, w)?;
    //    self.write_constraint_sentence(me.body(), w)?;
    //
    //    self.close_paren(w)?;
    //    self.outdent();
    //    Ok(())
    //}

    //fn write_function_signature<W: Write>(
    //    &mut self,
    //    me: &FunctionSignature,
    //    w: &mut W,
    //) -> Result<(), Error> {
    //    self.start_node(NODE_KIND_FUNCTION_SIGNATURE, w)?;
    //    self.indent();
    //
    //    self.write_source_span(me, w)?;
    //
    //    for parameter in me.parameters() {
    //        self.newln_and_indentation(w)?;
    //        self.field_name(FIELD_NAME_PARAMETER, w)?;
    //        self.write_function_parameter(parameter, w)?;
    //    }
    //
    //    self.write_function_type(me.target_type(), w)?;
    //
    //    self.close_paren(w)?;
    //    self.outdent();
    //    Ok(())
    //}

    //fn write_function_parameter<W: Write>(
    //    &mut self,
    //    me: &FunctionParameter,
    //    w: &mut W,
    //) -> Result<(), Error> {
    //    self.start_node(NODE_KIND_FUNCTION_PARAMETER, w)?;
    //    self.indent();
    //
    //    self.write_source_span(me, w)?;
    //
    //    self.newln_and_indentation(w)?;
    //    self.field_name(FIELD_NAME_NAME, w)?;
    //    self.write_identifier(me.name(), w)?;
    //
    //    self.write_function_type(me.target_type(), w)?;
    //
    //    self.close_paren(w)?;
    //    self.outdent();
    //    Ok(())
    //}

    //fn write_function_type<W: Write>(&mut self, me: &FunctionType, w: &mut W) -> Result<(), Error> {
    //    // TODO: cardinality
    //
    //    self.newln_and_indentation(w)?;
    //    self.field_name(FIELD_NAME_NAME, w)?;
    //
    //    // TODO: Optional
    //    match me.target_type().inner() {
    //        FunctionTypeReferenceInner::Wildcard => self.node(NODE_KIND_WILDCARD, w)?,
    //        FunctionTypeReferenceInner::Reference(v) => self.write_identifier_reference(v, w)?,
    //        FunctionTypeReferenceInner::MappingType(v) => self.write_mapping_type(v, w)?,
    //    }
    //
    //    Ok(())
    //}

    //fn write_mapping_type<W: Write>(&mut self, me: &MappingType, w: &mut W) -> Result<(), Error> {
    //    self.start_node(NODE_KIND_MAPPING_TYPE, w)?;
    //    self.indent();
    //
    //    self.write_source_span(me, w)?;
    //
    //    self.newln_and_indentation(w)?;
    //    self.field_name(FIELD_NAME_DOMAIN, w)?;
    //    self.write_type_reference(me.domain(), w)?;
    //
    //    self.newln_and_indentation(w)?;
    //    self.field_name(FIELD_NAME_RANGE, w)?;
    //    self.write_type_reference(me.range(), w)?;
    //
    //    self.close_paren(w)?;
    //    self.outdent();
    //    Ok(())
    //}

    //fn write_constraint_sentence<W: Write>(
    //    &mut self,
    //    me: &ConstraintSentence,
    //    w: &mut W,
    //) -> Result<(), Error> {
    //    self.start_node(NODE_KIND_CONSTRAINT_SENTENCE, w)?;
    //    self.indent();
    //
    //    match me {
    //        ConstraintSentence::Simple(v) => {
    //            self.newln_and_indentation(w)?;
    //            self.start_node(NODE_KIND_SIMPLE_SENTENCE, w)?;
    //            self.indent();
    //            match v {
    //                SimpleSentence::Atomic(v) => self.write_atomic_sentence(v, w)?,
    //                SimpleSentence::Equation(v) => self.write_equation(v, w)?,
    //                SimpleSentence::Inequation(v) => self.write_inequation(v, w)?,
    //            }
    //            self.close_paren(w)?;
    //            self.outdent();
    //        }
    //        ConstraintSentence::Boolean(v) => self.write_boolean_sentence(v, w)?,
    //        ConstraintSentence::Quantified(v) => self.write_quantified_sentence(v, w)?,
    //    }
    //
    //    self.close_paren(w)?;
    //    self.outdent();
    //    Ok(())
    //}

    //fn write_atomic_sentence<W: Write>(
    //    &mut self,
    //    me: &AtomicSentence,
    //    w: &mut W,
    //) -> Result<(), Error> {
    //    self.newln_and_indentation(w)?;
    //    self.start_node(NODE_KIND_ATOMIC_SENTENCE, w)?;
    //    self.indent();
    //
    //    self.write_source_span(me, w)?;
    //
    //    self.newln_and_indentation(w)?;
    //    self.field_name(FIELD_NAME_PREDICATE, w)?;
    //    self.write_term(me.predicate(), w)?;
    //
    //    for argument in me.arguments() {
    //        self.newln_and_indentation(w)?;
    //        self.field_name(FIELD_NAME_ARGUMENT, w)?;
    //        self.write_term(argument, w)?;
    //    }
    //
    //    self.close_paren(w)?;
    //    self.outdent();
    //    Ok(())
    //}

    //fn write_equation<W: Write>(&mut self, me: &Equation, w: &mut W) -> Result<(), Error> {
    //    self.newln_and_indentation(w)?;
    //    self.start_node(NODE_KIND_EQUATION, w)?;
    //    self.indent();
    //
    //    self.write_source_span(me, w)?;
    //
    //    self.newln_and_indentation(w)?;
    //    self.field_name(FIELD_NAME_LHS, w)?;
    //    self.write_term(me.left_operand(), w)?;
    //
    //    self.newln_and_indentation(w)?;
    //    self.field_name(FIELD_NAME_RHS, w)?;
    //    self.write_term(me.right_operand(), w)?;
    //
    //    self.close_paren(w)?;
    //    self.outdent();
    //    Ok(())
    //}

    //fn write_inequation<W: Write>(&mut self, me: &Inequation, w: &mut W) -> Result<(), Error> {
    //    self.newln_and_indentation(w)?;
    //    self.start_node(NODE_KIND_INEQUATION, w)?;
    //    self.indent();
    //
    //    self.write_source_span(me, w)?;
    //
    //    self.newln_and_indentation(w)?;
    //    self.field_name(FIELD_NAME_LHS, w)?;
    //    self.write_term(me.left_operand(), w)?;
    //
    //    self.newln_and_indentation(w)?;
    //    self.field_name(FIELD_NAME_RELATION, w)?;
    //    match me.relation() {
    //        InequalityRelation::NotEqual => self.node(NODE_KIND_NOT_EQUAL, w)?,
    //        InequalityRelation::LessThan => self.node(NODE_KIND_LESS_THAN, w)?,
    //        InequalityRelation::LessThanOrEqual => self.node(NODE_KIND_LESS_THAN_OR_EQUAL, w)?,
    //        InequalityRelation::GreaterThan => self.node(NODE_KIND_GREATER_THAN, w)?,
    //        InequalityRelation::GreaterThanOrEqual => {
    //            self.node(NODE_KIND_GREATER_THAN_OR_EQUAL, w)?
    //        }
    //    }
    //
    //    self.newln_and_indentation(w)?;
    //    self.field_name(FIELD_NAME_RHS, w)?;
    //    self.write_term(me.right_operand(), w)?;
    //
    //    self.close_paren(w)?;
    //    self.outdent();
    //    Ok(())
    //}

    //fn write_boolean_sentence<W: Write>(
    //    &mut self,
    //    me: &BooleanSentence,
    //    w: &mut W,
    //) -> Result<(), Error> {
    //    self.newln_and_indentation(w)?;
    //    self.start_node(NODE_KIND_BOOLEAN_SENTENCE, w)?;
    //    self.indent();
    //
    //    match me {
    //        BooleanSentence::Unary(v) => {
    //            self.newln_and_indentation(w)?;
    //            self.field_name(FIELD_NAME_OPERATOR, w)?;
    //            self.node(NODE_KIND_NEGATION, w)?;
    //
    //            self.newln_and_indentation(w)?;
    //            self.field_name(FIELD_NAME_RHS, w)?;
    //            self.write_constraint_sentence(v.operand(), w)?;
    //        }
    //        BooleanSentence::Binary(v) => {
    //            self.newln_and_indentation(w)?;
    //            self.field_name(FIELD_NAME_LHS, w)?;
    //            self.write_constraint_sentence(v.left_operand(), w)?;
    //
    //            self.newln_and_indentation(w)?;
    //            self.field_name(FIELD_NAME_OPERATOR, w)?;
    //            match v.operator() {
    //                ConnectiveOperator::Negation => unreachable!(),
    //                ConnectiveOperator::Conjunction => self.node(NODE_KIND_CONJUNCTION, w)?,
    //                ConnectiveOperator::Disjunction => self.node(NODE_KIND_DISJUNCTION, w)?,
    //                ConnectiveOperator::ExclusiveDisjunction => {
    //                    self.node(NODE_KIND_EXCLUSIVE_DISJUNCTION, w)?
    //                }
    //                ConnectiveOperator::Implication => self.node(NODE_KIND_IMPLICATION, w)?,
    //                ConnectiveOperator::Biconditional => self.node(NODE_KIND_BICONDITIONAL, w)?,
    //            }
    //
    //            self.newln_and_indentation(w)?;
    //            self.field_name(FIELD_NAME_RHS, w)?;
    //            self.write_constraint_sentence(v.right_operand(), w)?;
    //        }
    //    }
    //
    //    self.close_paren(w)?;
    //    self.outdent();
    //    Ok(())
    //}

    //fn write_quantified_sentence<W: Write>(
    //    &mut self,
    //    me: &QuantifiedSentence,
    //    w: &mut W,
    //) -> Result<(), Error> {
    //    self.newln_and_indentation(w)?;
    //    self.start_node(NODE_KIND_QUANTIFIED_SENTENCE, w)?;
    //    self.indent();
    //
    //    self.write_source_span(me, w)?;
    //
    //    self.newln_and_indentation(w)?;
    //    self.field_name(FIELD_NAME_BINDING, w)?;
    //    self.write_quantified_variable_binding(me.binding(), w)?;
    //
    //    self.newln_and_indentation(w)?;
    //    self.field_name(FIELD_NAME_BODY, w)?;
    //    self.write_constraint_sentence(me.body(), w)?;
    //
    //    self.close_paren(w)?;
    //    self.outdent();
    //    Ok(())
    //}

    //fn write_quantified_variable_binding<W: Write>(
    //    &mut self,
    //    me: &QuantifiedVariableBinding,
    //    w: &mut W,
    //) -> Result<(), Error> {
    //    self.start_node(NODE_KIND_QUANTIFIED_VARIABLE_BINDING, w)?;
    //    self.indent();
    //
    //    self.write_source_span(me, w)?;
    //
    //    self.newln_and_indentation(w)?;
    //    self.field_name(FIELD_NAME_QUANTIFIER, w)?;
    //    match me.quantifier() {
    //        Quantifier::Universal => self.node(NODE_KIND_UNIVERSAL, w)?,
    //        Quantifier::Existential => self.node(NODE_KIND_EXISTENTIAL, w)?,
    //    }
    //
    //    self.newln_and_indentation(w)?;
    //    self.field_name(FIELD_NAME_BINDING, w)?;
    //    if let Some(binding) = me.binding() {
    //        self.write_quantified_variable(binding, w)?;
    //    } else {
    //        self.node(NODE_KIND_RESERVED_SELF, w)?;
    //    }
    //
    //    self.close_paren(w)?;
    //    self.outdent();
    //    Ok(())
    //}

    //fn write_quantified_variable<W: Write>(
    //    &mut self,
    //    me: &QuantifiedVariable,
    //    w: &mut W,
    //) -> Result<(), Error> {
    //    self.start_node(NODE_KIND_QUANTIFIED_VARIABLE, w)?;
    //    self.indent();
    //
    //    self.write_source_span(me, w)?;
    //
    //    self.newln_and_indentation(w)?;
    //    self.field_name(FIELD_NAME_NAME, w)?;
    //    self.write_identifier(me.name(), w)?;
    //
    //    self.newln_and_indentation(w)?;
    //    self.field_name(FIELD_NAME_SOURCE, w)?;
    //    self.write_term(me.source(), w)?;
    //
    //    self.close_paren(w)?;
    //    self.outdent();
    //    Ok(())
    //}

    //fn write_predicate_value<W: Write>(
    //    &mut self,
    //    me: &PredicateValue,
    //    w: &mut W,
    //) -> Result<(), Error> {
    //    self.start_node(NODE_KIND_PREDICATE_VALUE, w)?;
    //    self.indent();
    //
    //    self.newln_and_indentation(w)?;
    //    match me {
    //        PredicateValue::Simple(v) => self.write_simple_value(v, w)?,
    //        PredicateValue::Sequence(vs) => self.write_list_of_predicate_values(vs, w)?,
    //    }
    //
    //    self.close_paren(w)?;
    //    self.outdent();
    //    Ok(())
    //}

    //fn write_list_of_predicate_values<W: Write>(
    //    &mut self,
    //    me: &SequenceOfPredicateValues,
    //    w: &mut W,
    //) -> Result<(), Error> {
    //    self.start_node(NODE_KIND_SEQUENCE_OF_PREDICATE_VALUES, w)?;
    //    self.indent();
    //
    //    self.write_source_span(me, w)?;
    //
    //    for value in me.iter() {
    //        self.newln_and_indentation(w)?;
    //        match value {
    //            PredicateSequenceMember::Simple(v) => self.write_simple_value(v, w)?,
    //            PredicateSequenceMember::Reference(v) => self.write_identifier_reference(v, w)?,
    //            PredicateSequenceMember::ValueConstructor(v) => {
    //                self.write_value_constructor(v, w)?
    //            }
    //            PredicateSequenceMember::Mapping(v) => self.write_mapping_value(v, w)?,
    //        }
    //    }
    //
    //    self.close_paren(w)?;
    //    self.outdent();
    //    Ok(())
    //}

    //fn write_term<W: Write>(&mut self, me: &Term, w: &mut W) -> Result<(), Error> {
    //    self.start_node(NODE_KIND_TERM, w)?;
    //    self.indent();
    //
    //    match me {
    //        Term::Sequence(v) => self.write_sequence_builder(v, w)?,
    //        Term::Function(v) => self.write_functional_term(v, w)?,
    //        Term::Composition(v) => self.write_function_composition(v, w)?,
    //        Term::Identifier(v) => {
    //            self.newln_and_indentation(w)?;
    //            self.write_identifier_reference(v, w)?
    //        }
    //        Term::ReservedSelf => self.node(NODE_KIND_RESERVED_SELF, w)?,
    //        Term::Value(v) => self.write_predicate_value(v, w)?,
    //    }
    //
    //    self.close_paren(w)?;
    //    self.outdent();
    //    Ok(())
    //}

    //fn write_function_composition<W: Write>(
    //    &mut self,
    //    me: &FunctionComposition,
    //    w: &mut W,
    //) -> Result<(), Error> {
    //    self.newln_and_indentation(w)?;
    //    self.start_node(NODE_KIND_FUNCTION_COMPOSITION, w)?;
    //    self.indent();
    //
    //    self.write_source_span(me, w)?;
    //
    //    self.newln_and_indentation(w)?;
    //    self.field_name(FIELD_NAME_SUBJECT, w)?;
    //    match me.subject() {
    //        Subject::ReservedSelf => self.node(NODE_KIND_RESERVED_SELF, w)?,
    //        Subject::Identifier(v) => self.write_identifier(v, w)?,
    //    }
    //
    //    for name in me.function_names() {
    //        self.newln_and_indentation(w)?;
    //        self.field_name(FIELD_NAME_NAME, w)?;
    //        self.write_identifier(name, w)?;
    //    }
    //
    //    self.close_paren(w)?;
    //    self.outdent();
    //    Ok(())
    //}

    //fn write_functional_term<W: Write>(
    //    &mut self,
    //    me: &FunctionalTerm,
    //    w: &mut W,
    //) -> Result<(), Error> {
    //    self.newln_and_indentation(w)?;
    //    self.start_node(NODE_KIND_FUNCTIONAL_TERM, w)?;
    //    self.indent();
    //
    //    self.write_source_span(me, w)?;
    //
    //    self.newln_and_indentation(w)?;
    //    self.field_name(FIELD_NAME_FUNCTION, w)?;
    //    self.write_term(me.function(), w)?;
    //
    //    for argument in me.arguments() {
    //        self.newln_and_indentation(w)?;
    //        self.field_name(FIELD_NAME_ARGUMENT, w)?;
    //        self.write_term(argument, w)?;
    //    }
    //
    //    self.close_paren(w)?;
    //    self.outdent();
    //    Ok(())
    //}

    //fn write_sequence_builder<W: Write>(
    //    &mut self,
    //    me: &SequenceBuilder,
    //    w: &mut W,
    //) -> Result<(), Error> {
    //    self.newln_and_indentation(w)?;
    //    self.start_node(NODE_KIND_SEQUENCE_BUILDER, w)?;
    //    self.indent();
    //
    //    self.write_source_span(me, w)?;
    //
    //    self.newln_and_indentation(w)?;
    //    self.field_name(FIELD_NAME_VARIABLE, w)?;
    //    match me.variables() {
    //        Variables::Named(v) => {
    //            self.newln_and_indentation(w)?;
    //            self.start_node(NODE_KIND_NAMED_VARIABLE_SET, w)?;
    //            self.indent();
    //
    //            self.write_source_span(v, w)?;
    //
    //            for name in v.as_ref() {
    //                self.newln_and_indentation(w)?;
    //                self.field_name(FIELD_NAME_NAME, w)?;
    //                self.write_identifier(name, w)?;
    //            }
    //
    //            self.close_paren(w)?;
    //            self.outdent();
    //        }
    //        Variables::Mapping(v) => {
    //            self.newln_and_indentation(w)?;
    //            self.start_node(NODE_KIND_MAPPING_VARIABLE, w)?;
    //            self.indent();
    //
    //            self.write_source_span(v, w)?;
    //
    //            self.newln_and_indentation(w)?;
    //            self.field_name(FIELD_NAME_DOMAIN, w)?;
    //            self.write_identifier(v.domain(), w)?;
    //
    //            self.newln_and_indentation(w)?;
    //            self.field_name(FIELD_NAME_RANGE, w)?;
    //            self.write_identifier(v.range(), w)?;
    //
    //            self.close_paren(w)?;
    //            self.outdent();
    //        }
    //    }
    //
    //    self.newln_and_indentation(w)?;
    //    self.field_name(FIELD_NAME_BODY, w)?;
    //    self.write_quantified_sentence(me.body(), w)?;
    //
    //    self.close_paren(w)?;
    //    self.outdent();
    //    Ok(())
    //}

    fn value_to_svalue(&self, me: &Value) -> SValue {
        match me {
            Value::Simple(v) => self.simple_value_to_svalue(v),
            Value::ValueConstructor(v) => self.value_constructor_to_svalue(v),
            Value::Mapping(v) => self.mapping_value_to_svalue(v),
            Value::Reference(v) => self.identifier_reference_to_svalue(v),
            Value::List(v) => self.list_value_to_svalue(v),
        }
    }

    fn simple_value_to_svalue(&self, me: &SimpleValue) -> SValue {
        match me {
            SimpleValue::Boolean(v) => {
                SValue::function_call_1(NODE_KIND_BOOLEAN, SValue::value(v.to_string()))
            }
            SimpleValue::Double(v) => {
                SValue::function_call_1(NODE_KIND_DOUBLE, SValue::value(v.to_string()))
            }
            SimpleValue::Decimal(v) => {
                SValue::function_call_1(NODE_KIND_DECIMAL, SValue::value(v.to_string()))
            }
            SimpleValue::Integer(v) => {
                SValue::function_call_1(NODE_KIND_INTEGER, SValue::value(v.to_string()))
            }
            SimpleValue::Unsigned(v) => {
                SValue::function_call_1(NODE_KIND_UNSIGNED, SValue::value(v.to_string()))
            }
            SimpleValue::String(v) => {
                let mut args: Vec<SValue> = vec![SValue::string(v.value())];
                if let Some(language_tag) = v.language() {
                    args.push_keyword_arg(
                        FIELD_NAME_LANGUAGE,
                        self.language_tag_to_svalue(language_tag),
                        self.options.style,
                    )
                }
                SValue::function_call(NODE_KIND_STRING, args)
            }
            SimpleValue::IriReference(v) => {
                SValue::function_call_1(NODE_KIND_IRI, SValue::string(v.to_string()))
            }
            SimpleValue::Binary(v) => {
                SValue::function_call_1(NODE_KIND_BINARY, SValue::value(v.to_string()))
            }
        }
    }

    fn language_tag_to_svalue(&self, me: &LanguageTag) -> SValue {
        SValue::function_call_1(NODE_KIND_LANGUAGE_TAG, SValue::quoted(me.to_string()))
    }

    fn mapping_value_to_svalue(&self, me: &MappingValue) -> SValue {
        let mut args = Vec::new();

        if let Some(span) = me.source_span() {
            args.push(self.span_to_svalue(span));
        }

        args.push_keyword_arg(
            FIELD_NAME_DOMAIN,
            self.simple_value_to_svalue(me.domain()),
            self.options.style,
        );

        args.push_keyword_arg(
            FIELD_NAME_RANGE,
            self.value_to_svalue(me.range()),
            self.options.style,
        );

        SValue::function_call(NODE_KIND_MAPPING_VALUE, args)
    }

    fn list_value_to_svalue(&self, me: &SequenceOfValues) -> SValue {
        let mut args = Vec::new();

        if let Some(span) = me.source_span() {
            args.push(self.span_to_svalue(span));
        }

        for value in me.iter() {
            args.push(match value {
                SequenceMember::Simple(v) => self.simple_value_to_svalue(v),
                SequenceMember::ValueConstructor(v) => self.value_constructor_to_svalue(v),
                SequenceMember::Reference(v) => self.identifier_reference_to_svalue(v),
                SequenceMember::Mapping(v) => self.mapping_value_to_svalue(v),
            });
        }

        SValue::function_call(NODE_KIND_SEQUENCE_OF_VALUES, args)
    }

    fn value_constructor_to_svalue(&self, me: &ValueConstructor) -> SValue {
        let mut args = Vec::new();

        if let Some(span) = me.source_span() {
            args.push(self.span_to_svalue(span));
        }

        args.push_keyword_arg(
            FIELD_NAME_NAME,
            self.identifier_reference_to_svalue(me.type_name()),
            self.options.style,
        );

        args.push_keyword_arg(
            FIELD_NAME_VALUE,
            self.simple_value_to_svalue(me.value()),
            self.options.style,
        );

        SValue::function_call(NODE_KIND_VALUE_CONSTRUCTOR, args)
    }

    fn definition_to_svalue(&self, me: &Definition) -> SValue {
        match me {
            Definition::Datatype(v) => self.datatype_to_svalue(v),
            Definition::Entity(v) => self.entity_to_svalue(v),
            Definition::Enum(v) => self.enum_to_svalue(v),
            Definition::Event(v) => self.event_to_svalue(v),
            Definition::Property(v) => self.property_to_svalue(v),
            Definition::Rdf(v) => self.rdf_to_svalue(v),
            Definition::Structure(v) => self.structure_to_svalue(v),
            Definition::TypeClass(v) => self.typeclass_to_svalue(v),
            Definition::Union(v) => self.union_to_svalue(v),
        }
    }

    fn datatype_to_svalue(&self, me: &DatatypeDef) -> SValue {
        let mut args = Vec::new();

        if let Some(span) = me.source_span() {
            args.push(self.span_to_svalue(span));
        }

        args.push_keyword_arg(
            FIELD_NAME_NAME,
            self.identifier_to_svalue(me.name()),
            self.options.style,
        );

        args.push_keyword_arg(
            FIELD_NAME_BASE,
            self.identifier_reference_to_svalue(me.base_type()),
            self.options.style,
        );

        if let Some(body) = &me.body() {
            args.push_keyword_arg(
                FIELD_NAME_BODY,
                self.annotation_only_body_to_svalue(body),
                self.options.style,
            );
        }

        SValue::function_call(NODE_KIND_DATA_TYPE_DEF, args)
    }

    fn annotation_only_body_to_svalue(&self, me: &AnnotationOnlyBody) -> SValue {
        let mut args = Vec::new();

        if let Some(span) = me.source_span() {
            args.push(self.span_to_svalue(span));
        }

        if me.has_annotations() {
            args.push(self.annotations_to_svalue(me));
        }

        SValue::function_call(NODE_KIND_ANNOTATION_ONLY_BODY, args)
    }

    fn entity_to_svalue(&self, me: &EntityDef) -> SValue {
        let mut args = Vec::new();

        if let Some(span) = me.source_span() {
            args.push(self.span_to_svalue(span));
        }

        args.push_keyword_arg(
            FIELD_NAME_NAME,
            self.identifier_to_svalue(me.name()),
            self.options.style,
        );

        if let Some(body) = &me.body() {
            args.push_keyword_arg(
                FIELD_NAME_BODY,
                self.entity_body_to_svalue(body),
                self.options.style,
            );
        }

        SValue::function_call(NODE_KIND_ENTITY_DEF, args)
    }

    fn entity_body_to_svalue(&self, me: &EntityBody) -> SValue {
        let mut args = Vec::new();

        if let Some(span) = me.source_span() {
            args.push(self.span_to_svalue(span));
        }

        args.push_keyword_arg(
            FIELD_NAME_IDENTITY,
            self.entity_identity_to_svalue(me.identity()),
            self.options.style,
        );

        if me.has_annotations() {
            args.push(self.annotations_to_svalue(me));
        }

        for member in me.members() {
            args.push(self.member_to_svalue(member));
        }

        SValue::function_call(NODE_KIND_ENTITY_BODY, args)
    }

    fn enum_to_svalue(&self, me: &EnumDef) -> SValue {
        let mut args = Vec::new();

        if let Some(span) = me.source_span() {
            args.push(self.span_to_svalue(span));
        }

        args.push_keyword_arg(
            FIELD_NAME_NAME,
            self.identifier_to_svalue(me.name()),
            self.options.style,
        );

        if let Some(body) = &me.body() {
            args.push_keyword_arg(
                FIELD_NAME_BODY,
                self.enum_body_to_svalue(body),
                self.options.style,
            );
        }

        SValue::function_call(NODE_KIND_ENUM_DEF, args)
    }

    fn enum_body_to_svalue(&self, me: &EnumBody) -> SValue {
        let mut args = Vec::new();

        if let Some(span) = me.source_span() {
            args.push(self.span_to_svalue(span));
        }

        if me.has_annotations() {
            args.push(self.annotations_to_svalue(me));
        }

        for variant in me.variants() {
            args.push(self.value_variant_to_svalue(variant));
        }

        SValue::function_call(NODE_KIND_ENUM_BODY, args)
    }

    fn value_variant_to_svalue(&self, me: &ValueVariant) -> SValue {
        let mut args = Vec::new();

        if let Some(span) = me.source_span() {
            args.push(self.span_to_svalue(span));
        }

        if let Some(body) = &me.body() {
            args.push_keyword_arg(
                FIELD_NAME_BODY,
                self.annotation_only_body_to_svalue(body),
                self.options.style,
            );
        }

        SValue::function_call(NODE_KIND_VALUE_VARIANT, args)
    }

    fn event_to_svalue(&self, me: &EventDef) -> SValue {
        let mut args = Vec::new();

        if let Some(span) = me.source_span() {
            args.push(self.span_to_svalue(span));
        }

        args.push_keyword_arg(
            FIELD_NAME_NAME,
            self.identifier_to_svalue(me.name()),
            self.options.style,
        );

        args.push_keyword_arg(
            FIELD_NAME_SOURCE,
            self.identifier_reference_to_svalue(me.event_source()),
            self.options.style,
        );

        if let Some(body) = &me.body() {
            args.push_keyword_arg(
                FIELD_NAME_BODY,
                self.structure_body_to_svalue(body),
                self.options.style,
            );
        }

        SValue::function_call(NODE_KIND_EVENT_DEF, args)
    }

    fn property_to_svalue(&self, me: &PropertyDef) -> SValue {
        let mut args = Vec::new();

        if let Some(span) = me.source_span() {
            args.push(self.span_to_svalue(span));
        }

        args.push(self.member_def_to_svalue(me.member_def()));

        SValue::function_call(NODE_KIND_PROPERTY_DEF, args)
    }

    fn rdf_to_svalue(&self, me: &RdfDef) -> SValue {
        let mut args = Vec::new();

        if let Some(span) = me.source_span() {
            args.push(self.span_to_svalue(span));
        }

        let body = me.body();
        if body.has_annotations() {
            args.push(self.annotations_to_svalue(body));
        }

        args.push_keyword_arg(
            FIELD_NAME_NAME,
            self.identifier_to_svalue(me.name()),
            self.options.style,
        );

        SValue::function_call(NODE_KIND_RDF_DEF, args)
    }

    fn structure_to_svalue(&self, me: &StructureDef) -> SValue {
        let mut args = Vec::new();

        if let Some(span) = me.source_span() {
            args.push(self.span_to_svalue(span));
        }

        args.push_keyword_arg(
            FIELD_NAME_NAME,
            self.identifier_to_svalue(me.name()),
            self.options.style,
        );

        if let Some(body) = &me.body() {
            args.push_keyword_arg(
                FIELD_NAME_BODY,
                self.structure_body_to_svalue(body),
                self.options.style,
            );
        }

        SValue::function_call(NODE_KIND_STRUCTURE_DEF, args)
    }

    fn structure_body_to_svalue(&self, me: &StructureBody) -> SValue {
        let mut args = Vec::new();

        if let Some(span) = me.source_span() {
            args.push(self.span_to_svalue(span));
        }

        if me.has_annotations() {
            args.push(self.annotations_to_svalue(me));
        }

        for member in me.members() {
            args.push(self.member_to_svalue(member));
        }

        SValue::function_call(NODE_KIND_STRUCTURE_BODY, args)
    }

    fn typeclass_to_svalue(&self, _me: &TypeClassDef) -> SValue {
        todo!()
    }

    fn union_to_svalue(&self, me: &UnionDef) -> SValue {
        let mut args = Vec::new();

        if let Some(span) = me.source_span() {
            args.push(self.span_to_svalue(span));
        }

        args.push_keyword_arg(
            FIELD_NAME_NAME,
            self.identifier_to_svalue(me.name()),
            self.options.style,
        );

        if let Some(body) = &me.body() {
            args.push_keyword_arg(
                FIELD_NAME_BODY,
                self.union_body_to_svalue(body),
                self.options.style,
            );
        }

        SValue::function_call(NODE_KIND_UNION_DEF, args)
    }

    fn union_body_to_svalue(&self, me: &UnionBody) -> SValue {
        let mut args = Vec::new();

        if let Some(span) = me.source_span() {
            args.push(self.span_to_svalue(span));
        }

        if me.has_annotations() {
            args.push(self.annotations_to_svalue(me));
        }

        for variant in me.variants() {
            args.push(self.type_variant_to_svalue(variant));
        }

        SValue::function_call(NODE_KIND_UNION_BODY, args)
    }

    fn type_variant_to_svalue(&self, me: &TypeVariant) -> SValue {
        let mut args = Vec::new();

        if let Some(span) = me.source_span() {
            args.push(self.span_to_svalue(span));
        }

        args.push_keyword_arg(
            FIELD_NAME_NAME,
            self.identifier_reference_to_svalue(me.name_reference()),
            self.options.style,
        );

        if let Some(rename) = &me.rename() {
            args.push_keyword_arg(
                FIELD_NAME_RENAME,
                self.identifier_to_svalue(rename),
                self.options.style,
            );
        }

        if let Some(body) = &me.body() {
            args.push_keyword_arg(
                FIELD_NAME_BODY,
                self.annotation_only_body_to_svalue(body),
                self.options.style,
            );
        }

        SValue::function_call(NODE_KIND_TYPE_VARIANT, args)
    }

    fn entity_identity_to_svalue(&self, me: &Member) -> SValue {
        SValue::function_call(
            NODE_KIND_ENTITY_IDENTITY,
            [
                SValue::keyword(FIELD_NAME_IDENTITY, self.options.style),
                self.member_to_svalue(me),
            ],
        )
    }

    fn member_to_svalue(&self, me: &Member) -> SValue {
        let mut args = Vec::new();

        if let Some(span) = me.source_span() {
            args.push(self.span_to_svalue(span));
        }

        args.push(match me.kind() {
            MemberKind::Reference(me) => self.property_ref_to_svalue(me),
            MemberKind::Definition(me) => self.member_def_to_svalue(me),
        });

        SValue::function_call(NODE_KIND_MEMBER, args)
    }

    fn member_def_to_svalue(&self, me: &MemberDef) -> SValue {
        let mut args = Vec::new();

        if let Some(span) = me.source_span() {
            args.push(self.span_to_svalue(span));
        }

        args.push_keyword_arg(
            FIELD_NAME_NAME,
            self.identifier_to_svalue(me.name()),
            self.options.style,
        );

        args.push_keyword_arg(
            FIELD_NAME_TARGET,
            self.type_reference_to_svalue(me.target_type()),
            self.options.style,
        );

        let target_cardinality = me.target_cardinality();
        if *target_cardinality != DEFAULT_CARDINALITY {
            args.push_keyword_arg(
                FIELD_NAME_CARDINALITY,
                self.cardinality_to_svalue(target_cardinality),
                self.options.style,
            );
        }

        if let Some(body) = &me.body() {
            args.push_keyword_arg(
                FIELD_NAME_BODY,
                self.annotation_only_body_to_svalue(body),
                self.options.style,
            );
        }

        SValue::function_call(NODE_KIND_MEMBER_DEF, args)
    }

    fn property_ref_to_svalue(&self, me: &IdentifierReference) -> SValue {
        let mut args = Vec::new();

        if let Some(span) = me.source_span() {
            args.push(self.span_to_svalue(span));
        }

        args.push(self.identifier_reference_to_svalue(me));

        SValue::function_call(NODE_KIND_PROPERTY_REF, args)
    }

    fn type_reference_to_svalue(&self, me: &TypeReference) -> SValue {
        SValue::function_call(
            NODE_KIND_TYPE_REFERENCE,
            [match me {
                TypeReference::Unknown => SValue::quoted("unknown"),
                TypeReference::Type(type_ref) => self.identifier_reference_to_svalue(&type_ref),
                TypeReference::MappingType(map_type_ref) => {
                    self.mapping_type_to_svalue(map_type_ref)
                }
            }],
        )
    }

    fn mapping_type_to_svalue(&self, me: &MappingType) -> SValue {
        SValue::function_call(
            NODE_KIND_MAPPING_TYPE,
            [
                SValue::keyword(FIELD_NAME_DOMAIN, self.options.style),
                self.type_reference_to_svalue(me.domain()),
                SValue::keyword(FIELD_NAME_RANGE, self.options.style),
                self.type_reference_to_svalue(me.range()),
            ],
        )
    }

    fn cardinality_to_svalue(&self, me: &Cardinality) -> SValue {
        let mut args = Vec::new();

        if let Some(span) = me.source_span() {
            args.push(self.span_to_svalue(span));
        }

        if let Some(ordering) = me.ordering() {
            args.push_keyword_arg(
                FIELD_NAME_ORDERING,
                SValue::quoted(ordering.to_string()),
                self.options.style,
            );
        }

        if let Some(uniqueness) = me.uniqueness() {
            args.push_keyword_arg(
                FIELD_NAME_UNIQUENESS,
                uniqueness.to_string(),
                self.options.style,
            );
        }

        args.push(me.min_occurs().to_string().into());

        if let Some(max) = me.max_occurs() {
            args.push(max.to_string().into());
        }

        SValue::function_call(NODE_KIND_CARDINALITY_EXPRESSION, args)
    }

    fn span_to_svalue(&self, me: &Span) -> SValue {
        SValue::function_call(
            NODE_KIND_SPAN,
            [
                SValue::keyword("start", self.options.style),
                me.start().into(),
                SValue::keyword("end", self.options.style),
                me.end().into(),
            ],
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for SExpressionOptions {
    fn default() -> Self {
        Self {
            line_width: 80,
            pair_kw_args: Default::default(),
            style: Default::default(),
            wrap_in_define: Default::default(),
        }
    }
}

impl SExpressionOptions {
    pub fn with_style(self, style: SExpressionStyle) -> Self {
        Self { style, ..self }
    }

    pub fn with_tree_sitter_style(self) -> Self {
        self.with_style(SExpressionStyle::TreeSitter)
    }

    pub fn with_racket_style(self) -> Self {
        self.with_style(SExpressionStyle::Racket)
    }

    pub fn pair_kw_args(self, pair_kw_args: bool) -> Self {
        Self {
            pair_kw_args,
            ..self
        }
    }

    pub fn line_width(self, line_width: usize) -> Self {
        Self { line_width, ..self }
    }

    pub fn style(&self) -> SExpressionStyle {
        self.style
    }

    pub fn is_tree_sitter_style(&self) -> bool {
        self.style == SExpressionStyle::TreeSitter
    }

    pub fn is_racket_style(&self) -> bool {
        self.style == SExpressionStyle::Racket
    }
}

impl SExpressionStyle {
    pub fn is_tree_sitter(&self) -> bool {
        *self == Self::TreeSitter
    }

    pub fn is_racket(&self) -> bool {
        *self == Self::Racket
    }
}

// ------------------------------------------------------------------------------------------------

impl From<String> for SValue {
    fn from(value: String) -> Self {
        Self::Single(value)
    }
}

impl From<&str> for SValue {
    fn from(value: &str) -> Self {
        Self::Single(value.to_string())
    }
}

impl From<usize> for SValue {
    fn from(value: usize) -> Self {
        Self::Single(value.to_string())
    }
}

impl From<Vec<SValue>> for SValue {
    fn from(value: Vec<SValue>) -> Self {
        Self::List(value)
    }
}

impl From<&[SValue]> for SValue {
    fn from(value: &[SValue]) -> Self {
        Self::List(value.to_vec())
    }
}

impl SValue {
    fn string<S>(s: S) -> Self
    where
        S: Into<String>,
    {
        Self::from(format!("\"{}\"", s.into()))
    }

    fn quoted<S>(s: S) -> Self
    where
        S: Into<String>,
    {
        Self::from(format!("'{}", s.into()))
    }

    fn keyword<S>(s: S, style: SExpressionStyle) -> Self
    where
        S: Into<String>,
    {
        Self::from(format!(
            "{}{}{}",
            match style {
                SExpressionStyle::TreeSitter => "",
                SExpressionStyle::Racket => "#:",
            },
            s.into(),
            match style {
                SExpressionStyle::TreeSitter => ":",
                SExpressionStyle::Racket => "",
            }
        ))
    }

    fn value<S>(s: S) -> Self
    where
        S: Into<String>,
    {
        Self::from(s.into())
    }

    fn function_call<S, V>(s: S, vs: V) -> Self
    where
        S: Into<String>,
        V: Into<Vec<SValue>>,
    {
        Self::Funcall(s.into(), vs.into())
    }

    fn function_call_1<S, V>(s: S, v: V) -> Self
    where
        S: Into<String>,
        V: Into<SValue>,
    {
        Self::Funcall(s.into(), vec![v.into()])
    }

    fn print_len(&self) -> usize {
        match self {
            SValue::Single(v) => v.len(),
            SValue::List(vs) | SValue::Funcall(_, vs) => {
                vs.iter().fold(0, |t, v| t + v.print_len())
                    // add inter-datum spaces
                    + if vs.len() < 2 { 0 } else { vs.len() - 1 }
            }
        }
    }

    fn print<W>(&self, writer: &mut W) -> Result<(), Error>
    where
        W: Write,
    {
        match self {
            SValue::Single(v) => writer.write_all(v.as_bytes())?,
            SValue::List(vs) => {
                writer.write_all(LPAREN)?;
                for (value, is_last) in vs.iter().enumerate().map(|(i, v)| (v, i == vs.len() - 1)) {
                    value.print(writer)?;
                    if !is_last {
                        writer.write_all(SPACE)?;
                    }
                }
                writer.write_all(RPAREN)?;
            }
            SValue::Funcall(v, vs) => {
                writer.write_all(LPAREN)?;
                writer.write_all(v.as_bytes())?;
                if !vs.is_empty() {
                    writer.write_all(SPACE)?;
                    for (value, is_last) in
                        vs.iter().enumerate().map(|(i, v)| (v, i == vs.len() - 1))
                    {
                        value.print(writer)?;
                        if !is_last {
                            writer.write_all(SPACE)?;
                        }
                    }
                }
                writer.write_all(RPAREN)?;
            }
        }

        Ok(())
    }

    fn pretty_print<W>(
        &self,
        line_width: usize,
        style: SExpressionStyle,
        writer: &mut W,
    ) -> Result<(), Error>
    where
        W: Write,
    {
        self.pp_indented(0, line_width, style, writer)?;
        writer.write_all(NEWLN)?;
        Ok(())
    }

    fn pp_indented<W>(
        &self,
        indent: usize,
        line_width: usize,
        style: SExpressionStyle,
        writer: &mut W,
    ) -> Result<(), Error>
    where
        W: Write,
    {
        match self {
            Self::Single(_) => self.print(writer)?,
            Self::List(vs) => self.pp_list_indented(indent, line_width, vs, style, writer)?,
            Self::Funcall(v, vs) => {
                self.pp_fncall_indented(indent, line_width, v, vs, style, writer)?
            }
        }
        Ok(())
    }

    fn pp_fncall_indented<W>(
        &self,
        indent: usize,
        line_width: usize,
        name: &str,
        vs: &[SValue],
        style: SExpressionStyle,
        writer: &mut W,
    ) -> Result<(), Error>
    where
        W: Write,
    {
        let print_width = self.print_len();
        if vs.is_empty() {
            writer.write_all(PARENS)?;
        } else if indent + print_width < line_width {
            self.print(writer)?;
        } else {
            let indent = if indent + name.len() < line_width {
                writer.write_all(LPAREN)?;
                writer.write_all(name.as_bytes())?;
                if indent + name.len() + 1 + vs.first().map(|v| v.print_len()).unwrap_or_default()
                    < line_width
                {
                    writer.write_all(SPACE)?;
                    indent + name.len() + 2 // one '(' and one ' '
                } else {
                    let indent = indent
                        + if style.is_tree_sitter() {
                            1 // one '('
                        } else {
                            2 // one '(' and one ' '
                        };
                    Self::newline_and_indent(indent, writer)?;
                    indent
                }
            } else {
                let indent = indent
                    + if style.is_tree_sitter() {
                        1 // one '('
                    } else {
                        2 // one '(' and one ' '
                    };
                writer.write_all(LPAREN)?;
                writer.write_all(name.as_bytes())?;
                Self::newline_and_indent(indent, writer)?;
                indent
            };
            for (i, v) in vs.iter().enumerate() {
                v.pp_indented(indent, line_width, style, writer)?;
                if i < vs.len() - 1 {
                    Self::newline_and_indent(indent, writer)?;
                }
            }
            writer.write_all(RPAREN)?;
        }
        Ok(())
    }

    #[inline(always)]
    fn newline_and_indent<W>(indent: usize, writer: &mut W) -> Result<(), Error>
    where
        W: Write,
    {
        writer.write_all(format!("\n{:indent$}", " ").as_bytes())?;
        Ok(())
    }

    fn pp_list_indented<W>(
        &self,
        indent: usize,
        line_width: usize,
        vs: &[SValue],
        style: SExpressionStyle,
        writer: &mut W,
    ) -> Result<(), Error>
    where
        W: Write,
    {
        let print_width = self.print_len();
        if vs.is_empty() {
            writer.write_all(PARENS)?;
        } else if indent + print_width < line_width {
            self.print(writer)?;
        } else {
            let indent = indent + 1; // one '('
            writer.write_all(LPAREN)?;
            for (i, v) in vs.iter().enumerate() {
                v.pp_indented(indent, line_width, style, writer)?;
                if i < vs.len() - 1 {
                    Self::newline_and_indent(indent, writer)?;
                }
            }
            writer.write_all(RPAREN)?;
        }
        Ok(())
    }
}

impl SList for Vec<SValue> {
    fn push_keyword<K>(&mut self, kw: K, style: SExpressionStyle)
    where
        K: Into<String>,
    {
        self.push(SValue::keyword(kw, style));
    }

    fn push_keyword_arg<K, V>(&mut self, kw: K, value: V, style: SExpressionStyle)
    where
        K: Into<String>,
        V: Into<SValue>,
    {
        self.push_keyword(kw, style);
        self.push(value.into());
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    fn assert_svalue_string(value: SValue, expected: &str) {
        let mut buffer = Cursor::new(Vec::new());
        value
            .pretty_print(10, SExpressionStyle::TreeSitter, &mut buffer)
            .unwrap();
        assert_eq!(String::from_utf8(buffer.into_inner()).unwrap(), expected);
    }

    #[test]
    fn test_svalue_single() {
        assert_svalue_string(SValue::string("hello"), "\"hello\"\n");
        assert_svalue_string(SValue::quoted("hello"), "'hello\n");
        assert_svalue_string(
            SValue::keyword("hello", SExpressionStyle::TreeSitter),
            "hello:\n",
        );
        assert_svalue_string(
            SValue::keyword("hello", SExpressionStyle::Racket),
            "#:hello\n",
        );
    }

    #[test]
    fn test_svalue_list() {
        let value = SValue::List(vec![
            SValue::string("hello"),
            SValue::value("hello"),
            SValue::quoted("hello"),
            SValue::keyword("hello", SExpressionStyle::TreeSitter),
            SValue::keyword("hello", SExpressionStyle::Racket),
        ]);
        assert_svalue_string(
            value,
            r#"("hello"
 hello
 'hello
 hello:
 #:hello)
"#,
        );
    }

    #[test]
    fn test_svalue_funcall() {
        let value = SValue::function_call(
            "write",
            vec![
                SValue::quoted("hello"),
                SValue::keyword("width", SExpressionStyle::Racket),
                SValue::value("20"),
                SValue::keyword("align", SExpressionStyle::Racket),
                SValue::quoted("right"),
            ],
        );
        assert_svalue_string(
            value,
            r#"(write
 'hello
 #:width
 20
 #:align
 'right)
"#,
        );
    }

    #[test]
    fn test_svalue_nested_funcall() {
        let value = SValue::function_call(
            "write",
            vec![
                SValue::function_call_1(
                    "string-append",
                    SValue::List(vec![
                        SValue::string("hello"),
                        SValue::string(" "),
                        SValue::string("world"),
                        SValue::string("!"),
                    ]),
                ),
                SValue::keyword("width", SExpressionStyle::Racket),
                SValue::value("20"),
                SValue::keyword("align", SExpressionStyle::Racket),
                SValue::quoted("right"),
            ],
        );
        assert_svalue_string(
            value,
            r#"(write
 (string-append
  ("hello"
   " "
   "world"
   "!"))
 #:width
 20
 #:align
 'right)
"#,
        );
    }
}
