/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use sdml_core::error::Error;
use sdml_core::model::{
    Annotation, AnnotationOnlyBody, AnnotationProperty, ByReferenceMember, ByReferenceMemberInner,
    ByValueMember, ByValueMemberInner, Cardinality, DatatypeDef, EntityBody, EntityDef,
    EntityGroup, EntityMember, EnumBody, EnumDef, EnumVariant, EventDef, Identifier,
    IdentifierReference, IdentityMember, IdentityMemberInner, Import, ImportStatement, LanguageTag,
    ListMember, ListOfValues, Module, ModuleBody, PropertyBody, PropertyDef, PropertyRole,
    QualifiedIdentifier, SimpleValue, Span, StructureBody, StructureDef, StructureGroup,
    TypeDefinition, TypeReference, TypeVariant, UnionBody, UnionDef, Value, ValueConstructor,
};
use sdml_core::syntax::{
    FIELD_NAME_BASE, FIELD_NAME_BODY, FIELD_NAME_IDENTITY, FIELD_NAME_LANGUAGE, FIELD_NAME_MAX,
    FIELD_NAME_MEMBER, FIELD_NAME_MIN, FIELD_NAME_MODULE, FIELD_NAME_NAME, FIELD_NAME_RENAME,
    FIELD_NAME_ROLE, FIELD_NAME_SOURCE, FIELD_NAME_SOURCE_CARDINALITY, FIELD_NAME_TARGET,
    FIELD_NAME_TARGET_CARDINALITY, FIELD_NAME_VALUE, NODE_KIND_ANNOTATION,
    NODE_KIND_ANNOTATION_ONLY_BODY, NODE_KIND_BOOLEAN, NODE_KIND_CARDINALITY_EXPRESSION,
    NODE_KIND_DATA_TYPE_DEF, NODE_KIND_DECIMAL, NODE_KIND_DOUBLE, NODE_KIND_ENTITY_BODY,
    NODE_KIND_ENTITY_DEF, NODE_KIND_ENTITY_GROUP, NODE_KIND_ENUM_BODY, NODE_KIND_ENUM_DEF,
    NODE_KIND_ENUM_VARIANT, NODE_KIND_EVENT_DEF, NODE_KIND_IDENTIFIER,
    NODE_KIND_IDENTIFIER_REFERENCE, NODE_KIND_IDENTITY_MEMBER, NODE_KIND_IMPORT, NODE_KIND_INTEGER,
    NODE_KIND_IRI_REFERENCE, NODE_KIND_LANGUAGE_TAG, NODE_KIND_LIST_OF_VALUES,
    NODE_KIND_MEMBER_BY_REFERENCE, NODE_KIND_MEMBER_BY_VALUE, NODE_KIND_MEMBER_IMPORT,
    NODE_KIND_MODULE, NODE_KIND_MODULE_BODY, NODE_KIND_MODULE_IMPORT, NODE_KIND_PROPERTY_BODY,
    NODE_KIND_QUALIFIED_IDENTIFIER, NODE_KIND_QUOTED_STRING, NODE_KIND_STRING,
    NODE_KIND_STRUCTURE_BODY, NODE_KIND_STRUCTURE_DEF, NODE_KIND_STRUCTURE_GROUP,
    NODE_KIND_TYPE_VARIANT, NODE_KIND_UNION_BODY, NODE_KIND_UNION_DEF, NODE_KIND_UNKNOWN_TYPE,
    NODE_KIND_VALUE_CONSTRUCTOR,
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
            match annotation {
                Annotation::Property(v) => write_annotation_property(v, $w)?,
                Annotation::Constraint(_) => todo!(),
            }
        }
    };
}

macro_rules! write_span {
    ($me: expr, $w: expr) => {
        if let Some(span) = $me.ts_span() {
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
    maybe_write_span(me.ts_span(), w)?;
    w.value_with_prefix(me, "'")?;

    w.close_paren()?;

    Ok(())
}

fn write_qualified_identifier<W: Write>(
    me: &QualifiedIdentifier,
    w: &mut Writer<W>,
) -> Result<(), Error> {
    w.start_node_and_newln(NODE_KIND_QUALIFIED_IDENTIFIER)?;
    w.indent();

    write_span!(me, w);

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
    write_identifier_reference(me.name(), w)?;

    w.newln_and_indentation()?;
    w.field_name(FIELD_NAME_VALUE)?;
    write_value(me.value(), w)?;

    w.close_paren()?;

    w.outdent();
    Ok(())
}

fn write_value<W: Write>(me: &Value, w: &mut Writer<W>) -> Result<(), Error> {
    match me {
        Value::Simple(v) => write_simple_value(v, w)?,
        Value::ValueConstructor(v) => write_value_constructor(v, w)?,
        Value::Reference(v) => write_identifier_reference(v, w)?,
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

fn write_list_of_values<W: Write>(me: &ListOfValues, w: &mut Writer<W>) -> Result<(), Error> {
    w.start_node(NODE_KIND_LIST_OF_VALUES)?;
    w.indent();

    write_span!(me, w);

    for value in me.values() {
        w.newln()?;
        write_list_member(value, w)?;
    }

    w.close_paren()?;

    w.outdent();
    Ok(())
}

fn write_language_tag<W: Write>(me: &LanguageTag, w: &mut Writer<W>) -> Result<(), Error> {
    w.start_node(NODE_KIND_LANGUAGE_TAG)?;
    w.value_with_prefix("'", me)?;
    w.close_paren()?;

    Ok(())
}

fn write_list_member<W: Write>(me: &ListMember, w: &mut Writer<W>) -> Result<(), Error> {
    match me {
        ListMember::Simple(v) => write_simple_value(v, w)?,
        ListMember::ValueConstructor(v) => write_value_constructor(v, w)?,
        ListMember::Reference(v) => write_identifier_reference(v, w)?,
    }

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

fn write_type_definition<W: Write>(me: &TypeDefinition, w: &mut Writer<W>) -> Result<(), Error> {
    match me {
        TypeDefinition::Datatype(v) => write_data_type_def(v, w)?,
        TypeDefinition::Entity(v) => write_entity_def(v, w)?,
        TypeDefinition::Enum(v) => write_enum_def(v, w)?,
        TypeDefinition::Event(v) => write_event_def(v, w)?,
        TypeDefinition::Structure(v) => write_structure_def(v, w)?,
        TypeDefinition::Union(v) => write_union_def(v, w)?,
        TypeDefinition::Property(v) => write_property_def(v, w)?,
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

fn write_enum_variant<W: Write>(me: &EnumVariant, w: &mut Writer<W>) -> Result<(), Error> {
    w.start_node_indented(NODE_KIND_ENUM_VARIANT)?;
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
        write_property_role(role, w)?;
    }

    w.close_paren()?;

    w.outdent();
    Ok(())
}

fn write_property_role<W: Write>(me: &PropertyRole, w: &mut Writer<W>) -> Result<(), Error> {
    w.start_node(NODE_KIND_PROPERTY_BODY)?;
    w.indent();

    w.newln_and_indentation()?;
    w.field_name(FIELD_NAME_NAME)?;
    write_identifier(me.name(), w)?;

    w.newln_and_indentation()?;
    w.field_name(FIELD_NAME_TARGET)?;
    write_type_reference(me.target_type(), w)?;

    if let Some(Some(card)) = &me.source_cardinality() {
        w.newln_and_indentation()?;
        w.field_name(FIELD_NAME_SOURCE_CARDINALITY)?;
        write_cardinality(card, w)?;
    }

    if let Some(card) = &me.target_cardinality() {
        w.newln_and_indentation()?;
        w.field_name(FIELD_NAME_TARGET_CARDINALITY)?;
        write_cardinality(card, w)?;
    }

    if let Some(body) = &me.body() {
        w.newln_and_indentation()?;
        w.field_name(FIELD_NAME_BODY)?;
        write_annotation_only_body(body, w)?
    }

    w.close_paren_and_newln()?;

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
    write_identifier_reference(me.name(), w)?;

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

    w.newln_and_indentation()?;
    w.field_name(FIELD_NAME_NAME)?;
    write_identifier(me.name(), w)?;

    match me.inner() {
        IdentityMemberInner::PropertyRole(role) => {
            w.newln_and_indentation()?;
            w.field_name(FIELD_NAME_ROLE)?;
            write_identifier(role, w)?;
        }
        IdentityMemberInner::Defined(def) => {
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

    w.newln_and_indentation()?;
    w.field_name(FIELD_NAME_NAME)?;
    write_identifier(me.name(), w)?;

    match me.inner() {
        ByValueMemberInner::PropertyRole(role) => {
            w.newln_and_indentation()?;
            w.field_name(FIELD_NAME_ROLE)?;
            write_identifier(role, w)?;
        }
        ByValueMemberInner::Defined(def) => {
            w.newln_and_indentation()?;
            w.field_name(FIELD_NAME_TARGET)?;
            write_type_reference(def.target_type(), w)?;

            if let Some(card) = &def.target_cardinality() {
                w.newln_and_indentation()?;
                w.field_name(FIELD_NAME_TARGET_CARDINALITY)?;
                write_cardinality(card, w)?;
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

    w.newln_and_indentation()?;
    w.field_name(FIELD_NAME_NAME)?;
    write_identifier(me.name(), w)?;

    match me.inner() {
        ByReferenceMemberInner::PropertyRole(role) => {
            w.newln_and_indentation()?;
            w.field_name(FIELD_NAME_ROLE)?;
            write_identifier(role, w)?;
        }
        ByReferenceMemberInner::Defined(def) => {
            w.newln_and_indentation()?;
            w.field_name(FIELD_NAME_TARGET)?;
            write_type_reference(def.target_type(), w)?;

            if let Some(card) = &def.source_cardinality() {
                w.newln_and_indentation()?;
                w.field_name(FIELD_NAME_SOURCE_CARDINALITY)?;
                write_cardinality(card, w)?;
            }

            if let Some(card) = &def.target_cardinality() {
                w.newln_and_indentation()?;
                w.field_name(FIELD_NAME_TARGET_CARDINALITY)?;
                write_cardinality(card, w)?;
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

    maybe_write_span(me.ts_span(), w)?;

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
