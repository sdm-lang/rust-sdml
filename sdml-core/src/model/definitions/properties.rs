use crate::{
    load::ModuleLoader,
    model::{
        annotations::AnnotationBuilder,
        check::{MaybeIncomplete, Validate},
        identifiers::{Identifier, IdentifierReference},
        members::MemberDef,
        modules::Module,
        values::Value,
        HasName, HasSourceSpan, References, Span,
    },
    store::ModuleStore,
};
use std::{collections::HashSet, fmt::Debug};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Definitions ❱ Properties
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
// Implementations ❱ Definitions ❱ PropertyDef
// ------------------------------------------------------------------------------------------------

impl From<&MemberDef> for PropertyDef {
    fn from(member: &MemberDef) -> Self {
        Self::new(member.clone())
    }
}

impl From<MemberDef> for PropertyDef {
    fn from(member: MemberDef) -> Self {
        Self::new(member)
    }
}

impl HasSourceSpan for PropertyDef {
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

impl References for PropertyDef {
    fn referenced_annotations<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        self.member.referenced_annotations(names);
    }

    fn referenced_types<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        self.member.referenced_types(names);
    }
}

impl MaybeIncomplete for PropertyDef {
    fn is_incomplete(&self, top: &Module, cache: &impl ModuleStore) -> bool {
        self.member.is_incomplete(top, cache)
    }
}

impl AnnotationBuilder for PropertyDef {
    fn with_predicate<I, V>(self, predicate: I, value: V) -> Self
    where
        Self: Sized,
        I: Into<IdentifierReference>,
        V: Into<Value>,
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
        top: &Module,
        cache: &impl ModuleStore,
        loader: &impl ModuleLoader,
        check_constraints: bool,
    ) {
        self.member.validate(top, cache, loader, check_constraints)
    }
}

impl PropertyDef {
    // --------------------------------------------------------------------------------------------
    // Constructor
    // --------------------------------------------------------------------------------------------

    pub fn new(member: MemberDef) -> Self {
        Self { span: None, member }
    }

    pub fn with_member_def(self, member: MemberDef) -> Self {
        let mut self_mut = self;
        self_mut.member = member;
        self_mut
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    pub const fn member_def(&self) -> &MemberDef {
        &self.member
    }

    pub fn set_member_def(&mut self, member: MemberDef) {
        self.member = member;
    }
}
