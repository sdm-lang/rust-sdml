/*!
Rust types that model the SDML Grammar.

More detailed description, with

# Example

YYYYY

*/

use crate::error::{invalid_identifier_error, invalid_language_tag_error};
use lazy_static::lazy_static;
use ordered_float::OrderedFloat;
use regex::Regex;
use rust_decimal::Decimal;
use std::{
    collections::HashSet,
    fmt::{Debug, Display},
    hash::Hash,
    ops::Range,
    str::FromStr,
};
use tree_sitter::Node;
use url::Url;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Tree Reference
// ------------------------------------------------------------------------------------------------

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Span(Range<usize>);

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Comments
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct Comment {
    span: Option<Span>,
    value: String,
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Identifiers
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct Identifier {
    span: Option<Span>,
    value: String,
}

#[derive(Clone, Debug)]
pub struct QualifiedIdentifier {
    span: Option<Span>,
    module: Identifier,
    member: Identifier,
}

#[derive(Clone, Debug)]
pub enum IdentifierReference {
    Identifier(Identifier),
    QualifiedIdentifier(QualifiedIdentifier),
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Modules
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct Module {
    span: Option<Span>,
    comments: Vec<Comment>,
    name: Identifier,
    base: Option<Url>,
    body: ModuleBody,
}

#[derive(Clone, Debug, Default)]
pub struct ModuleBody {
    span: Option<Span>,
    comments: Vec<Comment>,
    imports: Vec<ImportStatement>,
    annotations: Vec<Annotation>,
    definitions: Vec<TypeDefinition>,
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Modules ❱ Imports
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, Default)]
pub struct ImportStatement {
    span: Option<Span>,
    comments: Vec<Comment>,
    imported: Vec<Import>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Import {
    Module(Identifier),
    Member(QualifiedIdentifier),
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Annotations
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct Annotation {
    span: Option<Span>,
    comments: Vec<Comment>,
    name: IdentifierReference,
    value: Value,
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Annotations ❱ Values
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub enum Value {
    Simple(SimpleValue),
    ValueConstructor(ValueConstructor),
    Reference(IdentifierReference),
    List(ListOfValues),
}

#[derive(Clone, Debug)]
pub enum SimpleValue {
    String(LanguageString),
    Double(OrderedFloat<f64>),
    Decimal(Decimal),
    Integer(i64),
    Boolean(bool),
    IriReference(Url),
}

#[derive(Clone, Debug)]
pub struct LanguageString {
    span: Option<Span>,
    value: String,
    language: Option<LanguageTag>,
}

#[derive(Clone, Debug)]
pub struct LanguageTag {
    span: Option<Span>,
    value: String,
}

#[derive(Clone, Debug, Default)]
pub struct ListOfValues {
    span: Option<Span>,
    values: Vec<ListMember>,
}

#[derive(Clone, Debug)]
pub enum ListMember {
    Simple(SimpleValue),
    ValueConstructor(ValueConstructor),
    Reference(IdentifierReference),
}

#[derive(Clone, Debug)]
pub struct ValueConstructor {
    span: Option<Span>,
    type_name: IdentifierReference,
    value: SimpleValue,
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub enum TypeDefinition {
    Datatype(DatatypeDef),
    Entity(EntityDef),
    Enum(EnumDef),
    Event(EventDef),
    Structure(StructureDef),
    Union(UnionDef),
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Datatypes
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct DatatypeDef {
    span: Option<Span>,
    comments: Vec<Comment>,
    name: Identifier,
    base_type: IdentifierReference,
    body: Option<AnnotationOnlyBody>,
}

#[derive(Clone, Debug, Default)]
pub struct AnnotationOnlyBody {
    span: Option<Span>,
    comments: Vec<Comment>,
    annotations: Vec<Annotation>,
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Entities
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct EntityDef {
    span: Option<Span>,
    comments: Vec<Comment>,
    name: Identifier,
    body: Option<EntityBody>,
}

#[derive(Clone, Debug)]
pub struct EntityBody {
    span: Option<Span>,
    comments: Vec<Comment>,
    identity: IdentityMember,
    annotations: Vec<Annotation>,
    members: Vec<EntityMember>,
    groups: Vec<EntityGroup>,
}

#[derive(Clone, Debug)]
pub enum EntityMember {
    ByValue(ByValueMember),
    ByReference(ByReferenceMember),
}

#[derive(Clone, Debug, Default)]
pub struct EntityGroup {
    span: Option<Span>,
    comments: Vec<Comment>,
    annotations: Vec<Annotation>,
    members: Vec<EntityMember>,
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Enumerations
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct EnumDef {
    span: Option<Span>,
    comments: Vec<Comment>,
    name: Identifier,
    body: Option<EnumBody>,
}

#[derive(Clone, Debug, Default)]
pub struct EnumBody {
    span: Option<Span>,
    comments: Vec<Comment>,
    annotations: Vec<Annotation>,
    variants: Vec<EnumVariant>,
}

#[derive(Clone, Debug)]
pub struct EnumVariant {
    span: Option<Span>,
    comments: Vec<Comment>,
    name: Identifier,
    value: u32,
    body: Option<AnnotationOnlyBody>,
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Events
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct EventDef {
    span: Option<Span>,
    comments: Vec<Comment>,
    name: Identifier,
    event_source: IdentifierReference,
    body: Option<StructureBody>,
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Structures
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct StructureDef {
    span: Option<Span>,
    comments: Vec<Comment>,
    name: Identifier,
    body: Option<StructureBody>,
}

#[derive(Clone, Debug, Default)]
pub struct StructureBody {
    span: Option<Span>,
    comments: Vec<Comment>,
    annotations: Vec<Annotation>,
    members: Vec<ByValueMember>,
    groups: Vec<StructureGroup>,
}

#[derive(Clone, Debug, Default)]
pub struct StructureGroup {
    span: Option<Span>,
    comments: Vec<Comment>,
    annotations: Vec<Annotation>,
    members: Vec<ByValueMember>,
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Unions
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct UnionDef {
    span: Option<Span>,
    comments: Vec<Comment>,
    name: Identifier,
    body: Option<UnionBody>,
}

#[derive(Clone, Debug, Default)]
pub struct UnionBody {
    span: Option<Span>,
    comments: Vec<Comment>,
    annotations: Vec<Annotation>,
    variants: Vec<TypeVariant>,
}

#[derive(Clone, Debug)]
pub struct TypeVariant {
    span: Option<Span>,
    comments: Vec<Comment>,
    name: IdentifierReference,
    rename: Option<Identifier>,
    body: Option<AnnotationOnlyBody>,
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Members
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct IdentityMember {
    span: Option<Span>,
    comments: Vec<Comment>,
    name: Identifier,
    target_type: TypeReference,
    body: Option<AnnotationOnlyBody>,
}

#[derive(Clone, Debug)]
pub struct ByValueMember {
    span: Option<Span>,
    comments: Vec<Comment>,
    name: Identifier,
    target_type: TypeReference,
    target_cardinality: Option<Cardinality>,
    body: Option<AnnotationOnlyBody>,
}

#[derive(Clone, Debug)]
pub struct ByReferenceMember {
    span: Option<Span>,
    comments: Vec<Comment>,
    name: Identifier,
    source_cardinality: Option<Cardinality>,
    target_type: TypeReference,
    target_cardinality: Option<Cardinality>,
    body: Option<AnnotationOnlyBody>,
}

#[derive(Clone, Debug)]
pub enum TypeReference {
    Reference(IdentifierReference),
    Unknown,
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Members ❱ Cardinality
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cardinality {
    span: Option<Span>,
    min: u32,
    max: Option<u32>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Macros ❱ Basic Get/Set
// ------------------------------------------------------------------------------------------------

macro_rules! delegate {
    ($fnname: ident, $fntype: ty, $fieldname: ident $(, $paramname: ident => $paramtype: ty)* ) => {
        pub fn $fnname(&self $(, $paramname: $paramtype)*) -> $fntype {
            self.$fieldname.$fnname($($paramname: $paramtype),*)
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Private Macros ❱ impl Display
// ------------------------------------------------------------------------------------------------

macro_rules! simple_display_impl {
    ($tyname: ty, $field: ident) => {
        impl std::fmt::Display for $tyname {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.$field)
            }
        }
    };
}

macro_rules! enum_display_impl {
    ($tyname: ty => $($varname: ident),+) => {
        impl std::fmt::Display for $tyname {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", match self {
                    $(
                        Self::$varname(v) => v.to_string(),
                    )+
                })
            }
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Private Macros ❱ impl Into/AsRef String
// ------------------------------------------------------------------------------------------------

macro_rules! into_string_impl {
    ($tyname: ty, $field: ident) => {
        impl From<$tyname> for String {
            fn from(v: $tyname) -> Self {
                v.value
            }
        }
    };
}

macro_rules! as_str_impl {
    ($tyname: ty, $field: ident) => {
        impl AsRef<str> for $tyname {
            fn as_ref(&self) -> &str {
                self.value.as_str()
            }
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Private Macros ❱ has Spans
// ------------------------------------------------------------------------------------------------

macro_rules! has_owned_ts_span {
    () => {
        pub fn with_ts_span(self, span: Span) -> Self {
            let mut self_mut = self;
            self_mut.span = Some(span);
            self_mut
        }

        pub fn has_ts_span(&self) -> bool {
            self.span.is_some()
        }

        pub fn ts_span(&self) -> Option<&Span> {
            self.span.as_ref()
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Private Macros ❱ has Comments
// ------------------------------------------------------------------------------------------------

macro_rules! has_owned_comments {
    () => {
        pub fn add_comment(&mut self, add: Comment) {
            self.comments.push(add);
        }

        pub fn extend_comments<I>(&mut self, extend: I)
        where
            I: IntoIterator<Item = Comment>,
        {
            self.comments.extend(extend);
        }

        pub fn has_comments(&self) -> bool {
            !self.comments.is_empty()
        }

        pub fn comments(&self) -> impl Iterator<Item = &Comment> {
            self.comments.iter()
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Private Macros ❱ has Annotations
// ------------------------------------------------------------------------------------------------

macro_rules! has_owned_annotations {
    () => {
        pub fn add_annotation(&mut self, add: Annotation) {
            self.annotations.push(add);
        }

        pub fn extend_annotations<I>(&mut self, extend: I)
        where
            I: IntoIterator<Item = Annotation>,
        {
            self.annotations.extend(extend);
        }

        pub fn has_annotations(&self) -> bool {
            !self.annotations.is_empty()
        }

        pub fn annotations(&self) -> impl Iterator<Item = &Annotation> {
            self.annotations.iter()
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Private Macros ❱ has Members
// ------------------------------------------------------------------------------------------------

macro_rules! has_owned_members {
    ($tymember: ty) => {
        pub fn add_member(&mut self, add: $tymember) {
            self.members.push(add);
        }

        pub fn extend_members<I>(&mut self, extend: I)
        where
            I: IntoIterator<Item = $tymember>,
        {
            self.members.extend(extend);
        }

        pub fn has_members(&self) -> bool {
            !self.members.is_empty()
        }

        pub fn members(&self) -> impl Iterator<Item = &$tymember> {
            self.members.iter()
        }

        pub fn referenced_types(&self) -> HashSet<&IdentifierReference> {
            self.members()
                .filter_map(|m| {
                    if let TypeReference::Reference(ty) = m.target_type() {
                        Some(ty)
                    } else {
                        None
                    }
                })
                .collect()
        }
    };
}

macro_rules! delegate_referenced_types {
    ($field: ident) => {
        pub fn referenced_types(&self) -> HashSet<&IdentifierReference> {
            self.$field
                .as_ref()
                .map(|b| b.referenced_types())
                .unwrap_or_default()
        }
    };
    () => {};
}

// ------------------------------------------------------------------------------------------------
// Private Macros ❱ has Groups
// ------------------------------------------------------------------------------------------------

macro_rules! has_owned_groups {
    ($grouptype: ty) => {
        pub fn add_group(&mut self, add: $grouptype) {
            self.groups.push(add);
        }

        pub fn extend_groups<I>(&mut self, extend: I)
        where
            I: IntoIterator<Item = $grouptype>,
        {
            self.groups.extend(extend);
        }

        pub fn has_groups(&self) -> bool {
            !self.groups.is_empty()
        }

        pub fn groups(&self) -> impl Iterator<Item = &$grouptype> {
            self.groups.iter()
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Private Macros ❱ has Body (Optional)
// ------------------------------------------------------------------------------------------------

macro_rules! has_optional_body {
    ($bodytype: ty) => {
        pub fn add_body(&mut self, body: $bodytype) {
            self.body = Some(body);
        }

        pub fn has_body(&self) -> bool {
            self.body.is_some()
        }

        pub fn body(&self) -> Option<&$bodytype> {
            self.body.as_ref()
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Private Macros ❱ isa Type Definition
// ------------------------------------------------------------------------------------------------

macro_rules! type_definition_impl {
    ($bodytype: ty $(, $flname: ident, $fltype: ty )*) => {
        pub fn new(name: Identifier $(, $flname: $fltype )*) -> Self {
            Self {
                span: None,
                comments: Default::default(),
                name,
                $(
                    $flname,
                ),*
                    body: None,
            }
        }

        pub fn name(&self) -> &Identifier {
            &self.name
        }

        $(
            pub fn $flname(&self) -> &$fltype {
                &self.$flname
            }
        )*

        has_owned_ts_span!();

        has_owned_comments!();

        has_optional_body!($bodytype);
    };
}

// ------------------------------------------------------------------------------------------------
// Private Macros ❱ isa Member
// ------------------------------------------------------------------------------------------------

macro_rules! member_impl {
    ($($optional: ident, $opttype: ty ),*) => {
        pub fn new(name: Identifier, target_type: TypeReference) -> Self {
            Self {
                span: None,
                comments: Default::default(),
                name,
                target_type,
                body: None
                    $(,
                      $optional: None
                    )*
            }
        }

        pub fn new_unknown(name: Identifier) -> Self {
            Self::new(name, TypeReference::Unknown)
        }


        pub fn name(&self) -> &Identifier {
            &self.name
        }

        pub fn target_type(&self) -> &TypeReference {
            &self.target_type
        }

        $(
            pub fn $optional(&self) -> Option<&$opttype> {
                self.$optional.as_ref()
            }
        )*

        has_owned_ts_span!();

        has_owned_comments!();

        has_optional_body!(AnnotationOnlyBody);
    };
}

// ------------------------------------------------------------------------------------------------
// Private Macros ❱ is_complete
// ------------------------------------------------------------------------------------------------

macro_rules! is_body_complete_fn {
    () => {
        pub fn is_complete(&self) -> bool {
            self.body
                .as_ref()
                .map(|b| b.is_complete())
                .unwrap_or_default()
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

macro_rules! check_and_add_comment {
    ($context: ident, $node: ident, $parent: ident) => {
        if $context.save_comments() {
            let comment = Comment::new($context.node_source(&$node)?).with_ts_span($node.into());
            $parent.add_comment(comment);
        } else {
            trace!("not saving comments");
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

lazy_static! {
    static ref IDENTIFIER: Regex = Regex::new(r"^[\p{Lu}\p{Ll}]+(_[\p{Lu}\p{Ll}]+)*$").unwrap();
    static ref LANGUAGE_TAG: Regex =
        Regex::new(r"^[a-z]{2,3}(-[A-Z]{3})?(-[A-Z][a-z]{3})?(-([A-Z]{2}|[0-9]{3}))?$").unwrap();
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Comments
// ------------------------------------------------------------------------------------------------

impl From<String> for Comment {
    fn from(v: String) -> Self {
        Self::new(&v)
    }
}

impl From<&str> for Comment {
    fn from(v: &str) -> Self {
        Self::new(v)
    }
}

simple_display_impl!(Comment, value);
as_str_impl!(Comment, value);
into_string_impl!(Comment, value);

impl PartialEq for Comment {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Comment {}

impl Comment {
    pub fn new(s: &str) -> Self {
        Self {
            span: None,
            value: s.to_string(),
        }
    }

    has_owned_ts_span!();

    pub fn eq_with_span(&self, other: &Self) -> bool {
        self.span == other.span && self.value == other.value
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Identifiers
// ------------------------------------------------------------------------------------------------

const RESERVED_KEYWORDS: [&str; 18] = [
    "as",
    "base",
    "datatype",
    "end",
    "entity",
    "enum",
    "event",
    "group",
    "identity",
    "import",
    "is",
    "module",
    "of",
    "ref",
    "source",
    "structure",
    "union",
    "unknown",
];
const RESERVED_TYPES: [&str; 6] = ["string", "double", "decimal", "integer", "boolean", "iri"];
const RESERVED_MODULES: [&str; 6] = ["owl", "rdf", "rdfs", "sdml", "xml", "xsd"];

impl FromStr for Identifier {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Self::is_valid(s) {
            Ok(Self {
                span: None,
                value: s.to_string(),
            })
        } else {
            Err(invalid_identifier_error(s))
        }
    }
}

simple_display_impl!(Identifier, value);
as_str_impl!(Identifier, value);
into_string_impl!(Identifier, value);

impl PartialEq for Identifier {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Identifier {}

impl Hash for Identifier {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // ignore: self.span.hash(state);
        self.value.hash(state);
    }
}

impl Identifier {
    pub fn new_unchecked(s: &str) -> Self {
        Self {
            span: None,
            value: s.to_string(),
        }
    }

    pub fn with_module(&self, module: Identifier) -> QualifiedIdentifier {
        QualifiedIdentifier::new(module, self.clone())
    }

    pub fn with_member(&self, member: Identifier) -> QualifiedIdentifier {
        QualifiedIdentifier::new(self.clone(), member)
    }

    has_owned_ts_span!();

    #[inline(always)]
    pub fn is_valid(s: &str) -> bool {
        IDENTIFIER.is_match(s) && !Self::is_keyword(s) && !Self::is_type_name(s)
    }

    #[inline(always)]
    pub fn is_keyword(s: &str) -> bool {
        RESERVED_KEYWORDS.contains(&s)
    }

    #[inline(always)]
    pub fn is_type_name(s: &str) -> bool {
        RESERVED_TYPES.contains(&s)
    }

    #[inline(always)]
    pub fn is_reserved_module_name(s: &str) -> bool {
        RESERVED_MODULES.contains(&s)
    }

    #[inline(always)]
    pub fn eq_with_span(&self, other: &Self) -> bool {
        self.span == other.span && self.value == other.value
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for QualifiedIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.module, self.member)
    }
}

impl PartialEq for QualifiedIdentifier {
    fn eq(&self, other: &Self) -> bool {
        self.module == other.module && self.member == other.member
    }
}

impl Eq for QualifiedIdentifier {}

impl Hash for QualifiedIdentifier {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // ignore: self.span.hash(state);
        self.module.hash(state);
        self.member.hash(state);
    }
}

impl QualifiedIdentifier {
    pub fn new(module: Identifier, member: Identifier) -> Self {
        Self {
            span: None,
            module,
            member,
        }
    }

    has_owned_ts_span!();

    pub fn module(&self) -> &Identifier {
        &self.module
    }

    pub fn member(&self) -> &Identifier {
        &self.member
    }

    pub fn eq_with_span(&self, other: &Self) -> bool {
        self.span == other.span && self.module == other.module && self.member == other.member
    }
}

// ------------------------------------------------------------------------------------------------

impl From<Identifier> for IdentifierReference {
    fn from(v: Identifier) -> Self {
        Self::Identifier(v)
    }
}

impl From<QualifiedIdentifier> for IdentifierReference {
    fn from(v: QualifiedIdentifier) -> Self {
        Self::QualifiedIdentifier(v)
    }
}

enum_display_impl!(IdentifierReference => Identifier, QualifiedIdentifier);

impl PartialEq for IdentifierReference {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Identifier(l0), Self::Identifier(r0)) => l0.eq(r0),
            (Self::QualifiedIdentifier(l0), Self::QualifiedIdentifier(r0)) => l0.eq(r0),
            _ => false,
        }
    }
}

impl Eq for IdentifierReference {}

impl Hash for IdentifierReference {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

impl IdentifierReference {
    pub fn has_ts_span(&self) -> bool {
        match self {
            Self::Identifier(v) => v.has_ts_span(),
            Self::QualifiedIdentifier(v) => v.has_ts_span(),
        }
    }

    pub fn ts_span(&self) -> Option<&Span> {
        match self {
            Self::Identifier(v) => v.ts_span(),
            Self::QualifiedIdentifier(v) => v.ts_span(),
        }
    }

    pub fn eq_with_span(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Identifier(l0), Self::Identifier(r0)) => l0.eq_with_span(r0),
            (Self::QualifiedIdentifier(l0), Self::QualifiedIdentifier(r0)) => l0.eq_with_span(r0),
            _ => false,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Modules
// ------------------------------------------------------------------------------------------------

impl Module {
    pub fn new(name: Identifier, body: ModuleBody) -> Self {
        Self {
            span: None,
            comments: Default::default(),
            name,
            base: None,
            body,
        }
    }
    pub fn new_with_base(name: Identifier, base: Url, body: ModuleBody) -> Self {
        Self {
            span: None,
            comments: Default::default(),
            name,
            base: Some(base),
            body,
        }
    }

    has_owned_ts_span!();

    has_owned_comments!();

    pub fn name(&self) -> &Identifier {
        &self.name
    }

    pub fn base(&self) -> Option<&Url> {
        self.base.as_ref()
    }

    pub fn body(&self) -> &ModuleBody {
        &self.body
    }

    delegate!(imported_modules, HashSet<&Identifier>, body);

    delegate!(imported_types, HashSet<&QualifiedIdentifier>, body);

    delegate!(declared_types, HashSet<&Identifier>, body);

    delegate!(referenced_types, HashSet<&IdentifierReference>, body);

    delegate!(referenced_annotations, HashSet<&IdentifierReference>, body);

    pub fn is_complete(&self) -> bool {
        self.body.is_complete()
    }
}

// ------------------------------------------------------------------------------------------------

impl ModuleBody {
    has_owned_ts_span!();

    has_owned_comments!();

    has_owned_annotations!();

    pub fn has_imports(&self) -> bool {
        !self.imports.is_empty()
    }

    pub fn add_import(&mut self, add: ImportStatement) {
        self.imports.push(add);
    }

    pub fn extend_imports<I>(&mut self, extend: I)
    where
        I: IntoIterator<Item = ImportStatement>,
    {
        self.imports.extend(extend);
    }

    pub fn imports(&self) -> impl Iterator<Item = &ImportStatement> {
        self.imports.iter()
    }

    pub fn has_definitions(&self) -> bool {
        !self.definitions.is_empty()
    }

    pub fn add_definition(&mut self, add: TypeDefinition) {
        self.definitions.push(add);
    }

    pub fn extend_definitions<I>(&mut self, extend: I)
    where
        I: IntoIterator<Item = TypeDefinition>,
    {
        self.definitions.extend(extend);
    }

    pub fn definitions(&self) -> impl Iterator<Item = &TypeDefinition> {
        self.definitions.iter()
    }

    pub fn is_complete(&self) -> bool {
        self.definitions().all(|d| d.is_complete())
    }

    pub fn imported_modules(&self) -> HashSet<&Identifier> {
        self.imports()
            .flat_map(|stmt| stmt.imported_modules())
            .collect()
    }

    pub fn imported_types(&self) -> HashSet<&QualifiedIdentifier> {
        self.imports()
            .flat_map(|stmt| stmt.imported_types())
            .collect()
    }

    pub fn declared_types(&self) -> HashSet<&Identifier> {
        self.definitions().map(|def| def.name()).collect()
    }

    pub fn referenced_types(&self) -> HashSet<&IdentifierReference> {
        self.definitions()
            .flat_map(|def| def.referenced_types())
            .collect()
    }

    pub fn referenced_annotations(&self) -> HashSet<&IdentifierReference> {
        self.definitions()
            .flat_map(|def| def.referenced_annotations())
            .collect()
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Modules ❱ Imports
// ------------------------------------------------------------------------------------------------

impl FromIterator<Import> for ImportStatement {
    fn from_iter<T: IntoIterator<Item = Import>>(iter: T) -> Self {
        Self::new(Vec::from_iter(iter))
    }
}

impl ImportStatement {
    pub fn new(imported: Vec<Import>) -> Self {
        Self {
            span: None,
            comments: Default::default(),
            imported,
        }
    }

    has_owned_ts_span!();

    has_owned_comments!();

    pub fn has_imports(&self) -> bool {
        !self.imported.is_empty()
    }

    pub fn add_import(&mut self, add: Import) {
        self.imported.push(add);
    }

    pub fn extend_imports<I>(&mut self, extend: I)
    where
        I: IntoIterator<Item = Import>,
    {
        self.imported.extend(extend);
    }

    pub fn imports(&self) -> impl Iterator<Item = &Import> {
        self.imported.iter()
    }

    pub(crate) fn as_slice(&self) -> &[Import] {
        self.imported.as_slice()
    }

    pub fn imported_modules(&self) -> HashSet<&Identifier> {
        self.imports()
            .map(|imp| match imp {
                Import::Module(v) => v,
                Import::Member(v) => v.module(),
            })
            .collect()
    }

    pub fn imported_types(&self) -> HashSet<&QualifiedIdentifier> {
        self.imports()
            .filter_map(|imp| {
                if let Import::Member(imp) = imp {
                    Some(imp)
                } else {
                    None
                }
            })
            .collect()
    }
}

// ------------------------------------------------------------------------------------------------

impl From<Identifier> for Import {
    fn from(v: Identifier) -> Self {
        Self::Module(v)
    }
}

impl From<QualifiedIdentifier> for Import {
    fn from(v: QualifiedIdentifier) -> Self {
        Self::Member(v)
    }
}

enum_display_impl!(Import => Module, Member);

impl Import {
    pub fn has_ts_span(&self) -> bool {
        match self {
            Self::Module(v) => v.has_ts_span(),
            Self::Member(v) => v.has_ts_span(),
        }
    }

    pub fn ts_span(&self) -> Option<&Span> {
        match self {
            Self::Module(v) => v.ts_span(),
            Self::Member(v) => v.ts_span(),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Annotations
// ------------------------------------------------------------------------------------------------

impl Annotation {
    pub fn new(name: IdentifierReference, value: Value) -> Self {
        Self {
            span: None,
            comments: Default::default(),
            name,
            value,
        }
    }

    has_owned_ts_span!();

    has_owned_comments!();

    pub fn name(&self) -> &IdentifierReference {
        &self.name
    }

    pub fn value(&self) -> &Value {
        &self.value
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Annotations ❱ Values
// ------------------------------------------------------------------------------------------------

impl From<SimpleValue> for Value {
    fn from(v: SimpleValue) -> Self {
        Self::Simple(v)
    }
}

impl From<LanguageString> for Value {
    fn from(v: LanguageString) -> Self {
        Self::Simple(SimpleValue::String(v))
    }
}

impl From<f64> for Value {
    fn from(v: f64) -> Self {
        Self::Simple(SimpleValue::Double(v.into()))
    }
}

impl From<OrderedFloat<f64>> for Value {
    fn from(v: OrderedFloat<f64>) -> Self {
        Self::Simple(SimpleValue::Double(v))
    }
}

impl From<Decimal> for Value {
    fn from(v: Decimal) -> Self {
        Self::Simple(SimpleValue::Decimal(v))
    }
}

impl From<i64> for Value {
    fn from(v: i64) -> Self {
        Self::Simple(SimpleValue::Integer(v))
    }
}

impl From<bool> for Value {
    fn from(v: bool) -> Self {
        Self::Simple(SimpleValue::Boolean(v))
    }
}

impl From<Url> for Value {
    fn from(v: Url) -> Self {
        Self::Simple(SimpleValue::IriReference(v))
    }
}

impl From<ValueConstructor> for Value {
    fn from(v: ValueConstructor) -> Self {
        Self::ValueConstructor(v)
    }
}

impl From<IdentifierReference> for Value {
    fn from(v: IdentifierReference) -> Self {
        Self::Reference(v)
    }
}

impl From<ListOfValues> for Value {
    fn from(v: ListOfValues) -> Self {
        Self::List(v)
    }
}

// ------------------------------------------------------------------------------------------------

impl From<LanguageString> for SimpleValue {
    fn from(v: LanguageString) -> Self {
        Self::String(v)
    }
}

impl From<f64> for SimpleValue {
    fn from(v: f64) -> Self {
        Self::Double(v.into())
    }
}

impl From<OrderedFloat<f64>> for SimpleValue {
    fn from(v: OrderedFloat<f64>) -> Self {
        Self::Double(v)
    }
}

impl From<Decimal> for SimpleValue {
    fn from(v: Decimal) -> Self {
        Self::Decimal(v)
    }
}

impl From<i64> for SimpleValue {
    fn from(v: i64) -> Self {
        Self::Integer(v)
    }
}

impl From<bool> for SimpleValue {
    fn from(v: bool) -> Self {
        Self::Boolean(v)
    }
}

impl From<Url> for SimpleValue {
    fn from(v: Url) -> Self {
        Self::IriReference(v)
    }
}

impl Display for SimpleValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::String(v) => v.to_string(),
                Self::Double(v) => v.to_string(),
                Self::Decimal(v) => v.to_string(),
                Self::Integer(v) => v.to_string(),
                Self::Boolean(v) => v.to_string(),
                Self::IriReference(v) => v.to_string(),
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for LanguageString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}{}",
            self.value,
            if let Some(language) = &self.language {
                language.to_string()
            } else {
                String::new()
            }
        )
    }
}

impl From<String> for LanguageString {
    fn from(v: String) -> Self {
        Self::new(&v, None)
    }
}

impl From<&str> for LanguageString {
    fn from(v: &str) -> Self {
        Self::new(v, None)
    }
}

impl PartialEq for LanguageString {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.language == other.language
    }
}

impl Eq for LanguageString {}

impl LanguageString {
    pub(crate) fn new(value: &str, language: Option<LanguageTag>) -> Self {
        Self {
            span: None,
            value: value.to_string(),
            language,
        }
    }

    has_owned_ts_span!();

    pub fn value(&self) -> &String {
        &self.value
    }

    pub fn language(&self) -> Option<&LanguageTag> {
        self.language.as_ref()
    }

    pub fn eq_with_span(&self, other: &Self) -> bool {
        self.span == other.span && self.value == other.value && self.language == other.language
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for LanguageTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "@{}", self.value)
    }
}

impl FromStr for LanguageTag {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Self::is_valid(s) {
            Ok(Self {
                span: None,
                value: s.to_string(),
            })
        } else {
            Err(invalid_language_tag_error(s))
        }
    }
}

into_string_impl!(LanguageTag, value);
as_str_impl!(LanguageTag, value);

impl PartialEq for LanguageTag {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for LanguageTag {}

impl LanguageTag {
    #[allow(dead_code)]
    pub(crate) fn new_unchecked(s: &str) -> Self {
        Self {
            span: None,
            value: s.to_string(),
        }
    }

    has_owned_ts_span!();

    pub fn is_valid(s: &str) -> bool {
        LANGUAGE_TAG.is_match(s)
    }

    pub fn eq_with_span(&self, other: &Self) -> bool {
        self.span == other.span && self.value == other.value
    }
}

// ------------------------------------------------------------------------------------------------

impl From<Vec<ListMember>> for ListOfValues {
    fn from(values: Vec<ListMember>) -> Self {
        Self { span: None, values }
    }
}

impl FromIterator<ListMember> for ListOfValues {
    fn from_iter<T: IntoIterator<Item = ListMember>>(iter: T) -> Self {
        Self::from(Vec::from_iter(iter))
    }
}

impl ListOfValues {
    has_owned_ts_span!();

    pub fn add_value(&mut self, add: ListMember) {
        self.values.push(add);
    }

    pub fn extend_values<I>(&mut self, extend: I)
    where
        I: IntoIterator<Item = ListMember>,
    {
        self.values.extend(extend);
    }

    pub fn has_value(&self) -> bool {
        !self.values.is_empty()
    }

    pub fn values(&self) -> impl Iterator<Item = &ListMember> {
        self.values.iter()
    }
}

// ------------------------------------------------------------------------------------------------

impl From<SimpleValue> for ListMember {
    fn from(v: SimpleValue) -> Self {
        Self::Simple(v)
    }
}

impl From<ValueConstructor> for ListMember {
    fn from(v: ValueConstructor) -> Self {
        Self::ValueConstructor(v)
    }
}

impl From<IdentifierReference> for ListMember {
    fn from(v: IdentifierReference) -> Self {
        Self::Reference(v)
    }
}

// ------------------------------------------------------------------------------------------------

impl ValueConstructor {
    pub fn new(type_name: IdentifierReference, value: SimpleValue) -> Self {
        Self {
            span: None,
            type_name,
            value,
        }
    }

    has_owned_ts_span!();

    pub fn type_name(&self) -> &IdentifierReference {
        &self.type_name
    }

    pub fn value(&self) -> &SimpleValue {
        &self.value
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Type Definitions
// ------------------------------------------------------------------------------------------------

impl From<DatatypeDef> for TypeDefinition {
    fn from(v: DatatypeDef) -> Self {
        Self::Datatype(v)
    }
}

impl From<EntityDef> for TypeDefinition {
    fn from(v: EntityDef) -> Self {
        Self::Entity(v)
    }
}

impl From<EnumDef> for TypeDefinition {
    fn from(v: EnumDef) -> Self {
        Self::Enum(v)
    }
}

impl From<EventDef> for TypeDefinition {
    fn from(v: EventDef) -> Self {
        Self::Event(v)
    }
}

impl From<StructureDef> for TypeDefinition {
    fn from(v: StructureDef) -> Self {
        Self::Structure(v)
    }
}

impl From<UnionDef> for TypeDefinition {
    fn from(v: UnionDef) -> Self {
        Self::Union(v)
    }
}

impl TypeDefinition {
    pub fn name(&self) -> &Identifier {
        match self {
            Self::Datatype(v) => v.name(),
            Self::Entity(v) => v.name(),
            Self::Enum(v) => v.name(),
            Self::Event(v) => v.name(),
            Self::Structure(v) => v.name(),
            Self::Union(v) => v.name(),
        }
    }

    pub fn referenced_types(&self) -> HashSet<&IdentifierReference> {
        match self {
            Self::Datatype(v) => v.referenced_types(),
            Self::Entity(v) => v.referenced_types(),
            Self::Enum(v) => v.referenced_types(),
            Self::Event(v) => v.referenced_types(),
            Self::Structure(v) => v.referenced_types(),
            Self::Union(v) => v.referenced_types(),
        }
    }

    pub fn referenced_annotations(&self) -> HashSet<&IdentifierReference> {
        match self {
            Self::Datatype(v) => v.referenced_annotations(),
            Self::Entity(v) => v.referenced_annotations(),
            Self::Enum(v) => v.referenced_annotations(),
            Self::Event(v) => v.referenced_annotations(),
            Self::Structure(v) => v.referenced_annotations(),
            Self::Union(v) => v.referenced_annotations(),
        }
    }

    pub fn is_complete(&self) -> bool {
        match self {
            Self::Datatype(v) => v.is_complete(),
            Self::Entity(v) => v.is_complete(),
            Self::Enum(v) => v.is_complete(),
            Self::Event(v) => v.is_complete(),
            Self::Structure(v) => v.is_complete(),
            Self::Union(v) => v.is_complete(),
        }
    }

    pub fn has_ts_span(&self) -> bool {
        match self {
            Self::Datatype(v) => v.has_ts_span(),
            Self::Entity(v) => v.has_ts_span(),
            Self::Enum(v) => v.has_ts_span(),
            Self::Event(v) => v.has_ts_span(),
            Self::Structure(v) => v.has_ts_span(),
            Self::Union(v) => v.has_ts_span(),
        }
    }

    pub fn ts_span(&self) -> Option<&Span> {
        match self {
            Self::Datatype(v) => v.ts_span(),
            Self::Entity(v) => v.ts_span(),
            Self::Enum(v) => v.ts_span(),
            Self::Event(v) => v.ts_span(),
            Self::Structure(v) => v.ts_span(),
            Self::Union(v) => v.ts_span(),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Datatypes
// ------------------------------------------------------------------------------------------------

impl DatatypeDef {
    type_definition_impl!(AnnotationOnlyBody, base_type, IdentifierReference);

    pub fn referenced_annotations(&self) -> HashSet<&IdentifierReference> {
        self.body
            .as_ref()
            .map(|b| b.referenced_annotations())
            .unwrap_or_default()
    }

    pub fn referenced_types(&self) -> HashSet<&IdentifierReference> {
        [self.base_type()].into_iter().collect()
    }

    pub fn is_complete(&self) -> bool {
        true
    }
}

// ------------------------------------------------------------------------------------------------

impl AnnotationOnlyBody {
    has_owned_ts_span!();

    has_owned_comments!();

    has_owned_annotations!();

    pub fn referenced_annotations(&self) -> HashSet<&IdentifierReference> {
        self.annotations().map(|a| a.name()).collect()
    }
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Entities
// ------------------------------------------------------------------------------------------------

impl EntityDef {
    type_definition_impl!(EntityBody);

    pub fn referenced_annotations(&self) -> HashSet<&IdentifierReference> {
        self.body
            .as_ref()
            .map(|b| b.referenced_annotations())
            .unwrap_or_default()
    }

    delegate_referenced_types!(body);

    is_body_complete_fn!();
}

// ------------------------------------------------------------------------------------------------

impl EntityBody {
    pub fn new(identity: IdentityMember) -> Self {
        Self {
            span: None,
            comments: Default::default(),
            identity,
            annotations: Default::default(),
            members: Default::default(),
            groups: Default::default(),
        }
    }

    has_owned_ts_span!();

    has_owned_comments!();

    has_owned_annotations!();

    pub fn referenced_annotations(&self) -> HashSet<&IdentifierReference> {
        todo!()
    }

    has_owned_members!(EntityMember);

    has_owned_groups!(EntityGroup);

    delegate_referenced_types!();

    pub fn identity(&self) -> &IdentityMember {
        &self.identity
    }

    pub fn is_complete(&self) -> bool {
        self.members().all(|m| m.is_complete()) && self.groups().all(|m| m.is_complete())
    }
}

// ------------------------------------------------------------------------------------------------

impl From<ByValueMember> for EntityMember {
    fn from(v: ByValueMember) -> Self {
        Self::ByValue(v)
    }
}

impl From<ByReferenceMember> for EntityMember {
    fn from(v: ByReferenceMember) -> Self {
        Self::ByReference(v)
    }
}

impl EntityMember {
    pub fn name(&self) -> &Identifier {
        match self {
            EntityMember::ByValue(v) => v.name(),
            EntityMember::ByReference(v) => v.name(),
        }
    }

    pub fn target_type(&self) -> &TypeReference {
        match self {
            EntityMember::ByValue(v) => v.target_type(),
            EntityMember::ByReference(v) => v.target_type(),
        }
    }

    pub fn is_complete(&self) -> bool {
        match self {
            EntityMember::ByValue(v) => v.is_complete(),
            EntityMember::ByReference(v) => v.is_complete(),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl EntityGroup {
    has_owned_ts_span!();

    has_owned_comments!();

    has_owned_annotations!();

    has_owned_members!(EntityMember);

    pub fn is_complete(&self) -> bool {
        self.members().all(|m| m.is_complete())
    }
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Enumerations
// ------------------------------------------------------------------------------------------------

impl EnumDef {
    type_definition_impl!(EnumBody);

    pub fn referenced_annotations(&self) -> HashSet<&IdentifierReference> {
        self.body
            .as_ref()
            .map(|b| b.referenced_annotations())
            .unwrap_or_default()
    }

    pub fn is_complete(&self) -> bool {
        self.body.is_some()
    }

    pub fn referenced_types(&self) -> HashSet<&IdentifierReference> {
        Default::default()
    }
}

// ------------------------------------------------------------------------------------------------

impl EnumBody {
    has_owned_ts_span!();

    has_owned_comments!();

    has_owned_annotations!();

    pub fn referenced_annotations(&self) -> HashSet<&IdentifierReference> {
        let mut body: HashSet<&IdentifierReference> =
            self.annotations().map(|a| a.name()).collect();
        let variants: HashSet<&IdentifierReference> = self
            .variants()
            .flat_map(|v| v.referenced_annotations())
            .collect();
        body.extend(variants);
        body
    }

    pub fn has_variants(&self) -> bool {
        !self.variants.is_empty()
    }

    pub fn add_variant(&mut self, variant: EnumVariant) {
        self.variants.push(variant);
    }

    pub fn extend_variants<I>(&mut self, extend: I)
    where
        I: IntoIterator<Item = EnumVariant>,
    {
        self.variants.extend(extend);
    }

    pub fn variants(&self) -> impl Iterator<Item = &EnumVariant> {
        self.variants.iter()
    }

    pub fn referenced_types(&self) -> HashSet<&IdentifierReference> {
        Default::default()
    }
}

// ------------------------------------------------------------------------------------------------

impl EnumVariant {
    pub fn new(name: Identifier, value: u32) -> Self {
        Self {
            span: None,
            comments: Default::default(),
            name,
            value,
            body: None,
        }
    }

    pub fn new_with(name: Identifier, value: u32, body: AnnotationOnlyBody) -> Self {
        Self {
            span: None,
            comments: Default::default(),
            name,
            value,
            body: Some(body),
        }
    }

    has_owned_ts_span!();

    has_owned_comments!();

    has_optional_body!(AnnotationOnlyBody);

    pub fn referenced_annotations(&self) -> HashSet<&IdentifierReference> {
        self.body
            .as_ref()
            .map(|b| b.referenced_annotations())
            .unwrap_or_default()
    }

    pub fn name(&self) -> &Identifier {
        &self.name
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Events
// ------------------------------------------------------------------------------------------------

impl EventDef {
    type_definition_impl!(StructureBody, event_source, IdentifierReference);

    pub fn referenced_annotations(&self) -> HashSet<&IdentifierReference> {
        self.body
            .as_ref()
            .map(|b| b.referenced_annotations())
            .unwrap_or_default()
    }

    delegate_referenced_types!(body);

    is_body_complete_fn!();
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Structures
// ------------------------------------------------------------------------------------------------

impl StructureDef {
    type_definition_impl!(StructureBody);

    pub fn referenced_annotations(&self) -> HashSet<&IdentifierReference> {
        self.body
            .as_ref()
            .map(|b| b.referenced_annotations())
            .unwrap_or_default()
    }

    delegate_referenced_types!(body);

    is_body_complete_fn!();
}

// ------------------------------------------------------------------------------------------------

impl StructureBody {
    has_owned_ts_span!();

    has_owned_comments!();

    has_owned_annotations!();

    has_owned_members!(ByValueMember);

    has_owned_groups!(StructureGroup);

    pub fn referenced_annotations(&self) -> HashSet<&IdentifierReference> {
        todo!()
    }

    pub fn is_complete(&self) -> bool {
        self.members().all(|m| m.is_complete()) && self.groups().all(|m| m.is_complete())
    }
}

// ------------------------------------------------------------------------------------------------

impl StructureGroup {
    has_owned_ts_span!();

    has_owned_comments!();

    has_owned_annotations!();

    has_owned_members!(ByValueMember);

    pub fn is_complete(&self) -> bool {
        self.members().all(|m| m.is_complete())
    }
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Unions
// ------------------------------------------------------------------------------------------------

impl UnionDef {
    type_definition_impl!(UnionBody);

    pub fn referenced_annotations(&self) -> HashSet<&IdentifierReference> {
        self.body
            .as_ref()
            .map(|b| b.referenced_annotations())
            .unwrap_or_default()
    }

    pub fn is_complete(&self) -> bool {
        self.body.is_some()
    }

    pub fn referenced_types(&self) -> HashSet<&IdentifierReference> {
        self.body()
            .map(|b| b.referenced_types())
            .unwrap_or_default()
    }
}

// ------------------------------------------------------------------------------------------------

impl UnionBody {
    has_owned_ts_span!();

    has_owned_comments!();

    has_owned_annotations!();

    pub fn referenced_annotations(&self) -> HashSet<&IdentifierReference> {
        todo!()
    }

    pub fn has_variants(&self) -> bool {
        !self.variants.is_empty()
    }

    pub fn add_variant(&mut self, variant: TypeVariant) {
        self.variants.push(variant);
    }

    pub fn extend_variants<I>(&mut self, extend: I)
    where
        I: IntoIterator<Item = TypeVariant>,
    {
        self.variants.extend(extend);
    }

    pub fn variants(&self) -> impl Iterator<Item = &TypeVariant> {
        self.variants.iter()
    }

    pub fn referenced_types(&self) -> HashSet<&IdentifierReference> {
        self.variants().flat_map(|m| m.referenced_types()).collect()
    }
}

// ------------------------------------------------------------------------------------------------

impl TypeVariant {
    has_owned_ts_span!();

    has_owned_comments!();

    has_optional_body!(AnnotationOnlyBody);

    pub fn referenced_annotations(&self) -> HashSet<&IdentifierReference> {
        self.body
            .as_ref()
            .map(|b| b.referenced_annotations())
            .unwrap_or_default()
    }

    pub fn new(name: IdentifierReference) -> Self {
        Self {
            span: None,
            comments: Default::default(),
            name,
            rename: None,
            body: None,
        }
    }

    pub fn new_with(name: IdentifierReference, body: AnnotationOnlyBody) -> Self {
        Self {
            span: None,
            comments: Default::default(),
            name,
            rename: None,
            body: Some(body),
        }
    }

    pub fn with_rename(self, rename: Identifier) -> Self {
        let mut self_mut = self;
        self_mut.rename = Some(rename);
        self_mut
    }

    pub fn name(&self) -> &IdentifierReference {
        &self.name
    }

    pub fn rename(&self) -> Option<&Identifier> {
        self.rename.as_ref()
    }

    pub fn referenced_types(&self) -> HashSet<&IdentifierReference> {
        [&self.name].into_iter().collect()
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Members
// ------------------------------------------------------------------------------------------------

impl IdentityMember {
    member_impl!();

    pub fn referenced_annotations(&self) -> HashSet<&IdentifierReference> {
        self.body
            .as_ref()
            .map(|b| b.referenced_annotations())
            .unwrap_or_default()
    }

    pub fn is_complete(&self) -> bool {
        self.target_type().is_complete()
    }
}

// ------------------------------------------------------------------------------------------------

impl ByValueMember {
    member_impl!(target_cardinality, Cardinality);

    pub fn referenced_annotations(&self) -> HashSet<&IdentifierReference> {
        self.body
            .as_ref()
            .map(|b| b.referenced_annotations())
            .unwrap_or_default()
    }

    pub fn set_target_cardinality(&mut self, cardinality: Cardinality) {
        self.target_cardinality = Some(cardinality);
    }

    pub fn is_complete(&self) -> bool {
        self.target_type().is_complete()
    }
}

// ------------------------------------------------------------------------------------------------

impl ByReferenceMember {
    member_impl!(
        source_cardinality,
        Cardinality,
        target_cardinality,
        Cardinality
    );

    pub fn referenced_annotations(&self) -> HashSet<&IdentifierReference> {
        self.body
            .as_ref()
            .map(|b| b.referenced_annotations())
            .unwrap_or_default()
    }

    pub fn set_source_cardinality(&mut self, cardinality: Cardinality) {
        self.source_cardinality = Some(cardinality);
    }

    pub fn set_target_cardinality(&mut self, cardinality: Cardinality) {
        self.target_cardinality = Some(cardinality);
    }

    pub fn is_complete(&self) -> bool {
        self.target_type().is_complete()
    }
}

// ------------------------------------------------------------------------------------------------

impl From<Identifier> for TypeReference {
    fn from(value: Identifier) -> Self {
        Self::Reference(value.into())
    }
}

impl From<QualifiedIdentifier> for TypeReference {
    fn from(value: QualifiedIdentifier) -> Self {
        Self::Reference(value.into())
    }
}

impl From<IdentifierReference> for TypeReference {
    fn from(value: IdentifierReference) -> Self {
        Self::Reference(value)
    }
}

impl TypeReference {
    pub fn is_complete(&self) -> bool {
        matches!(self, Self::Reference(_))
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Members ❱ Cardinality
// ------------------------------------------------------------------------------------------------

impl Cardinality {
    pub fn new_range(min: u32, max: u32) -> Self {
        Self {
            span: None,
            min,
            max: Some(max),
        }
    }

    pub fn new_unbounded(min: u32) -> Self {
        Self {
            span: None,
            min,
            max: None,
        }
    }

    pub fn new_single(min_and_max: u32) -> Self {
        Self {
            span: None,
            min: min_and_max,
            max: Some(min_and_max),
        }
    }

    pub fn value_target_default() -> Self {
        Self {
            span: None,
            min: 1,
            max: Some(1),
        }
    }

    pub fn ref_source_default() -> Self {
        Self {
            span: None,
            min: 0,
            max: None,
        }
    }
    pub fn ref_target_default() -> Self {
        Self {
            span: None,
            min: 0,
            max: Some(1),
        }
    }

    has_owned_ts_span!();

    pub fn min_occurs(&self) -> u32 {
        self.min
    }

    pub fn max_occurs(&self) -> Option<u32> {
        self.max
    }

    pub fn is_range(&self) -> bool {
        self.max.map(|i| i != self.min).unwrap_or_default()
    }

    pub fn to_uml_string(&self) -> String {
        if self.is_range() {
            format!(
                "{}..{}",
                self.min,
                self.max
                    .map(|i| i.to_string())
                    .unwrap_or_else(|| "*".to_string())
            )
        } else {
            self.min.to_string()
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Source Ranges
// ------------------------------------------------------------------------------------------------

impl From<&Node<'_>> for Span {
    fn from(node: &Node<'_>) -> Self {
        Self(node.byte_range())
    }
}

impl From<Node<'_>> for Span {
    fn from(node: Node<'_>) -> Self {
        Self::from(&node)
    }
}

impl Debug for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Span")
            .field("start", &self.0.start)
            .field("end", &self.0.end)
            .finish()
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}..{}", self.0.start, self.0.end)
    }
}

impl Span {
    #[inline(always)]
    pub fn new(start: usize, end: usize) -> Self {
        assert!(start <= end);
        Self(start..end)
    }

    #[inline(always)]
    pub fn start(&self) -> usize {
        self.0.start
    }

    #[inline(always)]
    pub fn end(&self) -> usize {
        self.0.end
    }

    #[inline(always)]
    pub fn byte_range(&self) -> Range<usize> {
        self.0.clone()
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod parse;

pub mod error;

pub mod load;

pub mod walk;
