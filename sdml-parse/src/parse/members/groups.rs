use crate::parse::annotations::parse_annotation;
use crate::parse::members::parse_member;
use crate::parse::ParseContext;
use sdml_core::error::Error;
use sdml_core::model::annotations::HasAnnotations;
use sdml_core::model::definitions::HasMembers;
use sdml_core::model::members::MemberGroup;
use sdml_core::model::HasSourceSpan;
use sdml_core::syntax::{
    NODE_KIND_ANNOTATION, NODE_KIND_LINE_COMMENT,
    NODE_KIND_MEMBER, NODE_KIND_STRUCTURE_MEMBER,
};
use tree_sitter::TreeCursor;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------



// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------


pub(crate) fn parse_member_group<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<MemberGroup, Error> {
    rule_fn!("structure_group", cursor.node());
    let mut group = MemberGroup::default().with_source_span(cursor.node().into());
    let mut has_next = cursor.goto_first_child();
    if has_next {
        while has_next {
            let node = cursor.node();
            context.check_if_error(&node, RULE_NAME)?;
            if node.is_named() {
                match node.kind() {
                    NODE_KIND_ANNOTATION => {
                        group.add_to_annotations(parse_annotation(context, &mut node.walk())?);
                    }
                    NODE_KIND_MEMBER => {
                        group.add_to_members(parse_member(context, &mut node.walk())?);
                    }
                    NODE_KIND_LINE_COMMENT => {}
                    _ => {
                        unexpected_node!(
                            context,
                            RULE_NAME,
                            node,
                            [NODE_KIND_ANNOTATION, NODE_KIND_STRUCTURE_MEMBER,]
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

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
