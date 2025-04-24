/*!
One-line description.

TBD

# Example

TBD

 */

use crate::highlight::{none, Highlighter};
use sdml_core::{
    model::{
        annotations::{Annotation, AnnotationProperty, HasAnnotations},
        constraints::{
            BooleanSentence, ConnectiveOperator, Constraint, ConstraintBody, ConstraintSentence,
            FormalConstraint, FunctionBody, FunctionCardinality, FunctionComposition, FunctionDef,
            FunctionParameter, FunctionSignature, FunctionType, FunctionTypeReference,
            InequalityRelation, PredicateSequenceMember, PredicateValue, QuantifiedVariableBinding,
            Quantifier, SequenceBuilder, SequenceOfPredicateValues, SimpleSentence, Subject, Term,
        },
        definitions::{
            DatatypeDef, Definition, DimensionDef, DimensionIdentity, DimensionParent, EntityDef,
            EnumDef, EventDef, FromDefinition, HasOptionalFromDefinition, MethodDef, PropertyDef,
            RdfDef, RestrictionFacet, SourceEntity, StructureDef, TypeClassArgument, TypeClassDef,
            TypeClassReference, TypeVariable, TypeVariant, UnionDef, ValueVariant,
        },
        identifiers::IdentifierReference,
        members::{Cardinality, Member, MemberDef, MemberKind, TypeReference, DEFAULT_CARDINALITY},
        modules::{Import, Module},
        values::{Binary, MappingValue, SequenceMember, SimpleValue, Value, ValueConstructor},
        HasBody, HasName, HasNameReference, HasOptionalBody,
    },
    repr::RepresentationWriter,
    store::InMemoryModuleCache,
    syntax::*,
};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Level {
    #[default]
    Full,
    SuppressMemberBodies,
    SuppressMembers,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum Whitespace {
    Minimal = 0,
    #[default]
    Normal = 1,
    Additional = 2,
    Maximal = 3,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum OperatorForm {
    #[default]
    Textual,
    Symbolic,
}

/// The type that implements the generator.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Options {
    emit_base_iri: bool,
    level: Level,
    whitespace: Whitespace,
    operator_form: OperatorForm,
}

/// The type that implements the generator.
#[derive(Debug, Default)]
pub struct Writer;

// ------------------------------------------------------------------------------------------------
// Private Types/Values
// ------------------------------------------------------------------------------------------------

const SPACE_STR: &str = " ";
const EMPTY_STR: &str = "";

const ELIPPSIS: &[u8] = b" ;; ...\n";
const EOL: &[u8] = b"\n";
const SPACE: &[u8] = b" ";

#[derive(Copy, Clone, Debug)]
struct Context<T>
where
    T: Highlighter,
{
    options: Options,
    indent_depth: usize,
    indent_by: usize,
    hl: T,
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Options
// ------------------------------------------------------------------------------------------------

impl Default for Options {
    fn default() -> Self {
        Self {
            emit_base_iri: true,
            level: Default::default(),
            whitespace: Default::default(),
            operator_form: Default::default(),
        }
    }
}

impl Options {
    pub fn with_level(self, level: Level) -> Self {
        Self { level, ..self }
    }

    pub fn with_whitespace(self, whitespace: Whitespace) -> Self {
        Self { whitespace, ..self }
    }

    pub fn with_operator_form(self, operator_form: OperatorForm) -> Self {
        Self {
            operator_form,
            ..self
        }
    }

    pub fn with_emit_base_iri(self, emit_base_iri: bool) -> Self {
        Self {
            emit_base_iri,
            ..self
        }
    }

    #[inline(always)]
    pub fn whitespace_minimal(&self) -> bool {
        self.whitespace >= Whitespace::Minimal
    }

    #[inline(always)]
    pub fn whitespace_normal(&self) -> bool {
        self.whitespace >= Whitespace::Normal
    }

    #[inline(always)]
    pub fn whitespace_additional(&self) -> bool {
        self.whitespace >= Whitespace::Additional
    }

    #[inline(always)]
    pub fn whitespace_maximal(&self) -> bool {
        self.whitespace >= Whitespace::Maximal
    }

    #[inline(always)]
    pub const fn generate_definition_bodies(&self) -> bool {
        matches!(self.level, Level::Full | Level::SuppressMemberBodies)
    }

    #[inline(always)]
    pub const fn generate_member_bodies(&self) -> bool {
        matches!(self.level, Level::Full)
    }

    #[inline(always)]
    pub const fn generate_variant_bodies(&self) -> bool {
        matches!(self.level, Level::Full)
    }

    #[inline(always)]
    pub const fn generate_textual_operators(&self) -> bool {
        matches!(self.operator_form, OperatorForm::Textual)
    }

    #[inline(always)]
    pub const fn generate_symbolic_operators(&self) -> bool {
        matches!(self.operator_form, OperatorForm::Symbolic)
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Context
// ------------------------------------------------------------------------------------------------

impl Default for Context<none::None> {
    fn default() -> Self {
        Self {
            options: Default::default(),
            indent_by: 2,
            indent_depth: 0,
            hl: none::None,
        }
    }
}

impl From<Options> for Context<none::None> {
    fn from(options: Options) -> Self {
        Self {
            options,
            indent_by: 2,
            indent_depth: 0,
            hl: none::None,
        }
    }
}

impl<T> Context<T>
where
    T: Highlighter,
{
    fn indent(&mut self) {
        self.indent_depth += 1;
    }

    fn outdent(&mut self) {
        self.indent_depth -= 1;
    }

    fn indentation_str(&self) -> String {
        let n = self.indent_depth * self.indent_by;
        format!("{:n$}", "")
    }
}

macro_rules! contextfn {
    ($kind:ident $fn_name:ident => $const_name:expr) => {
        #[inline(always)]
        fn $fn_name(&self) -> String {
            self.hl.$kind($const_name)
        }
    };
    (operator $fn_name:ident => $const_text:expr, $const_symbol:expr) => {
        #[inline(always)]
        fn $fn_name(&self) -> String {
            if self.options.generate_textual_operators() {
                self.hl.operator($const_text)
            } else {
                self.hl.operator($const_symbol)
            }
        }
    };
}

impl<T> Context<T>
where
    T: Highlighter,
{
    contextfn!(keyword kw_a => KW_A);
    contextfn!(keyword kw_as => KW_RENAME_AS);
    contextfn!(keyword kw_assert => KW_ASSERT);
    contextfn!(keyword kw_class => KW_CLASS);
    contextfn!(keyword kw_datatype => KW_DATATYPE);
    contextfn!(keyword kw_dimension => KW_DIMENSION);
    contextfn!(keyword kw_end => KW_BLOCK_END);
    contextfn!(keyword kw_entity => KW_ENTITY);
    contextfn!(keyword kw_enum => KW_ENUM);
    contextfn!(keyword kw_event => KW_EVENT);
    contextfn!(keyword kw_fixed => KW_DATATYPE_FIXED);
    contextfn!(keyword kw_fn_def => KW_FN_DEF);
    contextfn!(keyword kw_from => KW_IMPORT_FROM);
    contextfn!(keyword kw_identity => KW_ENTITY_IDENTITY);
    contextfn!(keyword kw_import => KW_IMPORT);
    contextfn!(keyword kw_is => KW_BLOCK_IS);
    contextfn!(keyword kw_module => KW_MODULE);
    contextfn!(keyword kw_of => KW_BLOCK_OF);
    contextfn!(keyword kw_opaque => KW_DATATYPE_OPAQUE);
    contextfn!(keyword kw_parent => KW_DIMENSION_PARENT);
    contextfn!(keyword kw_property => KW_PROPERTY);
    contextfn!(keyword kw_rdf => KW_RDF);
    contextfn!(keyword kw_ref => KW_REF);
    contextfn!(keyword kw_self => KW_SELF);
    contextfn!(keyword kw_source => KW_SOURCE);
    contextfn!(keyword kw_structure => KW_STRUCTURE);
    contextfn!(keyword kw_type => KW_TYPE);
    contextfn!(keyword kw_union => KW_UNION);
    contextfn!(keyword kw_unknown => KW_TYPE_UNKNOWN);
    contextfn!(keyword kw_version => KW_MODULE_VERSION);
    contextfn!(keyword kw_wildcard => KW_WILDCARD);
    contextfn!(keyword kw_with => KW_WITH);

    contextfn!(operator by_defn => OP_ASSIGNMENT_BY_DEFINITION, OP_ASSIGNMENT_BY_DEFINITION_SYMBOL);
    contextfn!(operator fn_composition => OP_FN_COMPOSITION, OP_FN_COMPOSITION_SYMBOL);
    contextfn!(operator lop_and => OP_LOGICAL_CONJUNCTION, OP_LOGICAL_CONJUNCTION_SYMBOL);
    contextfn!(operator lop_exists => OP_LOGICAL_QUANTIFIER_EXISTS, OP_LOGICAL_QUANTIFIER_EXISTS_SYMBOL);
    contextfn!(operator lop_forall => OP_LOGICAL_QUANTIFIER_FORALL, OP_LOGICAL_QUANTIFIER_FORALL_SYMBOL);
    contextfn!(operator lop_iff => OP_LOGICAL_BICONDITIONAL, OP_LOGICAL_BICONDITIONAL_SYMBOL);
    contextfn!(operator lop_implies => OP_LOGICAL_IMPLICATION, OP_LOGICAL_IMPLICATION_SYMBOL);
    contextfn!(operator lop_not => OP_LOGICAL_NEGATION, OP_LOGICAL_NEGATION_SYMBOL);
    contextfn!(operator lop_or => OP_LOGICAL_DISJUNCTION, OP_LOGICAL_DISJUNCTION_SYMBOL);
    contextfn!(operator lop_xor => OP_LOGICAL_EXCLUSIVE_DISJUNCTION, OP_LOGICAL_EXCLUSIVE_DISJUNCTION_SYMBOL);
    contextfn!(operator op_assign => OP_ASSIGNMENT);
    contextfn!(operator op_member => OP_SET_MEMBERSHIP, OP_SET_MEMBERSHIP_SYMBOL);
    contextfn!(operator range => OP_TYPE_CARDINALITY_RANGE);
    contextfn!(operator rel_eq => OP_RELATION_EQUAL);
    contextfn!(operator rel_gt => OP_RELATION_GREATER_THAN);
    contextfn!(operator rel_gteq => OP_RELATION_GREATER_THAN_OR_EQUAL, OP_RELATION_GREATER_THAN_OR_EQUAL_SYMBOL);
    contextfn!(operator rel_lt => OP_RELATION_LESS_THAN);
    contextfn!(operator rel_lteq => OP_RELATION_LESS_THAN_OR_EQUAL, OP_RELATION_LESS_THAN_OR_EQUAL_SYMBOL);
    contextfn!(operator rel_neq => OP_RELATION_NOT_EQUAL, OP_RELATION_NOT_EQUAL_SYMBOL);
    contextfn!(operator type_assert => OP_TYPE_ASSERTION, OP_TYPE_ASSERTION_SYMBOL);
    contextfn!(operator type_combine => OP_TYPE_COMBINE, OP_TYPE_COMBINE_SYMBOL);
    contextfn!(operator type_restrict => OP_TYPE_RESTRICTION, OP_TYPE_RESTRICTION_SYMBOL);

    contextfn!(punctuation_bracket binary_end => PC_BINARY_END);
    contextfn!(punctuation_bracket binary_start => PC_BINARY_START);
    contextfn!(punctuation_bracket left_paren => PC_PAREN_LEFT);
    contextfn!(punctuation_bracket restriction_end => PC_BRACE_RIGHT);
    contextfn!(punctuation_bracket restriction_start => PC_BRACE_LEFT);
    contextfn!(punctuation_bracket right_paren => PC_PAREN_RIGHT);
    contextfn!(punctuation_bracket sequence_end => PC_BRACKET_RIGHT);
    contextfn!(punctuation_bracket sequence_start => PC_BRACKET_LEFT);
    contextfn!(punctuation_separator quantified_sentence_sep => PC_QUANTIFIED_SENTENCE_SEPARATOR);
    // contextfn!(punctuation_separator lang_at => PC_LANGUAGE_PREFIX);
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Writer
// ------------------------------------------------------------------------------------------------

impl RepresentationWriter for Writer {
    type Object = Module;
    type Cache = InMemoryModuleCache;
    type Options = Options;

    fn write_with<W>(
        &self,
        w: &mut W,
        module: &Self::Object,
        _: Option<&Self::Cache>,
        options: &Self::Options,
    ) -> Result<(), sdml_core::error::Error>
    where
        W: std::io::Write,
    {
        let mut ctx: Context<_> = (*options).into();

        w.write_all(
            format!(
                "{} {} ",
                ctx.kw_module(),
                ctx.hl.module_defn(module.name().as_ref())
            )
            .as_bytes(),
        )?;

        if ctx.options.emit_base_iri {
            if let Some(base) = module.base_uri() {
                w.write_all(format!("{} ", ctx.hl.value_iri(base.as_ref())).as_bytes())?;
            }
        }

        if module.is_versioned() {
            w.write_all(format!("{} ", ctx.kw_version()).as_bytes())?;
            if let Some(version_info) = module.version_info() {
                w.write_all(format!("{} ", ctx.hl.value_string(version_info.value())).as_bytes())?;
            }
            if let Some(version_uri) = module.version_uri() {
                w.write_all(format!("{} ", ctx.hl.value_iri(version_uri.value())).as_bytes())?;
            }
        }

        if module.has_imports() || module.has_annotations() || module.has_definitions() {
            ctx.indent();
            w.write_all(format!("{}\n", ctx.kw_is()).as_bytes())?;
            if ctx.options.whitespace_normal() {
                w.write_all(EOL)?;
            }

            if module.has_imports() {
                self.write_module_imports(w, module, &mut ctx)?;
                if ctx.options.whitespace_normal() {
                    w.write_all(EOL)?;
                }
            }

            if module.has_annotations() {
                self.write_annotations(w, module.annotations(), &mut ctx)?;
                if ctx.options.whitespace_normal() {
                    w.write_all(EOL)?;
                }
            }

            if module.has_definitions() {
                self.write_module_definitions(w, module, &mut ctx)?;
            }

            ctx.outdent();
            w.write_all(format!("{}\n", ctx.kw_end()).as_bytes())?;
        } else {
            w.write_all(format!("{} {}\n", ctx.kw_is(), ctx.kw_end()).as_bytes())?;
        }

        Ok(())
    }
}

impl Writer {
    // --------------------------------------------------------------------------------------------
    // Writer ❱ Imports
    // --------------------------------------------------------------------------------------------

    fn write_module_imports<T, W>(
        &self,
        w: &mut W,
        module: &Module,
        ctx: &mut Context<T>,
    ) -> Result<(), sdml_core::error::Error>
    where
        W: std::io::Write,
        T: Highlighter,
    {
        let import = |import: &Import| match import {
            Import::Module(import) => {
                if let Some(version_uri) = import.version_uri() {
                    format!(
                        "{} {}",
                        ctx.hl.module_ref(import.name().as_ref()),
                        ctx.hl.value_iri(version_uri.as_ref()),
                    )
                } else {
                    ctx.hl.module_ref(import.name().as_ref())
                }
            }
            Import::Member(qid) => ctx.hl.type_ref(&qid.to_string()),
        };
        if module.has_imports() {
            let indentation = ctx.indentation_str();
            for import_statement in module.imports() {
                let imported = if import_statement.imports_len() == 1 {
                    import_statement
                        .imports()
                        .map(import)
                        .collect::<Vec<String>>()
                        .join("")
                } else {
                    format!(
                        "{} {} {}",
                        ctx.sequence_start(),
                        import_statement
                            .imports()
                            .map(import)
                            .collect::<Vec<String>>()
                            .join(" "),
                        ctx.sequence_end(),
                    )
                };
                w.write_all(indentation.as_bytes())?;
                if let Some(path) = import_statement.from_module_path() {
                    let path_string = path.to_string();
                    w.write_all(format!("{} {path_string} ", ctx.kw_from()).as_bytes())?;
                }
                w.write_all(format!("{} {imported}\n", ctx.kw_import()).as_bytes())?;
                if ctx.options.whitespace_maximal() {
                    w.write_all(EOL)?;
                }
            }
        }

        Ok(())
    }

    // --------------------------------------------------------------------------------------------
    // Writer ❱ Annotations
    // --------------------------------------------------------------------------------------------

    fn write_annotations<'a, T, W>(
        &'a self,
        w: &mut W,
        annotations: impl Iterator<Item = &'a Annotation>,
        ctx: &mut Context<T>,
    ) -> Result<(), sdml_core::error::Error>
    where
        W: std::io::Write,
        T: Highlighter,
    {
        for annotation in annotations {
            match annotation {
                Annotation::Property(v) => self.write_annotation_property(w, v, ctx)?,
                Annotation::Constraint(v) => self.write_constraint(w, v, ctx)?,
            }
            if ctx.options.whitespace_maximal() {
                w.write_all(EOL)?;
            }
        }

        Ok(())
    }

    fn write_annotation_property<T, W>(
        &self,
        w: &mut W,
        annotation: &AnnotationProperty,
        ctx: &mut Context<T>,
    ) -> Result<(), sdml_core::error::Error>
    where
        W: std::io::Write,
        T: Highlighter,
    {
        // TODO: ensure wrapping
        w.write_all(
            format!(
                "{}{}{} {} ",
                ctx.indentation_str(),
                ctx.hl.annotation_property(PC_ANNOTATION_PREFIX),
                ctx.hl
                    .annotation_property(&annotation.name_reference().to_string()),
                ctx.rel_eq(),
            )
            .as_bytes(),
        )?;
        self.write_value(w, annotation.value(), ctx)?;
        w.write_all(EOL)?;
        Ok(())
    }

    // --------------------------------------------------------------------------------------------
    // Writer ❱ Annotations ❱ Constraints
    // --------------------------------------------------------------------------------------------

    fn write_constraint<T, W>(
        &self,
        w: &mut W,
        constraint: &Constraint,
        ctx: &mut Context<T>,
    ) -> Result<(), sdml_core::error::Error>
    where
        W: std::io::Write,
        T: Highlighter,
    {
        let indentation = ctx.indentation_str();
        w.write_all(format!("{indentation}{} ", ctx.kw_assert()).as_bytes())?;
        w.write_all(
            format!(
                "{} ",
                ctx.hl.annotation_constraint(constraint.name().as_ref())
            )
            .as_bytes(),
        )?;
        match constraint.body() {
            ConstraintBody::Informal(v) => {
                w.write_all(
                    format!(
                        "{} {:?}{}\n",
                        ctx.rel_eq(),
                        v.value(),
                        if let Some(lang) = v.language() {
                            lang.to_string()
                        } else {
                            String::new()
                        }
                    )
                    .as_bytes(),
                )?;
            }
            ConstraintBody::Formal(v) => {
                self.write_formal_constraint(w, v, ctx)?;
            }
        }
        Ok(())
    }

    fn write_formal_constraint<T, W>(
        &self,
        w: &mut W,
        constraint: &FormalConstraint,
        ctx: &mut Context<T>,
    ) -> Result<(), sdml_core::error::Error>
    where
        W: std::io::Write,
        T: Highlighter,
    {
        if constraint.has_definitions() {
            w.write_all(format!("{}\n", ctx.kw_with()).as_bytes())?;
            ctx.indent();

            if ctx.options.whitespace_maximal() {
                w.write_all(EOL)?;
            }
            for definition in constraint.definitions() {
                self.write_function_def(w, definition, ctx)?;
            }
            if ctx.options.whitespace_maximal() {
                w.write_all(EOL)?;
            }

            ctx.outdent();
            w.write_all(EOL)?;
            w.write_all(format!("{}{}\n", ctx.indentation_str(), ctx.kw_is()).as_bytes())?;
        } else {
            w.write_all(format!("{}\n", ctx.kw_is()).as_bytes())?;
        }
        ctx.indent();
        w.write_all(format!("{}", ctx.indentation_str()).as_bytes())?;
        self.write_constraint_sentence(w, constraint.body(), ctx)?;
        ctx.outdent();
        w.write_all(EOL)?;
        w.write_all(format!("{}{}\n", ctx.indentation_str(), ctx.kw_end()).as_bytes())?;
        Ok(())
    }

    fn write_function_def<T, W>(
        &self,
        w: &mut W,
        function: &FunctionDef,
        ctx: &mut Context<T>,
    ) -> Result<(), sdml_core::error::Error>
    where
        W: std::io::Write,
        T: Highlighter,
    {
        self.write_function_signature(w, function.signature(), ctx)?;

        w.write_all(format!(" {}\n", ctx.by_defn(),).as_bytes())?;

        self.write_function_body(w, function.body(), ctx)?;

        Ok(())
    }

    fn write_function_signature<T, W>(
        &self,
        w: &mut W,
        signature: &FunctionSignature,
        ctx: &mut Context<T>,
    ) -> Result<(), sdml_core::error::Error>
    where
        W: std::io::Write,
        T: Highlighter,
    {
        w.write_all(
            format!(
                "{}{} {}{}",
                ctx.indentation_str(),
                ctx.kw_fn_def(),
                ctx.hl.fn_defn(signature.name().as_ref()),
                ctx.left_paren(),
            )
            .as_bytes(),
        )?;
        for (i, param) in signature.parameters().enumerate() {
            self.write_function_parameter(w, param, ctx)?;
            if i < signature.parameters_len() - 1 {
                w.write_all(b" ")?;
            }
        }
        w.write_all(format!("{} {} ", ctx.right_paren(), ctx.type_assert(),).as_bytes())?;

        self.write_function_type(w, signature.target_type(), ctx)?;

        Ok(())
    }

    fn write_function_parameter<T, W>(
        &self,
        w: &mut W,
        param: &FunctionParameter,
        ctx: &mut Context<T>,
    ) -> Result<(), sdml_core::error::Error>
    where
        W: std::io::Write,
        T: Highlighter,
    {
        w.write_all(
            format!(
                "{} {} ",
                ctx.hl.var_param_defn(param.name().as_ref()),
                ctx.type_assert(),
            )
            .as_bytes(),
        )?;

        self.write_function_type(w, param.target_type(), ctx)
    }

    fn write_function_type<T, W>(
        &self,
        w: &mut W,
        fn_type: &FunctionType,
        ctx: &mut Context<T>,
    ) -> Result<(), sdml_core::error::Error>
    where
        W: std::io::Write,
        T: Highlighter,
    {
        self.write_function_cardinality(w, fn_type.target_cardinality(), ctx)?;

        match fn_type.target_type() {
            FunctionTypeReference::Wildcard => {
                w.write_all(ctx.kw_wildcard().as_bytes())?;
            }
            FunctionTypeReference::Reference(v) => {
                w.write_all(ctx.hl.type_ref(&v.to_string()).as_bytes())?;
            }
            FunctionTypeReference::MappingType(v) => {
                w.write_all(
                    format!(
                        "{} {} {}",
                        ctx.hl.type_ref(&v.domain().to_string()),
                        ctx.type_assert(),
                        ctx.hl.type_ref(&v.range().to_string()),
                    )
                    .as_bytes(),
                )?;
            }
        }

        Ok(())
    }

    fn write_function_cardinality<T, W>(
        &self,
        w: &mut W,
        fn_card: &FunctionCardinality,
        ctx: &mut Context<T>,
    ) -> Result<(), sdml_core::error::Error>
    where
        W: std::io::Write,
        T: Highlighter,
    {
        if fn_card.has_ordering() || fn_card.has_uniqueness() || fn_card.has_range() {
            w.write_all(ctx.restriction_start().as_bytes())?;

            if let Some(ordering) = fn_card.ordering() {
                w.write_all(
                    format!(
                        "{}{}",
                        ctx.hl.keyword(&ordering.to_string()),
                        if fn_card.has_uniqueness() || fn_card.has_range() {
                            SPACE_STR
                        } else {
                            EMPTY_STR
                        }
                    )
                    .as_bytes(),
                )?;
            }

            if let Some(uniqueness) = fn_card.uniqueness() {
                w.write_all(
                    format!(
                        "{}{}",
                        ctx.hl.keyword(&uniqueness.to_string()),
                        if fn_card.has_range() {
                            SPACE_STR
                        } else {
                            EMPTY_STR
                        }
                    )
                    .as_bytes(),
                )?;
            }

            if let Some(range) = fn_card.range() {
                w.write_all(
                    format!(
                        "{}{}",
                        range.min_occurs(),
                        if let Some(max_occurs) = range.max_occurs() {
                            if max_occurs == range.min_occurs() {
                                String::new()
                            } else {
                                format!("{}{}", ctx.range(), max_occurs)
                            }
                        } else {
                            ctx.range()
                        },
                    )
                    .as_bytes(),
                )?;
            }
        }
        Ok(())
    }

    fn write_function_body<T, W>(
        &self,
        w: &mut W,
        body: &FunctionBody,
        ctx: &mut Context<T>,
    ) -> Result<(), sdml_core::error::Error>
    where
        W: std::io::Write,
        T: Highlighter,
    {
        ctx.indent();

        if ctx.options.whitespace_maximal() {
            w.write_all(EOL)?;
        }
        w.write_all(ctx.indentation_str().as_bytes())?;

        match body {
            FunctionBody::Sentence(v) => self.write_constraint_sentence(w, v, ctx)?,
            FunctionBody::Term(v) => self.write_term(w, v, ctx)?,
        }

        if ctx.options.whitespace_maximal() {
            w.write_all(EOL)?;
        }

        ctx.outdent();

        Ok(())
    }

    fn write_constraint_sentence<T, W>(
        &self,
        w: &mut W,
        sentence: &ConstraintSentence,
        ctx: &mut Context<T>,
    ) -> Result<(), sdml_core::error::Error>
    where
        W: std::io::Write,
        T: Highlighter,
    {
        match sentence {
            ConstraintSentence::Simple(SimpleSentence::Atomic(v)) => {
                self.write_term(w, v.predicate(), ctx)?;
                w.write_all(ctx.left_paren().as_bytes())?;
                for (i, arg) in v.arguments().enumerate() {
                    self.write_term(w, arg, ctx)?;
                    if i < v.arguments_len() - 1 {
                        w.write_all(b" ")?;
                    }
                }
                w.write_all(ctx.right_paren().as_bytes())?;
            }
            ConstraintSentence::Simple(SimpleSentence::Equation(v)) => {
                self.write_term(w, v.left_operand(), ctx)?;
                w.write_all(format!(" {} ", ctx.op_assign()).as_bytes())?;
                self.write_term(w, v.right_operand(), ctx)?;
            }
            ConstraintSentence::Simple(SimpleSentence::Inequation(v)) => {
                self.write_term(w, v.left_operand(), ctx)?;
                w.write_all(
                    format!(
                        " {} ",
                        match v.relation() {
                            InequalityRelation::NotEqual => ctx.rel_neq(),
                            InequalityRelation::LessThan => ctx.rel_lt(),
                            InequalityRelation::LessThanOrEqual => ctx.rel_lteq(),
                            InequalityRelation::GreaterThan => ctx.rel_gt(),
                            InequalityRelation::GreaterThanOrEqual => ctx.rel_gteq(),
                        }
                    )
                    .as_bytes(),
                )?;
                self.write_term(w, v.right_operand(), ctx)?;
            }
            ConstraintSentence::Boolean(BooleanSentence::Unary(v)) => {
                w.write_all(format!("{} ", ctx.lop_not()).as_bytes())?;
                self.write_constraint_sentence(w, v.operand(), ctx)?;
            }
            ConstraintSentence::Boolean(BooleanSentence::Binary(v)) => {
                self.write_constraint_sentence(w, v.left_operand(), ctx)?;
                w.write_all(
                    format!(
                        " {} ",
                        match v.operator() {
                            ConnectiveOperator::Negation => ctx.lop_not(),
                            ConnectiveOperator::Conjunction => ctx.lop_and(),
                            ConnectiveOperator::Disjunction => ctx.lop_or(),
                            ConnectiveOperator::ExclusiveDisjunction => ctx.lop_xor(),
                            ConnectiveOperator::Implication => ctx.lop_implies(),
                            ConnectiveOperator::Biconditional => ctx.lop_iff(),
                        }
                    )
                    .as_bytes(),
                )?;
                self.write_constraint_sentence(w, v.right_operand(), ctx)?;
            }
            ConstraintSentence::Quantified(v) => {
                self.write_quantified_variable_binding(w, v.binding(), ctx)?;

                w.write_all(format!("{} ", ctx.quantified_sentence_sep()).as_bytes())?;

                self.write_constraint_sentence(w, v.body(), ctx)?;
            }
        }

        Ok(())
    }

    fn write_quantified_variable_binding<T, W>(
        &self,
        w: &mut W,
        binding: &QuantifiedVariableBinding,
        ctx: &mut Context<T>,
    ) -> Result<(), sdml_core::error::Error>
    where
        W: std::io::Write,
        T: Highlighter,
    {
        w.write_all(
            format!(
                "{} ",
                match binding.quantifier() {
                    Quantifier::Existential => ctx.lop_exists(),
                    Quantifier::Universal => ctx.lop_forall(),
                }
            )
            .as_bytes(),
        )?;

        let binding = binding.binding();
        {
            let variable = binding.variable();
            w.write_all(format!("{} ", variable.name()).as_bytes())?;
            if let Some(range) = variable.range() {
                w.write_all(format!(" {} {} ", ctx.type_assert(), range).as_bytes())?;
            }
        }

        w.write_all(format!(" {} ", ctx.op_member()).as_bytes())?;
        self.write_term(w, binding.source(), ctx)?;

        Ok(())
    }

    fn write_term<T, W>(
        &self,
        w: &mut W,
        term: &Term,
        ctx: &mut Context<T>,
    ) -> Result<(), sdml_core::error::Error>
    where
        W: std::io::Write,
        T: Highlighter,
    {
        match term {
            Term::Sequence(v) => {
                self.write_sequence_builder(w, v, ctx)?;
            }
            Term::Function(v) => {
                self.write_term(w, v.function(), ctx)?;
                w.write_all(ctx.left_paren().as_bytes())?;
                for (i, arg) in v.arguments().enumerate() {
                    self.write_term(w, arg, ctx)?;
                    if i < v.arguments_len() - 1 {
                        w.write_all(SPACE_STR.as_bytes())?;
                    }
                }
                w.write_all(ctx.right_paren().as_bytes())?;
            }
            Term::Composition(v) => {
                self.write_function_composition(w, v, ctx)?;
            }
            Term::Identifier(v) => {
                w.write_all(ctx.hl.field_ref(&v.to_string()).as_bytes())?;
            }
            Term::ReservedSelf => {
                w.write_all(ctx.kw_self().as_bytes())?;
            }
            Term::Value(PredicateValue::Simple(v)) => {
                self.write_simple_value(w, v, ctx)?;
            }
            Term::Value(PredicateValue::Sequence(v)) => {
                self.write_sequence_of_predicate_values(w, v, ctx)?;
            }
        }
        Ok(())
    }

    fn write_sequence_of_predicate_values<T, W>(
        &self,
        w: &mut W,
        sequence: &SequenceOfPredicateValues,
        ctx: &mut Context<T>,
    ) -> Result<(), sdml_core::error::Error>
    where
        W: std::io::Write,
        T: Highlighter,
    {
        w.write_all(format!("{} ", ctx.sequence_start()).as_bytes())?;
        for value in sequence.iter() {
            self.write_predicate_sequence_member(w, value, ctx)?;
            w.write_all(b" ")?;
        }
        w.write_all(ctx.sequence_end().as_bytes())?;
        Ok(())
    }

    fn write_predicate_sequence_member<T, W>(
        &self,
        w: &mut W,
        value: &PredicateSequenceMember,
        ctx: &mut Context<T>,
    ) -> Result<(), sdml_core::error::Error>
    where
        W: std::io::Write,
        T: Highlighter,
    {
        match value {
            PredicateSequenceMember::Simple(v) => {
                self.write_simple_value(w, v, ctx)?;
            }
            PredicateSequenceMember::ValueConstructor(v) => {
                self.write_value_constructor(w, v, ctx)?;
            }
            PredicateSequenceMember::Mapping(v) => {
                self.write_mapping_value(w, v, ctx)?;
            }
            PredicateSequenceMember::Reference(v) => {
                w.write_all(ctx.hl.type_ref(&v.to_string()).as_bytes())?;
            }
        }
        Ok(())
    }

    fn write_sequence_builder<T, W>(
        &self,
        _w: &mut W,
        _term: &SequenceBuilder,
        _ctx: &mut Context<T>,
    ) -> Result<(), sdml_core::error::Error>
    where
        W: std::io::Write,
        T: Highlighter,
    {
        todo!()
    }

    fn write_function_composition<T, W>(
        &self,
        w: &mut W,
        term: &FunctionComposition,
        ctx: &mut Context<T>,
    ) -> Result<(), sdml_core::error::Error>
    where
        W: std::io::Write,
        T: Highlighter,
    {
        let dot = ctx.fn_composition();
        match term.subject() {
            Subject::ReservedSelf => {
                w.write_all(ctx.kw_self().as_bytes())?;
            }
            Subject::Identifier(id) => {
                w.write_all(ctx.hl.field_ref(id.as_ref()).as_bytes())?;
            }
        }
        w.write_all(dot.as_bytes())?;
        let fn_composition: String = term
            .function_names()
            .map(|id| ctx.hl.fn_call(id.as_ref()))
            .collect::<Vec<String>>()
            .join(&dot);
        w.write_all(fn_composition.as_bytes())?;
        Ok(())
    }

    // --------------------------------------------------------------------------------------------
    // Writer ❱ Annotations ❱ Values
    // --------------------------------------------------------------------------------------------

    fn write_value<T, W>(
        &self,
        w: &mut W,
        value: &Value,
        ctx: &mut Context<T>,
    ) -> Result<(), sdml_core::error::Error>
    where
        W: std::io::Write,
        T: Highlighter,
    {
        match value {
            Value::Simple(v) => {
                self.write_simple_value(w, v, ctx)?;
            }
            Value::ValueConstructor(v) => {
                self.write_value_constructor(w, v, ctx)?;
            }
            Value::Mapping(v) => {
                self.write_mapping_value(w, v, ctx)?;
            }
            Value::Reference(v) => {
                w.write_all(ctx.hl.type_ref(&v.to_string()).as_bytes())?;
            }
            Value::Sequence(sequence) => {
                if sequence.has_ordering() || sequence.has_uniqueness() {
                    w.write_all(format!("{}", ctx.restriction_start()).as_bytes())?;
                    if let Some(ordering) = sequence.ordering() {
                        w.write_all(ctx.hl.keyword(&ordering.to_string()).as_bytes())?;
                    }
                    if sequence.has_ordering() && sequence.has_uniqueness() {
                        w.write_all(b" ")?;
                    }
                    if let Some(uniqueness) = sequence.uniqueness() {
                        w.write_all(ctx.hl.keyword(&uniqueness.to_string()).as_bytes())?;
                    }
                    w.write_all(format!("{} ", ctx.restriction_end()).as_bytes())?;
                }
                w.write_all(format!("{} ", ctx.sequence_start()).as_bytes())?;
                for value in sequence.iter() {
                    self.write_sequence_member(w, value, ctx)?;
                    w.write_all(b" ")?;
                }
                w.write_all(ctx.sequence_end().as_bytes())?;
            }
        }
        Ok(())
    }

    fn write_mapping_value<T, W>(
        &self,
        w: &mut W,
        value: &MappingValue,
        ctx: &mut Context<T>,
    ) -> Result<(), sdml_core::error::Error>
    where
        W: std::io::Write,
        T: Highlighter,
    {
        self.write_simple_value(w, value.domain(), ctx)?;
        w.write_all(format!(" {} ", ctx.type_assert()).as_bytes())?;
        self.write_value(w, value.range(), ctx)?;
        Ok(())
    }

    fn write_value_constructor<T, W>(
        &self,
        w: &mut W,
        value: &ValueConstructor,
        ctx: &mut Context<T>,
    ) -> Result<(), sdml_core::error::Error>
    where
        W: std::io::Write,
        T: Highlighter,
    {
        w.write_all(
            format!(
                "{}{}",
                ctx.hl.type_ref(&value.type_name().to_string()),
                ctx.left_paren(),
            )
            .as_bytes(),
        )?;
        self.write_simple_value(w, value.value(), ctx)?;
        w.write_all(ctx.right_paren().as_bytes())?;
        Ok(())
    }

    fn write_simple_value<T, W>(
        &self,
        w: &mut W,
        value: &SimpleValue,
        ctx: &mut Context<T>,
    ) -> Result<(), sdml_core::error::Error>
    where
        W: std::io::Write,
        T: Highlighter,
    {
        match value {
            SimpleValue::Boolean(v) => {
                w.write_all(ctx.hl.value_builtin(&v.to_string()).as_bytes())?;
            }
            SimpleValue::Double(v) => {
                w.write_all(ctx.hl.value_number(&v.to_string()).as_bytes())?;
            }
            SimpleValue::Decimal(v) => {
                w.write_all(ctx.hl.value_number(&v.to_string()).as_bytes())?;
            }
            SimpleValue::Integer(v) => {
                w.write_all(ctx.hl.value_number(&v.to_string()).as_bytes())?;
            }
            SimpleValue::Unsigned(v) => {
                w.write_all(ctx.hl.value_number(&v.to_string()).as_bytes())?;
            }
            SimpleValue::String(v) => {
                w.write_all(ctx.hl.value_string(v.value()).as_bytes())?;
                if let Some(language) = v.language() {
                    w.write_all(ctx.hl.annotation(&language.to_string()).as_bytes())?;
                }
            }
            SimpleValue::IriReference(v) => {
                w.write_all(ctx.hl.value_iri(&v).as_bytes())?;
            }
            SimpleValue::Binary(v) => {
                self.write_binary_value(w, v, ctx)?;
            }
        }
        Ok(())
    }

    fn write_binary_value<T, W>(
        &self,
        w: &mut W,
        value: &Binary,
        ctx: &mut Context<T>,
    ) -> Result<(), sdml_core::error::Error>
    where
        W: std::io::Write,
        T: Highlighter,
    {
        const HEX_BLOCK_WIDTH: usize = 16;
        const HEX_HALF_WIDTH: usize = HEX_BLOCK_WIDTH / 2;
        let bytes = value.as_bytes();
        let byte_count = bytes.len();

        w.write_all(format!("{} ", ctx.binary_start()).as_bytes())?;
        if byte_count > HEX_HALF_WIDTH {
            ctx.indent();
            w.write_all(format!("\n{}", ctx.indentation_str()).as_bytes())?;
        }
        for (i, byte) in bytes.iter().enumerate() {
            let i = i + 1;
            w.write_all(format!("{byte:02X}").as_bytes())?;

            if (i % HEX_HALF_WIDTH == 0) && !(i % HEX_BLOCK_WIDTH == 0) {
                w.write_all("   ".as_bytes())?;
            } else if i % HEX_BLOCK_WIDTH == 0 {
                w.write_all(format!("\n{}", ctx.indentation_str()).as_bytes())?;
            } else {
                w.write_all(SPACE_STR.as_bytes())?;
            }
        }

        if byte_count > HEX_HALF_WIDTH {
            ctx.outdent();
            w.write_all(format!("\n{}", ctx.indentation_str()).as_bytes())?;
        }
        w.write_all(ctx.binary_end().as_bytes())?;

        Ok(())
    }

    fn write_sequence_member<T, W>(
        &self,
        w: &mut W,
        value: &SequenceMember,
        ctx: &mut Context<T>,
    ) -> Result<(), sdml_core::error::Error>
    where
        W: std::io::Write,
        T: Highlighter,
    {
        match value {
            SequenceMember::Simple(v) => {
                self.write_simple_value(w, v, ctx)?;
            }
            SequenceMember::ValueConstructor(v) => {
                self.write_value_constructor(w, v, ctx)?;
            }
            SequenceMember::Reference(v) => {
                w.write_all(ctx.hl.type_ref(&v.to_string()).as_bytes())?;
            }
            SequenceMember::Mapping(v) => {
                self.write_mapping_value(w, v, ctx)?;
            }
        }

        Ok(())
    }

    // --------------------------------------------------------------------------------------------
    // Writer ❱ Definitions
    // --------------------------------------------------------------------------------------------

    fn write_module_definitions<T, W>(
        &self,
        w: &mut W,
        module: &Module,
        ctx: &mut Context<T>,
    ) -> Result<(), sdml_core::error::Error>
    where
        W: std::io::Write,
        T: Highlighter,
    {
        if module.has_definitions() {
            for definition in module.definitions() {
                match &definition {
                    Definition::Datatype(v) => self.write_datatype(w, v, ctx)?,
                    Definition::Dimension(v) => self.write_dimension(w, v, ctx)?,
                    Definition::Entity(v) => self.write_entity(w, v, ctx)?,
                    Definition::Enum(v) => self.write_enum(w, v, ctx)?,
                    Definition::Event(v) => self.write_event(w, v, ctx)?,
                    Definition::Property(v) => self.write_property(w, v, ctx)?,
                    Definition::Structure(v) => self.write_structure(w, v, ctx)?,
                    Definition::Rdf(v) => self.write_rdf(w, v, ctx)?,
                    Definition::TypeClass(v) => self.write_type_class(w, v, ctx)?,
                    Definition::Union(v) => self.write_union(w, v, ctx)?,
                }
                if ctx.options.whitespace_normal() {
                    w.write_all(EOL)?;
                }
            }
        }
        Ok(())
    }

    fn write_from_definition<T, W>(
        &self,
        w: &mut W,
        from_clause: &FromDefinition,
        ctx: &mut Context<T>,
    ) -> Result<(), sdml_core::error::Error>
    where
        W: std::io::Write,
        T: Highlighter,
    {
        w.write_all(
            format!(
                "{}{} {} {} {}\n",
                ctx.indentation_str(),
                ctx.kw_from(),
                ctx.hl.type_ref(&from_clause.definition().to_string()),
                ctx.kw_with(),
                match from_clause.member_count() {
                    0 => "_".to_string(),
                    1 => from_clause.members().next().unwrap().to_string(),
                    _ => {
                        format!(
                            "{} {} {}",
                            ctx.sequence_start(),
                            from_clause
                                .members()
                                .map(|m| m.to_string())
                                .collect::<Vec<String>>()
                                .join(" "),
                            ctx.sequence_end(),
                        )
                    }
                }
            )
            .as_bytes(),
        )?;
        Ok(())
    }

    // --------------------------------------------------------------------------------------------
    // Writer ❱ Definitions ❱ Datatypes
    // --------------------------------------------------------------------------------------------

    fn write_datatype<T, W>(
        &self,
        w: &mut W,
        defn: &DatatypeDef,
        ctx: &mut Context<T>,
    ) -> Result<(), sdml_core::error::Error>
    where
        W: std::io::Write,
        T: Highlighter,
    {
        w.write_all(
            format!(
                "{}{} {} {} {}{}",
                ctx.indentation_str(),
                ctx.kw_datatype(),
                ctx.hl.type_defn(defn.name().as_ref()),
                ctx.type_restrict(),
                if defn.is_opaque() {
                    format!("{} ", ctx.kw_opaque())
                } else {
                    EMPTY_STR.to_string()
                },
                ctx.hl.type_ref(&defn.base_type().to_string())
            )
            .as_bytes(),
        )?;

        if defn.has_restrictions() {
            w.write_all(format!(" {}\n", ctx.restriction_start()).as_bytes())?;
            ctx.indent();

            if ctx.options.whitespace_additional() {
                w.write_all(EOL)?;
            }
            for restriction in defn.restrictions() {
                self.write_datatype_restriction(w, restriction, ctx)?;
                if ctx.options.whitespace_maximal() {
                    w.write_all(EOL)?;
                }
            }
            if ctx.options.whitespace_additional() {
                w.write_all(EOL)?;
            }

            ctx.outdent();
            w.write_all(
                format!("{}{}\n", ctx.indentation_str(), ctx.restriction_end()).as_bytes(),
            )?;
        }

        if let Some(body) = defn.body() {
            if ctx.options.generate_definition_bodies() {
                w.write_all(format!(" {}\n", ctx.kw_is()).as_bytes())?;
                ctx.indent();

                if body.has_annotations() {
                    if ctx.options.whitespace_additional() {
                        w.write_all(EOL)?;
                    }
                    self.write_annotations(w, body.annotations(), ctx)?;
                    if ctx.options.whitespace_additional() {
                        w.write_all(EOL)?;
                    }
                }

                ctx.outdent();
                w.write_all(format!("{}{}\n", ctx.indentation_str(), ctx.kw_end()).as_bytes())?;
            } else {
                w.write_all(ELIPPSIS)?;
            }
        } else if !defn.has_restrictions() {
            w.write_all(EOL)?;
        }

        Ok(())
    }

    fn write_datatype_restriction<T, W>(
        &self,
        w: &mut W,
        restriction: &RestrictionFacet,
        ctx: &mut Context<T>,
    ) -> Result<(), sdml_core::error::Error>
    where
        W: std::io::Write,
        T: Highlighter,
    {
        w.write_all(
            format!(
                "{}{} {} {}",
                ctx.indentation_str(),
                ctx.hl.annotation_constraint(restriction.name()),
                ctx.op_assign(),
                if let Some(true) = restriction.fixed() {
                    format!("{} ", ctx.kw_fixed())
                } else {
                    String::new()
                }
            )
            .as_bytes(),
        )?;

        match restriction {
            RestrictionFacet::FractionDigits(v, _) => {
                w.write_all(format!("{}\n", ctx.hl.value_number(&v.to_string())).as_bytes())?
            }
            RestrictionFacet::TotalDigits(v, _) => {
                w.write_all(format!("{}\n", ctx.hl.value_number(&v.to_string())).as_bytes())?
            }
            RestrictionFacet::Length(v, _) => {
                w.write_all(format!("{}\n", ctx.hl.value_number(&v.to_string())).as_bytes())?
            }
            RestrictionFacet::MaxLength(v, _) => {
                w.write_all(format!("{}\n", ctx.hl.value_number(&v.to_string())).as_bytes())?
            }
            RestrictionFacet::MinLength(v, _) => {
                w.write_all(format!("{}\n", ctx.hl.value_number(&v.to_string())).as_bytes())?
            }
            RestrictionFacet::MaxExclusive(v, _) => {
                w.write_all(format!("{}\n", ctx.hl.value_number(&v.to_string())).as_bytes())?
            }
            RestrictionFacet::MinExclusive(v, _) => {
                w.write_all(format!("{}\n", ctx.hl.value_number(&v.to_string())).as_bytes())?
            }
            RestrictionFacet::MaxInclusive(v, _) => {
                w.write_all(format!("{}\n", ctx.hl.value_number(&v.to_string())).as_bytes())?
            }
            RestrictionFacet::MinInclusive(v, _) => {
                w.write_all(format!("{}\n", ctx.hl.value_number(&v.to_string())).as_bytes())?
            }
            RestrictionFacet::ExplicitTimezone(v, _) => {
                w.write_all(format!("{}\n", ctx.hl.value_number(&v.to_string())).as_bytes())?
            }
            RestrictionFacet::Pattern(vec) => {
                if vec.len() > 1 {
                    w.write_all(format!("{}\n", ctx.sequence_start()).as_bytes())?;
                    ctx.indent();
                    for v in vec {
                        w.write_all(format!("{}{:?}\n", ctx.indentation_str(), v).as_bytes())?;
                    }
                    ctx.outdent();
                    w.write_all(
                        format!("{}{}\n", ctx.indentation_str(), ctx.sequence_end()).as_bytes(),
                    )?
                } else {
                    let v = vec.first().unwrap();
                    w.write_all(format!("{:?}\n", v).as_bytes())?
                }
            }
        }

        Ok(())
    }

    // --------------------------------------------------------------------------------------------
    // Writer ❱ Definitions ❱ Dimension
    // --------------------------------------------------------------------------------------------

    fn write_dimension<T, W>(
        &self,
        w: &mut W,
        defn: &DimensionDef,
        ctx: &mut Context<T>,
    ) -> Result<(), sdml_core::error::Error>
    where
        W: std::io::Write,
        T: Highlighter,
    {
        w.write_all(
            format!(
                "{}{} {}",
                ctx.indentation_str(),
                ctx.kw_dimension(),
                ctx.hl.type_defn(defn.name().as_ref())
            )
            .as_bytes(),
        )?;

        if let Some(body) = defn.body() {
            if ctx.options.generate_definition_bodies() {
                ctx.indent();
                w.write_all(format!(" {}\n", ctx.kw_is()).as_bytes())?;
                if ctx.options.whitespace_additional() {
                    w.write_all(EOL)?;
                }

                if body.has_annotations() {
                    self.write_annotations(w, body.annotations(), ctx)?;
                    if body.has_members() {
                        w.write_all(EOL)?;
                    }
                    if ctx.options.whitespace_additional() {
                        w.write_all(EOL)?;
                    }
                }

                match body.identity() {
                    DimensionIdentity::Source(source_entity) => {
                        self.write_source_entity(w, source_entity, ctx)?
                    }
                    DimensionIdentity::Identity(member) => {
                        self.write_identity_member(w, member, ctx)?
                    }
                }
                if ctx.options.whitespace_additional() {
                    w.write_all(EOL)?;
                }

                if body.has_parents() {
                    for parent in body.parents() {
                        self.write_dimension_parent(w, parent, ctx)?;
                    }
                    if ctx.options.whitespace_additional() {
                        w.write_all(EOL)?;
                    }
                }

                if let Some(from_clause) = body.from_definition() {
                    self.write_from_definition(w, from_clause, ctx)?;
                    if ctx.options.whitespace_additional() {
                        w.write_all(EOL)?;
                    }
                }

                for member in body.members() {
                    self.write_member(w, member, ctx)?;
                    if ctx.options.whitespace_additional() {
                        w.write_all(EOL)?;
                    }
                }

                ctx.outdent();
                w.write_all(format!("{}{}\n", ctx.indentation_str(), ctx.kw_end()).as_bytes())?;
            } else {
                w.write_all(ELIPPSIS)?;
            }
        } else {
            w.write_all(EOL)?;
        }

        Ok(())
    }

    fn write_dimension_parent<T, W>(
        &self,
        w: &mut W,
        parent: &DimensionParent,
        ctx: &mut Context<T>,
    ) -> Result<(), sdml_core::error::Error>
    where
        W: std::io::Write,
        T: Highlighter,
    {
        w.write_all(
            format!(
                "{}{} {} {} {}",
                ctx.indentation_str(),
                ctx.kw_parent(),
                ctx.hl.field_ref(parent.name().as_ref()),
                ctx.type_assert(),
                parent.target_entity(),
            )
            .as_bytes(),
        )?;

        if let Some(body) = parent.body() {
            if body.has_annotations() && ctx.options.generate_member_bodies() {
                w.write_all(format!(" {}\n", ctx.kw_is()).as_bytes())?;
                ctx.indent();

                if ctx.options.whitespace_additional() {
                    w.write_all(EOL)?;
                }
                self.write_annotations(w, body.annotations(), ctx)?;
                if ctx.options.whitespace_additional() {
                    w.write_all(EOL)?;
                }

                ctx.outdent();
                w.write_all(format!("{}{}\n", ctx.indentation_str(), ctx.kw_end()).as_bytes())?;
            }
        } else {
            w.write_all(EOL)?;
        }

        Ok(())
    }

    // --------------------------------------------------------------------------------------------
    // Writer ❱ Definitions ❱ Entity
    // --------------------------------------------------------------------------------------------

    fn write_entity<T, W>(
        &self,
        w: &mut W,
        defn: &EntityDef,
        ctx: &mut Context<T>,
    ) -> Result<(), sdml_core::error::Error>
    where
        W: std::io::Write,
        T: Highlighter,
    {
        w.write_all(
            format!(
                "{}{} {}",
                ctx.indentation_str(),
                ctx.kw_entity(),
                ctx.hl.type_defn(defn.name().as_ref())
            )
            .as_bytes(),
        )?;

        if let Some(body) = defn.body() {
            if ctx.options.generate_definition_bodies() {
                ctx.indent();
                w.write_all(format!(" {}\n", ctx.kw_is()).as_bytes())?;
                if ctx.options.whitespace_additional() {
                    w.write_all(EOL)?;
                }

                if body.has_annotations() {
                    self.write_annotations(w, body.annotations(), ctx)?;
                    if ctx.options.whitespace_additional() {
                        w.write_all(EOL)?;
                    }
                }

                self.write_identity_member(w, body.identity(), ctx)?;
                if ctx.options.whitespace_additional() {
                    w.write_all(EOL)?;
                }

                if let Some(from_clause) = body.from_definition() {
                    self.write_from_definition(w, from_clause, ctx)?;
                    if ctx.options.whitespace_additional() {
                        w.write_all(EOL)?;
                    }
                }

                for member in body.members() {
                    self.write_member(w, member, ctx)?;
                    if ctx.options.whitespace_additional() {
                        w.write_all(EOL)?;
                    }
                }

                ctx.outdent();
                w.write_all(format!("{}{}\n", ctx.indentation_str(), ctx.kw_end()).as_bytes())?;
            } else {
                w.write_all(ELIPPSIS)?;
            }
        } else {
            w.write_all(EOL)?;
        }

        Ok(())
    }

    fn write_identity_member<T, W>(
        &self,
        w: &mut W,
        identity: &Member,
        ctx: &mut Context<T>,
    ) -> Result<(), sdml_core::error::Error>
    where
        W: std::io::Write,
        T: Highlighter,
    {
        w.write_all(format!("{}{} ", ctx.indentation_str(), ctx.kw_identity()).as_bytes())?;
        match identity.kind() {
            MemberKind::Reference(identifier_reference) => {
                self.write_member_reference_inner(w, identifier_reference, ctx)?
            }
            MemberKind::Definition(member_def) => {
                self.write_member_definition_inner(w, member_def, ctx)?
            }
        }
        Ok(())
    }

    // --------------------------------------------------------------------------------------------
    // Writer ❱ Definitions ❱ Enum
    // --------------------------------------------------------------------------------------------

    fn write_enum<T, W>(
        &self,
        w: &mut W,
        defn: &EnumDef,
        ctx: &mut Context<T>,
    ) -> Result<(), sdml_core::error::Error>
    where
        W: std::io::Write,
        T: Highlighter,
    {
        w.write_all(
            format!(
                "{}{} {}",
                ctx.indentation_str(),
                ctx.kw_enum(),
                ctx.hl.type_defn(defn.name().as_ref())
            )
            .as_bytes(),
        )?;

        if let Some(body) = defn.body() {
            if ctx.options.generate_definition_bodies() {
                ctx.indent();
                w.write_all(format!(" {}\n", ctx.kw_of()).as_bytes())?;
                if ctx.options.whitespace_additional() {
                    w.write_all(EOL)?;
                }

                if body.has_annotations() {
                    self.write_annotations(w, body.annotations(), ctx)?;
                    if ctx.options.whitespace_additional() {
                        w.write_all(EOL)?;
                    }
                }

                if let Some(from_clause) = body.from_definition() {
                    self.write_from_definition(w, from_clause, ctx)?;
                    if ctx.options.whitespace_additional() {
                        w.write_all(EOL)?;
                    }
                }

                for variant in body.variants() {
                    self.write_value_variant(w, variant, ctx)?;
                    if ctx.options.whitespace_additional() {
                        w.write_all(EOL)?;
                    }
                }

                ctx.outdent();
                w.write_all(format!("{}{}\n", ctx.indentation_str(), ctx.kw_end()).as_bytes())?;
            } else {
                w.write_all(ELIPPSIS)?;
            }
        } else {
            w.write_all(EOL)?;
        }

        Ok(())
    }

    fn write_value_variant<T, W>(
        &self,
        w: &mut W,
        variant: &ValueVariant,
        ctx: &mut Context<T>,
    ) -> Result<(), sdml_core::error::Error>
    where
        W: std::io::Write,
        T: Highlighter,
    {
        w.write_all(
            format!(
                "{}{}",
                ctx.indentation_str(),
                ctx.hl.value(variant.name().as_ref())
            )
            .as_bytes(),
        )?;

        if let Some(body) = variant.body() {
            if ctx.options.generate_member_bodies() {
                ctx.indent();
                w.write_all(format!(" {}\n", ctx.kw_is()).as_bytes())?;

                if body.has_annotations() {
                    if ctx.options.whitespace_additional() {
                        w.write_all(EOL)?;
                    }
                    self.write_annotations(w, body.annotations(), ctx)?;
                    if ctx.options.whitespace_additional() {
                        w.write_all(EOL)?;
                    }
                }

                ctx.outdent();
                w.write_all(format!("{}{}\n", ctx.indentation_str(), ctx.kw_end()).as_bytes())?;
            } else {
                w.write_all(ELIPPSIS)?;
            }
        } else {
            w.write_all(EOL)?;
        }

        Ok(())
    }

    // --------------------------------------------------------------------------------------------
    // Writer ❱ Definitions ❱ Event
    // --------------------------------------------------------------------------------------------

    fn write_event<T, W>(
        &self,
        w: &mut W,
        defn: &EventDef,
        ctx: &mut Context<T>,
    ) -> Result<(), sdml_core::error::Error>
    where
        W: std::io::Write,
        T: Highlighter,
    {
        w.write_all(
            format!(
                "{}{} {}",
                ctx.indentation_str(),
                ctx.kw_event(),
                ctx.hl.type_defn(defn.name().as_ref()),
            )
            .as_bytes(),
        )?;

        if let Some(body) = defn.body() {
            if ctx.options.generate_definition_bodies() {
                ctx.indent();
                w.write_all(format!(" {}\n", ctx.kw_is()).as_bytes())?;
                if ctx.options.whitespace_additional() {
                    w.write_all(EOL)?;
                }

                if body.has_annotations() {
                    self.write_annotations(w, body.annotations(), ctx)?;
                    if ctx.options.whitespace_additional() {
                        w.write_all(EOL)?;
                    }
                }

                self.write_source_entity(w, body.source_entity(), ctx)?;
                if ctx.options.whitespace_additional() {
                    w.write_all(EOL)?;
                }

                if let Some(from_clause) = body.from_definition() {
                    self.write_from_definition(w, from_clause, ctx)?;
                    if ctx.options.whitespace_additional() {
                        w.write_all(EOL)?;
                    }
                }

                for member in body.members() {
                    self.write_member(w, member, ctx)?;
                    if ctx.options.whitespace_additional() {
                        w.write_all(EOL)?;
                    }
                }

                ctx.outdent();
                w.write_all(format!("{}{}\n", ctx.indentation_str(), ctx.kw_end()).as_bytes())?;
            } else {
                w.write_all(ELIPPSIS)?;
            }
        } else {
            w.write_all(EOL)?;
        }

        Ok(())
    }

    fn write_source_entity<T, W>(
        &self,
        w: &mut W,
        source: &SourceEntity,
        ctx: &mut Context<T>,
    ) -> Result<(), sdml_core::error::Error>
    where
        W: std::io::Write,
        T: Highlighter,
    {
        w.write_all(
            format!(
                "{}{} {}",
                ctx.indentation_str(),
                ctx.kw_source(),
                ctx.hl.type_ref(&source.target_entity().to_string())
            )
            .as_bytes(),
        )?;
        if source.has_members() {
            w.write_all(format!(" {} ", ctx.kw_with()).as_bytes())?;
            if source.member_count() == 1 {
                w.write_all(format!("{}", source.members().next().unwrap()).as_bytes())?;
            } else {
                w.write_all(
                    format!(
                        "{} {} {}",
                        ctx.sequence_start(),
                        source
                            .members()
                            .map(|id| id.to_string())
                            .collect::<Vec<String>>()
                            .join(" "),
                        ctx.sequence_end(),
                    )
                    .as_bytes(),
                )?;
            }
        }
        w.write_all(EOL)?;

        Ok(())
    }

    // --------------------------------------------------------------------------------------------
    // Writer ❱ Definitions ❱ Member (Dimension, Entity, Event, Property, Structure)
    // --------------------------------------------------------------------------------------------

    fn write_member<T, W>(
        &self,
        w: &mut W,
        defn: &Member,
        ctx: &mut Context<T>,
    ) -> Result<(), sdml_core::error::Error>
    where
        W: std::io::Write,
        T: Highlighter,
    {
        match defn.kind() {
            MemberKind::Reference(v) => self.write_member_reference(w, v, ctx),
            MemberKind::Definition(v) => self.write_member_definition(w, v, ctx),
        }
    }

    fn write_member_definition<T, W>(
        &self,
        w: &mut W,
        defn: &MemberDef,
        ctx: &mut Context<T>,
    ) -> Result<(), sdml_core::error::Error>
    where
        W: std::io::Write,
        T: Highlighter,
    {
        w.write_all(ctx.indentation_str().as_bytes())?;
        self.write_member_definition_inner(w, defn, ctx)
    }

    fn write_member_definition_inner<T, W>(
        &self,
        w: &mut W,
        defn: &MemberDef,
        ctx: &mut Context<T>,
    ) -> Result<(), sdml_core::error::Error>
    where
        W: std::io::Write,
        T: Highlighter,
    {
        w.write_all(
            format!(
                "{} {} ",
                ctx.hl.field_defn(defn.name().as_ref()),
                ctx.type_assert()
            )
            .as_bytes(),
        )?;
        if *defn.target_cardinality() != DEFAULT_CARDINALITY {
            self.write_cardinality(w, defn.target_cardinality(), ctx)?;
        }
        self.write_type_reference(w, defn.target_type(), ctx)?;
        if let Some(body) = defn.body() {
            if body.has_annotations() && ctx.options.generate_member_bodies() {
                w.write_all(format!(" {}\n", ctx.kw_is()).as_bytes())?;
                ctx.indent();

                if ctx.options.whitespace_additional() {
                    w.write_all(EOL)?;
                }
                self.write_annotations(w, body.annotations(), ctx)?;
                if ctx.options.whitespace_additional() {
                    w.write_all(EOL)?;
                }

                ctx.outdent();
                w.write_all(format!("{}{}\n", ctx.indentation_str(), ctx.kw_end()).as_bytes())?;
            } else {
                w.write_all(ELIPPSIS)?;
            }
        } else {
            w.write_all(EOL)?;
        }
        Ok(())
    }

    fn write_member_reference<T, W>(
        &self,
        w: &mut W,
        defn: &IdentifierReference,
        ctx: &mut Context<T>,
    ) -> Result<(), sdml_core::error::Error>
    where
        W: std::io::Write,
        T: Highlighter,
    {
        w.write_all(ctx.indentation_str().as_bytes())?;
        self.write_member_reference_inner(w, defn, ctx)
    }

    fn write_member_reference_inner<T, W>(
        &self,
        w: &mut W,
        defn: &IdentifierReference,
        ctx: &mut Context<T>,
    ) -> Result<(), sdml_core::error::Error>
    where
        W: std::io::Write,
        T: Highlighter,
    {
        w.write_all(format!("{} {}\n", ctx.kw_ref(), defn).as_bytes())?;
        Ok(())
    }

    fn write_cardinality<T, W>(
        &self,
        w: &mut W,
        defn: &Cardinality,
        ctx: &mut Context<T>,
    ) -> Result<(), sdml_core::error::Error>
    where
        W: std::io::Write,
        T: Highlighter,
    {
        w.write_all(
            format!(
                "{}{}{}{}{}{} ",
                ctx.restriction_start(),
                if let Some(uniqueness) = defn.uniqueness() {
                    format!("{} ", ctx.hl.keyword(&uniqueness.to_string()))
                } else {
                    String::new()
                },
                if let Some(ordering) = defn.ordering() {
                    format!("{} ", ctx.hl.keyword(&ordering.to_string()))
                } else {
                    String::new()
                },
                defn.min_occurs(),
                if let Some(max_occurs) = defn.max_occurs() {
                    if max_occurs == defn.min_occurs() {
                        String::new()
                    } else {
                        format!("{}{}", ctx.range(), max_occurs)
                    }
                } else {
                    ctx.range()
                },
                ctx.restriction_end()
            )
            .as_bytes(),
        )?;
        Ok(())
    }

    #[allow(clippy::only_used_in_recursion)]
    fn write_type_reference<T, W>(
        &self,
        w: &mut W,
        defn: &TypeReference,
        ctx: &mut Context<T>,
    ) -> Result<(), sdml_core::error::Error>
    where
        W: std::io::Write,
        T: Highlighter,
    {
        match defn {
            TypeReference::Unknown => {
                w.write_all(ctx.kw_unknown().as_bytes())?;
            }
            TypeReference::Type(name_ref) => {
                w.write_all(ctx.hl.type_ref(&name_ref.to_string()).as_bytes())?;
            }
            TypeReference::MappingType(map_ref) => {
                w.write_all(ctx.left_paren().as_bytes())?;
                self.write_type_reference(w, map_ref.domain(), ctx)?;
                w.write_all(format!(" {} ", ctx.type_assert()).as_bytes())?;
                self.write_type_reference(w, map_ref.range(), ctx)?;
                w.write_all(ctx.right_paren().as_bytes())?;
            }
        }
        Ok(())
    }

    // --------------------------------------------------------------------------------------------
    // Writer ❱ Definitions ❱ Property
    // --------------------------------------------------------------------------------------------

    fn write_property<T, W>(
        &self,
        w: &mut W,
        defn: &PropertyDef,
        ctx: &mut Context<T>,
    ) -> Result<(), sdml_core::error::Error>
    where
        W: std::io::Write,
        T: Highlighter,
    {
        w.write_all(format!("{}{} ", ctx.indentation_str(), ctx.kw_property(),).as_bytes())?;

        self.write_member_definition_inner(w, defn.member_def(), ctx)
    }

    // --------------------------------------------------------------------------------------------
    // Writer ❱ Definitions ❱ RDF
    // --------------------------------------------------------------------------------------------

    fn write_rdf<T, W>(
        &self,
        w: &mut W,
        defn: &RdfDef,
        ctx: &mut Context<T>,
    ) -> Result<(), sdml_core::error::Error>
    where
        W: std::io::Write,
        T: Highlighter,
    {
        w.write_all(
            format!(
                "{}{} {}",
                ctx.indentation_str(),
                ctx.kw_rdf(),
                ctx.hl.type_defn(defn.name().as_ref())
            )
            .as_bytes(),
        )?;

        if let Some(body) = defn.body() {
            if ctx.options.generate_definition_bodies() {
                // 1. partition the annotations
                let annotations = if body.has_annotations() {
                    let (mut types, mut others): (Vec<&Annotation>, Vec<&Annotation>) =
                        body.annotations().partition(|an| {
                            if let Some(prop) = an.as_annotation_property() {
                                prop.is_rdf_type()
                            } else {
                                false
                            }
                        });
                    if !types.is_empty() {
                        w.write_all(
                            format!(
                                " {} ",
                                if ctx.options.operator_form == OperatorForm::Textual {
                                    ctx.kw_type()
                                } else {
                                    ctx.kw_a()
                                }
                            )
                            .as_bytes(),
                        )?;
                        if types.len() == 1 {
                            self.write_annotation_as_type_reference(
                                w,
                                types.remove(0),
                                &mut others,
                            )?;
                        } else {
                            w.write_all(format!("{} ", ctx.sequence_start()).as_bytes())?;
                            for a_type in types {
                                self.write_annotation_as_type_reference(w, a_type, &mut others)?;
                                w.write_all(SPACE)?;
                            }
                            w.write_all(ctx.sequence_end().as_bytes())?;
                        }
                    }
                    others
                } else {
                    Vec::default()
                };
                w.write_all(format!(" {}\n", ctx.kw_is()).as_bytes())?;
                ctx.indent();

                if !annotations.is_empty() {
                    if ctx.options.whitespace_additional() {
                        w.write_all(EOL)?;
                    }

                    self.write_annotations(w, annotations.into_iter(), ctx)?;
                    if ctx.options.whitespace_additional() {
                        w.write_all(EOL)?;
                    }
                }

                ctx.outdent();
                w.write_all(format!("{}{}\n", ctx.indentation_str(), ctx.kw_end()).as_bytes())?;
            } else {
                w.write_all(ELIPPSIS)?;
            }
        } else {
            w.write_all(EOL)?;
        }

        Ok(())
    }

    fn write_annotation_as_type_reference<'a, W>(
        &self,
        w: &mut W,
        annotation: &'a Annotation,
        others: &mut Vec<&'a Annotation>,
    ) -> Result<(), sdml_core::error::Error>
    where
        W: std::io::Write,
    {
        if let Some(property) = annotation.as_annotation_property() {
            if let Some(type_ref) = property.value().as_reference() {
                w.write_all(type_ref.to_string().as_bytes())?;
            } else {
                // TODO: handle as_iri() case
                others.push(annotation);
            }
        } else {
            others.push(annotation);
        }
        Ok(())
    }

    // --------------------------------------------------------------------------------------------
    // Writer ❱ Definitions ❱ Structure
    // --------------------------------------------------------------------------------------------

    fn write_structure<T, W>(
        &self,
        w: &mut W,
        defn: &StructureDef,
        ctx: &mut Context<T>,
    ) -> Result<(), sdml_core::error::Error>
    where
        W: std::io::Write,
        T: Highlighter,
    {
        w.write_all(
            format!(
                "{}{} {}",
                ctx.indentation_str(),
                ctx.kw_structure(),
                ctx.hl.type_defn(defn.name().as_ref())
            )
            .as_bytes(),
        )?;

        if let Some(body) = defn.body() {
            if ctx.options.generate_definition_bodies() {
                ctx.indent();
                w.write_all(format!(" {}\n", ctx.kw_is()).as_bytes())?;
                if ctx.options.whitespace_additional() {
                    w.write_all(EOL)?;
                }

                if body.has_annotations() {
                    self.write_annotations(w, body.annotations(), ctx)?;
                    if ctx.options.whitespace_additional() {
                        w.write_all(EOL)?;
                    }
                }

                if let Some(from_clause) = body.from_definition() {
                    self.write_from_definition(w, from_clause, ctx)?;
                    if ctx.options.whitespace_additional() {
                        w.write_all(EOL)?;
                    }
                }

                for member in body.members() {
                    self.write_member(w, member, ctx)?;
                    if ctx.options.whitespace_additional() {
                        w.write_all(EOL)?;
                    }
                }

                ctx.outdent();
                w.write_all(format!("{}{}\n", ctx.indentation_str(), ctx.kw_end()).as_bytes())?;
            } else {
                w.write_all(ELIPPSIS)?;
            }
        } else {
            w.write_all(EOL)?;
        }

        Ok(())
    }

    // --------------------------------------------------------------------------------------------
    // Writer ❱ Definitions ❱ Type Class
    // --------------------------------------------------------------------------------------------

    fn write_type_class<T, W>(
        &self,
        w: &mut W,
        defn: &TypeClassDef,
        ctx: &mut Context<T>,
    ) -> Result<(), sdml_core::error::Error>
    where
        W: std::io::Write,
        T: Highlighter,
    {
        w.write_all(
            format!(
                "{}{} {}",
                ctx.indentation_str(),
                ctx.kw_class(),
                ctx.hl.type_defn(defn.name().as_ref())
            )
            .as_bytes(),
        )?;

        if defn.has_variables() {
            w.write_all(format!(" {}", ctx.left_paren()).as_bytes())?;
            for variable in defn.variables() {
                self.write_type_variable(w, variable, ctx)?;
            }
            w.write_all(ctx.right_paren().as_bytes())?;
        }

        if let Some(body) = defn.body() {
            if ctx.options.generate_definition_bodies() {
                ctx.indent();
                w.write_all(format!(" {}\n", ctx.kw_is()).as_bytes())?;
                if ctx.options.whitespace_additional() {
                    w.write_all(EOL)?;
                }

                if body.has_annotations() {
                    self.write_annotations(w, body.annotations(), ctx)?;
                    if ctx.options.whitespace_additional() {
                        w.write_all(EOL)?;
                    }
                }

                if let Some(from_clause) = body.from_definition() {
                    self.write_from_definition(w, from_clause, ctx)?;
                    if ctx.options.whitespace_additional() {
                        w.write_all(EOL)?;
                    }
                }

                for method in body.methods() {
                    self.write_type_class_method(w, method, ctx)?;
                    if ctx.options.whitespace_additional() {
                        w.write_all(EOL)?;
                    }
                }

                ctx.outdent();
                w.write_all(format!("{}{}\n", ctx.indentation_str(), ctx.kw_end()).as_bytes())?;
            } else {
                w.write_all(ELIPPSIS)?;
            }
        } else {
            w.write_all(EOL)?;
        }

        Ok(())
    }

    fn write_type_variable<T, W>(
        &self,
        w: &mut W,
        variable: &TypeVariable,
        ctx: &mut Context<T>,
    ) -> Result<(), sdml_core::error::Error>
    where
        W: std::io::Write,
        T: Highlighter,
    {
        if let Some(cardinality) = variable.cardinality() {
            self.write_function_cardinality(w, cardinality, ctx)?;
            w.write_all(SPACE_STR.as_bytes())?;
        }

        w.write_all(format!("{}", variable.name()).as_bytes())?;

        if variable.has_restrictions() {
            w.write_all(format!(" {} ", ctx.type_assert()).as_bytes())?;

            let last = variable.restrictions_len() - 1;
            for (i, class_ref) in variable.restrictions().enumerate() {
                self.write_type_class_reference(w, class_ref, ctx)?;
                if i < last {
                    w.write_all(format!(" {} ", ctx.type_combine()).as_bytes())?;
                }
            }
        }

        Ok(())
    }

    fn write_type_class_reference<T, W>(
        &self,
        w: &mut W,
        class_ref: &TypeClassReference,
        ctx: &mut Context<T>,
    ) -> Result<(), sdml_core::error::Error>
    where
        W: std::io::Write,
        T: Highlighter,
    {
        w.write_all(class_ref.name().to_string().as_bytes())?;

        if class_ref.has_arguments() {
            w.write_all(ctx.left_paren().as_bytes())?;

            let last = class_ref.arguments_len() - 1;
            for (i, arg) in class_ref.arguments().enumerate() {
                match arg {
                    TypeClassArgument::Wildcard => w.write_all(ctx.kw_wildcard().as_bytes())?,
                    TypeClassArgument::Reference(v) => {
                        self.write_type_class_reference(w, v, ctx)?
                    }
                }
                if i < last {
                    w.write_all(SPACE_STR.as_bytes())?;
                }
            }

            w.write_all(ctx.right_paren().as_bytes())?;
        }
        Ok(())
    }

    fn write_type_class_method<T, W>(
        &self,
        w: &mut W,
        method: &MethodDef,
        ctx: &mut Context<T>,
    ) -> Result<(), sdml_core::error::Error>
    where
        W: std::io::Write,
        T: Highlighter,
    {
        self.write_function_signature(w, method.signature(), ctx)?;

        if let Some(body) = method.body() {
            w.write_all(format!(" {}\n", ctx.by_defn(),).as_bytes())?;
            self.write_function_body(w, body, ctx)?;
        }

        if method.has_annotations() {
            if method.has_body() {
                w.write_all(format!("\n{}{}\n", ctx.indentation_str(), ctx.kw_is()).as_bytes())?;
            } else {
                w.write_all(format!(" {}\n", ctx.kw_is()).as_bytes())?;
            }
            ctx.indent();

            if ctx.options.whitespace_additional() {
                w.write_all(EOL)?;
            }
            self.write_annotations(w, method.annotations(), ctx)?;
            if ctx.options.whitespace_additional() {
                w.write_all(EOL)?;
            }

            ctx.outdent();
            w.write_all(format!("{}{}", ctx.indentation_str(), ctx.kw_end()).as_bytes())?;
        }

        w.write_all(EOL)?;
        Ok(())
    }

    // --------------------------------------------------------------------------------------------
    // Writer ❱ Definitions ❱ Union
    // --------------------------------------------------------------------------------------------

    fn write_union<T, W>(
        &self,
        w: &mut W,
        defn: &UnionDef,
        ctx: &mut Context<T>,
    ) -> Result<(), sdml_core::error::Error>
    where
        W: std::io::Write,
        T: Highlighter,
    {
        w.write_all(
            format!(
                "{}{} {}",
                ctx.indentation_str(),
                ctx.kw_union(),
                ctx.hl.type_defn(defn.name().as_ref())
            )
            .as_bytes(),
        )?;

        if let Some(body) = defn.body() {
            if ctx.options.generate_definition_bodies() {
                ctx.indent();
                w.write_all(format!(" {}\n", ctx.kw_of()).as_bytes())?;
                if ctx.options.whitespace_additional() {
                    w.write_all(EOL)?;
                }

                if body.has_annotations() {
                    self.write_annotations(w, body.annotations(), ctx)?;
                    if ctx.options.whitespace_additional() {
                        w.write_all(EOL)?;
                    }
                }

                if let Some(from_clause) = body.from_definition() {
                    self.write_from_definition(w, from_clause, ctx)?;
                    if ctx.options.whitespace_additional() {
                        w.write_all(EOL)?;
                    }
                }

                if body.has_variants() {
                    for variant in body.variants() {
                        self.write_type_variant(w, variant, ctx)?;
                        if ctx.options.whitespace_additional() {
                            w.write_all(EOL)?;
                        }
                    }
                }

                ctx.outdent();
                w.write_all(format!("{}{}\n", ctx.indentation_str(), ctx.kw_end()).as_bytes())?;
            } else {
                w.write_all(ELIPPSIS)?;
            }
        } else {
            w.write_all(EOL)?;
        }

        Ok(())
    }

    fn write_type_variant<T, W>(
        &self,
        w: &mut W,
        variant: &TypeVariant,
        ctx: &mut Context<T>,
    ) -> Result<(), sdml_core::error::Error>
    where
        W: std::io::Write,
        T: Highlighter,
    {
        if let Some(rename) = variant.rename() {
            w.write_all(
                format!(
                    "{}{}",
                    ctx.indentation_str(),
                    ctx.hl.type_ref(&variant.name_reference().to_string())
                )
                .as_bytes(),
            )?;
            w.write_all(
                format!(" {} {}", ctx.kw_as(), ctx.hl.type_defn(rename.as_ref())).as_bytes(),
            )?;
        } else {
            w.write_all(
                format!(
                    "{}{}",
                    ctx.indentation_str(),
                    ctx.hl.type_ref(&variant.name_reference().to_string())
                )
                .as_bytes(),
            )?;
        }

        if let Some(body) = variant.body() {
            if ctx.options.generate_variant_bodies() {
                w.write_all(format!(" {}\n", ctx.kw_is()).as_bytes())?;
                ctx.indent();

                if body.has_annotations() {
                    if ctx.options.whitespace_additional() {
                        w.write_all(EOL)?;
                    }
                    self.write_annotations(w, body.annotations(), ctx)?;
                    if ctx.options.whitespace_additional() {
                        w.write_all(EOL)?;
                    }
                }

                ctx.outdent();
                w.write_all(format!("{}{}\n", ctx.indentation_str(), ctx.kw_end()).as_bytes())?;
            } else {
                w.write_all(ELIPPSIS)?;
            }
        } else {
            w.write_all(EOL)?;
        }

        Ok(())
    }
}
