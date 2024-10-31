/*!
This module provides SDML to Value functions for constructing template contexts.

Note that the created context values *are not* intended as a direct 1:1 representation of either the
published surface syntax grammar or the Rust model. The form is simplified for use in the template
language using the following guidelines.

1. Reduce layers in the model that do not add value; i.e. [Identifier` in the Rust model has an
   inner `value` field.
2. Where an `Option<T>` field is `None` do not add a key in the generated object.
3. Where a `Vec<T>` field `is_empty` do not add a key in the generated object.
4. Use the key `"__type"` as a discriminator where the content of an object is ambiguous, especially
   in arrays.
5. Only add `source_span` values for major objects such as definitions, not for individual names
   etc.

The upshot of this is that an `if` statement in a template is used to check for presence of a value
before you use it. The following demonstrates this pattern for optional fields and  possibly empty
collections.

```md
{% if module.base_uri -%}
 *Base URI*: {{ module.base_uri }}
{%- endif %}

{% if module.annotations -%}
  {% for ann in module.annotations -%}
    {{ ann.name }}
  {%- endfor %}
{%- endif %}
```

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
    DatatypeDef, Definition, DimensionDef, DimensionIdentity, DimensionParent, EntityDef, EnumDef,
    EventDef, HasMembers, HasVariants, MethodDef, PropertyDef, RdfDef, SourceEntity, StructureDef,
    TypeClassDef, TypeVariable, UnionDef,
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
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Convert a SDML `Module` into a context object, in the form shown as JSON below.
///
/// ```json
/// {
///   "__type": "module",
///   "name": "Identifier",
///   "is_library_module": true,
///   "source_file": "Path",            // optional
///   "source_span": {
///     "start": 0,
///     "end": 10,
///   }, // optional
///   "base_uri": "absolute-uri",       // optional
///   "version_info": "string",         // optional
///   "version_uri": "absolute-uri",    // optional
///   "imports": [],                    // optional
///   "definitions": [],                // optional
///   "annotations": []                 // optional
/// }
/// ```
///
/// # Import Object
///
/// Module import:
///
/// ```json
/// {
///   "module": "Identifier",
///   "version_uri": "absolute-uri"     // optional
/// }
/// ```
///
/// Member import:
///
/// ```json
/// {
///   "module": "Identifier",
///   "member": "Identifier"
/// }
/// ```
///
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

///
/// Convert a SDML `AnnotationProperty` into a context object, in the form shown as JSON below.
///
/// ```json
/// {
///     "__type": "property",
///     "source_span": {},              // optional
///     "name": "IdentifierReference",  // optional
///     "value": {}
/// }
/// ```
///
pub fn annotation_property_to_value(property: &AnnotationProperty) -> Value {
    let mut property_map = Map::default();

    add_source_span(property, &mut property_map);
    property_map.insert(KEY_META_TYPE.into(), VAL_MT_PROPERTY.into());
    property_map.insert(
        KEY_NAME.into(),
        property.name_reference().to_string().into(),
    );
    property_map.insert(KEY_VALUE.into(), value_to_value(property.value()));

    property_map.into()
}

///
/// Convert a SDML `Value` into a context object, in the form shown as JSON below.
///
/// # Simple Value
///
/// ```json
/// {
///     "__type": "boolean|double|decimal|integer|unsigned|string|uri|binary",
///     "value": ...
/// }
/// ```
///
/// # Value Constructor
///
/// ```json
/// {
///     "__type": "constructor",
///     "type_ref": "IdentifierReference",
///     "value": {}
/// }
/// ```
///
/// # Mapping
///
/// ```json
/// {
///     "__type": "mapping",
///     "domain": {},
///     "range": {}
/// }
/// ```
///
/// # Reference
///
/// ```json
/// {
///     "__type": "type_ref",
///     "value": "IdentifierReference",
/// }
/// ```
///
/// # Sequence
///
/// ```json
/// {
///     "__type": "sequence",
///     "members": []
/// }
/// ```
///
pub fn value_to_value(value: &SdmlValue) -> Value {
    let mut value_map = Map::default();

    match value {
        SdmlValue::Simple(v) => add_simple_value(v, &mut value_map),
        SdmlValue::ValueConstructor(v) => add_value_constructor(v, &mut value_map),
        SdmlValue::Mapping(v) => add_mapping_value(v, &mut value_map),
        SdmlValue::Reference(v) => {
            value_map.insert(KEY_META_TYPE.into(), KEY_TYPE_REF.into());
            value_map.insert(KEY_VALUE.into(), v.to_string().into());
        }
        SdmlValue::Sequence(vs) => add_value_list(vs, &mut value_map),
    }

    value_map.into()
}

///
/// Convert a SDML `Constraint` into a context object, in the form shown as JSON below.
///
/// ## Informal Constraint
///
/// ```json
/// {
///     "__type": "informal",
///     "source_span": {},              // optional
///     "name": "Identifier",
///     "value": "string",
///     "language": ""                  // optional
/// }
/// ```
///
/// ## Formal Constraint
///
/// ```json
/// {
///     "__type": "formal",
///     "source_span": {},             // optional
///     "name": "Identifier",
///     "definitions": [],             // optional
///     "sentence": {}
/// }
/// ```
///
pub fn annotation_constraint_to_value(constraint: &Constraint) -> Value {
    let mut constraint_map = Map::default();

    add_source_span(constraint, &mut constraint_map);
    constraint_map.insert(KEY_NAME.into(), constraint.name().to_string().into());

    match constraint.body() {
        ConstraintBody::Informal(v) => {
            constraint_map.insert(KEY_META_TYPE.into(), VAL_MT_INFORMAL.into());
            constraint_map.insert(KEY_VALUE.into(), v.value().to_string().into());
            if let Some(language) = v.language() {
                constraint_map.insert(KEY_LANGUAGE.into(), language.to_string().into());
            }
        }
        ConstraintBody::Formal(v) => {
            constraint_map.insert(KEY_META_TYPE.into(), VAL_MT_FORMAL.into());
            if v.has_definitions() {
                let mut definitions: Vec<Value> = Vec::default();
                for definition in v.definitions() {
                    add_definition(definition, &mut definitions);
                }
                constraint_map.insert(KEY_DEFINITIONS.into(), definitions.into());
            }
            let mut sentence_map = Map::default();
            add_constraint_sentence(v.body(), &mut sentence_map);
            constraint_map.insert(KEY_SENTENCE.into(), sentence_map.into());
        }
    }

    constraint_map.into()
}

///
/// Convert a SDML `DatatypeDef` into a context object, in the form shown as JSON below.
///
/// ```json
/// {
///     "__type": "datatype",
///     "source_span": {},              // optional
///     "name": "Identifier",
///     "is_opaque": false,
///     "base_type": "IdentifierReference",
///     "annotations": []               // optional
/// }
/// ```
///
pub fn datatype_to_value(defn: &DatatypeDef) -> Value {
    let mut defn_map = Map::default();

    add_source_span(defn, &mut defn_map);
    defn_map.insert(KEY_META_TYPE.into(), VAL_MT_DATATYPE.into());
    defn_map.insert(KEY_NAME.into(), defn.name().to_string().into());
    defn_map.insert(KEY_IS_OPAQUE.into(), defn.is_opaque().into());
    defn_map.insert(KEY_BASE_TYPE.into(), defn.base_type().to_string().into());

    if let Some(body) = defn.body() {
        add_annotations(body, &mut defn_map);
    }

    defn_map.into()
}

///
/// Convert a SDML `EntityDef` into a context object, in the form shown as JSON below.
///
/// ```json
/// {
///     "__type": "entity",
///     "source_span": {},              // optional
///     "name": "Identifier",
///     "identity": {},
///     "members": [],                  // optional
///     "annotations": []               // optional
/// }
/// ```
///
/// For `identity`, see MemberDef and for `members` see Member, in [`member_to_value`].
///
pub fn entity_to_value(defn: &EntityDef) -> Value {
    let mut defn_map = Map::default();

    add_source_span(defn, &mut defn_map);
    defn_map.insert(KEY_META_TYPE.into(), VAL_MT_ENTITY.into());
    defn_map.insert(KEY_NAME.into(), defn.name().to_string().into());

    if let Some(body) = defn.body() {
        defn_map.insert(KEY_IDENTITY.into(), member_to_value(body.identity()));

        if body.has_members() {
            let mut members: Vec<Value> = Vec::default();

            for member in body.members() {
                members.push(member_to_value(member));
            }

            defn_map.insert(KEY_MEMBERS.into(), members.into());
        }

        add_annotations(body, &mut defn_map);
    }

    defn_map.into()
}

///
/// Convert a SDML `Member` into a context object, in the form shown as JSON below.
///
///
/// A member is either a reference to a property or a definition of a new member.
///
/// # Property Reference
///
/// ```json
/// {
///     "__type": "reference",
///     "type_ref": "IdentifierReference",
/// }
/// ```
///
/// # Member Definition
///
/// ```json
/// {
///     "__type": "definition",
///     "source_span": {},              // optional
///     "name": "Identifier",
///     "cardinality": {
///         "ordering": "",
///         "uniqueness": "",
///         "min_occurs": 1,
///         "max_occurs": 0             // optional
///     },
///     "type_ref": "IdentifierReference"
/// }
/// ```
///
pub fn member_to_value(defn: &Member) -> Value {
    let mut defn_map = Map::default();

    add_source_span(defn, &mut defn_map);

    match defn.kind() {
        MemberKind::Reference(v) => {
            defn_map.insert(KEY_META_TYPE.into(), VAL_MT_DEFINITION.into());
            defn_map.insert(KEY_TYPE_REF.into(), v.to_string().into());
        }
        MemberKind::Definition(v) => {
            defn_map.insert(KEY_META_TYPE.into(), VAL_MT_DEFINITION.into());
            add_member_def(v, &mut defn_map)
        }
    }

    defn_map.into()
}

///
/// Convert a SDML `EnumDef` into a context object, in the form shown as JSON below.
///
/// ```json
/// {
///     "__type": "enum",
///     "source_span": {},              // optional
///     "name": "Identifier",
///     "variants": [                   // optional
///         {
///             "name": "Identifier",
///             "annotations": []       // optional
///         }
///     ],
///     "annotations": []               // optional
/// }
/// ```
///
pub fn enum_to_value(defn: &EnumDef) -> Value {
    let mut defn_map = Map::default();

    add_source_span(defn, &mut defn_map);
    defn_map.insert(KEY_META_TYPE.into(), VAL_MT_ENUM.into());
    defn_map.insert(KEY_NAME.into(), defn.name().to_string().into());

    if let Some(body) = defn.body() {
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
            defn_map.insert(KEY_VARIANTS.into(), variants.into());
        }

        add_annotations(body, &mut defn_map);
    }

    defn_map.into()
}

///
/// Convert a SDML `EventDef` into a context object, in the form shown as JSON below.
///
/// ```json
/// {
///     "__type": "event",
///     "source_span": {},              // optional
///     "name": "Identifier",
///     "event_source": {},             // SourceEntity
///     "members": [],                  // optional
///     "annotations": []               // optional
/// }
/// ```
///
/// For `members`, see [`member_to_value`], and for `event_source` see [`dimension_to_value`].
///
pub fn event_to_value(defn: &EventDef) -> Value {
    let mut defn_map = Map::default();

    add_source_span(defn, &mut defn_map);
    defn_map.insert(KEY_META_TYPE.into(), VAL_MT_EVENT.into());
    defn_map.insert(KEY_NAME.into(), defn.name().to_string().into());

    if let Some(body) = defn.body() {
        defn_map.insert(
            KEY_EVENT_SOURCE.into(),
            source_entity_to_value(body.event_source()),
        );
        if body.has_members() {
            let mut members: Vec<Value> = Vec::default();

            for member in body.members() {
                members.push(member_to_value(member));
            }

            defn_map.insert(KEY_MEMBERS.into(), members.into());
        }
        add_annotations(body, &mut defn_map);
    }

    defn_map.into()
}

///
/// Convert a SDML `DimensionDef` into a context object, in the form shown as JSON below.
///
/// ```json
/// {
///     "__type": "event",
///     "source_span": {},              // optional
///     "name": "Identifier",
///     "entity": {...},                // SourceEntity or Member
///     "parents": [],                  // optional
///     "members": [],                  // optional
///     "annotations": []               // optional
/// }
/// ```
///
/// # DimensionParent
///
/// ```json
/// {
///     "source_span": {},              // optional
///     "name": "Identifier",
///     "entity": "IdentifierReference",
///     "annotations": []               // optional
/// }
/// ```
///
/// # SourceEntity
///
/// ```json
/// {
///     "__type": "source_entity",
///     "source_span": {},              // optional
///     "entity": "IdentifierReference",
///     "members": []                   // optional
/// }
/// ```
///
/// For `members`, see [`member_to_value`].
///
pub fn dimension_to_value(defn: &DimensionDef) -> Value {
    let mut defn_map = Map::default();

    add_source_span(defn, &mut defn_map);
    defn_map.insert(KEY_META_TYPE.into(), VAL_MT_DIMENSION.into());
    defn_map.insert(KEY_NAME.into(), defn.name().to_string().into());

    if let Some(body) = defn.body() {
        let identity = match body.identity() {
            DimensionIdentity::Source(source) => source_entity_to_value(source),
            DimensionIdentity::Identity(identity) => member_to_value(identity),
        };
        defn_map.insert(KEY_IDENTITY.into(), identity);

        if body.has_parents() {
            let mut parents: Vec<Value> = Vec::default();

            for parent in body.parents() {
                parents.push(parent_to_value(parent));
            }

            defn_map.insert(KEY_PARENTS.into(), parents.into());
        }

        if body.has_members() {
            let mut members: Vec<Value> = Vec::default();

            for member in body.members() {
                members.push(member_to_value(member));
            }

            defn_map.insert(KEY_MEMBERS.into(), members.into());
        }

        add_annotations(body, &mut defn_map);
    }

    defn_map.into()
}

///
/// Convert a SDML `PropertyDef` into a context object, in the form shown as JSON below.
///
/// ```json
/// {
///     "__type": "property",
///     "source_span": {},              // optional
///     "name": "Identifier",
///     "cardinality": {
///         "ordering": "",
///         "uniqueness": "",
///         "min_occurs": 1,
///         "max_occurs": 0             // optional
///     },
///     "type_ref": "IdentifierReference"
/// }
/// ```
///
/// For `member` see member definition in [`member_to_value`].
///
pub fn property_to_value(defn: &PropertyDef) -> Value {
    let mut defn_map = Map::default();

    add_source_span(defn, &mut defn_map);
    defn_map.insert(KEY_META_TYPE.into(), VAL_MT_PROPERTY.into());

    add_member_def(defn.member_def(), &mut defn_map);

    defn_map.into()
}

///
/// Convert a SDML `RdfDef` into a context object, in the form shown as JSON below.
///
/// ```json
/// {
///     "__type": "rdf",
///     "source_span": {},              // optional
///     "name": "Identifier",
///     "annotations": []               // optional
/// }
/// ```
///
pub fn rdf_to_value(defn: &RdfDef) -> Value {
    let mut defn_map = Map::default();

    add_source_span(defn, &mut defn_map);
    defn_map.insert(KEY_META_TYPE.into(), VAL_MT_RDF.into());
    defn_map.insert(KEY_NAME.into(), defn.name().to_string().into());

    add_annotations(defn.body(), &mut defn_map);

    defn_map.into()
}

///
/// Convert a SDML `StructureDef` into a context object, in the form shown as JSON below.
///
/// ```json
/// {
///     "__type": "structure",
///     "source_span": {},              // optional
///     "name": "Identifier",
///     "members": [],                  // optional
///     "annotations": []               // optional
/// }
/// ```
///
/// For `members`, see [`member_to_value`].
///
pub fn structure_to_value(defn: &StructureDef) -> Value {
    let mut defn_map = Map::default();

    add_source_span(defn, &mut defn_map);
    defn_map.insert(KEY_META_TYPE.into(), VAL_MT_STRUCTURE.into());
    defn_map.insert(KEY_NAME.into(), defn.name().to_string().into());

    if let Some(body) = defn.body() {
        if body.has_members() {
            let mut members: Vec<Value> = Vec::default();

            for member in body.members() {
                members.push(member_to_value(member));
            }

            defn_map.insert(KEY_MEMBERS.into(), members.into());
        }
        add_annotations(body, &mut defn_map);
    }

    defn_map.into()
}

///
/// Convert a SDML `TypeClassDef` into a context object, in the form shown as JSON below.
///
/// ```json
/// {
///     "__type": "type_class",
///     "source_span": {},              // optional
///     "name": "Identifier",
///     "variables": [],                // optional
///     "methods": [],                  // optional
///     "annotations": []               // optional
/// }
/// ```
///
/// ## Variable
///
/// TBD
///
/// ## Method
///
/// TBD
///
pub fn type_class_to_value(defn: &TypeClassDef) -> Value {
    let mut defn_map = Map::default();

    add_source_span(defn, &mut defn_map);
    defn_map.insert(KEY_META_TYPE.into(), VAL_MT_TYPE_CLASS.into());
    defn_map.insert(KEY_NAME.into(), defn.name().to_string().into());

    if defn.has_variables() {
        let mut variables: Vec<Value> = Vec::default();
        for variable in defn.variables() {
            add_type_variable(variable, &mut variables);
        }
        defn_map.insert(KEY_VARIABLES.into(), variables.into());
    }

    if let Some(body) = defn.body() {
        if body.has_methods() {
            let mut methods: Vec<Value> = Vec::default();
            for method in body.methods() {
                add_type_method(method, &mut methods);
            }
            defn_map.insert(KEY_METHODS.into(), methods.into());
        }
        add_annotations(body, &mut defn_map);
    }

    defn_map.into()
}

///
/// Convert a SDML `UnionDef` into a context object, in the form shown as JSON below.
///
/// ```json
/// {
///     "__type": "union",
///     "source_span": {},              // optional
///     "name": "Identifier",
///     "variants": [                   // optional
///         {
///             "name": "IdentifierReference",
///             "rename": "Identifier",
///             "annotations": []
///         }
///     ],
///     "annotations": []               // optional
/// }
/// ```
///
pub fn union_to_value(defn: &UnionDef) -> Value {
    let mut defn_map = Map::default();

    add_source_span(defn, &mut defn_map);
    defn_map.insert(KEY_META_TYPE.into(), VAL_MT_UNION.into());
    defn_map.insert(KEY_NAME.into(), defn.name().to_string().into());

    if let Some(body) = defn.body() {
        add_annotations(body, &mut defn_map);

        if body.has_variants() {
            let mut variants: Vec<Value> = Vec::default();
            for variant in body.variants() {
                let mut variant_map = Map::default();
                variant_map.insert(KEY_NAME.into(), variant.name().to_string().into());
                if let Some(rename) = variant.rename() {
                    variant_map.insert(KEY_RENAME.into(), rename.to_string().into());
                }
                if let Some(body) = variant.body() {
                    add_annotations(body, &mut variant_map);
                }
                variants.push(variant_map.into());
            }
            defn_map.insert(KEY_VARIANTS.into(), variants.into());
        }
    }

    defn_map.into()
}

// ------------------------------------------------------------------------------------------------
// Private Values
// ------------------------------------------------------------------------------------------------

const KEY_META_TYPE: &str = "__type";

const VAL_MT_BINARY: &str = "binary";
const VAL_MT_BOOLEAN: &str = "boolean";
const VAL_MT_CONSTRUCTOR: &str = "constructor";
const VAL_MT_DATATYPE: &str = "datatype";
const VAL_MT_DECIMAL: &str = "decimal";
const VAL_MT_DEFINITION: &str = "definition";
const VAL_MT_DIMENSION: &str = "dimension";
const VAL_MT_DOUBLE: &str = "double";
const VAL_MT_ENTITY: &str = "entity";
const VAL_MT_ENUM: &str = "enum";
const VAL_MT_EVENT: &str = "event";
const VAL_MT_FORMAL: &str = "formal";
const VAL_MT_INFORMAL: &str = "informal";
const VAL_MT_INTEGER: &str = "integer";
const VAL_MT_MAPPING: &str = "mapping";
const VAL_MT_MAPPING_TYPE: &str = "mapping_type";
const VAL_MT_MODULE: &str = "module";
const VAL_MT_PROPERTY: &str = "property";
const VAL_MT_RDF: &str = "rdf";
const VAL_MT_SEQUENCE: &str = "sequence";
const VAL_MT_SOURCE_ENTITY: &str = "source_entity";
const VAL_MT_STRING: &str = "string";
const VAL_MT_STRUCTURE: &str = "structure";
const VAL_MT_TYPE_CLASS: &str = "type_class";
const VAL_MT_TYPE_REF: &str = "type_ref";
const VAL_MT_UNION: &str = "union";
const VAL_MT_UNSIGNED: &str = "unsigned";
const VAL_MT_URI: &str = "uri";
const VAL_MT_WILDCARD: &str = "wildcard";

const KEY_ANNOTATIONS: &str = "annotations";
const KEY_BASE_TYPE: &str = "base_type";
const KEY_BASE_URI: &str = "base_uri";
const KEY_CARDINALITY: &str = "cardinality";
const KEY_DEFINITIONS: &str = "definitions";
const KEY_DOMAIN: &str = "domain";
const KEY_END: &str = "end";
const KEY_ENTITY: &str = VAL_MT_ENTITY;
const KEY_EVENT_SOURCE: &str = "event_source";
const KEY_IDENTITY: &str = "identity";
const KEY_IMPORTS: &str = "imports";
const KEY_IS_LIBRARY_MODULE: &str = "is_library_module";
const KEY_IS_OPAQUE: &str = "is_opaque";
const KEY_IS_OPTIONAL: &str = "is_optional";
const KEY_LANGUAGE: &str = "language";
const KEY_MAX_OCCURS: &str = "max_occurs";
const KEY_MEMBER: &str = "member";
const KEY_MEMBERS: &str = "members";
const KEY_METHODS: &str = "methods";
const KEY_MIN_OCCURS: &str = "min_occurs";
const KEY_MODULE: &str = "module";
const KEY_NAME: &str = "name";
const KEY_ORDERING: &str = "ordering";
const KEY_PARENTS: &str = "parents";
const KEY_RANGE: &str = "range";
const KEY_RENAME: &str = "rename";
const KEY_SENTENCE: &str = "sentence";
const KEY_SOURCE_FILE: &str = "source_file";
const KEY_SOURCE_SPAN: &str = "source_span";
const KEY_START: &str = "start";
const KEY_TYPE: &str = "type";
const KEY_TYPE_REF: &str = "type_ref";
const KEY_UNIQUENESS: &str = "uniqueness";
const KEY_VALUE: &str = "value";
const KEY_VARIABLES: &str = "variables";
const KEY_VARIANTS: &str = "variants";
const KEY_VERSION_INFO: &str = "version_info";
const KEY_VERSION_URI: &str = "version_uri";

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

    // definitions
    if body.has_definitions() {
        let mut definitions: Vec<Value> = Vec::default();

        for definition in body.definitions() {
            match definition {
                Definition::Datatype(v) => definitions.push(datatype_to_value(v)),
                Definition::Dimension(v) => definitions.push(dimension_to_value(v)),
                Definition::Entity(v) => definitions.push(entity_to_value(v)),
                Definition::Enum(v) => definitions.push(enum_to_value(v)),
                Definition::Event(v) => definitions.push(event_to_value(v)),
                Definition::Property(v) => definitions.push(property_to_value(v)),
                Definition::Rdf(v) => definitions.push(rdf_to_value(v)),
                Definition::Structure(v) => definitions.push(structure_to_value(v)),
                Definition::TypeClass(v) => definitions.push(type_class_to_value(v)),
                Definition::Union(v) => definitions.push(union_to_value(v)),
            }
        }

        value.insert(KEY_DEFINITIONS.into(), definitions.into());
    }

    // annotations
    add_annotations(body, value);
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
                Annotation::Property(v) => annotations.push(annotation_property_to_value(v)),
                Annotation::Constraint(v) => annotations.push(annotation_constraint_to_value(v)),
            }
        }

        value.insert(KEY_ANNOTATIONS.into(), annotations.into());
    }
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

    value_map.insert(KEY_MEMBERS.into(), members.into());
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
        cardinality_map.insert(KEY_ORDERING.into(), ordering.to_string().into());
    }
    if let Some(uniqueness) = cardinality.uniqueness() {
        cardinality_map.insert(KEY_UNIQUENESS.into(), uniqueness.to_string().into());
    }
    if let Some(range) = cardinality.range() {
        cardinality_map.insert(KEY_MIN_OCCURS.into(), range.min_occurs().into());
        if let Some(max_occurs) = range.max_occurs() {
            cardinality_map.insert(KEY_MAX_OCCURS.into(), max_occurs.into());
        }
    }
    value.insert(KEY_CARDINALITY.into(), cardinality_map.into());

    let mut type_map = Map::default();
    let target_type = defn.target_type();
    type_map.insert(KEY_IS_OPTIONAL.into(), target_type.is_optional().into());
    match target_type.inner() {
        FunctionTypeReferenceInner::Wildcard => {
            type_map.insert(KEY_META_TYPE.into(), VAL_MT_WILDCARD.into());
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

fn parent_to_value(defn: &DimensionParent) -> Value {
    let mut defn_map = Map::default();

    add_source_span(defn, &mut defn_map);
    defn_map.insert(KEY_NAME.into(), defn.name().to_string().into());
    defn_map.insert(KEY_ENTITY.into(), defn.target_entity().to_string().into());

    if let Some(body) = defn.body() {
        add_annotations(body, &mut defn_map);
    }
    defn_map.into()
}

fn source_entity_to_value(defn: &SourceEntity) -> Value {
    let mut defn_map = Map::default();

    add_source_span(defn, &mut defn_map);
    defn_map.insert(KEY_META_TYPE.into(), VAL_MT_SOURCE_ENTITY.into());
    defn_map.insert(KEY_ENTITY.into(), defn.target_entity().to_string().into());

    let members: Vec<Value> = defn.members().map(|id| id.to_string().into()).collect();
    defn_map.insert(KEY_MEMBERS.into(), members.into());

    defn_map.into()
}

fn add_member_def(defn: &MemberDef, defn_map: &mut Map<String, Value>) {
    defn_map.insert(KEY_NAME.into(), defn.name().to_string().into());
    let cardinality = defn.target_cardinality();
    let mut cardinality_map = Map::default();
    if let Some(ordering) = cardinality.ordering() {
        cardinality_map.insert(KEY_ORDERING.into(), ordering.to_string().into());
    }
    if let Some(uniqueness) = cardinality.uniqueness() {
        cardinality_map.insert(KEY_UNIQUENESS.into(), uniqueness.to_string().into());
    }
    cardinality_map.insert(KEY_MIN_OCCURS.into(), cardinality.min_occurs().into());
    if let Some(max_occurs) = cardinality.max_occurs() {
        cardinality_map.insert(KEY_MAX_OCCURS.into(), max_occurs.into());
    }
    defn_map.insert(KEY_CARDINALITY.into(), cardinality_map.into());
    defn_map.insert(KEY_TYPE_REF.into(), defn.target_type().to_string().into());
    if let Some(body) = defn.body() {
        add_annotations(body, defn_map);
    }
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
