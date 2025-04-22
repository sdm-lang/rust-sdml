use crate::parse::{parse_comment, ParseContext};
use sdml_core::{
    error::Error,
    load::ModuleLoader as ModuleLoaderTrait,
    model::{
        identifiers::{Identifier, IdentifierReference, QualifiedIdentifier},
        HasSourceSpan,
    },
    syntax::{
        FIELD_NAME_MEMBER, FIELD_NAME_MODULE, NODE_KIND_IDENTIFIER, NODE_KIND_LINE_COMMENT,
        NODE_KIND_QUALIFIED_IDENTIFIER,
    },
};
use tree_sitter::{Node, TreeCursor};

// ------------------------------------------------------------------------------------------------
// Parser Functions
// ------------------------------------------------------------------------------------------------

pub(crate) fn parse_identifier<'a>(
    context: &mut ParseContext<'a>,
    node: &Node<'a>,
) -> Result<Identifier, Error> {
    rule_fn!("identifier", node);

    Ok(Identifier::new_unchecked(context.node_source(node)?).with_source_span(node.into()))
}

pub(crate) fn parse_qualified_identifier<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<QualifiedIdentifier, Error> {
    let node = cursor.node();
    rule_fn!("qualified_identifier", node);

    let child = node_field_named!(
        context,
        RULE_NAME,
        node,
        FIELD_NAME_MODULE,
        NODE_KIND_IDENTIFIER
    );
    context.check_if_error(&child, RULE_NAME)?;
    let module = parse_identifier(context, &child)?;

    let child = node_field_named!(
        context,
        RULE_NAME,
        node,
        FIELD_NAME_MEMBER,
        NODE_KIND_IDENTIFIER
    );
    context.check_if_error(&child, RULE_NAME)?;
    let member = parse_identifier(context, &child)?;

    Ok(QualifiedIdentifier::new(module, member).with_source_span(node.into()))
}

pub(crate) fn parse_identifier_reference<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<IdentifierReference, Error> {
    rule_fn!("identifier_reference", cursor.node());

    for node in cursor.node().named_children(cursor) {
        check_node!(context, RULE_NAME, &node);
        match node.kind() {
            NODE_KIND_IDENTIFIER => return Ok(parse_identifier(context, &node)?.into()),
            NODE_KIND_QUALIFIED_IDENTIFIER => {
                return Ok(parse_qualified_identifier(context, &mut node.walk())?.into());
            }
            NODE_KIND_LINE_COMMENT => {
                let comment = parse_comment(context, &node)?;
                context.push_comment(comment);
            }
            _ => {
                unexpected_node!(
                    context,
                    RULE_NAME,
                    node,
                    [NODE_KIND_IDENTIFIER, NODE_KIND_QUALIFIED_IDENTIFIER,]
                );
            }
        }
    }
    rule_unreachable!(RULE_NAME, cursor);
}
