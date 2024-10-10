/*!
One-line description.

# Context Structure

## Module Object

* `name: String`
* `library_module: bool`
* `source_file: String` (optional)
* `source_span` (optional) a span object
* `base_uri: String` (optional)
* `version_info: String` (optional)
* `version_uri: String` (optional)
* `imports` (optional) an array of import objects
* `annotations` (optional) an array of annotation objects
* `definitions` (optional) an array of definition objects

## Span Object

* `start: Number::PosInt`
* `end: Number::PosInt`

## Import Object

* `module: String`

One of:

* `version_uri: String`
* `member: String`

## Annotation Object

* `name: String`
* `type: String = property`
  * `value` a value object
* `type: String = informal`
  * `value: String`
  * `language: String` (optional)
* `type: String = formal`
  * `environment` (optional) an array of environment objects
  * `sentence` a sentence object

Sentences

## Definition Object

* `name: String`
* `type: String = datatype`
  * `is_opaque: bool`
  * `base_type: String`
* `type: String = entity`
  * `identity` (optional) member object
  * `members` (optional) an array of member objects
* `type: String = enum`
  * `variants` (optional) an array of:
    * `name: String`
    * `annotations` (optional) an array of annotation  objects
* `type: String = event`
  * `source: String`
  * `members` (optional) an array member objects
* `type: String = property`
  * member definition object
* `type: String = rdf`
* `type: String = structure`
  * `members` (optional) an array member objects
* `type: String = type_class` TBD
* `type: String = union`
  * `variants` (optional) an array of:
    * `name: String`
    * `rename: String` (optional)
    * `annotations` (optional) an array of annotation  objects
* `annotations` (optional) an array of annotation  objects

## Member Object

* `kind: String = reference`
  * `property: String`
* `kind: String = definition`
  * member definition object

## Member Definition Object

* `name: String`
* `cardinality`
  * `ordering: String` (optional)
  * `uniqueness: String` (optional)
  * `min_occurs: Number::PosInt`
  * `max_occurs: Number::PosInt` (optional)
* `type: String`
* `annotations` (optional) an array of annotation objects

## Value Object

A Value is either simple, type constructor, mapping, reference, or list.

A Sequence Member Value is either simple, type constructor, mapping, or reference.

Simple Values:

* `type: String = boolean` and `value: Boolean`
* `type: String = double` and `value: Number::Float`
* `type: String = decimal` and `value: String`
* `type: String = integer` and `value: Number::NegInt`
* `type: String = unsigned` and `value: Number::PosInt`
* `type: String = string` and `value: String`
* `type: String = uri` and `value: String`
* `type: String = binary` and `value: String`

* `type: String = constructor`
  * `type_name: String`
  * `value` a simple value
* `type: String = mapping`
  * `domain` a simple value
  * `range` a value
* `type: String = reference`
  * `value: String`
* `type: String = sequence` an array of sequence member values

 */

use sdml_core::model::annotations::{Annotation, AnnotationProperty, HasAnnotations};
use sdml_core::model::constraints::{
    AtomicSentence, BinaryBooleanSentence, BooleanSentence, Constraint, ConstraintBody,
    ConstraintSentence, EnvironmentDef, EnvironmentDefBody, Equation, FunctionDef,
    FunctionSignature, FunctionType, FunctionTypeReferenceInner, Inequation,
    PredicateSequenceMember, PredicateValue, QuantifiedSentence, SequenceOfPredicateValues,
    SimpleSentence, Subject, Term, UnaryBooleanSentence, Variables,
};
use sdml_core::model::definitions::{
    DatatypeDef, Definition, EntityDef, EnumDef, EventDef, HasMembers, HasVariants, MethodDef,
    PropertyDef, RdfDef, StructureDef, TypeClassDef, TypeVariable, UnionDef,
};
use sdml_core::model::members::{Member, MemberDef, MemberKind};
use sdml_core::model::modules::{Module, ModuleBody};
use sdml_core::model::values::{
    MappingValue, SequenceMember, SequenceOfValues, SimpleValue, Value as SdmlValue,
    ValueConstructor,
};
use sdml_core::model::{HasBody, HasName, HasNameReference, HasOptionalBody, HasSourceSpan};
use sdml_core::stdlib::is_library_module;
use sdml_core::store::ModuleStore;
use tera::{Map, Value};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn module_to_value(module: &Module, _cache: &impl ModuleStore) -> (String, Value) {
    let mut value = Map::default();

    value.insert(KEY_META_TYPE.into(), VAL_MT_MODULE.into());
    value.insert(KEY_NAME.into(), module.name().to_string().into());
    value.insert(
        KEY_IS_LIBRARY_MODULE.into(),
        is_library_module(module.name()).into(),
    );

    add_source_span(module, &mut value);

    if let Some(source_file) = module.source_file() {
        value.insert(
            KEY_SOURCE_FILE.into(),
            Value::String(source_file.to_string_lossy().into_owned()),
        );
    }
    if let Some(base_uri) = module.base_uri() {
        value.insert(KEY_BASE_URI.into(), Value::String(base_uri.to_string()));
    }
    if let Some(version_info) = module.version_info() {
        value.insert(
            KEY_VERSION_INFO.into(),
            Value::String(version_info.to_string()),
        );
    }
    if let Some(version_uri) = module.version_uri() {
        value.insert(
            KEY_VERSION_URI.into(),
            Value::String(version_uri.to_string()),
        );
    }

    add_module_body(module.body(), &mut value);

    (module.name().to_string(), value.into())
}

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

const KEY_META_TYPE: &str = "__type";

const VAL_MT_MODULE: &str = "module";
const VAL_MT_PROPERTY: &str = "property";
const VAL_MT_BOOLEAN: &str = "boolean";
const VAL_MT_DOUBLE: &str = "double";
const VAL_MT_DECIMAL: &str = "decimal";
const VAL_MT_INTEGER: &str = "integer";
const VAL_MT_UNSIGNED: &str = "unsigned";
const VAL_MT_STRING: &str = "string";
const VAL_MT_URI: &str = "uri";
const VAL_MT_BINARY: &str = "binary";
const VAL_MT_CONSTRUCTOR: &str = "constructor";
const VAL_MT_MAPPING: &str = "mapping";
const VAL_MT_MAPPING_TYPE: &str = "mapping_type";
const VAL_MT_SEQUENCE: &str = "sequence";
const VAL_MT_TYPE_REF: &str = "type_ref";

const KEY_NAME: &str = "name";
const KEY_VALUE: &str = "value";
const KEY_TYPE_REF: &str = VAL_MT_TYPE_REF;

const KEY_IS_LIBRARY_MODULE: &str = "is_library_module";
const KEY_BASE_URI: &str = "base_uri";
const KEY_VERSION_INFO: &str = "version_info";
const KEY_VERSION_URI: &str = "version_uri";
const KEY_MODULE: &str = "module";
const KEY_MEMBER: &str = "member";
const KEY_MEMBERS: &str = "members";
const KEY_IMPORTS: &str = "imports";
const KEY_DEFINITIONS: &str = "definitions";
const KEY_START: &str = "start";
const KEY_END: &str = "end";
const KEY_SOURCE_FILE: &str = "source_file";
const KEY_SOURCE_SPAN: &str = "source_span";
const KEY_ANNOTATIONS: &str = "annotations";
const KEY_DOMAIN: &str = "domain";
const KEY_RANGE: &str = "range";
const KEY_TYPE: &str = "type";

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn add_module_body(body: &ModuleBody, value: &mut Map<String, Value>) {
    // imports
    if body.has_imports() {
        let mut imports: Vec<Value> = Vec::default();
        for (module_name, maybe_version) in body.imported_module_versions() {
            let mut imported = Map::default();
            imported.insert(KEY_MODULE.into(), module_name.to_string().into());
            if let Some(version) = maybe_version {
                imported.insert(KEY_VERSION_URI.into(), version.to_string().into());
            }
            imports.push(imported.into());
        }

        for member_name in body.imported_types() {
            let mut imported = Map::default();
            imported.insert(KEY_MODULE.into(), member_name.module().to_string().into());
            imported.insert(KEY_MEMBER.into(), member_name.member().to_string().into());
            imports.push(imported.into());
        }
        value.insert(KEY_IMPORTS.into(), imports.into());
    }

    // annotations
    add_annotations(body, value);

    // definitions
    if body.has_definitions() {
        let mut definitions: Vec<Value> = Vec::default();

        for definition in body.definitions() {
            match definition {
                Definition::Datatype(v) => add_datatype(v, &mut definitions),
                Definition::Entity(v) => add_entity(v, &mut definitions),
                Definition::Enum(v) => add_enum(v, &mut definitions),
                Definition::Event(v) => add_event(v, &mut definitions),
                Definition::Property(v) => add_property(v, &mut definitions),
                Definition::Rdf(v) => add_rdf(v, &mut definitions),
                Definition::Structure(v) => add_structure(v, &mut definitions),
                Definition::TypeClass(v) => add_type_class(v, &mut definitions),
                Definition::Union(v) => add_union(v, &mut definitions),
            }
        }

        value.insert(KEY_DEFINITIONS.into(), definitions.into());
    }
}

fn add_source_span(annotated: &impl HasSourceSpan, value: &mut Map<String, Value>) {
    if let Some(source_span) = annotated.source_span() {
        let mut span_value = Map::default();
        span_value.insert(KEY_START.into(), Value::Number(source_span.start().into()));
        span_value.insert(KEY_END.into(), Value::Number(source_span.end().into()));
        value.insert(KEY_SOURCE_SPAN.into(), span_value.into());
    }
}

fn add_annotations(annotated: &impl HasAnnotations, value: &mut Map<String, Value>) {
    if annotated.has_annotations() {
        let mut annotations: Vec<Value> = Vec::default();

        for annotation in annotated.annotations() {
            match annotation {
                Annotation::Property(v) => add_annotation_property(v, &mut annotations),
                Annotation::Constraint(v) => add_annotation_constraint(v, &mut annotations),
            }
        }

        value.insert(KEY_ANNOTATIONS.into(), annotations.into());
    }
}

fn add_annotation_property(property: &AnnotationProperty, value: &mut Vec<Value>) {
    let mut property_map = Map::default();

    add_source_span(property, &mut property_map);
    property_map.insert(KEY_META_TYPE.into(), VAL_MT_PROPERTY.into());
    property_map.insert(
        KEY_NAME.into(),
        property.name_reference().to_string().into(),
    );
    property_map.insert(KEY_VALUE.into(), value_to_value(property.value()));

    value.push(property_map.into());
}

fn value_to_value(value: &SdmlValue) -> Value {
    let mut value_map = Map::default();

    match value {
        SdmlValue::Simple(v) => add_simple_value(v, &mut value_map),
        SdmlValue::ValueConstructor(v) => add_value_constructor(v, &mut value_map),
        SdmlValue::Mapping(v) => add_mapping_value(v, &mut value_map),
        SdmlValue::Reference(v) => {
            value_map.insert(KEY_META_TYPE.into(), KEY_TYPE_REF.into());
            value_map.insert(KEY_VALUE.into(), v.to_string().into());
        }
        SdmlValue::List(vs) => add_value_list(vs, &mut value_map),
    }

    value_map.into()
}

fn add_simple_value(value: &SimpleValue, value_map: &mut Map<String, Value>) {
    match value {
        SimpleValue::Boolean(v) => {
            value_map.insert(KEY_META_TYPE.into(), VAL_MT_BOOLEAN.into());
            value_map.insert(KEY_VALUE.into(), (*v).into());
        }
        SimpleValue::Double(v) => {
            value_map.insert(KEY_META_TYPE.into(), VAL_MT_DOUBLE.into());
            value_map.insert(KEY_VALUE.into(), (*v.as_ref()).into());
        }
        SimpleValue::Decimal(v) => {
            value_map.insert(KEY_META_TYPE.into(), VAL_MT_DECIMAL.into());
            value_map.insert(KEY_VALUE.into(), v.to_string().into());
        }
        SimpleValue::Integer(v) => {
            value_map.insert(KEY_META_TYPE.into(), VAL_MT_INTEGER.into());
            value_map.insert(KEY_VALUE.into(), (*v).into());
        }
        SimpleValue::Unsigned(v) => {
            value_map.insert(KEY_META_TYPE.into(), VAL_MT_UNSIGNED.into());
            value_map.insert(KEY_VALUE.into(), (*v).into());
        }
        SimpleValue::String(v) => {
            value_map.insert(KEY_META_TYPE.into(), VAL_MT_STRING.into());
            value_map.insert(KEY_VALUE.into(), v.to_string().into());
        }
        SimpleValue::IriReference(v) => {
            value_map.insert(KEY_META_TYPE.into(), VAL_MT_URI.into());
            value_map.insert(KEY_VALUE.into(), v.to_string().into());
        }
        SimpleValue::Binary(_) => {
            value_map.insert(KEY_META_TYPE.into(), VAL_MT_BINARY.into());
            // TODO: hex encode here
            value_map.insert(KEY_VALUE.into(), "...".into());
        }
    }
}

fn add_value_constructor(value: &ValueConstructor, value_map: &mut Map<String, Value>) {
    value_map.insert(KEY_META_TYPE.into(), VAL_MT_CONSTRUCTOR.into());

    value_map.insert(KEY_TYPE_REF.into(), value.type_name().to_string().into());

    let mut simple_value_map = Map::default();
    add_simple_value(value.value(), &mut simple_value_map);
    value_map.insert(KEY_VALUE.into(), simple_value_map.into());
}

fn add_mapping_value(value: &MappingValue, value_map: &mut Map<String, Value>) {
    value_map.insert(KEY_META_TYPE.into(), VAL_MT_MAPPING.into());

    let mut domain_map = Map::default();
    add_simple_value(value.domain(), &mut domain_map);
    value_map.insert(KEY_DOMAIN.into(), domain_map.into());

    value_map.insert(KEY_RANGE.into(), value_to_value(value.range()));
}

fn add_value_list(value: &SequenceOfValues, value_map: &mut Map<String, Value>) {
    value_map.insert(KEY_META_TYPE.into(), VAL_MT_SEQUENCE.into());

    let mut members: Vec<Value> = Vec::default();
    for member in value.iter() {
        let mut value_map = Map::default();
        match member {
            SequenceMember::Simple(v) => add_simple_value(v, &mut value_map),
            SequenceMember::ValueConstructor(v) => add_value_constructor(v, &mut value_map),
            SequenceMember::Mapping(v) => add_mapping_value(v, &mut value_map),
            SequenceMember::Reference(v) => {
                value_map.insert(KEY_META_TYPE.into(), VAL_MT_TYPE_REF.into());
                value_map.insert(KEY_TYPE_REF.into(), v.to_string().into());
            }
        }
        members.push(value_map.into());
    }

    value_map.insert("members".into(), members.into());
}

fn add_predicate_value_list(value: &SequenceOfPredicateValues, value_map: &mut Map<String, Value>) {
    value_map.insert(KEY_META_TYPE.into(), VAL_MT_SEQUENCE.into());

    let mut members: Vec<Value> = Vec::default();
    for member in value.iter() {
        let mut value_map = Map::default();
        match member {
            PredicateSequenceMember::Simple(v) => add_simple_value(v, &mut value_map),
            PredicateSequenceMember::ValueConstructor(v) => {
                add_value_constructor(v, &mut value_map)
            }
            PredicateSequenceMember::Mapping(v) => add_mapping_value(v, &mut value_map),
            PredicateSequenceMember::Reference(v) => {
                value_map.insert(KEY_META_TYPE.into(), VAL_MT_TYPE_REF.into());
                value_map.insert(KEY_TYPE_REF.into(), v.to_string().into());
            }
        }
        members.push(value_map.into());
    }

    value_map.insert(KEY_MEMBERS.into(), members.into());
}

fn add_annotation_constraint(constraint: &Constraint, value: &mut Vec<Value>) {
    let mut constraint_map = Map::default();

    add_source_span(constraint, &mut constraint_map);
    constraint_map.insert(KEY_NAME.into(), constraint.name().to_string().into());

    match constraint.body() {
        ConstraintBody::Informal(v) => {
            constraint_map.insert(KEY_META_TYPE.into(), "informal".into());
            constraint_map.insert(KEY_VALUE.into(), v.value().to_string().into());
            if let Some(language) = v.language() {
                constraint_map.insert("language".into(), language.to_string().into());
            }
        }
        ConstraintBody::Formal(v) => {
            constraint_map.insert(KEY_META_TYPE.into(), "formal".into());
            if v.has_definitions() {
                let mut definitions: Vec<Value> = Vec::default();
                for definition in v.definitions() {
                    add_definition(definition, &mut definitions);
                }
                constraint_map.insert("definitions".into(), definitions.into());
            }
            let mut sentence_map = Map::default();
            add_constraint_sentence(v.body(), &mut sentence_map);
            constraint_map.insert("sentence".into(), sentence_map.into());
        }
    }

    value.push(constraint_map.into());
}

fn add_definition(defn: &EnvironmentDef, value: &mut Vec<Value>) {
    let mut defn_map = Map::default();

    defn_map.insert(KEY_NAME.into(), defn.name().to_string().into());

    match defn.body() {
        EnvironmentDefBody::Function(v) => add_function(v, &mut defn_map),
        EnvironmentDefBody::Value(v) => match v {
            PredicateValue::Simple(v) => add_simple_value(v, &mut defn_map),
            PredicateValue::Sequence(v) => add_predicate_value_list(v, &mut defn_map),
        },
        EnvironmentDefBody::Sentence(v) => add_constraint_sentence(v, &mut defn_map),
    }

    value.push(defn_map.into());
}

fn add_function(defn: &FunctionDef, value: &mut Map<String, Value>) {
    value.insert(KEY_META_TYPE.into(), "function".into());

    add_function_signature(defn.signature(), value);

    add_constraint_sentence(defn.body(), value);
}

fn add_function_signature(defn: &FunctionSignature, value: &mut Map<String, Value>) {
    if defn.has_parameters() {
        let mut parameters: Vec<Value> = Vec::default();

        for parameter in defn.parameters() {
            let mut param_map = Map::default();
            param_map.insert(KEY_NAME.into(), parameter.name().to_string().into());
            add_function_type(parameter.target_type(), &mut param_map);
            parameters.push(param_map.into());
        }

        value.insert("parameters".into(), parameters.into());
    }

    let mut type_map = Map::default();
    add_function_type(defn.target_type(), &mut type_map);
    value.insert(KEY_TYPE.into(), type_map.into());
}

fn add_function_type(defn: &FunctionType, value: &mut Map<String, Value>) {
    let cardinality = defn.target_cardinality();
    let mut cardinality_map = Map::default();
    if let Some(ordering) = cardinality.ordering() {
        cardinality_map.insert("ordering".into(), ordering.to_string().into());
    }
    if let Some(uniqueness) = cardinality.uniqueness() {
        cardinality_map.insert("uniqueness".into(), uniqueness.to_string().into());
    }
    if let Some(range) = cardinality.range() {
        cardinality_map.insert("min_occurs".into(), range.min_occurs().into());
        if let Some(max_occurs) = range.max_occurs() {
            cardinality_map.insert("max_occurs".into(), max_occurs.into());
        }
    }
    value.insert("cardinality".into(), cardinality_map.into());

    let mut type_map = Map::default();
    let target_type = defn.target_type();
    type_map.insert("is_optional".into(), target_type.is_optional().into());
    match target_type.inner() {
        FunctionTypeReferenceInner::Wildcard => {
            type_map.insert(KEY_META_TYPE.into(), "wildcard".into());
        }
        FunctionTypeReferenceInner::Reference(v) => {
            type_map.insert(KEY_META_TYPE.into(), VAL_MT_TYPE_REF.into());
            type_map.insert(KEY_TYPE_REF.into(), v.to_string().into());
        }
        FunctionTypeReferenceInner::MappingType(v) => {
            type_map.insert(KEY_META_TYPE.into(), VAL_MT_MAPPING_TYPE.into());
            type_map.insert(KEY_DOMAIN.into(), v.domain().to_string().into());
            type_map.insert(KEY_RANGE.into(), v.range().to_string().into());
        }
    }

    value.insert("type".into(), type_map.into());
}

fn add_constraint_sentence(defn: &ConstraintSentence, value: &mut Map<String, Value>) {
    match defn {
        ConstraintSentence::Simple(v) => match v {
            SimpleSentence::Atomic(v) => add_atomic_sentence(v, value),
            SimpleSentence::Equation(v) => add_equation(v, value),
            SimpleSentence::Inequation(v) => add_inequation(v, value),
        },
        ConstraintSentence::Boolean(v) => match v {
            BooleanSentence::Unary(v) => add_unary_boolean(v, value),
            BooleanSentence::Binary(v) => add_binary_boolean(v, value),
        },
        ConstraintSentence::Quantified(v) => add_quantified_sentence(v, value),
    }
}

fn add_atomic_sentence(defn: &AtomicSentence, value: &mut Map<String, Value>) {
    value.insert(KEY_META_TYPE.into(), "atomic_sentence".into());

    value.insert("function".into(), term_to_value(defn.predicate()));

    if defn.has_arguments() {
        let mut arguments: Vec<Value> = Vec::default();
        for argument in defn.arguments() {
            arguments.push(term_to_value(argument));
        }
        value.insert("arguments".into(), arguments.into());
    }
}

fn add_equation(defn: &Equation, value: &mut Map<String, Value>) {
    value.insert(KEY_META_TYPE.into(), "equation".into());
    value.insert("lhs".into(), term_to_value(defn.left_operand()));
    value.insert("rhs".into(), term_to_value(defn.right_operand()));
}

fn add_inequation(defn: &Inequation, value: &mut Map<String, Value>) {
    value.insert("type".into(), "ineqation".into());
    value.insert("lhs".into(), term_to_value(defn.left_operand()));
    value.insert("relation".into(), defn.relation().to_string().into());
    value.insert("rhs".into(), term_to_value(defn.right_operand()));
}

fn add_unary_boolean(defn: &UnaryBooleanSentence, value: &mut Map<String, Value>) {
    value.insert(KEY_META_TYPE.into(), "unary_boolean_sentence".into());

    value.insert("operator".into(), defn.operator().to_string().into());

    let mut sentence_map = Map::default();
    add_constraint_sentence(defn.operand(), &mut sentence_map);
    value.insert("operand".into(), sentence_map.into());
}

fn add_binary_boolean(defn: &BinaryBooleanSentence, value: &mut Map<String, Value>) {
    value.insert(KEY_META_TYPE.into(), "binary_boolean_sentence".into());

    let mut sentence_map = Map::default();
    add_constraint_sentence(defn.left_operand(), &mut sentence_map);
    value.insert("lhs".into(), sentence_map.into());

    value.insert("operator".into(), defn.operator().to_string().into());

    let mut sentence_map = Map::default();
    add_constraint_sentence(defn.right_operand(), &mut sentence_map);
    value.insert("rhs".into(), sentence_map.into());
}

fn add_quantified_sentence(defn: &QuantifiedSentence, value: &mut Map<String, Value>) {
    value.insert(KEY_META_TYPE.into(), "quantified_sentence".into());

    let mut binding_map = Map::default();
    let variable_binding = defn.binding();
    binding_map.insert(
        "quantifier".into(),
        variable_binding.quantifier().to_string().into(),
    );
    if let Some(binding) = variable_binding.binding() {
        binding_map.insert("name".into(), binding.name().to_string().into());
        binding_map.insert("source".into(), term_to_value(binding.source()));
    }
    value.insert("binding".into(), binding_map.into());

    let mut sentence_map = Map::default();
    add_constraint_sentence(defn.body(), &mut sentence_map);
    value.insert("sentence".into(), sentence_map.into());
}

fn term_to_value(defn: &Term) -> Value {
    let mut term_map = Map::default();
    match defn {
        Term::Sequence(v) => {
            term_map.insert(KEY_META_TYPE.into(), "sequence_builder".into());

            match v.variables() {
                Variables::Named(v) => {
                    let mut names: Vec<Value> = Vec::default();
                    for name in v.names() {
                        names.push(name.to_string().into());
                    }
                    term_map.insert("named".into(), names.into());
                }
                Variables::Mapping(v) => {
                    term_map.insert("domain".into(), v.domain().to_string().into());
                    term_map.insert("range".into(), v.range().to_string().into());
                }
            }

            let mut sentence_map = Map::default();
            add_quantified_sentence(v.body(), &mut sentence_map);
            term_map.insert("sentence".into(), sentence_map.into());
        }
        Term::Function(v) => {
            term_map.insert(KEY_META_TYPE.into(), "function".into());
            term_map.insert("function".into(), term_to_value(v.function()));
            if v.has_arguments() {
                let mut arguments: Vec<Value> = Vec::default();
                for argument in v.arguments() {
                    arguments.push(term_to_value(argument));
                }
                term_map.insert("arguments".into(), arguments.into());
            }
        }
        Term::Composition(v) => {
            term_map.insert(KEY_META_TYPE.into(), "composition".into());
            term_map.insert(
                "subject".into(),
                match v.subject() {
                    Subject::ReservedSelf => "self".into(),
                    Subject::Identifier(v) => v.to_string().into(),
                },
            );
            if v.has_function_names() {
                let mut functions: Vec<Value> = Vec::default();
                for function in v.function_names() {
                    functions.push(function.to_string().into());
                }
                term_map.insert("functions".into(), functions.into());
            }
        }
        Term::Identifier(v) => {
            term_map.insert(KEY_META_TYPE.into(), "reference".into());
            term_map.insert("name".into(), v.to_string().into());
        }
        Term::ReservedSelf => {
            term_map.insert(KEY_META_TYPE.into(), "self".into());
        }
        Term::Value(v) => match v {
            PredicateValue::Simple(v) => add_simple_value(v, &mut term_map),
            PredicateValue::Sequence(v) => add_predicate_value_list(v, &mut term_map),
        },
    }
    term_map.into()
}

fn add_datatype(defn: &DatatypeDef, value: &mut Vec<Value>) {
    let mut defn_map = Map::default();

    add_source_span(defn, &mut defn_map);
    defn_map.insert(KEY_META_TYPE.into(), "datatype".into());
    defn_map.insert(KEY_NAME.into(), defn.name().to_string().into());
    defn_map.insert("is_opaque".into(), defn.is_opaque().into());
    defn_map.insert("base_type".into(), defn.base_type().to_string().into());

    if let Some(body) = defn.body() {
        add_annotations(body, &mut defn_map);
    }

    value.push(defn_map.into());
}

fn add_entity(defn: &EntityDef, value: &mut Vec<Value>) {
    let mut defn_map = Map::default();

    add_source_span(defn, &mut defn_map);
    defn_map.insert(KEY_META_TYPE.into(), "entity".into());
    defn_map.insert(KEY_NAME.into(), defn.name().to_string().into());

    if let Some(body) = defn.body() {
        defn_map.insert("identity".into(), member_to_value(body.identity()));

        add_annotations(body, &mut defn_map);

        if body.has_members() {
            let mut members: Vec<Value> = Vec::default();

            for member in body.members() {
                members.push(member_to_value(member));
            }

            defn_map.insert("members".into(), members.into());
        }
    }

    value.push(defn_map.into());
}

fn member_to_value(defn: &Member) -> Value {
    let mut defn_map = Map::default();

    add_source_span(defn, &mut defn_map);

    match defn.kind() {
        MemberKind::Reference(v) => {
            defn_map.insert("kind".into(), "reference".into());
            defn_map.insert("property".into(), v.to_string().into());
        }
        MemberKind::Definition(v) => {
            defn_map.insert("kind".into(), "definition".into());
            add_member_def(v, &mut defn_map)
        }
    }

    defn_map.into()
}

fn add_member_def(defn: &MemberDef, defn_map: &mut Map<String, Value>) {
    defn_map.insert(KEY_NAME.into(), defn.name().to_string().into());
    let cardinality = defn.target_cardinality();
    let mut cardinality_map = Map::default();
    if let Some(ordering) = cardinality.ordering() {
        cardinality_map.insert("ordering".into(), ordering.to_string().into());
    }
    if let Some(uniqueness) = cardinality.uniqueness() {
        cardinality_map.insert("uniqueness".into(), uniqueness.to_string().into());
    }
    cardinality_map.insert("min_occurs".into(), cardinality.min_occurs().into());
    if let Some(max_occurs) = cardinality.max_occurs() {
        cardinality_map.insert("max_occurs".into(), max_occurs.into());
    }
    defn_map.insert("cardinality".into(), cardinality_map.into());
    defn_map.insert(KEY_TYPE_REF.into(), defn.target_type().to_string().into());
    if let Some(body) = defn.body() {
        add_annotations(body, defn_map);
    }
}

fn add_enum(defn: &EnumDef, value: &mut Vec<Value>) {
    let mut defn_map = Map::default();

    add_source_span(defn, &mut defn_map);
    defn_map.insert(KEY_META_TYPE.into(), "enum".into());
    defn_map.insert(KEY_NAME.into(), defn.name().to_string().into());

    if let Some(body) = defn.body() {
        add_annotations(body, &mut defn_map);

        if body.has_variants() {
            let mut variants: Vec<Value> = Vec::default();
            for variant in body.variants() {
                let mut variant_map = Map::default();
                add_source_span(variant, &mut variant_map);
                variant_map.insert(KEY_NAME.into(), variant.name().to_string().into());
                if let Some(body) = variant.body() {
                    add_annotations(body, &mut variant_map);
                }
                variants.push(variant_map.into());
            }
            defn_map.insert("variants".into(), variants.into());
        }
    }

    value.push(defn_map.into());
}

fn add_event(defn: &EventDef, value: &mut Vec<Value>) {
    let mut defn_map = Map::default();

    add_source_span(defn, &mut defn_map);
    defn_map.insert(KEY_META_TYPE.into(), "event".into());
    defn_map.insert(KEY_NAME.into(), defn.name().to_string().into());
    defn_map.insert("source_ref".into(), defn.event_source().to_string().into());

    if let Some(body) = defn.body() {
        add_annotations(body, &mut defn_map);
        if body.has_members() {
            let mut members: Vec<Value> = Vec::default();

            for member in body.members() {
                members.push(member_to_value(member));
            }

            defn_map.insert("members".into(), members.into());
        }
    }

    value.push(defn_map.into());
}

fn add_property(defn: &PropertyDef, value: &mut Vec<Value>) {
    let mut defn_map = Map::default();

    add_source_span(defn, &mut defn_map);
    defn_map.insert(KEY_META_TYPE.into(), "property".into());
    defn_map.insert(KEY_NAME.into(), defn.name().to_string().into());

    let mut member_map = Map::default();
    add_member_def(defn.member_def(), &mut member_map);
    defn_map.insert("member".into(), member_map.into());

    value.push(defn_map.into());
}

fn add_rdf(defn: &RdfDef, value: &mut Vec<Value>) {
    let mut defn_map = Map::default();

    add_source_span(defn, &mut defn_map);
    defn_map.insert(KEY_META_TYPE.into(), "rdf".into());
    defn_map.insert(KEY_NAME.into(), defn.name().to_string().into());

    add_annotations(defn.body(), &mut defn_map);

    value.push(defn_map.into());
}

fn add_structure(defn: &StructureDef, value: &mut Vec<Value>) {
    let mut defn_map = Map::default();

    add_source_span(defn, &mut defn_map);
    defn_map.insert(KEY_META_TYPE.into(), "structure".into());
    defn_map.insert(KEY_NAME.into(), defn.name().to_string().into());

    if let Some(body) = defn.body() {
        add_annotations(body, &mut defn_map);
        if body.has_members() {
            let mut members: Vec<Value> = Vec::default();

            for member in body.members() {
                members.push(member_to_value(member));
            }

            defn_map.insert("members".into(), members.into());
        }
    }

    value.push(defn_map.into());
}

fn add_type_class(defn: &TypeClassDef, value: &mut Vec<Value>) {
    let mut defn_map = Map::default();

    add_source_span(defn, &mut defn_map);
    defn_map.insert(KEY_META_TYPE.into(), "type_class".into());
    defn_map.insert(KEY_NAME.into(), defn.name().to_string().into());

    if defn.has_variables() {
        let mut variables: Vec<Value> = Vec::default();
        for variable in defn.variables() {
            add_type_variable(variable, &mut variables);
        }
        defn_map.insert("variables".into(), variables.into());
    }

    if let Some(body) = defn.body() {
        add_annotations(body, &mut defn_map);
        if body.has_methods() {
            let mut methods: Vec<Value> = Vec::default();
            for method in body.methods() {
                add_type_method(method, &mut methods);
            }
            defn_map.insert("methods".into(), methods.into());
        }
    }

    value.push(defn_map.into());
}

fn add_type_variable(defn: &TypeVariable, value: &mut Vec<Value>) {
    let mut var_map = Map::default();

    var_map.insert(KEY_NAME.into(), defn.name().to_string().into());
    // TODO: cardinality
    // TODO: restrictions

    value.push(var_map.into());
}

fn add_type_method(defn: &MethodDef, value: &mut Vec<Value>) {
    let mut var_map = Map::default();

    var_map.insert(KEY_NAME.into(), defn.name().to_string().into());
    // TODO: signature
    // TODO: body
    // TODO: annotations

    value.push(var_map.into());
}

fn add_union(defn: &UnionDef, value: &mut Vec<Value>) {
    let mut defn_map = Map::default();

    add_source_span(defn, &mut defn_map);
    defn_map.insert(KEY_META_TYPE.into(), "union".into());
    defn_map.insert(KEY_NAME.into(), defn.name().to_string().into());

    if let Some(body) = defn.body() {
        add_annotations(body, &mut defn_map);

        if body.has_variants() {
            let mut variants: Vec<Value> = Vec::default();
            for variant in body.variants() {
                let mut variant_map = Map::default();
                variant_map.insert(KEY_NAME.into(), variant.name().to_string().into());
                if let Some(rename) = variant.rename() {
                    variant_map.insert("rename".into(), rename.to_string().into());
                }
                if let Some(body) = variant.body() {
                    add_annotations(body, &mut variant_map);
                }
                variants.push(variant_map.into());
            }
            defn_map.insert("variants".into(), variants.into());
        }
    }

    value.push(defn_map.into());
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------