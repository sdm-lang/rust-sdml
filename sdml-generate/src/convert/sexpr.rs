/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use sdml_core::error::Error;
use sdml_core::model::annotations::{
    Annotation, AnnotationOnlyBody, AnnotationProperty, HasAnnotations,
};
use sdml_core::model::constraints::{
    AtomicSentence, BooleanSentence, ConnectiveOperator, Constraint, ConstraintBody,
    ConstraintSentence, EnvironmentDef, EnvironmentDefBody, Equation, FormalConstraint,
    FunctionComposition, FunctionDef, FunctionParameter, FunctionSignature, FunctionType,
    FunctionTypeReference, FunctionalTerm, InequalityRelation, Inequation, IteratorSource,
    PredicateSequenceMember, PredicateValue, QuantifiedBinding, QuantifiedSentence,
    QuantifiedVariableBinding, Quantifier, QuantifierBoundNames, SequenceBuilder, SequenceIterator,
    SequenceOfPredicateValues, SimpleSentence, Subject, Term, TypeIterator, Variables,
};
use sdml_core::model::definitions::{
    DatatypeDef, Definition, EntityBody, EntityDef, EntityGroup, EntityMember, EnumBody, EnumDef,
    EventDef, HasGroups, HasMembers, HasVariants, PropertyBody, PropertyDef, PropertyRoleDef,
    StructureBody, StructureDef, StructureGroup, TypeVariant, UnionBody, UnionDef, ValueVariant,
};
use sdml_core::model::identifiers::{Identifier, IdentifierReference, QualifiedIdentifier};
use sdml_core::model::members::{
    ByReferenceMember, ByReferenceMemberDef, ByValueMember, ByValueMemberDef, Cardinality,
    HasCardinality, HasType, IdentityMember, IdentityMemberDef, MappingType, Member, MemberKind,
    TypeReference, DEFAULT_BY_REFERENCE_CARDINALITY, DEFAULT_BY_VALUE_CARDINALITY,
};
use sdml_core::model::modules::{Import, ImportStatement, Module, ModuleBody};
use sdml_core::model::values::{
    LanguageTag, MappingValue, SequenceMember, SequenceOfValues, SimpleValue, Value,
    ValueConstructor,
};
use sdml_core::model::{HasBody, HasName, HasNameReference, HasOptionalBody, HasSourceSpan, Span};
use sdml_core::syntax::{
    FIELD_NAME_ARGUMENT, FIELD_NAME_BASE, FIELD_NAME_BINDING, FIELD_NAME_BODY,
    FIELD_NAME_CARDINALITY, FIELD_NAME_DOMAIN, FIELD_NAME_FUNCTION, FIELD_NAME_IDENTITY,
    FIELD_NAME_INVERSE_NAME, FIELD_NAME_LANGUAGE, FIELD_NAME_LHS, FIELD_NAME_MAX,
    FIELD_NAME_MEMBER, FIELD_NAME_MIN, FIELD_NAME_MODULE, FIELD_NAME_NAME, FIELD_NAME_OPERATOR,
    FIELD_NAME_ORDERING, FIELD_NAME_PARAMETER, FIELD_NAME_PREDICATE, FIELD_NAME_PROPERTY,
    FIELD_NAME_QUANTIFIER, FIELD_NAME_RANGE, FIELD_NAME_RELATION, FIELD_NAME_RENAME,
    FIELD_NAME_RHS, FIELD_NAME_SIGNATURE, FIELD_NAME_SOURCE, FIELD_NAME_SUBJECT, FIELD_NAME_TARGET,
    FIELD_NAME_UNIQUENESS, FIELD_NAME_VALUE, FIELD_NAME_VARIABLE, NODE_KIND_ANNOTATION,
    NODE_KIND_ANNOTATION_ONLY_BODY, NODE_KIND_ATOMIC_SENTENCE, NODE_KIND_BICONDITIONAL,
    NODE_KIND_BOOLEAN, NODE_KIND_BOOLEAN_SENTENCE, NODE_KIND_CARDINALITY_EXPRESSION,
    NODE_KIND_CONJUNCTION, NODE_KIND_CONSTRAINT, NODE_KIND_CONSTRAINT_ENVIRONMENT,
    NODE_KIND_CONSTRAINT_SENTENCE, NODE_KIND_CONTROLLED_LANGUAGE_TAG, NODE_KIND_DATA_TYPE_DEF,
    NODE_KIND_DECIMAL, NODE_KIND_DISJUNCTION, NODE_KIND_DOUBLE, NODE_KIND_ENTITY_BODY,
    NODE_KIND_ENTITY_DEF, NODE_KIND_ENTITY_GROUP, NODE_KIND_ENUM_BODY, NODE_KIND_ENUM_DEF,
    NODE_KIND_ENVIRONMENT_DEFINITION, NODE_KIND_EQUATION, NODE_KIND_EVENT_DEF,
    NODE_KIND_EXCLUSIVE_DISJUNCTION, NODE_KIND_EXISTENTIAL, NODE_KIND_FORMAL_CONSTRAINT,
    NODE_KIND_FUNCTIONAL_TERM, NODE_KIND_FUNCTION_COMPOSITION, NODE_KIND_FUNCTION_DEF,
    NODE_KIND_FUNCTION_PARAMETER, NODE_KIND_FUNCTION_SIGNATURE, NODE_KIND_GREATER_THAN,
    NODE_KIND_GREATER_THAN_OR_EQUAL, NODE_KIND_IDENTIFIER, NODE_KIND_IDENTIFIER_REFERENCE,
    NODE_KIND_IDENTITY_MEMBER, NODE_KIND_IDENTITY_ROLE, NODE_KIND_IMPLICATION, NODE_KIND_IMPORT,
    NODE_KIND_INEQUATION, NODE_KIND_INFORMAL_CONSTRAINT, NODE_KIND_INTEGER,
    NODE_KIND_IRI_REFERENCE, NODE_KIND_LANGUAGE_TAG, NODE_KIND_LESS_THAN,
    NODE_KIND_LESS_THAN_OR_EQUAL, NODE_KIND_MAPPING_TYPE, NODE_KIND_MAPPING_VARIABLE,
    NODE_KIND_MEMBER_BY_REFERENCE, NODE_KIND_MEMBER_BY_VALUE, NODE_KIND_MEMBER_IMPORT,
    NODE_KIND_MODULE, NODE_KIND_MODULE_BODY, NODE_KIND_MODULE_IMPORT, NODE_KIND_NAMED_VARIABLE_SET,
    NODE_KIND_NEGATION, NODE_KIND_NOT_EQUAL, NODE_KIND_PREDICATE_VALUE, NODE_KIND_PROPERTY_BODY,
    NODE_KIND_QUALIFIED_IDENTIFIER, NODE_KIND_QUANTIFIED_SENTENCE,
    NODE_KIND_QUANTIFIED_VARIABLE_BINDING, NODE_KIND_QUANTIFIER_BOUND_NAMES,
    NODE_KIND_QUOTED_STRING, NODE_KIND_RESERVED_SELF, NODE_KIND_RESERVED_SELF_TYPE,
    NODE_KIND_ROLE_BY_REFERENCE, NODE_KIND_ROLE_BY_VALUE, NODE_KIND_SEQUENCE_BUILDER,
    NODE_KIND_SEQUENCE_ITERATOR, NODE_KIND_SEQUENCE_OF_PREDICATE_VALUES,
    NODE_KIND_SEQUENCE_OF_VALUES, NODE_KIND_SIMPLE_SENTENCE, NODE_KIND_STRING,
    NODE_KIND_STRUCTURE_BODY, NODE_KIND_STRUCTURE_DEF, NODE_KIND_STRUCTURE_GROUP, NODE_KIND_TERM,
    NODE_KIND_TYPE_ITERATOR, NODE_KIND_TYPE_VARIANT, NODE_KIND_UNION_BODY, NODE_KIND_UNION_DEF,
    NODE_KIND_UNIVERSAL, NODE_KIND_UNKNOWN_TYPE, NODE_KIND_VALUE_CONSTRUCTOR,
    NODE_KIND_VALUE_VARIANT, NODE_KIND_WILDCARD,
};
use std::fmt::Display;
use std::io::Write;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn write_as_sexpr<W: Write>(module: &Module, w: &mut W) -> Result<(), Error> {
    let mut writer = Writer::new(w);
    write_module(module, &mut writer)
}

write_to_string!(to_sexpr_string, write_as_sexpr);

write_to_file!(to_sexpr_file, write_as_sexpr);

print_to_stdout!(print_sexpr, write_as_sexpr);

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

macro_rules! write_annotations {
    ($iterator: expr, $w: expr) => {
        for annotation in $iterator {
            $w.newln()?;
            match &annotation {
                Annotation::Property(v) => write_annotation_property(v, $w)?,
                Annotation::Constraint(v) => write_constraint(v, $w)?,
            }
        }
    };
}

macro_rules! write_span {
    ($me: expr, $w: expr) => {
        if let Some(span) = $me.source_span() {
            $w.newln_and_indentation()?;
            write_span(span, $w)?;
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

struct Writer<W>
where
    W: Write,
{
    indent: String,
    indentation: String,
    w: W,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl<W> Writer<W>
where
    W: Write,
{
    fn new(w: W) -> Self {
        Self::new_with_indent(w, "  ")
    }

    fn new_with_indent<S: Into<String>>(w: W, indent: S) -> Self {
        Self {
            indent: indent.into(),
            indentation: String::new(),
            w,
        }
    }

    fn value_with_prefix<V: Display, S: AsRef<str>>(
        &mut self,
        value: V,
        prefix: S,
    ) -> Result<(), Error> {
        self.w
            .write_all(format!("{}{}", prefix.as_ref(), value).as_bytes())?;
        Ok(())
    }

    fn node<S: AsRef<str>>(&mut self, name: S) -> Result<(), Error> {
        self.w
            .write_all(format!("({})", name.as_ref()).as_bytes())?;
        Ok(())
    }

    fn node_and_value<S: AsRef<str>, V: Display>(
        &mut self,
        name: S,
        value: V,
    ) -> Result<(), Error> {
        self.w
            .write_all(format!("({} {})", name.as_ref(), value).as_bytes())?;
        Ok(())
    }

    fn start_node<S: AsRef<str>>(&mut self, name: S) -> Result<(), Error> {
        self.w
            .write_all(format!("({} ", name.as_ref()).as_bytes())?;
        Ok(())
    }

    fn start_node_and_newln<S: AsRef<str>>(&mut self, name: S) -> Result<(), Error> {
        self.w
            .write_all(format!("({}\n", name.as_ref()).as_bytes())?;
        Ok(())
    }

    fn start_node_indented<S: AsRef<str>>(&mut self, name: S) -> Result<(), Error> {
        self.w
            .write_all(format!("{}({} ", self.indentation, name.as_ref()).as_bytes())?;
        Ok(())
    }

    fn field_name<S: AsRef<str>>(&mut self, name: S) -> Result<(), Error> {
        self.w
            .write_all(format!("{}: ", name.as_ref()).as_bytes())?;
        Ok(())
    }

    fn field<S: AsRef<str>, V: Display>(&mut self, name: S, value: V) -> Result<(), Error> {
        self.w
            .write_all(format!("{}: {}", name.as_ref(), value).as_bytes())?;
        Ok(())
    }

    fn field_name_indented<S: AsRef<str>>(&mut self, name: S) -> Result<(), Error> {
        self.w
            .write_all(format!("{}{}: ", self.indentation, name.as_ref()).as_bytes())?;
        Ok(())
    }

    fn close_paren(&mut self) -> Result<(), Error> {
        self.w.write_all(")".as_bytes())?;
        Ok(())
    }

    fn close_paren_and_newln(&mut self) -> Result<(), Error> {
        self.w.write_all(")\n".as_bytes())?;
        Ok(())
    }

    fn space(&mut self) -> Result<(), Error> {
        self.w.write_all(" ".as_bytes())?;
        Ok(())
    }

    fn newln(&mut self) -> Result<(), Error> {
        self.w.write_all("\n".as_bytes())?;
        Ok(())
    }

    fn newln_and_indentation(&mut self) -> Result<(), Error> {
        self.w
            .write_all(format!("\n{}", self.indentation).as_bytes())?;
        Ok(())
    }

    fn indentation(&mut self) -> Result<(), Error> {
        self.w.write_all(self.indentation.as_bytes())?;
        Ok(())
    }

    fn indent(&mut self) {
        self.indentation.push_str(&self.indent)
    }

    fn outdent(&mut self) {
        self.indentation = self.indentation.as_str()[self.indent.len()..].to_string();
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn write_identifier<W: Write>(me: &Identifier, w: &mut Writer<W>) -> Result<(), Error> {
    w.start_node(NODE_KIND_IDENTIFIER)?;
    maybe_write_span(me.source_span(), w)?;
    w.value_with_prefix(me, "'")?;

    w.close_paren()?;
    Ok(())
}

fn write_qualified_identifier<W: Write>(
    me: &QualifiedIdentifier,
    w: &mut Writer<W>,
) -> Result<(), Error> {
    w.start_node(NODE_KIND_QUALIFIED_IDENTIFIER)?;
    w.indent();

    write_span!(me, w);

    w.newln()?;
    w.field_name_indented(FIELD_NAME_MODULE)?;
    w.value_with_prefix(me.module(), "'")?;

    w.newln()?;
    w.field_name_indented(FIELD_NAME_MEMBER)?;
    w.value_with_prefix(me.member(), "'")?;

    w.close_paren()?;
    w.outdent();
    Ok(())
}

fn write_identifier_reference<W: Write>(
    me: &IdentifierReference,
    w: &mut Writer<W>,
) -> Result<(), Error> {
    w.start_node_and_newln(NODE_KIND_IDENTIFIER_REFERENCE)?;
    w.indent();

    w.indentation()?;

    match me {
        IdentifierReference::Identifier(v) => write_identifier(v, w)?,
        IdentifierReference::QualifiedIdentifier(v) => write_qualified_identifier(v, w)?,
    }

    w.close_paren()?;
    w.outdent();
    Ok(())
}

fn write_module<W: Write>(me: &Module, w: &mut Writer<W>) -> Result<(), Error> {
    w.start_node_indented(NODE_KIND_MODULE)?;
    w.indent();

    write_span!(me, w);

    w.newln_and_indentation()?;
    w.field(FIELD_NAME_NAME, me.name())?;

    w.newln_and_indentation()?;
    w.field_name(FIELD_NAME_BODY)?;
    write_module_body(me.body(), w)?;

    w.close_paren_and_newln()?;
    w.outdent();
    Ok(())
}

fn write_module_body<W: Write>(me: &ModuleBody, w: &mut Writer<W>) -> Result<(), Error> {
    w.start_node(NODE_KIND_MODULE_BODY)?;
    w.indent();

    write_span!(me, w);

    for import in me.imports() {
        w.newln()?;
        write_import_statement(import, w)?;
    }

    write_annotations!(me.annotations(), w);

    for definition in me.definitions() {
        w.newln()?;
        write_type_definition(definition, w)?;
    }

    w.close_paren()?;
    w.outdent();
    Ok(())
}

fn write_import_statement<W: Write>(me: &ImportStatement, w: &mut Writer<W>) -> Result<(), Error> {
    w.start_node_indented(NODE_KIND_IMPORT)?;
    w.indent();

    write_span!(me, w);

    for import in me.imports() {
        w.newln()?;
        write_import(import, w)?;
    }

    w.close_paren()?;
    w.outdent();
    Ok(())
}

fn write_import<W: Write>(me: &Import, w: &mut Writer<W>) -> Result<(), Error> {
    match me {
        Import::Module(v) => {
            w.start_node_indented(NODE_KIND_MODULE_IMPORT)?;
            w.indent();

            write_span!(v, w);

            w.newln_and_indentation()?;
            w.field_name(FIELD_NAME_NAME)?;
            write_identifier(v, w)?;
        }
        Import::Member(v) => {
            w.start_node_indented(NODE_KIND_MEMBER_IMPORT)?;
            w.indent();

            write_span!(v, w);

            w.newln_and_indentation()?;
            w.field_name(FIELD_NAME_NAME)?;
            write_qualified_identifier(v, w)?;
        }
    };

    w.close_paren()?;
    w.outdent();
    Ok(())
}

fn write_annotation_property<W: Write>(
    me: &AnnotationProperty,
    w: &mut Writer<W>,
) -> Result<(), Error> {
    w.start_node_indented(NODE_KIND_ANNOTATION)?;
    w.indent();

    write_span!(me, w);

    w.newln_and_indentation()?;
    w.field_name(FIELD_NAME_NAME)?;
    write_identifier_reference(me.name_reference(), w)?;

    w.newln_and_indentation()?;
    w.field_name(FIELD_NAME_VALUE)?;
    write_value(me.value(), w)?;

    w.close_paren()?;
    w.outdent();
    Ok(())
}

fn write_constraint<W: Write>(me: &Constraint, w: &mut Writer<W>) -> Result<(), Error> {
    w.start_node_indented(NODE_KIND_CONSTRAINT)?;
    w.indent();

    write_span!(me, w);

    w.newln_and_indentation()?;
    w.field_name(FIELD_NAME_NAME)?;
    write_identifier(me.name(), w)?;

    w.newln_and_indentation()?;
    w.field_name(FIELD_NAME_BODY)?;
    match me.body() {
        ConstraintBody::Informal(v) => {
            w.start_node(NODE_KIND_INFORMAL_CONSTRAINT)?;
            w.indent();

            w.newln_and_indentation()?;
            w.node_and_value(NODE_KIND_QUOTED_STRING, format!("{:?}", v.value()))?;

            if let Some(language) = v.language() {
                w.newln_and_indentation()?;
                w.field_name(FIELD_NAME_LANGUAGE)?;
                w.start_node(NODE_KIND_CONTROLLED_LANGUAGE_TAG)?;
                w.value_with_prefix(language, "'")?;
                w.close_paren()?;
            }

            w.outdent();
            w.close_paren()?
        }
        ConstraintBody::Formal(v) => {
            write_formal_constraint(v, w)?;
        }
    }

    w.close_paren()?;
    w.outdent();
    Ok(())
}

fn write_formal_constraint<W: Write>(
    me: &FormalConstraint,
    w: &mut Writer<W>,
) -> Result<(), Error> {
    w.start_node(NODE_KIND_FORMAL_CONSTRAINT)?;
    w.indent();

    write_span!(me, w);

    if me.has_definitions() {
        w.newln_and_indentation()?;
        w.start_node(NODE_KIND_CONSTRAINT_ENVIRONMENT)?;
        w.indent();

        for defn in me.definitions() {
            w.newln()?;
            write_environment_definition(defn, w)?;
        }

        w.outdent();
        w.close_paren()?
    }

    w.newln_and_indentation()?;
    w.field_name(FIELD_NAME_BODY)?;
    write_constraint_sentence(me.body(), w)?;

    w.close_paren()?;
    w.outdent();
    Ok(())
}

fn write_environment_definition<W: Write>(
    me: &EnvironmentDef,
    w: &mut Writer<W>,
) -> Result<(), Error> {
    w.start_node_indented(NODE_KIND_ENVIRONMENT_DEFINITION)?;
    w.indent();

    write_span!(me, w);

    w.newln_and_indentation()?;
    w.field_name(FIELD_NAME_NAME)?;
    write_identifier(me.name(), w)?;

    w.newln_and_indentation()?;
    w.field_name(FIELD_NAME_BODY)?;

    match me.body() {
        EnvironmentDefBody::Function(v) => write_function_def(v, w)?,
        EnvironmentDefBody::Value(v) => write_predicate_value(v, w)?,
        EnvironmentDefBody::Sentence(v) => write_constraint_sentence(v, w)?,
    }

    w.close_paren()?;
    w.outdent();
    Ok(())
}

fn write_function_def<W: Write>(me: &FunctionDef, w: &mut Writer<W>) -> Result<(), Error> {
    w.start_node(NODE_KIND_FUNCTION_DEF)?;
    w.indent();

    write_span!(me, w);

    w.newln_and_indentation()?;
    w.field_name(FIELD_NAME_SIGNATURE)?;
    write_function_signature(me.signature(), w)?;

    w.newln_and_indentation()?;
    w.field_name(FIELD_NAME_BODY)?;
    write_constraint_sentence(me.body(), w)?;

    w.close_paren()?;
    w.outdent();
    Ok(())
}

fn write_function_signature<W: Write>(
    me: &FunctionSignature,
    w: &mut Writer<W>,
) -> Result<(), Error> {
    w.start_node(NODE_KIND_FUNCTION_SIGNATURE)?;
    w.indent();

    write_span!(me, w);

    for parameter in me.parameters() {
        w.newln_and_indentation()?;
        w.field_name(FIELD_NAME_PARAMETER)?;
        write_function_parameter(parameter, w)?;
    }

    write_function_type(me.target_type(), w)?;

    w.close_paren()?;
    w.outdent();
    Ok(())
}

fn write_function_parameter<W: Write>(
    me: &FunctionParameter,
    w: &mut Writer<W>,
) -> Result<(), Error> {
    w.start_node(NODE_KIND_FUNCTION_PARAMETER)?;
    w.indent();

    write_span!(me, w);

    w.newln_and_indentation()?;
    w.field_name(FIELD_NAME_NAME)?;
    write_identifier(me.name(), w)?;

    write_function_type(me.target_type(), w)?;

    w.close_paren()?;
    w.outdent();
    Ok(())
}

fn write_function_type<W: Write>(me: &FunctionType, w: &mut Writer<W>) -> Result<(), Error> {
    // TODO: cardinality

    w.newln_and_indentation()?;
    w.field_name(FIELD_NAME_NAME)?;
    match me.target_type() {
        FunctionTypeReference::Wildcard => w.node(NODE_KIND_WILDCARD)?,
        FunctionTypeReference::Reference(v) => write_identifier_reference(v, w)?,
        FunctionTypeReference::MappingType(v) => write_mapping_type(v, w)?,
    }

    Ok(())
}

fn write_mapping_type<W: Write>(me: &MappingType, w: &mut Writer<W>) -> Result<(), Error> {
    w.start_node(NODE_KIND_MAPPING_TYPE)?;
    w.indent();

    write_span!(me, w);

    w.newln_and_indentation()?;
    w.field_name(FIELD_NAME_DOMAIN)?;
    write_type_reference(me.domain(), w)?;

    w.newln_and_indentation()?;
    w.field_name(FIELD_NAME_RANGE)?;
    write_type_reference(me.range(), w)?;

    w.close_paren()?;
    w.outdent();
    Ok(())
}

fn write_constraint_sentence<W: Write>(
    me: &ConstraintSentence,
    w: &mut Writer<W>,
) -> Result<(), Error> {
    w.start_node(NODE_KIND_CONSTRAINT_SENTENCE)?;
    w.indent();

    match me {
        ConstraintSentence::Simple(v) => {
            w.newln_and_indentation()?;
            w.start_node(NODE_KIND_SIMPLE_SENTENCE)?;
            w.indent();
            match v {
                SimpleSentence::Atomic(v) => write_atomic_sentence(v, w)?,
                SimpleSentence::Equation(v) => write_equation(v, w)?,
                SimpleSentence::Inequation(v) => write_inequation(v, w)?,
            }
            w.close_paren()?;
            w.outdent();
        }
        ConstraintSentence::Boolean(v) => write_boolean_sentence(v, w)?,
        ConstraintSentence::Quantified(v) => write_quantified_sentence(v, w)?,
    }

    w.close_paren()?;
    w.outdent();
    Ok(())
}

fn write_atomic_sentence<W: Write>(me: &AtomicSentence, w: &mut Writer<W>) -> Result<(), Error> {
    w.newln_and_indentation()?;
    w.start_node(NODE_KIND_ATOMIC_SENTENCE)?;
    w.indent();

    write_span!(me, w);

    w.newln_and_indentation()?;
    w.field_name(FIELD_NAME_PREDICATE)?;
    write_term(me.predicate(), w)?;

    for argument in me.arguments() {
        w.newln_and_indentation()?;
        w.field_name(FIELD_NAME_ARGUMENT)?;
        write_term(argument, w)?;
    }

    w.close_paren()?;
    w.outdent();
    Ok(())
}

fn write_equation<W: Write>(me: &Equation, w: &mut Writer<W>) -> Result<(), Error> {
    w.newln_and_indentation()?;
    w.start_node(NODE_KIND_EQUATION)?;
    w.indent();

    write_span!(me, w);

    w.newln_and_indentation()?;
    w.field_name(FIELD_NAME_LHS)?;
    write_term(me.left_operand(), w)?;

    w.newln_and_indentation()?;
    w.field_name(FIELD_NAME_RHS)?;
    write_term(me.right_operand(), w)?;

    w.close_paren()?;
    w.outdent();
    Ok(())
}

fn write_inequation<W: Write>(me: &Inequation, w: &mut Writer<W>) -> Result<(), Error> {
    w.newln_and_indentation()?;
    w.start_node(NODE_KIND_INEQUATION)?;
    w.indent();

    write_span!(me, w);

    w.newln_and_indentation()?;
    w.field_name(FIELD_NAME_LHS)?;
    write_term(me.left_operand(), w)?;

    w.newln_and_indentation()?;
    w.field_name(FIELD_NAME_RELATION)?;
    match me.relation() {
        InequalityRelation::NotEqual => w.node(NODE_KIND_NOT_EQUAL)?,
        InequalityRelation::LessThan => w.node(NODE_KIND_LESS_THAN)?,
        InequalityRelation::LessThanOrEqual => w.node(NODE_KIND_LESS_THAN_OR_EQUAL)?,
        InequalityRelation::GreaterThan => w.node(NODE_KIND_GREATER_THAN)?,
        InequalityRelation::GreaterThanOrEqual => w.node(NODE_KIND_GREATER_THAN_OR_EQUAL)?,
    }

    w.newln_and_indentation()?;
    w.field_name(FIELD_NAME_RHS)?;
    write_term(me.right_operand(), w)?;

    w.close_paren()?;
    w.outdent();
    Ok(())
}

fn write_boolean_sentence<W: Write>(me: &BooleanSentence, w: &mut Writer<W>) -> Result<(), Error> {
    w.newln_and_indentation()?;
    w.start_node(NODE_KIND_BOOLEAN_SENTENCE)?;
    w.indent();

    match me {
        BooleanSentence::Unary(v) => {
            w.newln_and_indentation()?;
            w.field_name(FIELD_NAME_OPERATOR)?;
            w.node(NODE_KIND_NEGATION)?;

            w.newln_and_indentation()?;
            w.field_name(FIELD_NAME_RHS)?;
            write_constraint_sentence(v.operand(), w)?;
        }
        BooleanSentence::Binary(v) => {
            w.newln_and_indentation()?;
            w.field_name(FIELD_NAME_LHS)?;
            write_constraint_sentence(v.left_operand(), w)?;

            w.newln_and_indentation()?;
            w.field_name(FIELD_NAME_OPERATOR)?;
            match v.operator() {
                ConnectiveOperator::Negation => unreachable!(),
                ConnectiveOperator::Conjunction => w.node(NODE_KIND_CONJUNCTION)?,
                ConnectiveOperator::Disjunction => w.node(NODE_KIND_DISJUNCTION)?,
                ConnectiveOperator::ExclusiveDisjunction => {
                    w.node(NODE_KIND_EXCLUSIVE_DISJUNCTION)?
                }
                ConnectiveOperator::Implication => w.node(NODE_KIND_IMPLICATION)?,
                ConnectiveOperator::Biconditional => w.node(NODE_KIND_BICONDITIONAL)?,
            }

            w.newln_and_indentation()?;
            w.field_name(FIELD_NAME_RHS)?;
            write_constraint_sentence(v.right_operand(), w)?;
        }
    }

    w.close_paren()?;
    w.outdent();
    Ok(())
}

fn write_quantified_sentence<W: Write>(
    me: &QuantifiedSentence,
    w: &mut Writer<W>,
) -> Result<(), Error> {
    w.newln_and_indentation()?;
    w.start_node(NODE_KIND_QUANTIFIED_SENTENCE)?;
    w.indent();

    write_span!(me, w);

    for binding in me.variable_bindings() {
        w.newln_and_indentation()?;
        w.field_name(FIELD_NAME_BINDING)?;
        write_quantified_variable_binding(binding, w)?;
    }

    w.newln_and_indentation()?;
    w.field_name(FIELD_NAME_BODY)?;
    write_constraint_sentence(me.body(), w)?;

    w.close_paren()?;
    w.outdent();
    Ok(())
}

fn write_quantified_variable_binding<W: Write>(
    me: &QuantifiedVariableBinding,
    w: &mut Writer<W>,
) -> Result<(), Error> {
    w.start_node(NODE_KIND_QUANTIFIED_VARIABLE_BINDING)?;
    w.indent();

    write_span!(me, w);

    w.newln_and_indentation()?;
    w.field_name(FIELD_NAME_QUANTIFIER)?;
    match me.quantifier() {
        Quantifier::Universal => w.node(NODE_KIND_UNIVERSAL)?,
        Quantifier::Existential => w.node(NODE_KIND_EXISTENTIAL)?,
    }

    for binding in me.bindings() {
        w.newln_and_indentation()?;
        w.field_name(FIELD_NAME_BINDING)?;
        match binding {
            QuantifiedBinding::ReservedSelf => w.node(NODE_KIND_RESERVED_SELF)?,
            QuantifiedBinding::Named(v) => write_quantifier_bound_names(v, w)?,
        }
    }

    w.close_paren()?;
    w.outdent();
    Ok(())
}

fn write_quantifier_bound_names<W: Write>(
    me: &QuantifierBoundNames,
    w: &mut Writer<W>,
) -> Result<(), Error> {
    w.start_node(NODE_KIND_QUANTIFIER_BOUND_NAMES)?;
    w.indent();

    write_span!(me, w);

    for name in me.names() {
        w.newln_and_indentation()?;
        w.field_name(FIELD_NAME_NAME)?;
        write_identifier(name, w)?;
    }

    w.newln_and_indentation()?;
    w.field_name(FIELD_NAME_SOURCE)?;
    match me.source() {
        IteratorSource::Type(v) => write_type_iterator(v, w)?,
        IteratorSource::Sequence(v) => write_sequence_iterator(v, w)?,
    }

    w.close_paren()?;
    w.outdent();
    Ok(())
}

fn write_type_iterator<W: Write>(me: &TypeIterator, w: &mut Writer<W>) -> Result<(), Error> {
    w.start_node(NODE_KIND_TYPE_ITERATOR)?;
    w.indent();

    match me {
        TypeIterator::SelfType => w.node(NODE_KIND_RESERVED_SELF_TYPE)?,
        TypeIterator::Type(v) => {
            w.newln_and_indentation()?;
            write_identifier_reference(v, w)?
        }
    }

    w.close_paren()?;
    w.outdent();
    Ok(())
}

fn write_sequence_iterator<W: Write>(
    me: &SequenceIterator,
    w: &mut Writer<W>,
) -> Result<(), Error> {
    w.start_node(NODE_KIND_SEQUENCE_ITERATOR)?;
    w.indent();

    match me {
        SequenceIterator::Call(v) => write_function_composition(v, w)?,
        SequenceIterator::Variable(v) => {
            w.newln_and_indentation()?;
            write_identifier(v, w)?
        }
        SequenceIterator::Builder(v) => write_sequence_builder(v, w)?,
    }

    w.close_paren()?;
    w.outdent();
    Ok(())
}

fn write_predicate_value<W: Write>(me: &PredicateValue, w: &mut Writer<W>) -> Result<(), Error> {
    w.start_node(NODE_KIND_PREDICATE_VALUE)?;
    w.indent();

    w.newln_and_indentation()?;
    match me {
        PredicateValue::Simple(v) => write_simple_value(v, w)?,
        PredicateValue::List(vs) => write_list_of_predicate_values(vs, w)?,
    }

    w.close_paren()?;
    w.outdent();
    Ok(())
}

fn write_list_of_predicate_values<W: Write>(
    me: &SequenceOfPredicateValues,
    w: &mut Writer<W>,
) -> Result<(), Error> {
    w.start_node(NODE_KIND_SEQUENCE_OF_PREDICATE_VALUES)?;
    w.indent();

    write_span!(me, w);

    for value in me.iter() {
        w.newln_and_indentation()?;
        match value {
            PredicateSequenceMember::Simple(v) => write_simple_value(v, w)?,
            PredicateSequenceMember::Reference(v) => write_identifier_reference(v, w)?,
        }
    }

    w.close_paren()?;
    w.outdent();
    Ok(())
}

fn write_term<W: Write>(me: &Term, w: &mut Writer<W>) -> Result<(), Error> {
    w.start_node(NODE_KIND_TERM)?;
    w.indent();

    w.newln_and_indentation()?;
    match me {
        Term::Call(v) => write_function_composition(v, w)?,
        Term::Variable(v) => write_identifier(v, w)?,
        Term::Type(v) => write_qualified_identifier(v, w)?,
        Term::Value(v) => write_predicate_value(v, w)?,
        Term::Function(v) => write_functional_term(v, w)?,
        Term::Sequence(v) => write_sequence_builder(v, w)?,
    }

    w.close_paren()?;
    w.outdent();
    Ok(())
}

fn write_function_composition<W: Write>(
    me: &FunctionComposition,
    w: &mut Writer<W>,
) -> Result<(), Error> {
    w.newln_and_indentation()?;
    w.start_node(NODE_KIND_FUNCTION_COMPOSITION)?;
    w.indent();

    write_span!(me, w);

    w.newln_and_indentation()?;
    w.field_name(FIELD_NAME_SUBJECT)?;
    match me.subject() {
        Subject::ReservedSelf => w.node(NODE_KIND_RESERVED_SELF)?,
        Subject::Identifier(v) => write_identifier(v, w)?,
    }

    for name in me.function_names() {
        w.newln_and_indentation()?;
        w.field_name(FIELD_NAME_NAME)?;
        write_identifier(name, w)?;
    }

    w.close_paren()?;
    w.outdent();
    Ok(())
}

fn write_functional_term<W: Write>(me: &FunctionalTerm, w: &mut Writer<W>) -> Result<(), Error> {
    w.newln_and_indentation()?;
    w.start_node(NODE_KIND_FUNCTIONAL_TERM)?;
    w.indent();

    write_span!(me, w);

    w.newln_and_indentation()?;
    w.field_name(FIELD_NAME_FUNCTION)?;
    write_term(me.function(), w)?;

    for argument in me.arguments() {
        w.newln_and_indentation()?;
        w.field_name(FIELD_NAME_ARGUMENT)?;
        write_term(argument, w)?;
    }

    w.close_paren()?;
    w.outdent();
    Ok(())
}

fn write_sequence_builder<W: Write>(me: &SequenceBuilder, w: &mut Writer<W>) -> Result<(), Error> {
    w.newln_and_indentation()?;
    w.start_node(NODE_KIND_SEQUENCE_BUILDER)?;
    w.indent();

    write_span!(me, w);

    w.newln_and_indentation()?;
    w.field_name(FIELD_NAME_VARIABLE)?;
    match me.variables() {
        Variables::Named(v) => {
            w.newln_and_indentation()?;
            w.start_node(NODE_KIND_NAMED_VARIABLE_SET)?;
            w.indent();

            write_span!(v, w);

            for name in v.as_ref() {
                w.newln_and_indentation()?;
                w.field_name(FIELD_NAME_NAME)?;
                write_identifier(name, w)?;
            }

            w.close_paren()?;
            w.outdent();
        }
        Variables::Mapping(v) => {
            w.newln_and_indentation()?;
            w.start_node(NODE_KIND_MAPPING_VARIABLE)?;
            w.indent();

            write_span!(v, w);

            w.newln_and_indentation()?;
            w.field_name(FIELD_NAME_DOMAIN)?;
            write_identifier(v.domain(), w)?;

            w.newln_and_indentation()?;
            w.field_name(FIELD_NAME_RANGE)?;
            write_identifier(v.range(), w)?;

            w.close_paren()?;
            w.outdent();
        }
    }

    for binding in me.bindings() {
        w.newln_and_indentation()?;
        w.field_name(FIELD_NAME_BINDING)?;
        write_quantified_variable_binding(binding, w)?;
    }

    w.newln_and_indentation()?;
    w.field_name(FIELD_NAME_BODY)?;
    write_constraint_sentence(me.body(), w)?;

    w.close_paren()?;
    w.outdent();
    Ok(())
}

fn write_value<W: Write>(me: &Value, w: &mut Writer<W>) -> Result<(), Error> {
    match me {
        Value::Simple(v) => write_simple_value(v, w)?,
        Value::ValueConstructor(v) => write_value_constructor(v, w)?,
        Value::Reference(v) => write_identifier_reference(v, w)?,
        Value::Mapping(v) => write_mapping_value(v, w)?,
        Value::List(vs) => write_list_of_values(vs, w)?,
    }

    Ok(())
}

fn write_simple_value<W: Write>(me: &SimpleValue, w: &mut Writer<W>) -> Result<(), Error> {
    match me {
        SimpleValue::String(v) => {
            w.start_node(NODE_KIND_STRING)?;
            w.indent();

            w.newln_and_indentation()?;
            w.node_and_value(NODE_KIND_QUOTED_STRING, format!("{:?}", v.value()))?;

            if let Some(language) = v.language() {
                w.newln_and_indentation()?;
                w.field_name(FIELD_NAME_LANGUAGE)?;
                write_language_tag(language, w)?;
            }

            w.outdent();
            w.close_paren()?
        }
        SimpleValue::Double(v) => w.node_and_value(NODE_KIND_DOUBLE, v)?,
        SimpleValue::Decimal(v) => w.node_and_value(NODE_KIND_DECIMAL, v)?,
        SimpleValue::Integer(v) => w.node_and_value(NODE_KIND_INTEGER, v)?,
        SimpleValue::Boolean(v) => w.node_and_value(NODE_KIND_BOOLEAN, v)?,
        SimpleValue::IriReference(v) => {
            w.node_and_value(NODE_KIND_IRI_REFERENCE, format!("<{}>", v))?
        }
    }

    Ok(())
}

fn write_mapping_value<W: Write>(me: &MappingValue, w: &mut Writer<W>) -> Result<(), Error> {
    w.start_node(NODE_KIND_SEQUENCE_OF_VALUES)?;
    w.indent();

    write_span!(me, w);

    w.newln_and_indentation()?;
    w.field_name(FIELD_NAME_DOMAIN)?;
    write_simple_value(me.domain(), w)?;

    w.newln_and_indentation()?;
    w.field_name(FIELD_NAME_RANGE)?;
    write_value(me.range(), w)?;

    w.close_paren()?;
    w.outdent();
    Ok(())
}

fn write_list_of_values<W: Write>(me: &SequenceOfValues, w: &mut Writer<W>) -> Result<(), Error> {
    w.start_node(NODE_KIND_SEQUENCE_OF_VALUES)?;
    w.indent();

    write_span!(me, w);

    for value in me.iter() {
        w.newln()?;
        match value {
            SequenceMember::Simple(v) => write_simple_value(v, w)?,
            SequenceMember::ValueConstructor(v) => write_value_constructor(v, w)?,
            SequenceMember::Reference(v) => write_identifier_reference(v, w)?,
            SequenceMember::Mapping(v) => write_mapping_value(v, w)?,
        }
    }

    w.close_paren()?;
    w.outdent();
    Ok(())
}

fn write_language_tag<W: Write>(me: &LanguageTag, w: &mut Writer<W>) -> Result<(), Error> {
    w.start_node(NODE_KIND_LANGUAGE_TAG)?;
    w.value_with_prefix(me, "'")?;
    w.close_paren()?;

    Ok(())
}

fn write_value_constructor<W: Write>(
    me: &ValueConstructor,
    w: &mut Writer<W>,
) -> Result<(), Error> {
    w.start_node(NODE_KIND_VALUE_CONSTRUCTOR)?;
    w.indent();

    write_span!(me, w);

    w.newln_and_indentation()?;
    w.field_name(FIELD_NAME_NAME)?;
    write_identifier_reference(me.type_name(), w)?;

    w.newln_and_indentation()?;
    w.field_name(FIELD_NAME_VALUE)?;
    write_simple_value(me.value(), w)?;

    w.close_paren()?;
    w.outdent();
    Ok(())
}

fn write_type_definition<W: Write>(me: &Definition, w: &mut Writer<W>) -> Result<(), Error> {
    match me {
        Definition::Datatype(v) => write_data_type_def(v, w)?,
        Definition::Entity(v) => write_entity_def(v, w)?,
        Definition::Enum(v) => write_enum_def(v, w)?,
        Definition::Event(v) => write_event_def(v, w)?,
        Definition::Structure(v) => write_structure_def(v, w)?,
        Definition::Union(v) => write_union_def(v, w)?,
        Definition::Property(v) => write_property_def(v, w)?,
    }

    Ok(())
}

fn write_data_type_def<W: Write>(me: &DatatypeDef, w: &mut Writer<W>) -> Result<(), Error> {
    w.start_node_indented(NODE_KIND_DATA_TYPE_DEF)?;
    w.indent();

    write_span!(me, w);

    w.newln_and_indentation()?;
    w.field_name(FIELD_NAME_NAME)?;
    write_identifier(me.name(), w)?;

    w.newln_and_indentation()?;
    w.field_name(FIELD_NAME_BASE)?;
    write_identifier_reference(me.base_type(), w)?;

    if let Some(body) = &me.body() {
        w.newln_and_indentation()?;
        w.field_name(FIELD_NAME_BODY)?;
        write_annotation_only_body(body, w)?
    }

    w.close_paren()?;
    w.outdent();
    Ok(())
}

fn write_annotation_only_body<W: Write>(
    me: &AnnotationOnlyBody,
    w: &mut Writer<W>,
) -> Result<(), Error> {
    w.start_node(NODE_KIND_ANNOTATION_ONLY_BODY)?;
    w.indent();

    write_span!(me, w);

    write_annotations!(me.annotations(), w);

    w.close_paren()?;
    w.outdent();
    Ok(())
}

fn write_entity_def<W: Write>(me: &EntityDef, w: &mut Writer<W>) -> Result<(), Error> {
    w.start_node_indented(NODE_KIND_ENTITY_DEF)?;
    w.indent();

    write_span!(me, w);

    w.newln_and_indentation()?;
    w.field_name(FIELD_NAME_NAME)?;
    write_identifier(me.name(), w)?;

    if let Some(body) = me.body() {
        w.newln_and_indentation()?;
        w.field_name(FIELD_NAME_NAME)?;
        write_entity_body(body, w)?
    }

    w.close_paren()?;
    w.outdent();
    Ok(())
}

fn write_entity_body<W: Write>(me: &EntityBody, w: &mut Writer<W>) -> Result<(), Error> {
    w.start_node(NODE_KIND_ENTITY_BODY)?;
    w.indent();

    write_span!(me, w);

    w.newln_and_indentation()?;
    w.field_name(FIELD_NAME_IDENTITY)?;
    write_identity_member(me.identity(), w)?;

    write_annotations!(me.annotations(), w);

    for member in me.members() {
        w.newln()?;
        write_entity_member(member, w)?;
    }

    for group in me.groups() {
        w.newln()?;
        write_entity_group(group, w)?
    }

    w.close_paren()?;
    w.outdent();
    Ok(())
}

fn write_entity_member<W: Write>(me: &EntityMember, w: &mut Writer<W>) -> Result<(), Error> {
    match me {
        EntityMember::ByValue(v) => write_by_value_member(v, w)?,
        EntityMember::ByReference(v) => write_by_reference_member(v, w)?,
    }

    Ok(())
}

fn write_entity_group<W: Write>(me: &EntityGroup, w: &mut Writer<W>) -> Result<(), Error> {
    w.start_node_indented(NODE_KIND_ENTITY_GROUP)?;
    w.indent();

    write_span!(me, w);

    write_annotations!(me.annotations(), w);

    for member in me.members() {
        w.newln()?;
        write_entity_member(member, w)?;
    }

    w.close_paren()?;
    w.outdent();
    Ok(())
}

fn write_enum_def<W: Write>(me: &EnumDef, w: &mut Writer<W>) -> Result<(), Error> {
    w.start_node_indented(NODE_KIND_ENUM_DEF)?;
    w.indent();

    write_span!(me, w);

    w.newln_and_indentation()?;
    w.field_name(FIELD_NAME_NAME)?;
    write_identifier(me.name(), w)?;

    if let Some(body) = &me.body() {
        w.newln_and_indentation()?;
        w.field_name(FIELD_NAME_BODY)?;
        write_enum_body(body, w)?
    }

    w.close_paren()?;
    w.outdent();
    Ok(())
}

fn write_enum_body<W: Write>(me: &EnumBody, w: &mut Writer<W>) -> Result<(), Error> {
    w.start_node(NODE_KIND_ENUM_BODY)?;
    w.indent();

    write_span!(me, w);

    write_annotations!(me.annotations(), w);

    for variant in me.variants() {
        w.newln()?;
        write_enum_variant(variant, w)?;
    }

    w.close_paren()?;
    w.outdent();
    Ok(())
}

fn write_enum_variant<W: Write>(me: &ValueVariant, w: &mut Writer<W>) -> Result<(), Error> {
    w.start_node_indented(NODE_KIND_VALUE_VARIANT)?;
    w.indent();

    write_span!(me, w);

    w.newln_and_indentation()?;
    w.field_name(FIELD_NAME_NAME)?;
    write_identifier(me.name(), w)?;

    w.newln_and_indentation()?;
    w.field_name(FIELD_NAME_VALUE)?;

    if let Some(body) = &me.body() {
        write_annotation_only_body(body, w)?
    }

    w.close_paren()?;
    w.outdent();
    Ok(())
}

fn write_event_def<W: Write>(me: &EventDef, w: &mut Writer<W>) -> Result<(), Error> {
    w.start_node_indented(NODE_KIND_EVENT_DEF)?;
    w.indent();

    write_span!(me, w);

    w.newln_and_indentation()?;
    w.field_name(FIELD_NAME_NAME)?;
    write_identifier(me.name(), w)?;

    w.newln_and_indentation()?;
    w.field_name(FIELD_NAME_SOURCE)?;
    write_identifier_reference(me.event_source(), w)?;

    if let Some(body) = me.body() {
        w.newln_and_indentation()?;
        w.field_name(FIELD_NAME_BODY)?;
        write_structure_body(body, w)?
    }

    w.close_paren()?;
    w.outdent();
    Ok(())
}

fn write_structure_def<W: Write>(me: &StructureDef, w: &mut Writer<W>) -> Result<(), Error> {
    w.start_node_indented(NODE_KIND_STRUCTURE_DEF)?;
    w.indent();

    write_span!(me, w);

    w.newln_and_indentation()?;
    w.field_name(FIELD_NAME_NAME)?;
    write_identifier(me.name(), w)?;

    if let Some(body) = me.body() {
        w.newln_and_indentation()?;
        w.field_name(FIELD_NAME_BODY)?;
        write_structure_body(body, w)?
    }

    w.close_paren()?;
    w.outdent();
    Ok(())
}

fn write_structure_body<W: Write>(me: &StructureBody, w: &mut Writer<W>) -> Result<(), Error> {
    w.start_node(NODE_KIND_STRUCTURE_BODY)?;
    w.indent();

    write_span!(me, w);

    write_annotations!(me.annotations(), w);

    for member in me.members() {
        w.newln()?;
        write_by_value_member(member, w)?;
    }

    for group in me.groups() {
        w.newln()?;
        write_structure_group(group, w)?
    }

    w.close_paren()?;
    w.outdent();
    Ok(())
}

fn write_structure_group<W: Write>(me: &StructureGroup, w: &mut Writer<W>) -> Result<(), Error> {
    w.start_node_indented(NODE_KIND_STRUCTURE_GROUP)?;
    w.indent();

    write_span!(me, w);

    write_annotations!(me.annotations(), w);

    for member in me.members() {
        w.newln()?;
        write_by_value_member(member, w)?;
    }

    w.close_paren()?;
    w.outdent();
    Ok(())
}

fn write_union_def<W: Write>(me: &UnionDef, w: &mut Writer<W>) -> Result<(), Error> {
    w.start_node_indented(NODE_KIND_UNION_DEF)?;
    w.indent();

    write_span!(me, w);

    w.newln_and_indentation()?;
    w.field_name(FIELD_NAME_NAME)?;
    write_identifier(me.name(), w)?;

    if let Some(body) = &me.body() {
        w.newln_and_indentation()?;
        w.field_name(FIELD_NAME_BODY)?;
        write_union_body(body, w)?
    }

    w.close_paren()?;
    w.outdent();
    Ok(())
}

fn write_property_def<W: Write>(me: &PropertyDef, w: &mut Writer<W>) -> Result<(), Error> {
    w.start_node_indented(NODE_KIND_UNION_DEF)?;
    w.indent();

    write_span!(me, w);

    w.newln_and_indentation()?;
    w.field_name(FIELD_NAME_NAME)?;
    write_identifier(me.name(), w)?;

    if let Some(body) = &me.body() {
        w.newln_and_indentation()?;
        w.field_name(FIELD_NAME_BODY)?;
        write_property_body(body, w)?
    }

    w.close_paren_and_newln()?;
    w.outdent();
    Ok(())
}

fn write_property_body<W: Write>(me: &PropertyBody, w: &mut Writer<W>) -> Result<(), Error> {
    w.start_node(NODE_KIND_PROPERTY_BODY)?;
    w.indent();

    write_span!(me, w);

    write_annotations!(me.annotations(), w);

    for role in me.roles() {
        w.newln()?;
        match role.definition() {
            PropertyRoleDef::Identity(def) => {
                write_identity_role(def, role.name(), w)?;
            }
            PropertyRoleDef::ByReference(def) => {
                write_by_reference_role(def, role.name(), w)?;
            }
            PropertyRoleDef::ByValue(def) => {
                write_by_value_role(def, role.name(), w)?;
            }
        }
    }

    w.close_paren()?;
    w.outdent();
    Ok(())
}

fn write_identity_role<W: Write>(
    me: &IdentityMemberDef,
    name: &Identifier,
    w: &mut Writer<W>,
) -> Result<(), Error> {
    w.start_node_indented(NODE_KIND_IDENTITY_ROLE)?;
    w.indent();

    w.newln_and_indentation()?;
    w.field_name(FIELD_NAME_NAME)?;
    write_identifier(name, w)?;

    w.newln_and_indentation()?;
    w.field_name(FIELD_NAME_TARGET)?;
    write_type_reference(me.target_type(), w)?;

    if let Some(body) = me.body() {
        w.newln_and_indentation()?;
        w.field_name(FIELD_NAME_BODY)?;
        write_annotation_only_body(body, w)?
    }

    w.close_paren()?;
    w.outdent();
    Ok(())
}

fn write_by_value_role<W: Write>(
    me: &ByValueMemberDef,
    name: &Identifier,
    w: &mut Writer<W>,
) -> Result<(), Error> {
    w.start_node_indented(NODE_KIND_ROLE_BY_VALUE)?;
    w.indent();

    w.newln_and_indentation()?;
    w.field_name(FIELD_NAME_NAME)?;
    write_identifier(name, w)?;

    w.newln_and_indentation()?;
    w.field_name(FIELD_NAME_CARDINALITY)?;
    write_cardinality(me.target_cardinality(), w)?;

    w.newln_and_indentation()?;
    w.field_name(FIELD_NAME_TARGET)?;
    write_type_reference(me.target_type(), w)?;

    if let Some(body) = me.body() {
        w.newln_and_indentation()?;
        w.field_name(FIELD_NAME_BODY)?;
        write_annotation_only_body(body, w)?
    }

    w.close_paren()?;
    w.outdent();
    Ok(())
}

fn write_by_reference_role<W: Write>(
    me: &ByReferenceMemberDef,
    name: &Identifier,
    w: &mut Writer<W>,
) -> Result<(), Error> {
    w.start_node_indented(NODE_KIND_ROLE_BY_REFERENCE)?;
    w.indent();

    w.newln_and_indentation()?;
    w.field_name(FIELD_NAME_NAME)?;
    write_identifier(name, w)?;

    if let Some(inverse_name) = me.inverse_name() {
        w.newln_and_indentation()?;
        w.field_name(FIELD_NAME_INVERSE_NAME)?;
        write_identifier(inverse_name, w)?;
    }

    w.newln_and_indentation()?;
    w.field_name(FIELD_NAME_CARDINALITY)?;
    write_cardinality(me.target_cardinality(), w)?;

    w.newln_and_indentation()?;
    w.field_name(FIELD_NAME_TARGET)?;
    write_type_reference(me.target_type(), w)?;

    if let Some(body) = me.body() {
        w.newln_and_indentation()?;
        w.field_name(FIELD_NAME_BODY)?;
        write_annotation_only_body(body, w)?
    }

    w.close_paren()?;
    w.outdent();
    Ok(())
}

fn write_union_body<W: Write>(me: &UnionBody, w: &mut Writer<W>) -> Result<(), Error> {
    w.start_node(NODE_KIND_UNION_BODY)?;
    w.indent();

    write_span!(me, w);

    write_annotations!(me.annotations(), w);

    for variant in me.variants() {
        w.newln()?;
        write_type_variant(variant, w)?;
    }

    w.close_paren()?;
    w.outdent();
    Ok(())
}

fn write_type_variant<W: Write>(me: &TypeVariant, w: &mut Writer<W>) -> Result<(), Error> {
    w.start_node_indented(NODE_KIND_TYPE_VARIANT)?;
    w.indent();

    write_span!(me, w);

    w.newln_and_indentation()?;
    w.field_name(FIELD_NAME_NAME)?;
    write_identifier_reference(me.name_reference(), w)?;

    if let Some(rename) = &me.rename() {
        w.newln_and_indentation()?;
        w.field_name(FIELD_NAME_RENAME)?;
        write_identifier(rename, w)?;
    }

    if let Some(body) = &me.body() {
        write_annotation_only_body(body, w)?
    }

    w.close_paren()?;
    w.outdent();
    Ok(())
}

fn write_identity_member<W: Write>(me: &IdentityMember, w: &mut Writer<W>) -> Result<(), Error> {
    w.start_node(NODE_KIND_IDENTITY_MEMBER)?;
    w.indent();

    write_span!(me, w);

    match me.kind() {
        MemberKind::PropertyReference(property) => {
            w.newln_and_indentation()?;
            w.field_name(FIELD_NAME_PROPERTY)?;
            write_identifier_reference(property, w)?;

            w.newln_and_indentation()?;
            w.field_name(FIELD_NAME_NAME)?;
            write_identifier(me.name(), w)?;
        }
        MemberKind::Definition(def) => {
            w.newln_and_indentation()?;
            w.field_name(FIELD_NAME_TARGET)?;
            write_type_reference(def.target_type(), w)?;

            if let Some(body) = &def.body() {
                w.newln_and_indentation()?;
                w.field_name(FIELD_NAME_BODY)?;
                write_annotation_only_body(body, w)?
            }
        }
    }

    w.close_paren()?;
    w.outdent();
    Ok(())
}

fn write_by_value_member<W: Write>(me: &ByValueMember, w: &mut Writer<W>) -> Result<(), Error> {
    w.start_node_indented(NODE_KIND_MEMBER_BY_VALUE)?;
    w.indent();

    write_span!(me, w);

    match me.kind() {
        MemberKind::PropertyReference(property) => {
            w.newln_and_indentation()?;
            w.field_name(FIELD_NAME_PROPERTY)?;
            write_identifier_reference(property, w)?;

            w.newln_and_indentation()?;
            w.field_name(FIELD_NAME_NAME)?;
            write_identifier(me.name(), w)?;
        }
        MemberKind::Definition(def) => {
            w.newln_and_indentation()?;
            w.field_name(FIELD_NAME_NAME)?;
            write_identifier(me.name(), w)?;

            w.newln_and_indentation()?;
            w.field_name(FIELD_NAME_TARGET)?;
            write_type_reference(def.target_type(), w)?;

            let target_cardinality = def.target_cardinality();
            if *target_cardinality == DEFAULT_BY_VALUE_CARDINALITY {
                w.newln_and_indentation()?;
                w.field_name(FIELD_NAME_CARDINALITY)?;
                write_cardinality(target_cardinality, w)?;
            }

            if let Some(body) = &def.body() {
                w.newln_and_indentation()?;
                w.field_name(FIELD_NAME_BODY)?;
                write_annotation_only_body(body, w)?
            }
        }
    }

    w.close_paren()?;
    w.outdent();
    Ok(())
}

fn write_by_reference_member<W: Write>(
    me: &ByReferenceMember,
    w: &mut Writer<W>,
) -> Result<(), Error> {
    w.start_node_indented(NODE_KIND_MEMBER_BY_REFERENCE)?;
    w.indent();

    write_span!(me, w);

    match me.kind() {
        MemberKind::PropertyReference(property) => {
            w.newln_and_indentation()?;
            w.field_name(FIELD_NAME_PROPERTY)?;
            write_identifier_reference(&property, w)?;

            w.newln_and_indentation()?;
            w.field_name(FIELD_NAME_NAME)?;
            write_identifier(me.name(), w)?;
        }
        MemberKind::Definition(def) => {
            w.newln_and_indentation()?;
            w.field_name(FIELD_NAME_NAME)?;
            write_identifier(me.name(), w)?;

            w.newln_and_indentation()?;
            w.field_name(FIELD_NAME_TARGET)?;
            write_type_reference(def.target_type(), w)?;

            if let Some(name) = &def.inverse_name() {
                w.newln_and_indentation()?;
                w.field_name(FIELD_NAME_INVERSE_NAME)?;
                write_identifier(name, w)?;
            }

            let target_cardinality = def.target_cardinality();
            if *target_cardinality == DEFAULT_BY_REFERENCE_CARDINALITY {
                w.newln_and_indentation()?;
                w.field_name(FIELD_NAME_CARDINALITY)?;
                write_cardinality(target_cardinality, w)?;
            }

            if let Some(body) = &def.body() {
                w.newln_and_indentation()?;
                w.field_name(FIELD_NAME_BODY)?;
                write_annotation_only_body(body, w)?
            }
        }
    }

    w.close_paren()?;
    w.outdent();
    Ok(())
}

fn write_type_reference<W: Write>(me: &TypeReference, w: &mut Writer<W>) -> Result<(), Error> {
    if let TypeReference::Reference(reference) = me {
        write_identifier_reference(reference, w)?;
    } else {
        w.node(NODE_KIND_UNKNOWN_TYPE)?;
    }

    Ok(())
}

fn write_cardinality<W: Write>(me: &Cardinality, w: &mut Writer<W>) -> Result<(), Error> {
    w.start_node(NODE_KIND_CARDINALITY_EXPRESSION)?;

    maybe_write_span(me.source_span(), w)?;

    if let Some(ordering) = me.ordering() {
        w.field(FIELD_NAME_ORDERING, ordering)?;
        w.space()?;
    }

    if let Some(uniqueness) = me.uniqueness() {
        w.field(FIELD_NAME_UNIQUENESS, uniqueness)?;
        w.space()?;
    }

    w.field(FIELD_NAME_MIN, me.min_occurs())?;

    if let Some(max) = me.max_occurs() {
        w.space()?;
        w.field(FIELD_NAME_MAX, max)?;
    }

    w.close_paren()?;
    Ok(())
}

#[allow(dead_code)]
fn maybe_write_span<W: Write>(me: Option<&Span>, w: &mut Writer<W>) -> Result<(), Error> {
    if let Some(me) = me {
        write_span(me, w)?;
    }

    Ok(())
}

#[allow(dead_code)]
fn write_span<W: Write>(me: &Span, w: &mut Writer<W>) -> Result<(), Error> {
    w.start_node("span")?;
    w.field("start", me.start())?;
    w.space()?;
    w.field("end", me.end())?;
    w.close_paren()?;
    w.space()?;

    Ok(())
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
