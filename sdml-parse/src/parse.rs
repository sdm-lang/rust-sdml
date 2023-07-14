/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::error::{
    ErrorCounters, FileId, MEMBER_ALREADY_IMPORTED, MEMBER_NAME_USED, MODULE_ALREADY_IMPORTED,
    TYPE_DEFINITION_NAME_USED, UNEXPECTED_NODE_KIND,
};
use crate::load::ModuleLoader;
use codespan_reporting::diagnostic::Label;
use codespan_reporting::term;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use codespan_reporting::term::Chars;
use rust_decimal::Decimal;
use sdml_core::error::{invalid_value_for_type, module_parse_error, unexpected_node_kind, Error};
use sdml_core::model::{
    Annotation, AnnotationOnlyBody, AnnotationProperty, ByReferenceMember, ByReferenceMemberDef,
    ByValueMember, ByValueMemberDef, Cardinality, Constraint, DatatypeDef, EntityBody, EntityDef,
    EntityGroup, EnumBody, EnumDef, ValueVariant, EventDef, Identifier, IdentifierReference,
    IdentityMember, IdentityMemberDef, Import, ImportStatement, LanguageString, LanguageTag,
    ListOfValues, Module, ModuleBody, PropertyBody, PropertyDef, PropertyRole, QualifiedIdentifier,
    SimpleValue, StructureBody, StructureDef, StructureGroup, Definition, TypeReference,
    TypeVariant, UnionBody, UnionDef, Value, ValueConstructor,
};
use sdml_core::syntax::{
    FIELD_NAME_BASE, FIELD_NAME_BODY, FIELD_NAME_IDENTITY, FIELD_NAME_MAX, FIELD_NAME_MEMBER,
    FIELD_NAME_MIN, FIELD_NAME_MODULE, FIELD_NAME_NAME, FIELD_NAME_RENAME, FIELD_NAME_ROLE,
    FIELD_NAME_SOURCE, FIELD_NAME_SOURCE_CARDINALITY, FIELD_NAME_TARGET,
    FIELD_NAME_TARGET_CARDINALITY, FIELD_NAME_VALUE, NAME_SDML, NODE_KIND_ANNOTATION,
    NODE_KIND_ANNOTATION_PROPERTY, NODE_KIND_BOOLEAN, NODE_KIND_BUILTIN_SIMPLE_TYPE,
    NODE_KIND_CONSTRAINT, NODE_KIND_DATA_TYPE_DEF, NODE_KIND_DECIMAL, NODE_KIND_DOUBLE,
    NODE_KIND_ENTITY_DEF, NODE_KIND_ENTITY_GROUP, NODE_KIND_ENUM_DEF, NODE_KIND_VALUE_VARIANT,
    NODE_KIND_EVENT_DEF, NODE_KIND_FORMAL_CONSTRAINT, NODE_KIND_IDENTIFIER,
    NODE_KIND_IDENTIFIER_REFERENCE, NODE_KIND_IDENTITY_MEMBER, NODE_KIND_IMPORT,
    NODE_KIND_IMPORT_STATEMENT, NODE_KIND_INFORMAL_CONSTRAINT, NODE_KIND_INTEGER,
    NODE_KIND_IRI_REFERENCE, NODE_KIND_LANGUAGE_TAG, NODE_KIND_LINE_COMMENT,
    NODE_KIND_LIST_OF_VALUES, NODE_KIND_MEMBER_BY_REFERENCE, NODE_KIND_MEMBER_BY_VALUE,
    NODE_KIND_MEMBER_IMPORT, NODE_KIND_MODULE, NODE_KIND_MODULE_IMPORT, NODE_KIND_PROPERTY_DEF,
    NODE_KIND_PROPERTY_ROLE, NODE_KIND_QUALIFIED_IDENTIFIER, NODE_KIND_QUOTED_STRING,
    NODE_KIND_SIMPLE_VALUE, NODE_KIND_STRING, NODE_KIND_STRUCTURE_DEF, NODE_KIND_STRUCTURE_GROUP,
    NODE_KIND_STRUCTURE_MEMBER, NODE_KIND_DEFINITION, NODE_KIND_TYPE_VARIANT, NODE_KIND_UNION_DEF,
    NODE_KIND_UNKNOWN_TYPE, NODE_KIND_UNSIGNED, NODE_KIND_VALUE_CONSTRUCTOR,
};
use std::collections::HashSet;
use std::str::FromStr;
use tracing::{error,trace};
use tree_sitter::Parser;
use tree_sitter::{Node, TreeCursor};
use tree_sitter_sdml::language;
use url::Url;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub(crate) struct Parsed {
    module: Module,
    counters: ErrorCounters,
}

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

macro_rules! emit_diagnostic {
    ($files: expr, $diagnostic: expr) => {
        // TODO: parameterize this ---------vvvvvvvvvvvvvvvvvvv
        let writer = StandardStream::stderr(ColorChoice::Always);
        let mut config = codespan_reporting::term::Config::default();
        config.chars = Chars::box_drawing();
        emit_diagnostic!($files, $diagnostic, &config => writer);
    };
    // ($files: expr, $diagnostic: expr => $writer: expr) => {
    //     let config = codespan_reporting::term::Config::default();
    //     emit_diagnostic!($files, $diagnostic, &config => $writer);
    // };
    // ($files: expr, $diagnostic: expr, $config: expr) => {
    //     let writer = StandardStream::stderr(ColorChoice::Always);
    //     emit_diagnostic!($files, $diagnostic, $config => writer);
    // };
    ($files: expr, $diagnostic: expr, $config: expr => $writer: expr) => {
        term::emit(&mut $writer.lock(), &$config, $files, &$diagnostic)?
    };
}

macro_rules! unexpected_node {
    ($context: expr, $parse_fn: expr, $node: expr, [ $($expected: expr, )+ ]) => {
        let diagnostic = UNEXPECTED_NODE_KIND.into_diagnostic()
            .with_labels(vec![
                Label::primary($context.file_id, $node.start_byte()..$node.end_byte())
                    .with_message(message_expecting_one_of_node(&[
                        $($expected, )+
                    ])),
                Label::secondary($context.file_id, $node.start_byte()..$node.end_byte())
                    .with_message(message_found_node($node.kind())),
                ]);

        $context.counts.report(diagnostic.severity);
        emit_diagnostic!($context.loader.files(), diagnostic);

        return Err(unexpected_node_kind(
            $parse_fn,
            [
                $($expected, )+
            ].join(" | "),
            $node.kind(),
            $node.into(),
        ))
    };
    ($context: expr, $parse_fn: expr, $node: expr, $expected: expr) => {
        let diagnostic = UNEXPECTED_NODE_KIND.into_diagnostic()
            .with_labels(vec![
                Label::primary($context.file_id, $node.start_byte()..$node.end_byte())
                    .with_message(message_expecting_node($expected)),
                Label::secondary($context.file_id, $node.start_byte()..$node.end_byte())
                    .with_message(message_found_node($node.kind())),
                ]);

        $context.counts.report(diagnostic.severity);
        emit_diagnostic!($context.loader.files(), diagnostic);

        return Err(unexpected_node_kind(
            $parse_fn,
            $expected,
            $node.kind(),
            $node.into(),
        ))
    };
}

macro_rules! rule_fn {
    ($name:literal, $node:expr) => {
        const RULE_NAME: &str = $name;
        trace!("{}: {:?}", RULE_NAME, $node);
    };
    ($name:literal, $node:expr, $arg: expr) => {
        const RULE_NAME: &str = $name;
        trace!("{}({:?}): {:?}", RULE_NAME, $arg, $node);
    };
}

macro_rules! check_and_add_comment {
    ($context: ident, $node: ident, $parent: ident) => {
        if $context.save_comments() {
            let comment = ::sdml_core::model::Comment::new($context.node_source(&$node)?)
                .with_ts_span($node.into());
            $parent.add_to_comments(comment);
        } else {
            trace!("not saving comments");
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// This should only be called by `ModuleLoader`
pub(crate) fn parse_str(file_id: FileId, loader: &mut ModuleLoader) -> Result<Parsed, Error> {
    let source = loader.files().get(file_id).unwrap().source();
    let mut parser = Parser::new();
    parser
        .set_language(language())
        .expect("Error loading SDML grammar");
    let tree = parser.parse(source, None).unwrap();

    let node = tree.root_node();
    let mut context = ParseContext::new(file_id, loader);

    if node.kind() == NODE_KIND_MODULE {
        let mut cursor = tree.walk();
        let module = parse_module(&mut context, &mut cursor)?;
        Ok(Parsed {
            module,
            counters: context.counts,
        })
    } else {
        unexpected_node!(context, "parse_str", node, NODE_KIND_MODULE);
    }
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
struct ParseContext<'a> {
    loader: &'a ModuleLoader,
    file_id: FileId,
    imports: HashSet<Import>,
    type_names: HashSet<Identifier>,
    member_names: HashSet<Identifier>,
    save_comments: bool,
    counts: ErrorCounters,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Parsed {
    pub(crate) fn into_inner(self) -> (Module, ErrorCounters) {
        (self.module, self.counters)
    }
}

// ------------------------------------------------------------------------------------------------

impl<'a> ParseContext<'a> {
    fn new(file_id: FileId, loader: &'a ModuleLoader) -> Self {
        Self {
            file_id,
            loader,
            imports: Default::default(),
            type_names: Default::default(),
            member_names: Default::default(),
            save_comments: Default::default(),
            counts: Default::default(),
        }
    }

    fn source(&self) -> &[u8] {
        self.loader
            .files()
            .get(self.file_id)
            .unwrap()
            .source()
            .as_bytes()
    }

    fn node_source(&'a self, node: &'a Node<'a>) -> Result<&'a str, Error> {
        Ok(node.utf8_text(self.source())?)
    }

    fn check_if_error(&self, node: &Node<'a>, rule: &str) -> Result<(), Error> {
        if node.is_error() {
            Err(module_parse_error(node.kind(), node.into(), Some(rule)))
        } else {
            Ok(())
        }
    }

    fn add_import(&mut self, import: &Import) -> Result<(), Error> {
        if let Some(previous) = self.imports.get(import) {
            let diagnostic = if matches!(previous, Import::Module(_)) {
                MEMBER_ALREADY_IMPORTED
            } else {
                MODULE_ALREADY_IMPORTED
            }
            .into_diagnostic()
            .with_labels(vec![
                Label::primary(self.file_id, import.ts_span().unwrap().byte_range())
                    .with_message("this module"),
                Label::secondary(self.file_id, previous.ts_span().unwrap().byte_range())
                    .with_message("was initially imported here"),
            ]);

            self.counts.report(diagnostic.severity);
            emit_diagnostic!(self.loader.files(), diagnostic);
        } else {
            self.imports.insert(import.clone());
        }
        Ok(())
    }

    fn start_type(&mut self, name: &Identifier) -> Result<(), Error> {
        if let Some(type_defn) = self.type_names.get(name) {
            let diagnostic = TYPE_DEFINITION_NAME_USED
                .into_diagnostic()
                .with_labels(vec![
                    Label::primary(self.file_id, name.ts_span().unwrap().byte_range())
                        .with_message("this type name"),
                    Label::secondary(self.file_id, type_defn.ts_span().unwrap().byte_range())
                        .with_message("was previously defined here"),
                ]);

            self.counts.report(diagnostic.severity);
            emit_diagnostic!(self.loader.files(), diagnostic);
        } else {
            self.type_names.insert(name.clone());
        }
        Ok(())
    }

    fn start_member(&mut self, name: &Identifier) -> Result<(), Error> {
        if let Some(member) = self.member_names.get(name) {
            let diagnostic = MEMBER_NAME_USED.into_diagnostic().with_labels(vec![
                Label::primary(self.file_id, name.ts_span().unwrap().byte_range())
                    .with_message("this member name"),
                Label::secondary(self.file_id, member.ts_span().unwrap().byte_range())
                    .with_message("was previously defined here"),
            ]);

            self.counts.report(diagnostic.severity);
            emit_diagnostic!(self.loader.files(), diagnostic);
        } else {
            self.member_names.insert(name.clone());
        }
        Ok(())
    }

    fn end_type(&mut self) {
        self.member_names.clear()
    }

    fn save_comments(&self) -> bool {
        self.save_comments
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn parse_module<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<Module, Error> {
    let node = cursor.node();
    rule_fn!("parse_module", node);
    context.check_if_error(&node, RULE_NAME)?;

    let child = node.child_by_field_name(FIELD_NAME_NAME).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let name = parse_identifier(context, &child)?;

    let child = node.child_by_field_name(FIELD_NAME_BODY).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let body = parse_module_body(context, &mut child.walk())?;

    Ok(Module::new(name, body).with_ts_span(node.into()))
}

fn parse_module_body<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<ModuleBody, Error> {
    rule_fn!("parse_module_body", cursor.node());
    let mut body = ModuleBody::default().with_ts_span(cursor.node().into());
    let mut has_next = cursor.goto_first_child();
    if has_next {
        while has_next {
            let node = cursor.node();
            context.check_if_error(&node, RULE_NAME)?;
            if node.is_named() {
                match node.kind() {
                    NODE_KIND_IMPORT_STATEMENT => {
                        body.add_to_imports(parse_import_statement(context, &mut node.walk())?)
                    }
                    NODE_KIND_ANNOTATION => {
                        body.add_annotation(parse_annotation(context, &mut node.walk())?)
                    }
                    NODE_KIND_DEFINITION => {
                        body.add_to_definitions(parse_type_definition(context, &mut node.walk())?)
                    }
                    NODE_KIND_LINE_COMMENT => {
                        check_and_add_comment!(context, node, body);
                    }
                    _ => {
                        unexpected_node!(
                            context,
                            RULE_NAME,
                            node,
                            [
                                NODE_KIND_IMPORT_STATEMENT,
                                NODE_KIND_ANNOTATION,
                                NODE_KIND_DEFINITION,
                                NODE_KIND_LINE_COMMENT,
                            ]
                        );
                    }
                }
            }
            has_next = cursor.goto_next_sibling();
        }
        assert!(cursor.goto_parent());
    }
    Ok(body)
}

fn parse_import_statement<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<ImportStatement, Error> {
    rule_fn!("parse_import_statement", cursor.node());
    let mut import = ImportStatement::default().with_ts_span(cursor.node().into());
    let mut has_next = cursor.goto_first_child();
    if has_next {
        while has_next {
            let node = cursor.node();
            context.check_if_error(&node, RULE_NAME)?;
            if node.is_named() {
                match node.kind() {
                    NODE_KIND_IMPORT => {
                        let imported = parse_import(context, &mut node.walk())?;
                        let _ = context.add_import(&imported);
                        import.add_to_imports(imported);
                    }
                    NODE_KIND_LINE_COMMENT => {
                        check_and_add_comment!(context, node, import);
                    }
                    _ => {
                        unexpected_node!(
                            context,
                            RULE_NAME,
                            node,
                            [NODE_KIND_IMPORT, NODE_KIND_LINE_COMMENT,]
                        );
                    }
                }
            }
            has_next = cursor.goto_next_sibling();
        }
        assert!(cursor.goto_parent());
    }
    Ok(import)
}

fn parse_import<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<Import, Error> {
    rule_fn!("parse_import", cursor.node());
    let mut has_next = cursor.goto_first_child();
    if has_next {
        while has_next {
            let node = cursor.node();
            context.check_if_error(&node, RULE_NAME)?;
            if node.is_named() {
                match node.kind() {
                    NODE_KIND_MODULE_IMPORT => {
                        let node = node.child_by_field_name(FIELD_NAME_NAME).unwrap();
                        context.check_if_error(&node, RULE_NAME)?;
                        let name = parse_identifier(context, &node)?;
                        return Ok(name.into());
                    }
                    NODE_KIND_MEMBER_IMPORT => {
                        let node = node.child_by_field_name(FIELD_NAME_NAME).unwrap();
                        context.check_if_error(&node, RULE_NAME)?;
                        let name = parse_qualified_identifier(context, &mut node.walk())?;
                        return Ok(name.into());
                    }
                    NODE_KIND_LINE_COMMENT => {
                        trace!("no comments here"); //check_and_add_comment!(context, node, import);
                    }
                    _ => {
                        unexpected_node!(
                            context,
                            RULE_NAME,
                            node,
                            [
                                NODE_KIND_MODULE_IMPORT,
                                NODE_KIND_MEMBER_IMPORT,
                                NODE_KIND_LINE_COMMENT,
                            ]
                        );
                    }
                }
            }
            has_next = cursor.goto_next_sibling();
        }
        assert!(cursor.goto_parent());
    }
    unreachable!()
}

fn parse_identifier<'a>(
    context: &mut ParseContext<'a>,
    node: &Node<'a>,
) -> Result<Identifier, Error> {
    rule_fn!("parse_identifier", node);
    Ok(Identifier::new_unchecked(context.node_source(node)?).with_ts_span(node.into()))
}

fn parse_qualified_identifier<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<QualifiedIdentifier, Error> {
    let node = cursor.node();
    rule_fn!("parse_qualified_identifier", node);

    let child = node.child_by_field_name(FIELD_NAME_MODULE).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let module = parse_identifier(context, &child)?;

    let child = node.child_by_field_name(FIELD_NAME_MEMBER).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let member = parse_identifier(context, &child)?;

    Ok(QualifiedIdentifier::new(module, member))
}

fn parse_identifier_reference<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<IdentifierReference, Error> {
    rule_fn!("parse_identifier_reference", cursor.node());
    let mut has_next = cursor.goto_first_child();
    if has_next {
        while has_next {
            let node = cursor.node();
            context.check_if_error(&node, RULE_NAME)?;
            if node.is_named() {
                match node.kind() {
                    NODE_KIND_IDENTIFIER => return Ok(parse_identifier(context, &node)?.into()),
                    NODE_KIND_QUALIFIED_IDENTIFIER => {
                        return Ok(parse_qualified_identifier(context, &mut node.walk())?.into());
                    }
                    NODE_KIND_LINE_COMMENT => {
                        trace!("no comments here"); //check_and_add_comment!(context, node, import);
                    }
                    _ => {
                        unexpected_node!(
                            context,
                            RULE_NAME,
                            node,
                            [
                                NODE_KIND_IDENTIFIER,
                                NODE_KIND_QUALIFIED_IDENTIFIER,
                                NODE_KIND_LINE_COMMENT,
                            ]
                        );
                    }
                }
            }
            has_next = cursor.goto_next_sibling();
        }
        assert!(cursor.goto_parent());
    }
    unreachable!()
}

fn parse_annotation<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<Annotation, Error> {
    rule_fn!("parse_annotation", cursor.node());
    let mut has_next = cursor.goto_first_child();
    if has_next {
        while has_next {
            let node = cursor.node();
            context.check_if_error(&node, RULE_NAME)?;
            if node.is_named() {
                match node.kind() {
                    NODE_KIND_ANNOTATION_PROPERTY => {
                        return Ok(parse_annotation_property(context, &mut node.walk())?.into())
                    }
                    NODE_KIND_CONSTRAINT => {
                        return Ok(parse_constraint(context, &mut node.walk())?.into())
                    }
                    NODE_KIND_LINE_COMMENT => {
                        trace!("no comments here"); //check_and_add_comment!(context, node, import);
                    }
                    _ => {
                        unexpected_node!(
                            context,
                            RULE_NAME,
                            node,
                            [
                                NODE_KIND_ANNOTATION_PROPERTY,
                                NODE_KIND_CONSTRAINT,
                                NODE_KIND_LINE_COMMENT,
                            ]
                        );
                    }
                }
            }
            has_next = cursor.goto_next_sibling();
        }
        assert!(cursor.goto_parent());
    }
    unreachable!()
}

fn parse_annotation_property<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<AnnotationProperty, Error> {
    let node = cursor.node();
    rule_fn!("parse_annotation_property", node);

    let child = node.child_by_field_name(FIELD_NAME_NAME).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let name = parse_identifier_reference(context, &mut child.walk())?;

    let child = node.child_by_field_name(FIELD_NAME_VALUE).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let value = parse_value(context, &mut child.walk())?;

    Ok(AnnotationProperty::new(name, value).with_ts_span(node.into()))
}

fn parse_constraint<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<Constraint, Error> {
    rule_fn!("parse_constraint", cursor.node());
    let mut has_next = cursor.goto_first_child();
    let mut constraint_name: Option<Identifier> = None;
    if has_next {
        while has_next {
            let node = cursor.node();
            context.check_if_error(&node, RULE_NAME)?;
            if node.is_named() {
                match node.kind() {
                    NODE_KIND_IDENTIFIER => {
                        constraint_name = Some(parse_identifier(context, &node)?);
                    }
                    NODE_KIND_INFORMAL_CONSTRAINT => {
                        return Ok(parse_informal_constraint(
                            context,
                            constraint_name,
                            &mut node.walk(),
                        )?
                        .into())
                    }
                    NODE_KIND_FORMAL_CONSTRAINT => {
                        return Ok(parse_formal_constraint(
                            context,
                            constraint_name,
                            &mut node.walk(),
                        )?
                        .into())
                    }
                    NODE_KIND_LINE_COMMENT => {
                        trace!("no comments here"); //check_and_add_comment!(context, node, import);
                    }
                    _ => {
                        unexpected_node!(
                            context,
                            RULE_NAME,
                            node,
                            [
                                NODE_KIND_IDENTIFIER,
                                NODE_KIND_INFORMAL_CONSTRAINT,
                                NODE_KIND_FORMAL_CONSTRAINT,
                                NODE_KIND_LINE_COMMENT,
                            ]
                        );
                    }
                }
            }
            has_next = cursor.goto_next_sibling();
        }
        assert!(cursor.goto_parent());
    }
    unreachable!()
}

fn parse_informal_constraint<'a>(
    context: &mut ParseContext<'a>,
    name: Option<Identifier>,
    cursor: &mut TreeCursor<'a>,
) -> Result<String, Error> {
    let node = cursor.node();
    rule_fn!("parse_informal_constraint", node, name);

    let child = node.child_by_field_name(FIELD_NAME_VALUE).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let node_value = context.node_source(&node)?;
    let value = node_value[1..(node_value.len() - 1)].to_string();

    Ok(value)
}

fn parse_value<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<Value, Error> {
    rule_fn!("parse_value", cursor.node());
    let mut has_next = cursor.goto_first_child();
    if has_next {
        while has_next {
            let node = cursor.node();
            context.check_if_error(&node, RULE_NAME)?;
            if node.is_named() {
                match node.kind() {
                    NODE_KIND_SIMPLE_VALUE => {
                        return Ok(parse_simple_value(context, &mut node.walk())?.into());
                    }
                    NODE_KIND_VALUE_CONSTRUCTOR => {
                        return Ok(parse_value_constructor(context, cursor)?.into());
                    }
                    NODE_KIND_IDENTIFIER_REFERENCE => {
                        return Ok(parse_identifier_reference(context, cursor)?.into());
                    }
                    NODE_KIND_LIST_OF_VALUES => {
                        return Ok(parse_list_of_values(context, cursor)?.into());
                    }
                    NODE_KIND_LINE_COMMENT => {
                        trace!("no comments here"); //check_and_add_comment!(context, node, import);
                    }
                    _ => {
                        unexpected_node!(
                            context,
                            RULE_NAME,
                            node,
                            [
                                NODE_KIND_SIMPLE_VALUE,
                                NODE_KIND_VALUE_CONSTRUCTOR,
                                NODE_KIND_IDENTIFIER_REFERENCE,
                                NODE_KIND_LIST_OF_VALUES,
                                NODE_KIND_LINE_COMMENT,
                            ]
                        );
                    }
                }
            }
            has_next = cursor.goto_next_sibling();
        }
    }
    unreachable!()
}

fn parse_simple_value<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<SimpleValue, Error> {
    rule_fn!("parse_simple_value", cursor.node());
    let mut has_next = cursor.goto_first_child();
    if has_next {
        while has_next {
            let node = cursor.node();
            context.check_if_error(&node, RULE_NAME)?;
            if node.is_named() {
                match node.kind() {
                    NODE_KIND_STRING => {
                        let value = parse_string(context, cursor)?;
                        return Ok(SimpleValue::String(value));
                    }
                    NODE_KIND_DOUBLE => {
                        let value = context.node_source(&node)?;
                        let value = f64::from_str(value).expect("Invalid value for Double");
                        return Ok(SimpleValue::Double(value.into()));
                    }
                    NODE_KIND_DECIMAL => {
                        let value = context.node_source(&node)?;
                        let value = Decimal::from_str(value).expect("Invalid value for Decimal");
                        return Ok(SimpleValue::Decimal(value));
                    }
                    NODE_KIND_INTEGER => {
                        let value = context.node_source(&node)?;
                        let value = i64::from_str(value).expect("Invalid value for Integer");
                        return Ok(SimpleValue::Integer(value));
                    }
                    NODE_KIND_BOOLEAN => {
                        let value = context.node_source(&node)?;
                        let value = bool::from_str(value).expect("Invalid value for Boolean");
                        return Ok(SimpleValue::Boolean(value));
                    }
                    NODE_KIND_IRI_REFERENCE => {
                        let value = context.node_source(&node)?;
                        let value = Url::from_str(&value[1..(value.len() - 1)])
                            .expect("Invalid value for IriReference");
                        return Ok(SimpleValue::IriReference(value));
                    }
                    NODE_KIND_LINE_COMMENT => {
                        trace!("no comments here"); //check_and_add_comment!(context, node, import);
                    }
                    _ => {
                        unexpected_node!(
                            context,
                            RULE_NAME,
                            node,
                            [
                                NODE_KIND_STRING,
                                NODE_KIND_DOUBLE,
                                NODE_KIND_DECIMAL,
                                NODE_KIND_INTEGER,
                                NODE_KIND_BOOLEAN,
                                NODE_KIND_IRI_REFERENCE,
                                NODE_KIND_LINE_COMMENT,
                            ]
                        );
                    }
                }
            }
            has_next = cursor.goto_next_sibling();
        }
        assert!(cursor.goto_parent());
    }
    unreachable!()
}

fn parse_string<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<LanguageString, Error> {
    rule_fn!("parse_string", cursor.node());
    let root_node = cursor.node();
    let mut has_next = cursor.goto_first_child();
    if has_next {
        let mut value = String::new();
        let mut language = None;
        while has_next {
            let node = cursor.node();
            context.check_if_error(&node, RULE_NAME)?;
            if node.is_named() {
                match node.kind() {
                    NODE_KIND_QUOTED_STRING => {
                        let node_value = context.node_source(&node)?;
                        value = node_value[1..(node_value.len() - 1)].to_string();
                    }
                    NODE_KIND_LANGUAGE_TAG => {
                        let node_value = context.node_source(&node)?;
                        language = Some(LanguageTag::new_unchecked(&node_value[1..]));
                    }
                    NODE_KIND_LINE_COMMENT => {
                        trace!("no comments here"); //check_and_add_comment!(context, node, import);
                    }
                    _ => {
                        unexpected_node!(
                            context,
                            RULE_NAME,
                            node,
                            [
                                NODE_KIND_QUOTED_STRING,
                                NODE_KIND_LANGUAGE_TAG,
                                NODE_KIND_LINE_COMMENT,
                            ]
                        );
                    }
                }
            }
            has_next = cursor.goto_next_sibling();
        }
        assert!(cursor.goto_parent());
        return Ok(LanguageString::new(&value, language).with_ts_span(root_node.into()));
    }
    unreachable!()
}

fn parse_value_constructor<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<ValueConstructor, Error> {
    let node = cursor.node();
    rule_fn!("parse_value_constructor", node);

    let child = node.child_by_field_name(FIELD_NAME_NAME).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let name = parse_identifier_reference(context, &mut child.walk())?;

    let child = node.child_by_field_name(FIELD_NAME_VALUE).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let value = parse_simple_value(context, &mut child.walk())?;

    Ok(ValueConstructor::new(name, value).with_ts_span(node.into()))
}

fn parse_list_of_values<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<ListOfValues, Error> {
    rule_fn!("parse_list_of_values", cursor.node());
    let mut list_of_values = ListOfValues::default();
    let mut has_next = cursor.goto_first_child();
    if has_next {
        while has_next {
            let node = cursor.node();
            context.check_if_error(&node, RULE_NAME)?;
            if node.is_named() {
                match node.kind() {
                    NODE_KIND_SIMPLE_VALUE => {
                        list_of_values
                            .add_to_values(parse_simple_value(context, &mut node.walk())?.into());
                    }
                    NODE_KIND_VALUE_CONSTRUCTOR => {
                        list_of_values
                            .add_to_values(parse_value_constructor(context, cursor)?.into());
                    }
                    NODE_KIND_IDENTIFIER_REFERENCE => {
                        list_of_values
                            .add_to_values(parse_identifier_reference(context, cursor)?.into());
                    }
                    NODE_KIND_LINE_COMMENT => {
                        trace!("no comments here"); //check_and_add_comment!(context, node, import);
                    }
                    _ => {
                        unexpected_node!(
                            context,
                            RULE_NAME,
                            node,
                            [
                                NODE_KIND_SIMPLE_VALUE,
                                NODE_KIND_VALUE_CONSTRUCTOR,
                                NODE_KIND_IDENTIFIER_REFERENCE,
                                NODE_KIND_LINE_COMMENT,
                            ]
                        );
                    }
                }
            }
            has_next = cursor.goto_next_sibling();
        }
        assert!(cursor.goto_parent());
    }
    Ok(list_of_values)
}

fn parse_type_definition<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<Definition, Error> {
    rule_fn!("parse_type_definition", cursor.node());
    let mut has_next = cursor.goto_first_child();
    if has_next {
        while has_next {
            let node = cursor.node();
            context.check_if_error(&node, RULE_NAME)?;
            if node.is_named() {
                match node.kind() {
                    NODE_KIND_DATA_TYPE_DEF => {
                        return Ok(parse_data_type_def(context, &mut node.walk())?.into());
                    }
                    NODE_KIND_ENTITY_DEF => {
                        return Ok(parse_entity_def(context, &mut node.walk())?.into());
                    }
                    NODE_KIND_ENUM_DEF => {
                        return Ok(parse_enum_def(context, &mut node.walk())?.into());
                    }
                    NODE_KIND_EVENT_DEF => {
                        return Ok(parse_event_def(context, &mut node.walk())?.into());
                    }
                    NODE_KIND_STRUCTURE_DEF => {
                        return Ok(parse_structure_def(context, &mut node.walk())?.into());
                    }
                    NODE_KIND_UNION_DEF => {
                        return Ok(parse_union_def(context, &mut node.walk())?.into());
                    }
                    NODE_KIND_PROPERTY_DEF => {
                        return Ok(parse_property_def(context, &mut node.walk())?.into());
                    }
                    NODE_KIND_LINE_COMMENT => {
                        trace!("no comments here"); //check_and_add_comment!(context, node, import);
                    }
                    _ => {
                        unexpected_node!(
                            context,
                            RULE_NAME,
                            node,
                            [
                                NODE_KIND_DATA_TYPE_DEF,
                                NODE_KIND_ENTITY_DEF,
                                NODE_KIND_ENUM_DEF,
                                NODE_KIND_EVENT_DEF,
                                NODE_KIND_STRUCTURE_DEF,
                                NODE_KIND_UNION_DEF,
                                NODE_KIND_LINE_COMMENT,
                            ]
                        );
                    }
                }
            }
            has_next = cursor.goto_next_sibling();
        }
        assert!(cursor.goto_parent());
    }
    unreachable!()
}

fn parse_data_type_def<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<DatatypeDef, Error> {
    let node = cursor.node();
    rule_fn!("parse_data_type_def", node);

    let child = node.child_by_field_name(FIELD_NAME_NAME).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let name = parse_identifier(context, &child)?;

    let child = node.child_by_field_name(FIELD_NAME_BASE).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let base_type = parse_data_type_base(context, &mut child.walk())?;

    context.start_type(&name)?;
    let mut data_type = DatatypeDef::new(name, base_type).with_ts_span(node.into());

    if let Some(child) = node.child_by_field_name(FIELD_NAME_BODY) {
        context.check_if_error(&child, RULE_NAME)?;
        let body = parse_annotation_only_body(context, &mut child.walk())?;
        data_type.set_body(body);
    }

    context.end_type();
    Ok(data_type)
}

fn parse_data_type_base<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<IdentifierReference, Error> {
    rule_fn!("parse_data_type_base", cursor.node());
    let mut has_next = cursor.goto_first_child();
    while has_next {
        let node = cursor.node();
        context.check_if_error(&node, RULE_NAME)?;
        if node.is_named() {
            match node.kind() {
                NODE_KIND_IDENTIFIER_REFERENCE => {
                    return parse_identifier_reference(context, &mut node.walk());
                }
                NODE_KIND_BUILTIN_SIMPLE_TYPE => {
                    let module = Identifier::new_unchecked(NAME_SDML);
                    let member = Identifier::new_unchecked(context.node_source(&node)?)
                        .with_ts_span(node.into());
                    return Ok(IdentifierReference::QualifiedIdentifier(
                        QualifiedIdentifier::new(module, member),
                    ));
                }
                NODE_KIND_LINE_COMMENT => {
                    trace!("no comments here"); //check_and_add_comment!(context, node, import);
                }
                _ => {
                    unexpected_node!(
                        context,
                        RULE_NAME,
                        node,
                        [
                            NODE_KIND_IDENTIFIER_REFERENCE,
                            NODE_KIND_BUILTIN_SIMPLE_TYPE,
                            NODE_KIND_LINE_COMMENT,
                        ]
                    );
                }
            }
        }
        has_next = cursor.goto_next_sibling();
    }
    unreachable!()
}

fn parse_annotation_only_body<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<AnnotationOnlyBody, Error> {
    rule_fn!("parse_annotation_only_body", cursor.node());
    let mut body = AnnotationOnlyBody::default().with_ts_span(cursor.node().into());
    let mut has_next = cursor.goto_first_child();
    if has_next {
        while has_next {
            let node = cursor.node();
            context.check_if_error(&node, RULE_NAME)?;
            if node.is_named() {
                match node.kind() {
                    NODE_KIND_ANNOTATION => {
                        body.add_annotation(parse_annotation(context, &mut node.walk())?);
                    }
                    NODE_KIND_LINE_COMMENT => {
                        check_and_add_comment!(context, node, body);
                    }
                    _ => {
                        unexpected_node!(
                            context,
                            RULE_NAME,
                            node,
                            [NODE_KIND_ANNOTATION, NODE_KIND_LINE_COMMENT,]
                        );
                    }
                }
            }
            has_next = cursor.goto_next_sibling();
        }
        assert!(cursor.goto_parent());
    }
    Ok(body)
}

fn parse_entity_def<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<EntityDef, Error> {
    let node = cursor.node();
    rule_fn!("parse_entity_def", node);

    let child = node.child_by_field_name(FIELD_NAME_NAME).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let name = parse_identifier(context, &child)?;

    context.start_type(&name)?;
    let mut entity = EntityDef::new(name).with_ts_span(node.into());

    if let Some(child) = node.child_by_field_name(FIELD_NAME_BODY) {
        context.check_if_error(&child, RULE_NAME)?;
        let body = parse_entity_body(context, &mut child.walk())?;
        entity.set_body(body);
    }

    context.end_type();
    Ok(entity)
}

fn parse_entity_body<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<EntityBody, Error> {
    let node = cursor.node();
    rule_fn!("parse_entity_body", node);

    let child = node.child_by_field_name(FIELD_NAME_IDENTITY).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let identity = parse_identity_member(context, &mut child.walk())?;
    let mut body = EntityBody::new(identity).with_ts_span(node.into());

    let mut has_next = cursor.goto_first_child();
    if has_next {
        while has_next {
            let node = cursor.node();
            context.check_if_error(&node, RULE_NAME)?;
            if node.is_named() {
                match node.kind() {
                    NODE_KIND_ANNOTATION => {
                        body.add_annotation(parse_annotation(context, &mut node.walk())?);
                    }
                    NODE_KIND_MEMBER_BY_VALUE => {
                        body.add_to_members(
                            parse_by_value_member(context, &mut node.walk())?.into(),
                        );
                    }
                    NODE_KIND_MEMBER_BY_REFERENCE => {
                        body.add_to_members(
                            parse_by_reference_member(context, &mut node.walk())?.into(),
                        );
                    }
                    NODE_KIND_ENTITY_GROUP => {
                        body.add_to_groups(parse_entity_group(context, &mut node.walk())?);
                    }
                    NODE_KIND_LINE_COMMENT => {
                        check_and_add_comment!(context, node, body);
                    }
                    NODE_KIND_IDENTITY_MEMBER => (),
                    _ => {
                        unexpected_node!(
                            context,
                            RULE_NAME,
                            node,
                            [
                                NODE_KIND_ANNOTATION,
                                NODE_KIND_MEMBER_BY_VALUE,
                                NODE_KIND_MEMBER_BY_REFERENCE,
                                NODE_KIND_ENTITY_GROUP,
                                NODE_KIND_LINE_COMMENT,
                            ]
                        );
                    }
                }
            }
            has_next = cursor.goto_next_sibling();
        }
        assert!(cursor.goto_parent());
    }
    Ok(body)
}

fn parse_entity_group<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<EntityGroup, Error> {
    rule_fn!("parse_entity_body", cursor.node());
    let mut group = EntityGroup::default().with_ts_span(cursor.node().into());
    let mut has_next = cursor.goto_first_child();
    if has_next {
        while has_next {
            let node = cursor.node();
            context.check_if_error(&node, RULE_NAME)?;
            if node.is_named() {
                match node.kind() {
                    NODE_KIND_ANNOTATION => {
                        group.add_annotation(parse_annotation(context, &mut node.walk())?);
                    }
                    NODE_KIND_MEMBER_BY_VALUE => {
                        group.add_to_members(parse_by_value_member(context, cursor)?.into());
                    }
                    NODE_KIND_MEMBER_BY_REFERENCE => {
                        group.add_to_members(parse_by_reference_member(context, cursor)?.into());
                    }
                    NODE_KIND_LINE_COMMENT => {
                        check_and_add_comment!(context, node, group);
                    }
                    _ => {
                        unexpected_node!(
                            context,
                            RULE_NAME,
                            node,
                            [
                                NODE_KIND_ANNOTATION,
                                NODE_KIND_MEMBER_BY_VALUE,
                                NODE_KIND_MEMBER_BY_REFERENCE,
                                NODE_KIND_LINE_COMMENT,
                            ]
                        );
                    }
                }
            }
            has_next = cursor.goto_next_sibling();
        }
        assert!(cursor.goto_parent());
    }
    Ok(group)
}

fn parse_enum_def<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<EnumDef, Error> {
    let node = cursor.node();
    rule_fn!("parse_enum_def", node);

    let child = node.child_by_field_name(FIELD_NAME_NAME).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let name = parse_identifier(context, &child)?;

    context.start_type(&name)?;
    let mut new_enum = EnumDef::new(name).with_ts_span(node.into());

    if let Some(child) = node.child_by_field_name(FIELD_NAME_BODY) {
        context.check_if_error(&child, RULE_NAME)?;
        let body = parse_enum_body(context, &mut child.walk())?;
        new_enum.set_body(body);
    }

    context.end_type();
    Ok(new_enum)
}

fn parse_enum_body<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<EnumBody, Error> {
    rule_fn!("parse_enum_body", cursor.node());
    let mut body = EnumBody::default().with_ts_span(cursor.node().into());
    let mut has_next = cursor.goto_first_child();
    if has_next {
        while has_next {
            let node = cursor.node();
            context.check_if_error(&node, RULE_NAME)?;
            if node.is_named() {
                match node.kind() {
                    NODE_KIND_ANNOTATION => {
                        body.add_annotation(parse_annotation(context, &mut node.walk())?);
                    }
                    NODE_KIND_VALUE_VARIANT => {
                        body.add_to_variants(parse_enum_variant(context, &mut node.walk())?);
                    }
                    NODE_KIND_LINE_COMMENT => {
                        check_and_add_comment!(context, node, body);
                    }
                    _ => {
                        unexpected_node!(
                            context,
                            RULE_NAME,
                            node,
                            [
                                NODE_KIND_ANNOTATION,
                                NODE_KIND_VALUE_VARIANT,
                                NODE_KIND_LINE_COMMENT,
                            ]
                        );
                    }
                }
            }
            has_next = cursor.goto_next_sibling();
        }
        assert!(cursor.goto_parent());
    }
    Ok(body)
}

fn parse_event_def<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<EventDef, Error> {
    let node = cursor.node();
    rule_fn!("parse_event_def", node);

    let child = node.child_by_field_name(FIELD_NAME_NAME).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let name = parse_identifier(context, &child)?;

    let child = node.child_by_field_name(FIELD_NAME_SOURCE).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let event_source = parse_identifier_reference(context, &mut child.walk())?;

    context.start_type(&name)?;
    let mut event = EventDef::new(name, event_source).with_ts_span(node.into());

    if let Some(child) = node.child_by_field_name(FIELD_NAME_BODY) {
        context.check_if_error(&child, RULE_NAME)?;
        let body = parse_structure_body(context, &mut child.walk())?;
        event.set_body(body);
    }

    context.end_type();
    Ok(event)
}

fn parse_structure_body<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<StructureBody, Error> {
    rule_fn!("parse_structure_body", cursor.node());
    let mut body = StructureBody::default().with_ts_span(cursor.node().into());
    let mut has_next = cursor.goto_first_child();
    if has_next {
        while has_next {
            let node = cursor.node();
            context.check_if_error(&node, RULE_NAME)?;
            if node.is_named() {
                match node.kind() {
                    NODE_KIND_ANNOTATION => {
                        body.add_annotation(parse_annotation(context, &mut node.walk())?);
                    }
                    NODE_KIND_MEMBER_BY_VALUE => {
                        body.add_to_members(
                            parse_by_value_member(context, &mut node.walk())?,
                        );
                    }
                    NODE_KIND_STRUCTURE_GROUP => {
                        body.add_to_groups(parse_structure_group(context, &mut node.walk())?);
                    }
                    NODE_KIND_LINE_COMMENT => {
                        check_and_add_comment!(context, node, body);
                    }
                    _ => {
                        unexpected_node!(
                            context,
                            RULE_NAME,
                            node,
                            [
                                NODE_KIND_ANNOTATION,
                                NODE_KIND_STRUCTURE_MEMBER,
                                NODE_KIND_STRUCTURE_GROUP,
                                NODE_KIND_LINE_COMMENT,
                            ]
                        );
                    }
                }
            }
            has_next = cursor.goto_next_sibling();
        }
        assert!(cursor.goto_parent());
    }
    Ok(body)
}

fn parse_structure_group<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<StructureGroup, Error> {
    rule_fn!("parse_structure_body: {:?}", cursor.node());
    let mut group = StructureGroup::default().with_ts_span(cursor.node().into());
    let mut has_next = cursor.goto_first_child();
    if has_next {
        while has_next {
            let node = cursor.node();
            context.check_if_error(&node, RULE_NAME)?;
            if node.is_named() {
                match node.kind() {
                    NODE_KIND_ANNOTATION => {
                        group.add_annotation(parse_annotation(context, &mut node.walk())?);
                    }
                    NODE_KIND_MEMBER_BY_VALUE => {
                        group.add_to_members(
                            parse_by_value_member(context, &mut node.walk())?,
                        );
                    }
                    NODE_KIND_LINE_COMMENT => {
                        check_and_add_comment!(context, node, group);
                    }
                    _ => {
                        unexpected_node!(
                            context,
                            RULE_NAME,
                            node,
                            [
                                NODE_KIND_ANNOTATION,
                                NODE_KIND_STRUCTURE_MEMBER,
                                NODE_KIND_LINE_COMMENT,
                            ]
                        );
                    }
                }
            }
            has_next = cursor.goto_next_sibling();
        }
        assert!(cursor.goto_parent());
    }
    Ok(group)
}

fn parse_structure_def<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<StructureDef, Error> {
    let node = cursor.node();
    rule_fn!("parse_structure_def", node);

    let child = node.child_by_field_name(FIELD_NAME_NAME).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let name = parse_identifier(context, &child)?;

    context.start_type(&name)?;
    let mut structure = StructureDef::new(name).with_ts_span(node.into());

    if let Some(child) = node.child_by_field_name(FIELD_NAME_BODY) {
        context.check_if_error(&child, RULE_NAME)?;
        let body = parse_structure_body(context, &mut child.walk())?;
        structure.set_body(body);
    }

    context.end_type();
    Ok(structure)
}

fn parse_union_def<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<UnionDef, Error> {
    let node = cursor.node();
    rule_fn!("parse_union_def", node);

    let child = node.child_by_field_name(FIELD_NAME_NAME).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let name = parse_identifier(context, &child)?;

    context.start_type(&name)?;
    let mut new_enum = UnionDef::new(name).with_ts_span(node.into());

    if let Some(child) = node.child_by_field_name(FIELD_NAME_BODY) {
        context.check_if_error(&child, RULE_NAME)?;
        let body = parse_union_body(context, &mut child.walk())?;
        new_enum.set_body(body);
    }

    context.end_type();
    Ok(new_enum)
}

fn parse_union_body<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<UnionBody, Error> {
    rule_fn!("parse_union_body", cursor.node());
    let mut body = UnionBody::default().with_ts_span(cursor.node().into());
    let mut has_next = cursor.goto_first_child();
    if has_next {
        while has_next {
            let node = cursor.node();
            context.check_if_error(&node, RULE_NAME)?;
            if node.is_named() {
                match node.kind() {
                    NODE_KIND_ANNOTATION => {
                        body.add_annotation(parse_annotation(context, &mut node.walk())?);
                    }
                    NODE_KIND_TYPE_VARIANT => {
                        body.add_to_variants(parse_type_variant(context, &mut node.walk())?);
                    }
                    NODE_KIND_LINE_COMMENT => {
                        check_and_add_comment!(context, node, body);
                    }
                    _ => {
                        unexpected_node!(
                            context,
                            RULE_NAME,
                            node,
                            [
                                NODE_KIND_ANNOTATION,
                                NODE_KIND_TYPE_VARIANT,
                                NODE_KIND_LINE_COMMENT,
                            ]
                        );
                    }
                }
            }
            has_next = cursor.goto_next_sibling();
        }
        assert!(cursor.goto_parent());
    }
    Ok(body)
}

fn parse_property_def<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<PropertyDef, Error> {
    let node = cursor.node();
    rule_fn!("parse_property_def", node);

    let child = node.child_by_field_name(FIELD_NAME_NAME).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let name = parse_identifier(context, &child)?;

    context.start_type(&name)?;
    let mut new_prop = PropertyDef::new(name).with_ts_span(node.into());

    if let Some(child) = node.child_by_field_name(FIELD_NAME_BODY) {
        context.check_if_error(&child, RULE_NAME)?;
        let body = parse_property_body(context, &mut child.walk())?;
        new_prop.set_body(body);
    }

    context.end_type();
    Ok(new_prop)
}

fn parse_property_body<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<PropertyBody, Error> {
    rule_fn!("parse_property_body", cursor.node());
    let mut body = PropertyBody::default().with_ts_span(cursor.node().into());
    let mut has_next = cursor.goto_first_child();
    if has_next {
        while has_next {
            let node = cursor.node();
            context.check_if_error(&node, RULE_NAME)?;
            if node.is_named() {
                match node.kind() {
                    NODE_KIND_ANNOTATION => {
                        body.add_annotation(parse_annotation(context, &mut node.walk())?);
                    }
                    NODE_KIND_PROPERTY_ROLE => {
                        body.add_to_roles(parse_property_role(context, &mut node.walk())?);
                    }
                    NODE_KIND_LINE_COMMENT => {
                        check_and_add_comment!(context, node, body);
                    }
                    _ => {
                        unexpected_node!(
                            context,
                            RULE_NAME,
                            node,
                            [
                                NODE_KIND_ANNOTATION,
                                NODE_KIND_PROPERTY_ROLE,
                                NODE_KIND_LINE_COMMENT,
                            ]
                        );
                    }
                }
            }
            has_next = cursor.goto_next_sibling();
        }
        assert!(cursor.goto_parent());
    }
    Ok(body)
}

fn parse_identity_member<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<IdentityMember, Error> {
    let node = cursor.node();
    rule_fn!("parse_identity_member", node);

    let child = node.child_by_field_name(FIELD_NAME_NAME).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let name = parse_identifier(context, &child)?;

    if let Some(child) = node.child_by_field_name(FIELD_NAME_TARGET) {
        context.start_member(&name)?;

        context.check_if_error(&child, RULE_NAME)?;
        let type_reference = parse_type_reference(context, &mut child.walk())?;
        let mut member_def = IdentityMemberDef::new(type_reference);

        if let Some(child) = node.child_by_field_name(FIELD_NAME_BODY) {
            context.check_if_error(&child, RULE_NAME)?;
            let body = parse_annotation_only_body(context, &mut child.walk())?;
            member_def.set_body(body);
        };

        Ok(IdentityMember::new_with_definition(name, member_def).with_ts_span(node.into()))
    } else if let Some(child) = node.child_by_field_name(FIELD_NAME_ROLE) {
        context.check_if_error(&child, RULE_NAME)?;
        let role = parse_identifier(context, &child)?;

        context.start_member(&Identifier::new_unchecked(&format!("{}_{}", name, role)))?;

        Ok(IdentityMember::new_with_role(name, role).with_ts_span(node.into()))
    } else {
        unreachable!();
    }
}

fn parse_by_value_member<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<ByValueMember, Error> {
    let node = cursor.node();
    rule_fn!("parse_by_value_member", node);

    let child = node.child_by_field_name(FIELD_NAME_NAME).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let name = parse_identifier(context, &child)?;
    trace!("name: {:?}", name);

    if let Some(child) = node.child_by_field_name(FIELD_NAME_TARGET) {
        context.start_member(&name)?;

        context.check_if_error(&child, RULE_NAME)?;
        let type_reference = parse_type_reference(context, &mut child.walk())?;
        let mut member_def = ByValueMemberDef::new(type_reference);

        if let Some(child) = node.child_by_field_name(FIELD_NAME_TARGET_CARDINALITY) {
            context.check_if_error(&child, RULE_NAME)?;
            let cardinality = parse_cardinality(context, &mut child.walk())?;
            member_def.set_target_cardinality(cardinality);
        }

        if let Some(child) = node.child_by_field_name(FIELD_NAME_BODY) {
            context.check_if_error(&child, RULE_NAME)?;
            let body = parse_annotation_only_body(context, &mut child.walk())?;
            member_def.set_body(body);
        }

        Ok(ByValueMember::new_with_definition(name, member_def).with_ts_span(node.into()))
    } else if let Some(child) = node.child_by_field_name(FIELD_NAME_ROLE) {
        context.check_if_error(&child, RULE_NAME)?;
        let role = parse_identifier(context, &child)?;

        context.start_member(&Identifier::new_unchecked(&format!("{}_{}", name, role)))?;

        Ok(ByValueMember::new_with_role(name, role).with_ts_span(node.into()))
    } else {
        error!("Not expecting {:?}", node);
        unreachable!();
    }
}

fn parse_by_reference_member<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<ByReferenceMember, Error> {
    let node = cursor.node();
    rule_fn!("parse_by_reference_member", node);

    let child = node.child_by_field_name(FIELD_NAME_NAME).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let name = parse_identifier(context, &child)?;

    if let Some(child) = node.child_by_field_name(FIELD_NAME_TARGET) {
        context.start_member(&name)?;

        context.check_if_error(&child, RULE_NAME)?;
        let type_reference = parse_type_reference(context, &mut child.walk())?;
        let mut member_def = ByReferenceMemberDef::new(type_reference);

        if let Some(child) = node.child_by_field_name(FIELD_NAME_SOURCE_CARDINALITY) {
            context.check_if_error(&child, RULE_NAME)?;
            let cardinality = parse_cardinality(context, &mut child.walk())?;
            member_def.set_source_cardinality(cardinality);
        }

        if let Some(child) = node.child_by_field_name(FIELD_NAME_TARGET_CARDINALITY) {
            context.check_if_error(&child, RULE_NAME)?;
            let cardinality = parse_cardinality(context, &mut child.walk())?;
            member_def.set_target_cardinality(cardinality);
        }

        if let Some(child) = node.child_by_field_name(FIELD_NAME_BODY) {
            context.check_if_error(&child, RULE_NAME)?;
            let body = parse_annotation_only_body(context, &mut child.walk())?;
            member_def.set_body(body);
        }

        Ok(ByReferenceMember::new_with_definition(name, member_def).with_ts_span(node.into()))
    } else if let Some(child) = node.child_by_field_name(FIELD_NAME_ROLE) {
        context.check_if_error(&child, RULE_NAME)?;
        let role = parse_identifier(context, &child)?;

        context.start_member(&Identifier::new_unchecked(&format!("{}_{}", name, role)))?;

        Ok(ByReferenceMember::new_with_role(name, role).with_ts_span(node.into()))
    } else {
        unreachable!();
    }
}

fn parse_property_role<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<PropertyRole, Error> {
    let node = cursor.node();
    rule_fn!("parse_property_role", node);

    let child = node.child_by_field_name(FIELD_NAME_NAME).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let name = parse_identifier(context, &child)?;

    let child = node.child_by_field_name(FIELD_NAME_TARGET).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let type_reference = parse_type_reference(context, &mut child.walk())?;

    context.start_member(&name)?;
    let mut member = PropertyRole::new(name, type_reference).with_ts_span(node.into());

    if let Some(child) = node.child_by_field_name(FIELD_NAME_SOURCE_CARDINALITY) {
        context.check_if_error(&child, RULE_NAME)?;
        let cardinality = parse_cardinality(context, &mut child.walk())?;
        member.set_source_cardinality(Some(cardinality));
    }

    if let Some(child) = node.child_by_field_name(FIELD_NAME_TARGET_CARDINALITY) {
        context.check_if_error(&child, RULE_NAME)?;
        let cardinality = parse_cardinality(context, &mut child.walk())?;
        member.set_target_cardinality(cardinality);
    }

    if let Some(child) = node.child_by_field_name(FIELD_NAME_BODY) {
        context.check_if_error(&child, RULE_NAME)?;
        let body = parse_annotation_only_body(context, &mut child.walk())?;
        member.set_body(body);
    }

    Ok(member)
}

fn parse_type_reference<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<TypeReference, Error> {
    rule_fn!("parse_type_reference", cursor.node());
    let mut has_next = cursor.goto_first_child();
    while has_next {
        let node = cursor.node();
        context.check_if_error(&node, RULE_NAME)?;
        if node.is_named() {
            match node.kind() {
                NODE_KIND_UNKNOWN_TYPE => {
                    return Ok(TypeReference::Unknown);
                }
                NODE_KIND_IDENTIFIER_REFERENCE => {
                    let reference = parse_identifier_reference(context, &mut node.walk())?;
                    return Ok(TypeReference::Reference(reference));
                }
                NODE_KIND_BUILTIN_SIMPLE_TYPE => {
                    let module = Identifier::new_unchecked(NAME_SDML);
                    let member = Identifier::new_unchecked(context.node_source(&node)?)
                        .with_ts_span(node.into());
                    return Ok(TypeReference::Reference(
                        QualifiedIdentifier::new(module, member).into(),
                    ));
                }
                NODE_KIND_LINE_COMMENT => {
                    trace!("no comments here"); //check_and_add_comment!(context, node, import);
                }
                _ => {
                    unexpected_node!(
                        context,
                        RULE_NAME,
                        node,
                        [
                            NODE_KIND_UNKNOWN_TYPE,
                            NODE_KIND_IDENTIFIER_REFERENCE,
                            NODE_KIND_BUILTIN_SIMPLE_TYPE,
                            NODE_KIND_LINE_COMMENT,
                        ]
                    );
                }
            }
        }
        has_next = cursor.goto_next_sibling();
    }
    unreachable!()
}

fn parse_enum_variant<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<ValueVariant, Error> {
    let node = cursor.node();
    rule_fn!("parse_enum_variant", node);

    let child = node.child_by_field_name(FIELD_NAME_NAME).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let name = parse_identifier(context, &child)?;

    let child = node.child_by_field_name(FIELD_NAME_VALUE).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let text = context.node_source(&child)?;
    let value = u32::from_str(text).map_err(|_| invalid_value_for_type(text, "unsigned"))?;

    context.start_member(&name)?;
    let mut enum_variant = ValueVariant::new(name, value).with_ts_span(node.into());

    if let Some(child) = node.child_by_field_name(FIELD_NAME_BODY) {
        context.check_if_error(&child, RULE_NAME)?;
        let body = parse_annotation_only_body(context, &mut child.walk())?;
        enum_variant.set_body(body);
    }

    Ok(enum_variant)
}

fn parse_type_variant<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<TypeVariant, Error> {
    let node = cursor.node();
    rule_fn!("parse_type_variant", node);

    let child = node.child_by_field_name(FIELD_NAME_NAME).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let name = parse_identifier_reference(context, &mut child.walk())?;

    let type_variant = TypeVariant::new(name).with_ts_span(node.into());

    let mut type_variant = if let Some(child) = node.child_by_field_name(FIELD_NAME_RENAME) {
        context.check_if_error(&child, RULE_NAME)?;
        let rename = parse_identifier(context, &child)?;
        context.start_member(&rename)?;
        type_variant.with_rename(rename)
    } else {
        // FIX: context.start_member(type_variant.name());
        type_variant
    };

    if let Some(child) = node.child_by_field_name(FIELD_NAME_BODY) {
        context.check_if_error(&child, RULE_NAME)?;
        let body = parse_annotation_only_body(context, &mut child.walk())?;
        type_variant.set_body(body);
    }

    Ok(type_variant)
}

fn parse_cardinality<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<Cardinality, Error> {
    let node = cursor.node();
    rule_fn!("parse_cardinality", node);

    let child = node.child_by_field_name(FIELD_NAME_MIN).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let text = context.node_source(&child)?;
    let min = u32::from_str(text).map_err(|_| invalid_value_for_type(text, NODE_KIND_UNSIGNED))?;

    if let Some(child) = node.child_by_field_name("range") {
        if let Some(child) = child.child_by_field_name(FIELD_NAME_MAX) {
            context.check_if_error(&child, RULE_NAME)?;
            let text = context.node_source(&child)?;
            let max =
                u32::from_str(text).map_err(|_| invalid_value_for_type(text, NODE_KIND_UNSIGNED))?;
            Ok(Cardinality::new_range(min, max).with_ts_span(node.into()))
        } else {
            Ok(Cardinality::new_unbounded(min).with_ts_span(node.into()))
        }
    } else {
        Ok(Cardinality::new_single(min).with_ts_span(node.into()))
    }
}

// ------------------------------------------------------------------------------------------------
// Message Formatters
// ------------------------------------------------------------------------------------------------

fn message_found_node(found: &str) -> String {
    format!("found `{found}`")
}

fn message_expecting_node(expecting: &str) -> String {
    format!("expecting: `{expecting}`")
}

fn message_expecting_one_of_node(expecting: &[&str]) -> String {
    format!(
        "expecting on of: {}",
        expecting
            .iter()
            .map(|s| format!("`{s}`"))
            .collect::<Vec<String>>()
            .join("|")
    )
}

// ------------------------------------------------------------------------------------------------
// Constraints
// ------------------------------------------------------------------------------------------------

fn parse_formal_constraint<'a>(
    _context: &mut ParseContext<'a>,
    name: Option<Identifier>,
    cursor: &mut TreeCursor<'a>,
) -> Result<String, Error> {
    rule_fn!("parse_formal_constraint", cursor.node(), name);
    todo!();
}
