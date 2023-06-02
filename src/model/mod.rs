/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use lazy_static::lazy_static;
use regex::Regex;
use rust_decimal::Decimal;
use std::{collections::HashSet, fmt::Display, str::FromStr};
use tree_sitter::Node;
use url::Url;

use crate::error::{invalid_identifier_error, invalid_language_tag_error};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Tree Reference
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Span {
    start: usize,
    end: usize,
}

pub type ByteSpan = Span;
pub type CharSpan = Span;

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Identifiers
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Identifier {
    span: Option<Span>,
    value: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct QualifiedIdentifier {
    span: Option<Span>,
    module: Identifier,
    member: Identifier,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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
    name: Identifier,
    body: ModuleBody,
}

#[derive(Clone, Debug, Default)]
pub struct ModuleBody {
    span: Option<Span>,
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
    imported: Vec<Import>,
}

#[derive(Clone, Debug)]
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
    Double(f64),
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
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Datatypes
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct DatatypeDef {
    span: Option<Span>,
    name: Identifier,
    base_type: IdentifierReference,
    body: Option<AnnotationOnlyBody>,
}

#[derive(Clone, Debug, Default)]
pub struct AnnotationOnlyBody {
    span: Option<Span>,
    annotations: Vec<Annotation>,
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Entities
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct EntityDef {
    span: Option<Span>,
    name: Identifier,
    body: Option<EntityBody>,
}

#[derive(Clone, Debug)]
pub struct EntityBody {
    span: Option<Span>,
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
    annotations: Vec<Annotation>,
    members: Vec<EntityMember>,
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Enumerations
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct EnumDef {
    span: Option<Span>,
    name: Identifier,
    body: Option<EnumBody>,
}

#[derive(Clone, Debug, Default)]
pub struct EnumBody {
    span: Option<Span>,
    annotations: Vec<Annotation>,
    variants: Vec<EnumVariant>,
}

#[derive(Clone, Debug)]
pub struct EnumVariant {
    span: Option<Span>,
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
    name: Identifier,
    body: Option<StructureBody>,
}

#[derive(Clone, Debug, Default)]
pub struct StructureBody {
    span: Option<Span>,
    annotations: Vec<Annotation>,
    members: Vec<ByValueMember>,
    groups: Vec<StructureGroup>,
}

#[derive(Clone, Debug, Default)]
pub struct StructureGroup {
    span: Option<Span>,
    annotations: Vec<Annotation>,
    members: Vec<ByValueMember>,
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Members
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct IdentityMember {
    span: Option<Span>,
    name: Identifier,
    target_type: TypeReference,
    body: Option<AnnotationOnlyBody>,
}

#[derive(Clone, Debug)]
pub struct ByValueMember {
    span: Option<Span>,
    name: Identifier,
    target_type: TypeReference,
    target_cardinality: Option<Cardinality>,
    body: Option<AnnotationOnlyBody>,
}

#[derive(Clone, Debug)]
pub struct ByReferenceMember {
    span: Option<Span>,
    name: Identifier,
    source_cardinality: Option<Cardinality>,
    target_type: TypeReference,
    target_cardinality: Option<Cardinality>,
    body: Option<AnnotationOnlyBody>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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
// Private Macros
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

macro_rules! has_span_impl {
    ($tyname: ty) => {
        has_span_impl!($tyname, span);
    };
    ($tyname: ty, $field: ident) => {
        impl $tyname {
            pub fn with_ts_span(self, span: Span) -> Self {
                let mut self_mut = self;
                self_mut.$field = Some(span);
                self_mut
            }

            pub fn has_ts_span(&self) -> bool {
                self.$field.is_some()
            }

            pub fn ts_span(&self) -> Option<&Span> {
                self.$field.as_ref()
            }
        }
    };
}

macro_rules! has_annotations_impl {
    ($tyname: ty) => {
        impl $tyname {
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
        }
    };
}

macro_rules! has_members_impl {
    ($tyname: ty, $membertype: ty) => {
        impl $tyname {
            pub fn add_member(&mut self, add: $membertype) {
                self.members.push(add);
            }

            pub fn extend_members<I>(&mut self, extend: I)
            where
                I: IntoIterator<Item = $membertype>,
            {
                self.members.extend(extend);
            }

            pub fn has_members(&self) -> bool {
                !self.members.is_empty()
            }

            pub fn members(&self) -> impl Iterator<Item = &$membertype> {
                self.members.iter()
            }
        }
    };
}

macro_rules! has_groups_impl {
    ($tyname: ty, $grouptype: ty) => {
        impl $tyname {
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
        }
    };
}

macro_rules! has_body_impl {
    ($tyname: ty, $bodytype: ty) => {
        has_body_impl!($tyname, $bodytype, body);
    };
    ($tyname: ty, $bodytype: ty, $field: ident) => {
        impl $tyname {
            pub fn add_body(&mut self, body: $bodytype) {
                self.$field = Some(body);
            }

            pub fn has_body(&self) -> bool {
                self.$field.is_some()
            }

            pub fn body(&self) -> Option<&$bodytype> {
                self.$field.as_ref()
            }
        }
    };
}

macro_rules! type_definition_impl {
    ($tyname: ty, $bodytype: ty $(, $flname: ident, $fltype: ty )*) => {
        impl $tyname {
            pub fn new(name: Identifier $(, $flname: $fltype )*) -> Self {
                Self {
                    span: None,
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
        }
        has_span_impl!($tyname);
        has_body_impl!($tyname, $bodytype);
    };
}

macro_rules! member_impl {
    ($tyname: ty $(, $optional: ident, $opttype: ty )*) => {
        impl $tyname {
            pub fn new(name: Identifier, target_type: TypeReference) -> Self {
                Self {
                    span: None,
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
        }
        has_span_impl!($tyname);
        has_body_impl!($tyname, AnnotationOnlyBody);
    };
}

macro_rules! referenced_types_impl {
    ($tyname: ty => $field: ident) => {
        impl $tyname {
            pub fn referenced_types(&self) -> HashSet<&IdentifierReference> {
                self.$field
                    .as_ref()
                    .map(|b| b.referenced_types())
                    .unwrap_or_default()
            }
        }
    };
    ($tyname: ty) => {
        impl $tyname {
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
        }
    };
}

macro_rules! referenced_annotations_impl {
    ($tyname: ty => $field: ident) => {
        impl $tyname {
            pub fn referenced_annotations(&self) -> HashSet<&IdentifierReference> {
                self.$field
                    .as_ref()
                    .map(|b| b.referenced_annotations())
                    .unwrap_or_default()
            }
        }
    };
    ($tyname: ty) => {
        impl $tyname {
            pub fn referenced_annotations(&self) -> HashSet<&IdentifierReference> {
                self.annotations().map(|a| a.name()).collect()
            }
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
// Implementations ❱ Identifiers
// ------------------------------------------------------------------------------------------------

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
has_span_impl!(Identifier);

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

    pub fn is_valid(s: &str) -> bool {
        IDENTIFIER.is_match(s)
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for QualifiedIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.module, self.member)
    }
}

has_span_impl!(QualifiedIdentifier);

impl QualifiedIdentifier {
    pub fn new(module: Identifier, member: Identifier) -> Self {
        Self {
            span: None,
            module,
            member,
        }
    }

    pub fn module(&self) -> &Identifier {
        &self.module
    }

    pub fn member(&self) -> &Identifier {
        &self.member
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

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Modules
// ------------------------------------------------------------------------------------------------

has_span_impl!(Module);

impl Module {
    pub fn new(name: Identifier, body: ModuleBody) -> Self {
        Self {
            span: None,
            name,
            body,
        }
    }

    pub fn name(&self) -> &Identifier {
        &self.name
    }

    pub fn body(&self) -> &ModuleBody {
        &self.body
    }

    pub fn imported_modules(&self) -> HashSet<&Identifier> {
        self.body.imported_modules()
    }

    pub fn imported_types(&self) -> HashSet<&QualifiedIdentifier> {
        self.body.imported_types()
    }

    pub fn declared_types(&self) -> HashSet<&Identifier> {
        self.body.declared_types()
    }

    pub fn referenced_types(&self) -> HashSet<&IdentifierReference> {
        self.body.referenced_types()
    }

    pub fn referenced_annotations(&self) -> HashSet<&IdentifierReference> {
        self.body.referenced_annotations()
    }
}

// ------------------------------------------------------------------------------------------------

has_span_impl!(ModuleBody);
has_annotations_impl!(ModuleBody);

impl ModuleBody {
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

    pub fn imported_modules(&self) -> HashSet<&Identifier> {
        self.imports()
            .map(|stmt| stmt.imported_modules())
            .flatten()
            .collect()
    }

    pub fn imported_types(&self) -> HashSet<&QualifiedIdentifier> {
        self.imports()
            .map(|stmt| stmt.imported_types())
            .flatten()
            .collect()
    }

    pub fn declared_types(&self) -> HashSet<&Identifier> {
        self.definitions().map(|def| def.name()).collect()
    }

    pub fn referenced_types(&self) -> HashSet<&IdentifierReference> {
        self.definitions()
            .map(|def| def.referenced_types())
            .flatten()
            .collect()
    }

    pub fn referenced_annotations(&self) -> HashSet<&IdentifierReference> {
        self.definitions()
            .map(|def| def.referenced_annotations())
            .flatten()
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

has_span_impl!(ImportStatement);

impl ImportStatement {
    pub fn new(imported: Vec<Import>) -> Self {
        Self {
            span: None,
            imported,
        }
    }

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
        &self.imported.as_slice()
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

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Annotations
// ------------------------------------------------------------------------------------------------

has_span_impl!(Annotation);

impl Annotation {
    pub fn new(name: IdentifierReference, value: Value) -> Self {
        Self {
            span: None,
            name,
            value,
        }
    }

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

has_span_impl!(LanguageString);

impl LanguageString {
    pub(crate) fn new(value: &str, language: Option<LanguageTag>) -> Self {
        Self {
            span: None,
            value: value.to_string(),
            language,
        }
    }

    pub fn value(&self) -> &String {
        &self.value
    }

    pub fn language(&self) -> Option<&LanguageTag> {
        self.language.as_ref()
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

has_span_impl!(LanguageTag);
into_string_impl!(LanguageTag, value);
as_str_impl!(LanguageTag, value);

impl LanguageTag {
    pub(crate) fn new_unchecked(s: &str) -> Self {
        Self {
            span: None,
            value: s.to_string(),
        }
    }

    pub fn is_valid(s: &str) -> bool {
        LANGUAGE_TAG.is_match(s)
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

has_span_impl!(ListOfValues);

impl ListOfValues {
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

has_span_impl!(ValueConstructor);

impl ValueConstructor {
    pub fn new(type_name: IdentifierReference, value: SimpleValue) -> Self {
        Self {
            span: None,
            type_name,
            value,
        }
    }

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

impl TypeDefinition {
    pub fn name(&self) -> &Identifier {
        match self {
            TypeDefinition::Datatype(v) => v.name(),
            TypeDefinition::Entity(v) => v.name(),
            TypeDefinition::Enum(v) => v.name(),
            TypeDefinition::Event(v) => v.name(),
            TypeDefinition::Structure(v) => v.name(),
        }
    }

    pub fn referenced_types(&self) -> HashSet<&IdentifierReference> {
        match self {
            TypeDefinition::Datatype(v) => v.referenced_types(),
            TypeDefinition::Entity(v) => v.referenced_types(),
            TypeDefinition::Enum(v) => v.referenced_types(),
            TypeDefinition::Event(v) => v.referenced_types(),
            TypeDefinition::Structure(v) => v.referenced_types(),
        }
    }

    pub fn referenced_annotations(&self) -> HashSet<&IdentifierReference> {
        match self {
            TypeDefinition::Datatype(v) => v.referenced_annotations(),
            TypeDefinition::Entity(v) => v.referenced_annotations(),
            TypeDefinition::Enum(v) => v.referenced_annotations(),
            TypeDefinition::Event(v) => v.referenced_annotations(),
            TypeDefinition::Structure(v) => v.referenced_annotations(),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Datatypes
// ------------------------------------------------------------------------------------------------

type_definition_impl!(
    DatatypeDef,
    AnnotationOnlyBody,
    base_type,
    IdentifierReference
);
referenced_annotations_impl!(DatatypeDef => body);

impl DatatypeDef {
    pub fn referenced_types(&self) -> HashSet<&IdentifierReference> {
        [self.base_type()].into_iter().collect()
    }
}

// ------------------------------------------------------------------------------------------------

has_span_impl!(AnnotationOnlyBody);
has_annotations_impl!(AnnotationOnlyBody);
referenced_annotations_impl!(AnnotationOnlyBody);

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Entities
// ------------------------------------------------------------------------------------------------

type_definition_impl!(EntityDef, EntityBody);
referenced_annotations_impl!(EntityDef => body);
referenced_types_impl!(EntityDef => body);

// ------------------------------------------------------------------------------------------------

has_span_impl!(EntityBody);
has_annotations_impl!(EntityBody);
has_members_impl!(EntityBody, EntityMember);
has_groups_impl!(EntityBody, EntityGroup);
referenced_annotations_impl!(EntityBody);
referenced_types_impl!(EntityBody);

impl EntityBody {
    pub fn new(identity: IdentityMember) -> Self {
        Self {
            span: None,
            identity,
            annotations: Default::default(),
            members: Default::default(),
            groups: Default::default(),
        }
    }

    pub fn identity(&self) -> &IdentityMember {
        &self.identity
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
}

// ------------------------------------------------------------------------------------------------

has_span_impl!(EntityGroup);
has_annotations_impl!(EntityGroup);
has_members_impl!(EntityGroup, EntityMember);

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Enumerations
// ------------------------------------------------------------------------------------------------

type_definition_impl!(EnumDef, EnumBody);
referenced_annotations_impl!(EnumDef => body);

impl EnumDef {
    pub fn referenced_types(&self) -> HashSet<&IdentifierReference> {
        Default::default()
    }
}

// ------------------------------------------------------------------------------------------------

has_span_impl!(EnumBody);
has_annotations_impl!(EnumBody);
referenced_annotations_impl!(EnumBody);

impl EnumBody {
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

has_span_impl!(EnumVariant);
has_body_impl!(EnumVariant, AnnotationOnlyBody);
referenced_annotations_impl!(EnumVariant => body);

impl EnumVariant {
    pub fn new(name: Identifier, value: u32) -> Self {
        Self {
            span: None,
            name,
            value,
            body: None,
        }
    }

    pub fn new_with(name: Identifier, value: u32, body: AnnotationOnlyBody) -> Self {
        Self {
            span: None,
            name,
            value,
            body: Some(body),
        }
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

type_definition_impl!(EventDef, StructureBody, event_source, IdentifierReference);
referenced_annotations_impl!(EventDef => body);
referenced_types_impl!(EventDef => body);

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Structures
// ------------------------------------------------------------------------------------------------

type_definition_impl!(StructureDef, StructureBody);
referenced_annotations_impl!(StructureDef => body);
referenced_types_impl!(StructureDef => body);

// ------------------------------------------------------------------------------------------------

has_span_impl!(StructureBody);
has_annotations_impl!(StructureBody);
has_members_impl!(StructureBody, ByValueMember);
has_groups_impl!(StructureBody, StructureGroup);
referenced_annotations_impl!(StructureBody);
referenced_types_impl!(StructureBody);

// ------------------------------------------------------------------------------------------------

has_span_impl!(StructureGroup);
has_annotations_impl!(StructureGroup);
has_members_impl!(StructureGroup, ByValueMember);

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Members
// ------------------------------------------------------------------------------------------------

member_impl!(IdentityMember);
referenced_annotations_impl!(IdentityMember => body);

// ------------------------------------------------------------------------------------------------

member_impl!(ByValueMember, target_cardinality, Cardinality);
referenced_annotations_impl!(ByValueMember => body);

impl ByValueMember {
    pub fn set_target_cardinality(&mut self, cardinality: Cardinality) {
        self.target_cardinality = Some(cardinality);
    }
}

// ------------------------------------------------------------------------------------------------

member_impl!(
    ByReferenceMember,
    source_cardinality,
    Cardinality,
    target_cardinality,
    Cardinality
);
referenced_annotations_impl!(ByReferenceMember => body);

impl ByReferenceMember {
    pub fn set_source_cardinality(&mut self, cardinality: Cardinality) {
        self.source_cardinality = Some(cardinality);
    }

    pub fn set_target_cardinality(&mut self, cardinality: Cardinality) {
        self.target_cardinality = Some(cardinality);
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Members ❱ Cardinality
// ------------------------------------------------------------------------------------------------

has_span_impl!(Cardinality);

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
        Self {
            start: node.start_byte(),
            end: node.end_byte(),
        }
    }
}

impl From<Node<'_>> for Span {
    fn from(node: Node<'_>) -> Self {
        Self {
            start: node.start_byte(),
            end: node.end_byte(),
        }
    }
}

impl Span {
    pub fn start(&self) -> usize {
        self.start
    }
    pub fn end(&self) -> usize {
        self.end
    }
    pub fn byte_span_to_char_span(source: &str, byte_span: ByteSpan) -> CharSpan {
        let start = source[..byte_span.start].chars().count();
        let size = source[byte_span.start..byte_span.end].chars().count();
        Span {
            start,
            end: start + size,
        }
    }

    pub fn char_span_to_byte_span(source: &str, char_span: CharSpan) -> ByteSpan {
        let mut iter = source.char_indices();
        let start = iter.nth(char_span.start).map(|(i, _)| i).unwrap_or(0);
        let end = iter
            .nth(char_span.end - char_span.start - 1)
            .map(|(i, _)| i)
            .unwrap_or(source.len());
        Span { start, end }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod check;

pub mod parse;

pub mod resolve;

pub mod walk;
