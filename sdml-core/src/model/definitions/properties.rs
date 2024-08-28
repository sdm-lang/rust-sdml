use crate::model::{
    annotations::AnnotationBuilder, check::Validate, identifiers::Identifier, members::MemberDef,
    HasName, Span,
};
use std::fmt::Debug;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Properties
// ------------------------------------------------------------------------------------------------

/// Corresponds to the grammar rule `property_def`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct PropertyDef {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    member: MemberDef,
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Properties
// ------------------------------------------------------------------------------------------------

impl From<MemberDef> for PropertyDef {
    fn from(member: MemberDef) -> Self {
        Self::new(member)
    }
}

impl_has_source_span_for!(PropertyDef);

impl_references_for!(PropertyDef => delegate member);

impl_maybe_incomplete_for!(PropertyDef; delegate member);

impl AnnotationBuilder for PropertyDef {
    fn with_predicate<I, V>(self, predicate: I, value: V) -> Self
    where
        Self: Sized,
        I: Into<crate::model::identifiers::IdentifierReference>,
        V: Into<crate::model::values::Value>,
    {
        let mut self_mut = self;
        self_mut.member = self_mut.member.with_predicate(predicate, value);
        self_mut
    }
}

impl HasName for PropertyDef {
    fn name(&self) -> &Identifier {
        self.member.name()
    }

    fn set_name(&mut self, name: Identifier) {
        self.member.set_name(name);
    }
}

impl Validate for PropertyDef {
    fn validate(
        &self,
        top: &crate::model::modules::Module,
        cache: &impl crate::cache::ModuleStore,
        loader: &impl crate::load::ModuleLoader,
        check_constraints: bool,
    ) {
        self.member.validate(top, cache, loader, check_constraints)
    }
}

impl PropertyDef {
    pub fn new(member: MemberDef) -> Self {
        Self { span: None, member }
    }

    builder_fn!(pub with_member_def, member => MemberDef);

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    get_and_set!(pub member, member_def, set_member_def => MemberDef);
}
