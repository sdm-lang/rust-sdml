/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::api::{
    Datatype, Entity, Enum, EnumVariant, Event, MemberCardinality, MemberTypeTarget, ParseTree, Structure, TypeDefinition,
};
use crate::error::Error;
use rust_decimal::Decimal;
use url::Url;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub enum SimpleValue {
    String(String, Option<String>),
    Double(f64),
    Decimal(Decimal),
    Integer(i64),
    Boolean(bool),
    IriReference(Url),
}

#[derive(Clone, Debug)]
pub enum Value {
    Simple(SimpleValue),
    List(Vec<SimpleValue>),
    ValueConstructor(String, SimpleValue),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cardinality {
    min: u32,
    max: Option<u32>,
}

pub trait TreeWalker {
    fn start_module(&self, _name: &str) -> Result<(), Error> {
        Ok(())
    }
    fn import(&self, _name: &str) -> Result<(), Error> {
        Ok(())
    }
    fn annotation(&self, _name: &str, _value: Value) -> Result<(), Error> {
        Ok(())
    }

    fn start_datatype(&self, _name: &str, _base_type: &str) -> Result<(), Error> {
        Ok(())
    }
    // annotation
    fn end_datatype(&self, _name: &str) -> Result<(), Error> {
        Ok(())
    }

    fn start_entity(&self, _name: &str) -> Result<(), Error> {
        Ok(())
    }
    // annotation
    fn start_identity_member(
        &self,
        _name: &str,
        _target_type: Option<&str>,
    ) -> Result<(), Error> {
        Ok(())
    }
    fn start_by_value_member(
        &self,
        _name: &str,
        _to: Cardinality,
        _target_type: Option<&str>,
    ) -> Result<(), Error> {
        Ok(())
    }
    fn start_by_reference_member(
        &self,
        _name: &str,
        _from: Cardinality,
        _to: Cardinality,
        _target_type: Option<&str>,
    ) -> Result<(), Error> {
        Ok(())
    }
    // annotation
    fn end_member(&self, _name: &str) -> Result<(), Error> {
        Ok(())
    }
    fn end_entity(&self, _name: &str) -> Result<(), Error> {
        Ok(())
    }

    fn start_enum(&self, _name: &str) -> Result<(), Error> {
        Ok(())
    }
    // annotation
    fn start_variant(&self, _name: &str, _value: u64) -> Result<(), Error> {
        Ok(())
    }
    // annotation
    fn end_variant(&self, _name: &str) -> Result<(), Error> {
        Ok(())
    }
    fn end_enum(&self, _name: &str) -> Result<(), Error> {
        Ok(())
    }

    fn start_event(&self, _name: &str, _source: &str) -> Result<(), Error> {
        Ok(())
    }
    // annotation
    // start_member..end_member
    fn start_group(&self, _index: usize) -> Result<(), Error> {
        Ok(())
    }
    // annotation
    // start_member..end_member
    fn end_group(&self, _index: usize) -> Result<(), Error> {
        Ok(())
    }
    fn end_event(&self, _name: &str) -> Result<(), Error> {
        Ok(())
    }

    fn start_structure(&self, _name: &str) -> Result<(), Error> {
        Ok(())
    }
    // annotation
    // start_member..end_member
    // start_group..end_group
    fn end_structure(&self, _name: &str) -> Result<(), Error> {
        Ok(())
    }

    fn end_module(&self, _name: &str) -> Result<(), Error> {
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn walk_tree(tree: &ParseTree<'_>, walker: &impl TreeWalker) -> Result<(), Error> {
    let module = tree.module();
    let module_name = module.name();

    walker.start_module(module_name.as_ref())?;

    let module_body = module.body();

    for import in module_body.imports() {
        for imported in import.imported() {
            walker.import(imported.text()?)?;
        }
    }

    for annotation in module_body.annotations() {
        walker.annotation(annotation.name().as_ref(), annotation.value().into())?;
    }

    for type_definition in module_body.definitions() {
        println!("{:#?}", type_definition);
        match type_definition {
            TypeDefinition::Datatype(def) => {
                walk_datatype_def(&def, walker)?;
            }
            TypeDefinition::Entity(def) => {
                walk_entity_def(&def, walker)?;
            }
            TypeDefinition::Enum(def) => {
                walk_enum_def(&def, walker)?;
            }
            TypeDefinition::Event(def) => {
                walk_event_def(&def, walker)?;
            }
            TypeDefinition::Structure(def) => {
                walk_structure_def(&def, walker)?;
            }
        }
    }

    walker.end_module(module_name.as_ref())
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

impl From<crate::api::Value<'_>> for Value {
    fn from(v: crate::api::Value<'_>) -> Self {
        match v {
            crate::api::Value::String(v) => {
                Value::Simple(SimpleValue::String(v.string().value().to_string(), None))
            }
            crate::api::Value::Double(v) => Value::Simple(SimpleValue::Double(v.value())),
            crate::api::Value::Decimal(v) => Value::Simple(SimpleValue::Decimal(v.value())),
            crate::api::Value::Integer(v) => Value::Simple(SimpleValue::Integer(v.value())),
            crate::api::Value::Boolean(v) => Value::Simple(SimpleValue::Boolean(v.value())),
            crate::api::Value::IriReference(v) => {
                Value::Simple(SimpleValue::IriReference(v.value()))
            }
            crate::api::Value::ListOfValues(v) => {
                Value::List(v.values().iter().map(|v| v.into()).collect())
            }
            crate::api::Value::ValueConstructor(v) => {
                Value::ValueConstructor(v.name().as_ref().to_string(), v.value().into())
            }
        }
    }
}

impl From<crate::api::SimpleValue<'_>> for SimpleValue {
    fn from(v: crate::api::SimpleValue<'_>) -> Self {
        Self::from(&v)
    }
}

impl From<&crate::api::SimpleValue<'_>> for SimpleValue {
    fn from(v: &crate::api::SimpleValue<'_>) -> Self {
        match v {
            crate::api::SimpleValue::String(v) => {
                SimpleValue::String(v.string().value().to_string(), None)
            }
            crate::api::SimpleValue::Double(v) => SimpleValue::Double(v.value()),
            crate::api::SimpleValue::Decimal(v) => SimpleValue::Decimal(v.value()),
            crate::api::SimpleValue::Integer(v) => SimpleValue::Integer(v.value()),
            crate::api::SimpleValue::Boolean(v) => SimpleValue::Boolean(v.value()),
            crate::api::SimpleValue::IriReference(v) => SimpleValue::IriReference(v.value()),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl From<MemberCardinality<'_>> for Cardinality {
    fn from(v: MemberCardinality<'_>) -> Self {
        let min = v.min_occurs();
        if let Some(range) = v.range() {
            let max = range.max_occurs();
            Self::new_range(min, max)
        } else {
            Self::new_single(min)
        }
    }
}

impl Cardinality {
    pub fn new_range(min: u32, max: Option<u32>) -> Self {
        Self { min, max }
    }

    pub fn new_unbounded(min: u32) -> Self {
        Self { min, max: None }
    }

    pub fn new_single(min_and_max: u32) -> Self {
        Self { min: min_and_max, max: Some(min_and_max) }
    }

    pub fn value_target_default() -> Self {
        Self {
            min: 1,
            max: Some(1),
        }
    }

    pub fn ref_source_default() -> Self {
        Self {
            min: 0,
            max: None,
        }
    }
    pub fn ref_target_default() -> Self {
        Self {
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
            format!("{}..{}", self.min, self.max.map(|i|i.to_string()).unwrap_or_else(|| "*".to_string()))
        } else {
            self.min.to_string()
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

pub fn walk_datatype_def(def: &Datatype<'_>, walker: &impl TreeWalker) -> Result<(), Error> {
    let name = def.name();
    walker.start_datatype(name.as_ref(), def.base_type().as_ref())?;
    if let Some(body) = def.body() {
        for annotation in body.annotations() {
            walker.annotation(annotation.name().as_ref(), annotation.value().into())?;
        }
    }
    walker.end_datatype(name.as_ref())
}

pub fn walk_entity_def(def: &Entity<'_>, walker: &impl TreeWalker) -> Result<(), Error> {
    let name = def.name();
    walker.start_entity(name.as_ref())?;
    if let Some(body) = def.body() {
        for annotation in body.annotations() {
            walker.annotation(annotation.name().as_ref(), annotation.value().into())?;
        }
        let identity = body.identity();
        if let MemberTypeTarget::IdentifierReference(v) = identity.target_type() {
            walker.start_identity_member(identity.name().as_ref(), Some(v.as_ref()))?;
        } else {
            walker.start_identity_member(identity.name().as_ref(), None)?;
        }
        if let Some(body) = identity.body() {
            for annotation in body.annotations() {
                walker.annotation(annotation.name().as_ref(), annotation.value().into())?;
            }
        }
        walker.end_member(identity.name().as_ref())?;
        for member in body.value_members() {
            let to_cardinality: Cardinality = member.target_cardinality().map(|c|c.into()).unwrap_or_else(Cardinality::value_target_default);
            if let MemberTypeTarget::IdentifierReference(v) = member.target_type() {
                walker.start_by_value_member(member.name().as_ref(), to_cardinality, Some(v.as_ref()))?;
            } else {
                walker.start_by_value_member(member.name().as_ref(), to_cardinality, None)?;
            }
            if let Some(body) = member.body() {
                for annotation in body.annotations() {
                    walker.annotation(annotation.name().as_ref(), annotation.value().into())?;
                }
            }
            walker.end_member(member.name().as_ref())?;
        }
        for member in body.ref_members() {
            let from_cardinality: Cardinality = member.source_cardinality().map(|c|c.into()).unwrap_or_else(Cardinality::ref_source_default);
            let to_cardinality: Cardinality = member.target_cardinality().map(|c|c.into()).unwrap_or_else(Cardinality::ref_target_default);
            if let MemberTypeTarget::IdentifierReference(v) = member.target_type() {
                walker.start_by_reference_member(member.name().as_ref(), from_cardinality, to_cardinality, Some(v.as_ref()))?;
            } else {
                walker.start_by_reference_member(member.name().as_ref(), from_cardinality, to_cardinality, None)?;
            }
            if let Some(body) = member.body() {
                for annotation in body.annotations() {
                    walker.annotation(annotation.name().as_ref(), annotation.value().into())?;
                }
            }
            walker.end_member(member.name().as_ref())?;
        }
        for group in body.groups() {
            for member in group.value_members() {
                let to_cardinality: Cardinality = member.target_cardinality().map(|c|c.into()).unwrap_or_else(Cardinality::value_target_default);
                if let MemberTypeTarget::IdentifierReference(v) = member.target_type() {
                    walker.start_by_value_member(member.name().as_ref(), to_cardinality, Some(v.as_ref()))?;
                } else {
                    walker.start_by_value_member(member.name().as_ref(), to_cardinality, None)?;
                }
                if let Some(body) = member.body() {
                    for annotation in body.annotations() {
                        walker.annotation(annotation.name().as_ref(), annotation.value().into())?;
                    }
                }
                walker.end_member(member.name().as_ref())?;
            }
            for member in group.ref_members() {
                let from_cardinality: Cardinality = member.source_cardinality().map(|c|c.into()).unwrap_or_else(Cardinality::ref_source_default);
                let to_cardinality: Cardinality = member.target_cardinality().map(|c|c.into()).unwrap_or_else(Cardinality::ref_target_default);
                if let MemberTypeTarget::IdentifierReference(v) = member.target_type() {
                    walker.start_by_reference_member(member.name().as_ref(), from_cardinality, to_cardinality, Some(v.as_ref()))?;
                } else {
                    walker.start_by_reference_member(member.name().as_ref(), from_cardinality, to_cardinality, None)?;
                }
                if let Some(body) = member.body() {
                    for annotation in body.annotations() {
                        walker.annotation(annotation.name().as_ref(), annotation.value().into())?;
                    }
                }
                walker.end_member(member.name().as_ref())?;
            }
        }
    }
    walker.end_entity(name.as_ref())
}

pub fn walk_enum_def(def: &Enum<'_>, walker: &impl TreeWalker) -> Result<(), Error> {
    let name = def.name();
    walker.start_enum(name.as_ref())?;
    if let Some(body) = def.body() {
        for annotation in body.annotations() {
            walker.annotation(annotation.name().as_ref(), annotation.value().into())?;
        }
        for variant in body.variants() {
            walk_enum_variant(&variant, walker)?;
        }
    }
    walker.end_enum(name.as_ref())
}

pub fn walk_event_def(def: &Event<'_>, walker: &impl TreeWalker) -> Result<(), Error> {
    let name = def.name();
    walker.start_event(name.as_ref(), def.event_source().as_ref())?;
    if let Some(body) = def.body() {
        for annotation in body.annotations() {
            walker.annotation(annotation.name().as_ref(), annotation.value().into())?;
            // members
            // groups
        }
    }
    walker.end_event(name.as_ref())
}

pub fn walk_structure_def(def: &Structure<'_>, walker: &impl TreeWalker) -> Result<(), Error> {
    let name = def.name();
    walker.start_structure(name.as_ref())?;
    if let Some(body) = def.body() {
        for annotation in body.annotations() {
            walker.annotation(annotation.name().as_ref(), annotation.value().into())?;
            // members
            // groups
        }
    }
    walker.end_structure(name.as_ref())
}

pub fn walk_enum_variant(variant: &EnumVariant<'_>, walker: &impl TreeWalker) -> Result<(), Error> {
    let name = variant.name();
    walker.start_variant(name.as_ref(), variant.value())?;
    if let Some(body) = variant.body() {
        for annotation in body.annotations() {
            walker.annotation(annotation.name().as_ref(), annotation.value().into())?;
        }
    }
    walker.end_variant(name.as_ref())
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
