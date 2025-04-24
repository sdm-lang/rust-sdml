/*!
This module provides SDML to JSON Value functions for the entire model.

# Model Representation in JSON

# Template Context

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

use sdml_core::{
    config::is_library_module,
    model::{
        annotations::{Annotation, AnnotationOnlyBody, AnnotationProperty, HasAnnotations},
        constraints::{
            AtomicSentence, BinaryBooleanSentence, BooleanSentence, ConnectiveOperator, Constraint,
            ConstraintBody, ConstraintSentence, ControlledLanguageString, Equation,
            FormalConstraint, FunctionBody, FunctionCardinality, FunctionComposition, FunctionDef,
            FunctionParameter, FunctionSignature, FunctionType, FunctionTypeReference,
            FunctionalTerm, InequalityRelation, Inequation, PredicateSequenceMember,
            PredicateValue, QuantifiedSentence, QuantifiedVariable, QuantifiedVariableBinding,
            SequenceBuilder, SequenceOfPredicateValues, SimpleSentence, Subject, Term,
            UnaryBooleanSentence, Variable,
        },
        definitions::{
            DatatypeDef, Definition, DimensionBody, DimensionDef, DimensionIdentity,
            DimensionParent, EntityBody, EntityDef, EnumBody, EnumDef, EventBody, EventDef,
            FromDefinition, HasOptionalFromDefinition, MethodDef, PropertyDef, RdfDef,
            RestrictionFacet, SourceEntity, StructureBody, StructureDef, TypeClassArgument,
            TypeClassBody, TypeClassDef, TypeClassReference, TypeVariable, TypeVariant, UnionBody,
            UnionDef, ValueVariant,
        },
        identifiers::{Identifier, IdentifierReference, QualifiedIdentifier},
        members::{Cardinality, MappingType, Member, MemberDef, MemberKind, TypeReference},
        modules::{Import, ImportStatement, MemberImport, Module, ModuleImport, ModulePath},
        values::{
            MappingValue, SequenceMember, SequenceOfValues, SimpleValue, Value as SdmlValue,
            ValueConstructor,
        },
        HasBody, HasName, HasNameReference, HasOptionalBody, HasSourceSpan, Span,
    },
    store::ModuleStore,
    syntax::*,
};
use sdml_errors::Error;
use serde_json::{Map, Number, Value};
use std::io::{Cursor, Write};

// ------------------------------------------------------------------------------------------------
// Macros -- only here for the ToJson definition
// ------------------------------------------------------------------------------------------------

macro_rules! new_map {
    () => {{
        let map: Map<String, Value> = Map::default();
        map
    }};
    ($capacity:expr) => {{
        let map: Map<String, Value> = Map::with_capacity($capacity);
        map
    }};
}

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ValueOptions {
    context_only: bool,
    include_spans: bool,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct WriteOptions {
    values: ValueOptions,
    pretty_print: bool,
}

pub trait ToJson {
    fn to_json_with(&self, opts: ValueOptions) -> Value {
        let mut value_map = new_map!();
        self.add_to_json_with(&mut value_map, opts);
        value_map.into()
    }
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions);
}

// ------------------------------------------------------------------------------------------------
// Macros
// ------------------------------------------------------------------------------------------------

macro_rules! set {
    ($obj:expr, $opts:expr, name => $named:expr) => {
        set!($obj, FIELD_NAME_NAME => $named.name().to_json_with($opts));
    };
    ($obj:expr, $opts:expr, nameref => $named:expr) => {
        set!($obj, FIELD_NAME_NAME => $named.name_reference().to_json_with($opts));
    };
    ($obj:expr, $key:expr => $value:expr) => {
        $obj.insert($key.into(), Value::from($value));
    };
    (unless $test:expr ; $obj:expr => $meta_type:expr) => {
        if !$test {
            set!($obj, KEY_META_TYPE => $meta_type);
        }
    };
    ($obj:expr => $meta_type:expr) => {
        set!($obj, KEY_META_TYPE => $meta_type);
    };
}

macro_rules! set_source_span {
    ($spanned:expr, $obj:expr, $opts:expr, $if:expr) => {
        if $if {
            set_source_span!($spanned, $obj, $opts);
        }
    };
    ($spanned:expr, $obj:expr, $opts:expr) => {
        if $opts.include_spans {
            if let Some(source_span) = $spanned.source_span() {
                set!($obj, FIELD_NAME_SPAN => source_span.to_json_with($opts));
            }
        }
    };
}

macro_rules! set_variant {
    ($obj:expr, $opts:expr => $enum_type:expr) => {
        if !$opts.context_only {
            set!($obj => $enum_type);
        }
    };
    ($obj:expr, $opts:expr => $variant:expr, $var_type:expr) => {
        if $opts.context_only {
            $variant.add_to_json_with($obj, $opts);
        } else {
            let mut inner_map = new_map!();
            $variant.add_to_json_with(&mut inner_map, $opts);
            set!($obj, $var_type => inner_map);
        }
    };
}

macro_rules! add_enum {
    ($me:expr, $obj:expr, $opts:expr => $enum_type:expr $( ; $var_name:ident => $var_type:expr )+) => {
        set_variant!($obj, $opts => $enum_type);
        match $me {
            $(
            Self::$var_name(v) => set_variant!($obj, $opts => v, $var_type),
            )+
        }
    };
    ($me:expr, $obj:expr, $opts:expr => $enum_type:expr
     $( ; $var_name:ident => $var_type:expr )+ ; ! $bool_var_name:ident => $bool_var_type:expr) => {
        set_variant!($obj, $opts => $enum_type);
        match $me {
            $(
            Self::$var_name(v) => set_variant!($obj, $opts => v, $var_type),
            )+
            Self::$bool_var_name => {
                set!($obj, $bool_var_type => true);
            }
        }
    };
}

macro_rules! add_body {
    ($obj:expr, $opts:expr, $thing:expr) => {
        if let Some(body) = $thing.body() {
            if $opts.context_only {
                body.add_to_json_with($obj, $opts);
            } else {
                set!($obj, FIELD_NAME_BODY => body.to_json_with($opts));
            }
        }
    };
}

macro_rules! add_collection {
    ($obj:expr, $opts:expr, $thing:expr, $has_things:ident, $things:ident, $name:expr) => {
        if $thing.$has_things() {
            let $things = $thing
                .$things()
                .map(|thing| thing.to_json_with($opts))
                .collect::<Vec<Value>>();
            set!($obj, $name => $things);
        }
    };
}

macro_rules! add_annotations {
    ($obj:expr, $opts:expr, $annotated:expr) => {
        add_collection!(
            $obj,
            $opts,
            $annotated,
            has_annotations,
            annotations,
            FIELD_NAME_ANNOTATIONS
        );
    };
}

macro_rules! add_from_definition {
    ($me:expr, $value_map:expr, $opts:expr) => {
        if let Some(from) = $me.from_definition() {
            set!($value_map, FIELD_NAME_FROM => from.to_json_with($opts));
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Public Names
// ------------------------------------------------------------------------------------------------

const KEY_META_TYPE: &str = "__type";

const JSON_FIELD_NAME_DEFINITIONS: &str = "definitions";

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

#[inline]
pub fn write_module<W>(w: &mut W, module: &Module, cache: &impl ModuleStore) -> Result<(), Error>
where
    W: Write,
{
    write_module_with_options(w, module, cache, Default::default())
}

pub fn write_module_with_options<W>(
    w: &mut W,
    module: &Module,
    _: &impl ModuleStore,
    options: WriteOptions,
) -> Result<(), Error>
where
    W: Write,
{
    let value = module_to_value(module, options.values);
    if options.pretty_print {
        Ok(serde_json::to_writer_pretty(w, &value).map_err(into_generator_error)?)
    } else {
        Ok(serde_json::to_writer(w, &value).map_err(into_generator_error)?)
    }
}

#[inline]
pub fn write_module_to_string(module: &Module, cache: &impl ModuleStore) -> Result<String, Error> {
    write_module_with_options_to_string(module, cache, Default::default())
}

pub fn write_module_with_options_to_string(
    module: &Module,
    cache: &impl ModuleStore,
    options: WriteOptions,
) -> Result<String, Error> {
    let mut buffer = Cursor::new(Vec::new());
    write_module_with_options(&mut buffer, module, cache, options)?;
    Ok(String::from_utf8(buffer.into_inner())?)
}

///
/// ```json
/// {
///   "identifier-1": {
///     "__type": "module",
///     ...
///   },
///   "identifier-2": {
///     "__type": "module",
///     ...
///   }
/// }
/// ```
pub fn module_list_to_value(modules: &[&Module], opts: ValueOptions) -> Value {
    println!("context::module_list_to_value(modules: [...], opts: {opts:?})",);
    let values: Map<String, Value> = modules
        .iter()
        .map(|module| module_to_value_and_name(module, opts))
        .collect();
    values.into()
}

///
/// Convert a SDML `Module` into a JSON Value
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
pub fn module_to_value(module: &Module, opts: ValueOptions) -> Value {
    module.to_json_with(opts)
}

fn module_to_value_and_name(module: &Module, opts: ValueOptions) -> (String, Value) {
    (module.name().to_string(), module.to_json_with(opts))
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl ValueOptions {
    pub fn for_context() -> Self {
        Self {
            context_only: true,
            include_spans: false,
        }
    }

    pub fn for_model() -> Self {
        Self {
            context_only: false,
            include_spans: true,
        }
    }

    pub fn emit_context_only(self, context_only: bool) -> Self {
        let mut self_mut = self;
        self_mut.context_only = context_only;
        self_mut
    }

    pub fn with_spans_included(self, include_spans: bool) -> Self {
        let mut self_mut = self;
        self_mut.include_spans = include_spans;
        self_mut
    }
}

// ------------------------------------------------------------------------------------------------

impl WriteOptions {
    pub fn for_context() -> Self {
        Self {
            values: ValueOptions::for_context(),
            pretty_print: Default::default(),
        }
    }

    pub fn for_model() -> Self {
        Self {
            values: ValueOptions::for_model(),
            pretty_print: Default::default(),
        }
    }

    pub fn emit_context_only(self, context_only: bool) -> Self {
        let mut self_mut = self;
        self_mut.values.context_only = context_only;
        self_mut
    }

    pub fn with_pretty_printing(self, pretty_print: bool) -> Self {
        let mut self_mut = self;
        self_mut.pretty_print = pretty_print;
        self_mut
    }

    pub fn with_spans_included(self, include_spans: bool) -> Self {
        let mut self_mut = self;
        self_mut.values.include_spans = include_spans;
        self_mut
    }
}

// ------------------------------------------------------------------------------------------------

impl ToJson for Module {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        set!(value_map => NODE_KIND_MODULE);
        set_source_span!(self, value_map, opts);
        set!(value_map, opts, name => self);
        set!(value_map, FIELD_NAME_IS_LIBRARY_MODULE => is_library_module(self.name()));
        if let Some(source_file) = self.source_file() {
            set!(
                value_map,
                FIELD_NAME_SOURCE_FILE =>
                    source_file.to_string_lossy().into_owned()
            );
        }
        if let Some(base_uri) = self.base_uri() {
            set!(value_map, FIELD_NAME_BASE => base_uri.to_string());
        }
        if let Some(version_info) = self.version_info() {
            set!(value_map, FIELD_NAME_VERSION_INFO => version_info.to_string());
        }
        if let Some(version_uri) = self.version_uri() {
            set!(value_map, FIELD_NAME_VERSION_URI => version_uri.to_string());
        }
        if opts.context_only && self.has_imports() {
            let import_array: Vec<Value> = self
                .imports()
                .map(|stmt| {
                    let from_path = stmt.from_module_path();
                    stmt.imports()
                        .map(|im| ContextImport(from_path, im).to_json_with(opts))
                        .collect::<Vec<_>>()
                })
                .flatten()
                .collect();

            set!(value_map, FIELD_NAME_IMPORTS => import_array);
        } else {
            add_collection!(
                value_map,
                opts,
                self,
                has_imports,
                imports,
                FIELD_NAME_IMPORTS
            );
        }
        add_annotations!(value_map, opts, self);
        add_collection!(
            value_map,
            opts,
            self,
            has_definitions,
            definitions,
            JSON_FIELD_NAME_DEFINITIONS
        );
    }
}

impl ToJson for Span {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        set!(unless opts.context_only; value_map => NODE_KIND_SPAN);
        set!(value_map, FIELD_NAME_START => Value::Number(self.start().into()));
        set!(value_map, FIELD_NAME_END => Value::Number(self.end().into()));
    }
}

impl ToJson for ImportStatement {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        set!(value_map => NODE_KIND_IMPORT_STATEMENT);
        if let Some(from_path) = self.from_module_path() {
            set!(value_map, KW_IMPORT_FROM => Value::String(from_path.to_string()));
        }
        add_collection!(
            value_map,
            opts,
            self,
            has_imports,
            imports,
            FIELD_NAME_IMPORTS
        );
    }
}

#[derive(Debug)]
struct ContextImport<'a>(Option<&'a ModulePath>, &'a Import);

impl ToJson for ContextImport<'_> {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        set_variant!(value_map, opts => NODE_KIND_IMPORT);

        if let Some(from_path) = self.0 {
            set!(value_map, KW_IMPORT_FROM => Value::String(from_path.to_string()));
        }

        match self.1 {
            Import::Module(v) => set_variant!(value_map, opts => v, NODE_VARIANT_MODULE),
            Import::Member(v) => {
                set_variant!(value_map, opts => v, NODE_VARIANT_MEMBER)
            }
        }
    }
}

impl ToJson for Import {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        set_variant!(value_map, opts => NODE_KIND_IMPORT);

        match self {
            Import::Module(v) => set_variant!(value_map, opts => v, NODE_VARIANT_MODULE),
            Import::Member(v) => {
                set_variant!(value_map, opts => v, NODE_VARIANT_MEMBER)
            }
        }
    }
}

impl ToJson for MemberImport {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        set!(unless opts.context_only; value_map => NODE_KIND_MEMBER_IMPORT);
        set!(value_map, FIELD_NAME_MODULE => self.name().module().to_json_with(opts));
        set!(value_map, FIELD_NAME_MEMBER => self.name().member().to_json_with(opts));
        if let Some(renamed_as) = self.renamed_as() {
            set!(value_map, FIELD_NAME_RENAME => renamed_as.to_string());
        }
    }
}

impl ToJson for ModuleImport {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        set!(unless opts.context_only; value_map => NODE_KIND_MODULE_IMPORT);
        set!(value_map, FIELD_NAME_MODULE => self.name().to_json_with(opts));
        if let Some(version) = self.version_uri() {
            set!(value_map, FIELD_NAME_VERSION_URI => version.to_string());
        }
        if let Some(renamed_as) = self.renamed_as() {
            set!(value_map, FIELD_NAME_RENAME => renamed_as.to_json_with(opts));
        }
    }
}

impl ToJson for Definition {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        add_enum!(self,
            value_map, opts => NODE_KIND_DEFINITION
                ; Datatype => NODE_VARIANT_DATATYPE
                ; Dimension => NODE_VARIANT_DIMENSION
                ; Entity => NODE_VARIANT_ENTITY
                ; Enum => NODE_VARIANT_ENUM
                ; Event => NODE_VARIANT_EVENT
                ; Property => NODE_VARIANT_PROPERTY
                ; Rdf => NODE_VARIANT_RDF
                ; Structure => NODE_VARIANT_STRUCTURE
                ; TypeClass => NODE_VARIANT_TYPE_CLASS
                ; Union => NODE_VARIANT_UNION
        );
    }
}

impl ToJson for FromDefinition {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        set!(value_map => NODE_KIND_FROM_DEFINITION_CLAUSE);
        set_source_span!(self, value_map, opts);
        set!(value_map, FIELD_NAME_FROM => self.definition().to_json_with(opts));
        add_collection!(value_map, opts, self, has_members, members, FIELD_NAME_WITH);
    }
}

impl ToJson for Annotation {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        add_enum!(self,
            value_map, opts => NODE_KIND_ANNOTATION
                ; Property => NODE_VARIANT_PROPERTY
                ; Constraint => NODE_VARIANT_CONSTRAINT
        );
    }
}

impl ToJson for TypeReference {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        set_variant!(value_map, opts => NODE_KIND_TYPE_REFERENCE);

        match self {
            TypeReference::Unknown => {
                set!(value_map, NODE_VARIANT_UNKNOWN => true);
            }
            TypeReference::Type(v) => {
                set!(value_map, NODE_VARIANT_REFERENCE => v.to_json_with(opts));
            }
            TypeReference::MappingType(v) => {
                set_variant!(value_map, opts => v, NODE_VARIANT_MAPPING)
            }
        }
    }
}

impl ToJson for MappingType {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        set!(value_map => NODE_KIND_MAPPING_TYPE);
        set_source_span!(self, value_map, opts);
        set!(value_map, FIELD_NAME_DOMAIN => self.domain().to_json_with(opts));
        set!(value_map, FIELD_NAME_RANGE => self.range().to_json_with(opts));
    }
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
impl ToJson for AnnotationProperty {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        set!(value_map => NODE_KIND_ANNOTATION_PROPERTY);
        set_source_span!(self, value_map, opts);
        set!(value_map, opts, nameref => self);
        set!(value_map, FIELD_NAME_VALUE => self.value().to_json_with(opts));
    }
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
impl ToJson for SdmlValue {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        add_enum!(self,
            value_map, opts => NODE_KIND_VALUE
                ; Simple => NODE_VARIANT_SIMPLE
                ; ValueConstructor => NODE_VARIANT_VALUE_CONSTRUCTOR
                ; Mapping => NODE_VARIANT_MAPPING
                ; Reference => NODE_VARIANT_REFERENCE
                ; Sequence => NODE_VARIANT_SEQUENCE
        );
    }
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
impl ToJson for Constraint {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        set_variant!(value_map, opts => NODE_KIND_CONSTRAINT);
        set_source_span!(self, value_map, opts);
        set!(value_map, opts, name => self);
        if opts.context_only {
            self.body().add_to_json_with(value_map, opts);
        } else {
            set!(value_map, FIELD_NAME_BODY => self.body().to_json_with(opts));
        }
    }
}

impl ToJson for ConstraintBody {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        add_enum!(self,
            value_map, opts => NODE_KIND_CONSTRAINT_BODY
                ; Formal => NODE_VARIANT_FORMAL
                ; Informal => NODE_VARIANT_INFORMAL
        );
    }
}

impl ToJson for ControlledLanguageString {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, _: ValueOptions) {
        set!(value_map => NODE_KIND_INFORMAL_CONSTRAINT);
        set!(value_map, FIELD_NAME_VALUE => self.value().to_string());
        if let Some(language) = self.language() {
            set!(value_map, FIELD_NAME_LANGUAGE => language.value().to_string());
        }
    }
}

impl ToJson for FormalConstraint {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        set!(value_map => NODE_KIND_FORMAL_CONSTRAINT);
        add_collection!(
            value_map,
            opts,
            self,
            has_definitions,
            definitions,
            FIELD_NAME_ENVIRONMENT
        );
        set!(value_map, FIELD_NAME_BODY => self.body().to_json_with(opts));
    }
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
impl ToJson for DatatypeDef {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        set!(value_map => NODE_KIND_DATA_TYPE_DEF);
        set_source_span!(self, value_map, opts);
        set!(value_map, opts, name => self);
        set!(value_map, FIELD_NAME_IS_OPAQUE => self.is_opaque());
        set!(value_map, FIELD_NAME_BASE => self.base_type().to_string());
        if self.has_restrictions() {
            let mut inner_map: Map<String, Value> = new_map!(self.restriction_count() + 1);
            set!(&mut inner_map => NODE_KIND_DATATYPE_DEF_RESTRICTION);
            for restriction in self.restrictions() {
                match restriction {
                    RestrictionFacet::FractionDigits(v, f) => {
                        let mut restriction_map = new_map!(2);
                        set!(restriction_map, FIELD_NAME_VALUE => *v);
                        set!(restriction_map, FIELD_NAME_IS_FIXED => *f);
                        set!(inner_map, KW_FACET_FRACTION_DIGITS => restriction_map);
                    }
                    RestrictionFacet::TotalDigits(v, f) => {
                        let mut restriction_map = new_map!(2);
                        set!(restriction_map, FIELD_NAME_VALUE => *v);
                        set!(restriction_map, FIELD_NAME_IS_FIXED => *f);
                        set!(inner_map, KW_FACET_TOTAL_DIGITS => restriction_map);
                    }
                    RestrictionFacet::Length(v, f) => {
                        let mut restriction_map = new_map!(2);
                        set!(restriction_map, FIELD_NAME_VALUE => *v);
                        set!(restriction_map, FIELD_NAME_IS_FIXED => *f);
                        set!(inner_map, KW_FACET_LENGTH => restriction_map);
                    }
                    RestrictionFacet::MaxLength(v, f) => {
                        let mut restriction_map = new_map!(2);
                        set!(restriction_map, FIELD_NAME_VALUE => *v);
                        set!(restriction_map, FIELD_NAME_IS_FIXED => *f);
                        set!(inner_map, KW_FACET_MAX_LENGTH => restriction_map);
                    }
                    RestrictionFacet::MinLength(v, f) => {
                        let mut restriction_map = new_map!(2);
                        set!(restriction_map, FIELD_NAME_VALUE => *v);
                        set!(restriction_map, FIELD_NAME_IS_FIXED => *f);
                        set!(inner_map, KW_FACET_MIN_LENGTH => restriction_map);
                    }
                    RestrictionFacet::MaxExclusive(v, f) => {
                        let mut restriction_map = new_map!(2);
                        set!(restriction_map, FIELD_NAME_VALUE => *v);
                        set!(restriction_map, FIELD_NAME_IS_FIXED => *f);
                        set!(inner_map, KW_FACET_MAX_EXCLUSIVE => restriction_map);
                    }
                    RestrictionFacet::MinExclusive(v, f) => {
                        let mut restriction_map = new_map!(2);
                        set!(restriction_map, FIELD_NAME_VALUE => *v);
                        set!(restriction_map, FIELD_NAME_IS_FIXED => *f);
                        set!(inner_map, KW_FACET_MIN_EXCLUSIVE => restriction_map);
                    }
                    RestrictionFacet::MaxInclusive(v, f) => {
                        let mut restriction_map = new_map!(2);
                        set!(restriction_map, FIELD_NAME_VALUE => *v);
                        set!(restriction_map, FIELD_NAME_IS_FIXED => *f);
                        set!(inner_map, KW_FACET_MAX_INCLUSIVE => restriction_map);
                    }
                    RestrictionFacet::MinInclusive(v, f) => {
                        let mut restriction_map = new_map!(2);
                        set!(restriction_map, FIELD_NAME_VALUE => *v);
                        set!(restriction_map, FIELD_NAME_IS_FIXED => *f);
                        set!(inner_map, KW_FACET_MIN_INCLUSIVE => restriction_map);
                    }
                    RestrictionFacet::ExplicitTimezone(v, f) => {
                        let mut restriction_map = new_map!(2);
                        set!(restriction_map, FIELD_NAME_VALUE => v.to_string());
                        set!(restriction_map, FIELD_NAME_IS_FIXED => *f);
                        set!(inner_map, KW_FACET_EXPLICIT_TIMEZONE => restriction_map);
                    }
                    RestrictionFacet::Pattern(vs) => {
                        let mut restriction_map = new_map!(1);
                        let value_array: Vec<serde_json::Value> =
                            vs.iter().map(|s| s.to_string().into()).collect();
                        set!(restriction_map, FIELD_NAME_VALUE => value_array);
                        set!(inner_map, KW_FACET_PATTERN => restriction_map);
                    }
                }
            }
            set!(value_map, FIELD_NAME_RESTRICTION => inner_map);
        }
        add_body!(value_map, opts, self);
    }
}

impl ToJson for AnnotationOnlyBody {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        if self.has_annotations() {
            let annotations = self
                .annotations()
                .map(|ann| ann.to_json_with(opts))
                .collect::<Vec<Value>>();
            set!(unless opts.context_only; value_map => NODE_KIND_ANNOTATION_ONLY_BODY);
            set!(value_map, FIELD_NAME_ANNOTATIONS => annotations);
        }
    }
}

impl ToJson for Identifier {
    fn to_json_with(&self, opts: ValueOptions) -> Value {
        if opts.context_only {
            self.to_string().into()
        } else {
            let mut map = new_map!(2);
            self.add_to_json_with(&mut map, opts);
            map.into()
        }
    }
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, _: ValueOptions) {
        set!(value_map => NODE_KIND_IDENTIFIER);
        set!(value_map, FIELD_NAME_VALUE => self.to_string());
    }
}

impl ToJson for QualifiedIdentifier {
    fn to_json_with(&self, opts: ValueOptions) -> Value {
        if opts.context_only {
            self.to_string().into()
        } else {
            let mut map = new_map!(3);
            self.add_to_json_with(&mut map, opts);
            map.into()
        }
    }
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        set!(value_map => NODE_KIND_QUALIFIED_IDENTIFIER);
        set!(value_map, FIELD_NAME_MODULE => self.module().to_json_with(opts));
        set!(value_map, FIELD_NAME_MEMBER => self.member().to_json_with(opts));
    }
}

impl ToJson for IdentifierReference {
    fn to_json_with(&self, opts: ValueOptions) -> Value {
        if opts.context_only {
            self.to_string().into()
        } else {
            let mut map = new_map!();
            self.add_to_json_with(&mut map, opts);
            map.into()
        }
    }
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        add_enum!(self,
            value_map, opts => NODE_KIND_IDENTIFIER_REFERENCE
                ; Identifier => NODE_VARIANT_IDENTIFIER
                ; QualifiedIdentifier => NODE_VARIANT_QUALIFIED_IDENTIFIER
        );
    }
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
impl ToJson for EntityDef {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        set!(value_map => NODE_KIND_ENTITY_DEF);
        set_source_span!(self, value_map, opts);
        set!(value_map, opts, name => self);
        add_body!(value_map, opts, self)
    }
}

impl ToJson for EntityBody {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        set!(unless opts.context_only; value_map => NODE_KIND_ENTITY_BODY);
        set!(value_map, FIELD_NAME_IDENTITY => self.identity().to_json_with(opts));
        add_annotations!(value_map, opts, self);
        add_from_definition!(self, value_map, opts);
        add_collection!(
            value_map,
            opts,
            self,
            has_members,
            members,
            FIELD_NAME_MEMBERS
        );
    }
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
impl ToJson for Member {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        self.kind().add_to_json_with(value_map, opts)
    }
}

impl ToJson for MemberKind {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        add_enum!(self,
            value_map, opts => NODE_KIND_MEMBER
                ; Reference => NODE_VARIANT_REFERENCE
                ; Definition => NODE_VARIANT_DEFINITION
        );
    }
}

impl ToJson for MemberDef {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        set!(value_map => NODE_KIND_MEMBER_DEF);
        set_source_span!(self, value_map, opts);
        set!(value_map, opts, name => self);
        if !self.target_cardinality().is_default() {
            set!(value_map, FIELD_NAME_CARDINALITY => self.target_cardinality().to_json_with(opts));
        }
        set!(value_map, FIELD_NAME_TYPE => self.target_type().to_json_with(opts));
        add_body!(value_map, opts, self);
    }
}

impl ToJson for Cardinality {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        set!(unless opts.context_only; value_map => NODE_KIND_CARDINALITY_EXPRESSION);
        if let Some(ordering) = self.ordering() {
            set!(value_map, FIELD_NAME_ORDERING => ordering.to_string());
        }
        if let Some(uniqueness) = self.uniqueness() {
            set!(value_map, FIELD_NAME_UNIQUENESS => uniqueness.to_string());
        }
        let range = self.range();
        set!(value_map, FIELD_NAME_MIN => range.min_occurs());
        if let Some(max_occurs) = range.max_occurs() {
            set!(value_map, FIELD_NAME_MAX => max_occurs);
        }
    }
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
impl ToJson for EnumDef {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        set!(value_map => NODE_KIND_ENUM_DEF);
        set_source_span!(self, value_map, opts);
        set!(value_map, opts, name => self);
        add_body!(value_map, opts, self);
    }
}

impl ToJson for EnumBody {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        set!(unless opts.context_only; value_map => NODE_KIND_ENUM_BODY);
        add_annotations!(value_map, opts, self);
        add_from_definition!(self, value_map, opts);
        add_collection!(
            value_map,
            opts,
            self,
            has_variants,
            variants,
            FIELD_NAME_VARIANTS
        );
    }
}

impl ToJson for ValueVariant {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        set!(unless opts.context_only; value_map => NODE_KIND_VALUE_VARIANT);
        set_source_span!(self, value_map, opts);
        set!(value_map, opts, name => self);
        add_body!(value_map, opts, self);
    }
}

///
/// Convert a SDML `EventDef` into a context object, in the form shown as JSON below.
///
/// ```json
/// {
///     "__type": "event_def",
///     "source_span": {},              // optional
///     "name": "Identifier",
///     "source_entity": {},             // SourceEntity
///     "members": [],                  // optional
///     "annotations": []               // optional
/// }
/// ```
///
/// For `members`, see [`member_to_value`], and for `event_source` see [`dimension_to_value`].
///
impl ToJson for EventDef {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        set!(value_map => NODE_KIND_EVENT_DEF);
        set_source_span!(self, value_map, opts);
        set!(value_map, opts, name => self);
        add_body!(value_map, opts, self);
    }
}

impl ToJson for EventBody {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        set!(unless opts.context_only; value_map => NODE_KIND_EVENT_BODY);
        add_annotations!(value_map, opts, self);
        add_from_definition!(self, value_map, opts);
        set!(value_map, FIELD_NAME_SOURCE => self.source_entity().to_json_with(opts));
        add_collection!(
            value_map,
            opts,
            self,
            has_members,
            members,
            FIELD_NAME_MEMBERS
        );
    }
}

impl ToJson for SourceEntity {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        set!(value_map => NODE_KIND_SOURCE_ENTITY);
        set_source_span!(self, value_map, opts);
        set!(value_map, FIELD_NAME_ENTITY => self.target_entity().to_json_with(opts));
        add_collection!(
            value_map,
            opts,
            self,
            has_members,
            members,
            FIELD_NAME_MEMBERS
        );
    }
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
impl ToJson for DimensionDef {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        set!(value_map => NODE_KIND_DIMENSION_DEF);
        set_source_span!(self, value_map, opts);
        set!(value_map, opts, name => self);
        add_body!(value_map, opts, self);
    }
}

impl ToJson for DimensionBody {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        set!(unless opts.context_only; value_map => NODE_KIND_DIMENSION_BODY);
        add_annotations!(value_map, opts, self);
        set!(value_map, FIELD_NAME_IDENTITY => self.identity().to_json_with(opts));
        add_collection!(
            value_map,
            opts,
            self,
            has_parents,
            parents,
            FIELD_NAME_PARENTS
        );
        add_from_definition!(self, value_map, opts);
        add_collection!(
            value_map,
            opts,
            self,
            has_members,
            members,
            FIELD_NAME_MEMBERS
        );
    }
}

impl ToJson for DimensionIdentity {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        add_enum!(self,
            value_map, opts => NODE_KIND_DIMENSION_IDENTITY
                ; Source => FIELD_NAME_SOURCE
                ; Identity => FIELD_NAME_IDENTITY
        );
    }
}

impl ToJson for DimensionParent {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        set!(unless opts.context_only; value_map => NODE_KIND_DIMENSION_PARENT);
        set_source_span!(self, value_map, opts);
        set!(value_map, opts, name => self);
        set!(value_map, FIELD_NAME_ENTITY => self.target_entity().to_json_with(opts));
        add_body!(value_map, opts, self);
    }
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
impl ToJson for PropertyDef {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        set!(value_map => NODE_KIND_PROPERTY_DEF);
        set_source_span!(self, value_map, opts);
        set!(value_map, FIELD_NAME_MEMBER => self.member_def().to_json_with(opts));
    }
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
impl ToJson for RdfDef {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        set!(value_map => NODE_KIND_RDF_DEF);
        set_source_span!(self, value_map, opts);
        set!(value_map, opts, name => self);
        add_body!(value_map, opts, self);
    }
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
impl ToJson for StructureDef {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        set!(value_map => NODE_KIND_STRUCTURE_DEF);
        set_source_span!(self, value_map, opts);
        set!(value_map, opts, name => self);
        add_body!(value_map, opts, self);
    }
}

impl ToJson for StructureBody {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        set!(unless opts.context_only; value_map => NODE_KIND_STRUCTURE_BODY);
        add_annotations!(value_map, opts, self);
        add_from_definition!(self, value_map, opts);
        add_collection!(
            value_map,
            opts,
            self,
            has_members,
            members,
            FIELD_NAME_MEMBERS
        );
    }
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
impl ToJson for TypeClassDef {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        set!(value_map => NODE_KIND_TYPE_CLASS_DEF);
        set_source_span!(self, value_map, opts);
        set!(value_map, opts, name => self);
        add_collection!(
            value_map,
            opts,
            self,
            has_variables,
            variables,
            FIELD_NAME_VARIABLES
        );
        add_body!(value_map, opts, self);
    }
}

impl ToJson for TypeClassBody {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        set!(unless opts.context_only; value_map => NODE_KIND_TYPE_CLASS_BODY);
        set_source_span!(self, value_map, opts, !opts.context_only);
        add_annotations!(value_map, opts, self);
        add_from_definition!(self, value_map, opts);
        add_collection!(
            value_map,
            opts,
            self,
            has_methods,
            methods,
            FIELD_NAME_METHODS
        );
    }
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
impl ToJson for UnionDef {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        set!(value_map => NODE_KIND_UNION_DEF);
        set_source_span!(self, value_map, opts);
        set!(value_map, opts, name => self);
        add_body!(value_map, opts, self);
    }
}

impl ToJson for UnionBody {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        set!(unless opts.context_only; value_map => NODE_KIND_UNION_BODY);
        set_source_span!(self, value_map, opts, !opts.context_only);
        add_annotations!(value_map, opts, self);
        add_collection!(
            value_map,
            opts,
            self,
            has_variants,
            variants,
            FIELD_NAME_VARIANTS
        );
    }
}

impl ToJson for TypeVariant {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        set!(unless opts.context_only; value_map => NODE_KIND_TYPE_VARIANT);
        set_source_span!(self, value_map, opts);
        set!(value_map, opts, nameref => self);
        if let Some(rename) = self.rename() {
            set!(value_map, FIELD_NAME_RENAME => rename.to_json_with(opts));
        }
        add_body!(value_map, opts, self);
    }
}

impl ToJson for SimpleValue {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        set_variant!(value_map, opts => NODE_KIND_SIMPLE_VALUE);

        match self {
            SimpleValue::Boolean(v) => {
                set!(value_map => NODE_KIND_BOOLEAN);
                set!(value_map, FIELD_NAME_VALUE => *v);
            }
            SimpleValue::Double(v) => {
                set!(value_map => NODE_KIND_DOUBLE);
                set!(value_map, FIELD_NAME_VALUE => *v.as_ref());
            }
            SimpleValue::Decimal(v) => {
                set!(value_map => NODE_KIND_DECIMAL);
                set!(value_map, FIELD_NAME_VALUE => v.to_string());
            }
            SimpleValue::Integer(v) => {
                set!(value_map => NODE_KIND_INTEGER);
                set!(value_map, FIELD_NAME_VALUE => *v);
            }
            SimpleValue::Unsigned(v) => {
                set!(value_map => NODE_KIND_UNSIGNED);
                set!(value_map, FIELD_NAME_VALUE => *v);
            }
            SimpleValue::String(v) => {
                set!(value_map => NODE_KIND_STRING);
                set!(value_map, FIELD_NAME_VALUE => v.value().to_string());
                if let Some(language) = v.language() {
                    set!(value_map, FIELD_NAME_LANGUAGE => language.inner().to_string());
                }
            }
            SimpleValue::IriReference(v) => {
                set!(value_map => NODE_KIND_IRI);
                set!(value_map, FIELD_NAME_VALUE => v.to_string());
            }
            SimpleValue::Binary(v) => {
                let value: Vec<Value> = v
                    .as_bytes()
                    .iter()
                    .map(|b| Number::from(*b).into())
                    .collect();

                set!(value_map => NODE_KIND_BINARY);
                set!(value_map, FIELD_NAME_VALUE => value);
            }
        }
    }
}

impl ToJson for ValueConstructor {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        set!(value_map => NODE_KIND_VALUE_CONSTRUCTOR);
        set_source_span!(self, value_map, opts);
        set!(value_map, FIELD_NAME_TYPE => self.type_name().to_json_with(opts));
        set!(value_map, FIELD_NAME_VALUE => self.value().to_json_with(opts));
    }
}

impl ToJson for MappingValue {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        set!(value_map => NODE_KIND_MAPPING_VALUE);
        set_source_span!(self, value_map, opts);
        set!(value_map, FIELD_NAME_DOMAIN => self.domain().to_json_with(opts));
        set!(value_map, FIELD_NAME_RANGE => self.range().to_json_with(opts));
    }
}

impl ToJson for SequenceOfValues {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        set!(value_map => NODE_KIND_SEQUENCE_OF_VALUES);
        set_source_span!(self, value_map, opts);
        if let Some(ordering) = self.ordering() {
            set!(value_map, FIELD_NAME_ORDERING => ordering.to_string());
        }
        if let Some(uniqueness) = self.uniqueness() {
            set!(value_map, FIELD_NAME_UNIQUENESS => uniqueness.to_string());
        }
        if !self.is_empty() {
            let values = self
                .iter()
                .map(|thing| thing.to_json_with(opts))
                .collect::<Vec<Value>>();
            set!(value_map, FIELD_NAME_MEMBERS => values);
        }
    }
}

impl ToJson for SequenceMember {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        add_enum!(self,
            value_map, opts => NODE_KIND_SEQUENCE_MEMBER
                ; Simple => NODE_VARIANT_SIMPLE
                ; ValueConstructor => NODE_VARIANT_VALUE_CONSTRUCTOR
                ; Mapping => NODE_VARIANT_MAPPING
                ; Reference => NODE_VARIANT_REFERENCE
        );
    }
}

impl ToJson for SequenceOfPredicateValues {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        set!(value_map => NODE_KIND_SEQUENCE_OF_PREDICATE_VALUES);
        set_source_span!(self, value_map, opts);
        if !self.is_empty() {
            let values = self
                .iter()
                .map(|thing| thing.to_json_with(opts))
                .collect::<Vec<Value>>();
            set!(value_map, FIELD_NAME_MEMBERS => values);
        }
    }
}

impl ToJson for PredicateSequenceMember {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        add_enum!(self,
            value_map, opts => NODE_KIND_PREDICATE_SEQUENCE_MEMBER
                ; Simple => NODE_VARIANT_SIMPLE
                ; ValueConstructor => NODE_VARIANT_VALUE_CONSTRUCTOR
                ; Mapping => NODE_VARIANT_MAPPING
                ; Reference => NODE_VARIANT_REFERENCE
        );
    }
}

impl ToJson for PredicateValue {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        add_enum!(self,
            value_map, opts => NODE_KIND_PREDICATE_VALUE
                ; Simple => NODE_VARIANT_SIMPLE
                ; Sequence => NODE_VARIANT_SEQUENCE
        );
    }
}

impl ToJson for FunctionDef {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        set!(value_map => NODE_KIND_FUNCTION_DEF);
        set_source_span!(self, value_map, opts);
        set!(value_map, FIELD_NAME_SIGNATURE => self.signature().to_json_with(opts));
        set!(value_map, FIELD_NAME_BODY => self.body().to_json_with(opts));
    }
}

impl ToJson for FunctionSignature {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        set!(value_map => NODE_KIND_FUNCTION_SIGNATURE);
        set_source_span!(self, value_map, opts);
        add_collection!(
            value_map,
            opts,
            self,
            has_parameters,
            parameters,
            FIELD_NAME_PARAMETERS
        );
        set!(value_map, FIELD_NAME_TYPE => self.target_type().to_json_with(opts));
    }
}

impl ToJson for FunctionParameter {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        set!(value_map => NODE_KIND_FUNCTION_PARAMETER);
        set_source_span!(self, value_map, opts);
        set!(value_map, opts, name => self);
        set!(value_map, FIELD_NAME_TYPE => self.target_type().to_json_with(opts));
    }
}

impl ToJson for FunctionType {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        set!(value_map => NODE_KIND_FUNCTION_TYPE);
        set_source_span!(self, value_map, opts);
        if !self.target_cardinality().is_default() {
            set!(
                value_map,
                FIELD_NAME_CARDINALITY =>
                self.target_cardinality().to_json_with(opts)
            );
        }
        set!(value_map, FIELD_NAME_TYPE => self.target_type().to_json_with(opts));
    }
}

impl ToJson for FunctionCardinality {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        set!(unless opts.context_only; value_map => NODE_KIND_FUNCTION_CARDINALITY_EXPRESSION);
        set_source_span!(self, value_map, opts);
        if let Some(ordering) = self.ordering() {
            set!(value_map, FIELD_NAME_ORDERING => ordering.to_string());
        }
        if let Some(uniqueness) = self.uniqueness() {
            set!(value_map, FIELD_NAME_UNIQUENESS => uniqueness.to_string());
        }
        if let Some(range) = self.range() {
            set!(value_map, FIELD_NAME_MIN => range.min_occurs());
            if let Some(max_occurs) = range.max_occurs() {
                set!(value_map, FIELD_NAME_MAX => max_occurs);
            }
        }
    }
}

impl ToJson for FunctionTypeReference {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        add_enum!(self,
            value_map, opts => NODE_KIND_FUNCTION_TYPE_REFERENCE
            ; Reference => NODE_VARIANT_REFERENCE
            ; MappingType => NODE_VARIANT_MAPPING
            ; ! Wildcard => NODE_VARIANT_WILDCARD
        );
    }
}

impl ToJson for FunctionBody {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        add_enum!(self,
            value_map, opts => NODE_KIND_FUNCTION_BODY
            ; Sentence => NODE_VARIANT_SENTENCE
            ; Term => NODE_VARIANT_TERM
        );
    }
}

impl ToJson for ConstraintSentence {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        add_enum!(self,
            value_map, opts => NODE_KIND_CONSTRAINT_SENTENCE
                ; Simple => NODE_VARIANT_SIMPLE
                ; Boolean => NODE_VARIANT_BOOLEAN
                ; Quantified => NODE_VARIANT_QUANTIFIED
        );
    }
}

impl ToJson for SimpleSentence {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        add_enum!(self,
            value_map, opts => NODE_KIND_SIMPLE_SENTENCE
                ; Atomic => NODE_VARIANT_ATOMIC
                ; Equation => NODE_VARIANT_EQUATION
                ; Inequation => NODE_VARIANT_INEQUATION
        );
    }
}

impl ToJson for AtomicSentence {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        set!(value_map => NODE_KIND_ATOMIC_SENTENCE);
        set_source_span!(self, value_map, opts);
        set!(value_map, FIELD_NAME_PREDICATE => self.predicate().to_json_with(opts));
        add_collection!(
            value_map,
            opts,
            self,
            has_arguments,
            arguments,
            FIELD_NAME_ARGUMENTS
        );
    }
}

impl ToJson for Equation {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        set!(value_map => NODE_KIND_EQUATION);
        set_source_span!(self, value_map, opts);
        set!(value_map, FIELD_NAME_LHS => self.left_operand().to_json_with(opts));
        set!(value_map, FIELD_NAME_RHS => self.right_operand().to_json_with(opts));
    }
}

impl ToJson for Inequation {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        set!(value_map => NODE_KIND_INEQUATION);
        set_source_span!(self, value_map, opts);
        set!(value_map, FIELD_NAME_LHS => self.left_operand().to_json_with(opts));
        set!(value_map, FIELD_NAME_RELATION => match self.relation() {
            InequalityRelation::NotEqual => NODE_KIND_OP_INEQUALITY,
            InequalityRelation::LessThan => NODE_KIND_OP_LESS_THAN,
            InequalityRelation::LessThanOrEqual => NODE_KIND_OP_LESS_THAN_OR_EQUAL,
            InequalityRelation::GreaterThan => NODE_KIND_OP_GREATER_THAN,
            InequalityRelation::GreaterThanOrEqual => NODE_KIND_OP_GREATER_THAN_OR_EQUAL,
        });
        set!(value_map, FIELD_NAME_RHS => self.right_operand().to_json_with(opts));
    }
}

impl ToJson for BooleanSentence {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        add_enum!(self,
            value_map, opts => NODE_KIND_BOOLEAN_SENTENCE
                ; Unary => NODE_VARIANT_UNARY
                ; Binary => NODE_VARIANT_BINARY
        );
    }
}

impl ToJson for UnaryBooleanSentence {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        set!(value_map => NODE_KIND_UNARY_BOOLEAN_SENTENCE);
        set_source_span!(self, value_map, opts);
        set!(value_map, FIELD_NAME_OPERATOR => self.operator().to_string());
        set!(value_map, FIELD_NAME_OPERAND => self.operand().to_json_with(opts));
    }
}

impl ToJson for BinaryBooleanSentence {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        set!(value_map => NODE_KIND_BINARY_BOOLEAN_SENTENCE);
        set_source_span!(self, value_map, opts);
        set!(value_map, FIELD_NAME_LHS => self.left_operand().to_json_with(opts));
        set!(value_map, FIELD_NAME_OPERATOR => match self.operator() {
            ConnectiveOperator::Negation => NODE_KIND_LOGICAL_OP_NEGATION,
            ConnectiveOperator::Conjunction => NODE_KIND_LOGICAL_OP_CONJUNCTION,
            ConnectiveOperator::Disjunction => NODE_KIND_LOGICAL_OP_DISJUNCTION,
            ConnectiveOperator::ExclusiveDisjunction => NODE_KIND_LOGICAL_OP_EXCLUSIVE_DISJUNCTION,
            ConnectiveOperator::Implication => NODE_KIND_LOGICAL_OP_IMPLICATION,
            ConnectiveOperator::Biconditional => NODE_KIND_LOGICAL_OP_BICONDITIONAL,
        });
        set!(value_map, FIELD_NAME_RHS => self.right_operand().to_json_with(opts));
    }
}

impl ToJson for QuantifiedSentence {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        set!(value_map => NODE_KIND_QUANTIFIED_SENTENCE);
        set_source_span!(self, value_map, opts);
        set!(value_map, FIELD_NAME_BINDING => self.binding().to_json_with(opts));
        set!(value_map, FIELD_NAME_SENTENCE => self.body().to_json_with(opts));
    }
}

impl ToJson for QuantifiedVariableBinding {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        set!(value_map => NODE_KIND_QUANTIFIED_VARIABLE_BINDING);
        set_source_span!(self, value_map, opts);
        set!(
            value_map,
            FIELD_NAME_QUANTIFIER =>
            self.quantifier().to_string()
        );
        set!(value_map, FIELD_NAME_BINDING => self.binding().to_json_with(opts));
    }
}

impl ToJson for QuantifiedVariable {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        set!(value_map => NODE_KIND_QUANTIFIED_VARIABLE);
        set_source_span!(self, value_map, opts);
        set!(value_map, FIELD_NAME_VARIABLE => self.variable().to_json_with(opts));
        set!(value_map, FIELD_NAME_SOURCE => self.source().to_json_with(opts));
    }
}

impl ToJson for Term {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        set_variant!(value_map, opts => NODE_KIND_TERM);

        match self {
            Self::Sequence(v) => set_variant!(value_map, opts => v, NODE_VARIANT_SEQUENCE),
            Self::Function(v) => set_variant!(value_map, opts => v, NODE_VARIANT_FUNCTION),
            Self::Composition(v) => set_variant!(value_map, opts => v, NODE_VARIANT_COMPOSITION),
            Self::Identifier(v) => set_variant!(value_map, opts => v, NODE_VARIANT_IDENTIFIER),
            Self::ReservedSelf => {
                set!(value_map, NODE_VARIANT_SELF => true);
            }
            Self::Value(v) => set_variant!(value_map, opts => v, NODE_VARIANT_VALUE),
        }
    }
}

impl ToJson for SequenceBuilder {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        set!(value_map => NODE_KIND_SEQUENCE_BUILDER);
        set_source_span!(self, value_map, opts);
        let mut variable_array: Vec<Value> = Vec::default();
        for variable in self.variables() {
            variable_array.push(variable.to_json_with(opts));
        }
        set!(value_map, FIELD_NAME_VARIABLES => Value::Array(variable_array));
        set!(value_map, FIELD_NAME_BODY => self.body().to_json_with(opts));
    }
}

impl ToJson for Variable {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, _opts: ValueOptions) {
        set!(value_map => NODE_KIND_VARIABLE);
        set!(value_map, FIELD_NAME_NAME => self.name().as_ref());
        if let Some(range) = self.range() {
            set!(value_map, FIELD_NAME_RANGE => range.to_string().as_str());
        }
    }
}

impl ToJson for FunctionalTerm {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        set!(value_map => NODE_KIND_FUNCTIONAL_TERM);
        set_source_span!(self, value_map, opts);
        set!(value_map, FIELD_NAME_FUNCTION => self.function().to_json_with(opts));
        add_collection!(
            value_map,
            opts,
            self,
            has_arguments,
            arguments,
            FIELD_NAME_ARGUMENTS
        );
    }
}

impl ToJson for FunctionComposition {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        set!(value_map => NODE_KIND_FUNCTION_COMPOSITION);
        set_source_span!(self, value_map, opts);
        set!(value_map, FIELD_NAME_SUBJECT => self.subject().to_json_with(opts));
        add_collection!(
            value_map,
            opts,
            self,
            has_function_names,
            function_names,
            FIELD_NAME_FUNCTIONS
        );
    }
}

impl ToJson for Subject {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        set_variant!(value_map, opts => NODE_KIND_SUBJECT);

        match self {
            Self::ReservedSelf => {
                set!(value_map, NODE_VARIANT_SELF => true);
            }
            Self::Identifier(v) => set_variant!(value_map, opts => v, NODE_VARIANT_IDENTIFIER),
        }
    }
}

impl ToJson for TypeVariable {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        set!(value_map => NODE_KIND_TYPE_VARIABLE);
        set_source_span!(self, value_map, opts);
        set!(value_map, opts, name => self);
        if let Some(cardinality) = self.cardinality() {
            set!(value_map, FIELD_NAME_CARDINALITY => cardinality.to_json_with(opts));
        }
        add_collection!(
            value_map,
            opts,
            self,
            has_restrictions,
            restrictions,
            FIELD_NAME_RESTRICTION
        );
    }
}

impl ToJson for TypeClassReference {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        set!(value_map => NODE_KIND_TYPE_CLASS_REFERENCE);
        set_source_span!(self, value_map, opts);
        set!(value_map, opts, name => self);
        add_collection!(
            value_map,
            opts,
            self,
            has_arguments,
            arguments,
            FIELD_NAME_ARGUMENTS
        );
    }
}

impl ToJson for TypeClassArgument {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        add_enum!(self,
            value_map, opts => NODE_KIND_TYPE_CLASS_ARGUMENT
            ; Reference => NODE_VARIANT_REFERENCE
            ; ! Wildcard => NODE_VARIANT_WILDCARD
        );
    }
}

impl ToJson for MethodDef {
    fn add_to_json_with(&self, value_map: &mut Map<String, Value>, opts: ValueOptions) {
        set!(value_map => NODE_KIND_METHOD_DEF);
        set_source_span!(self, value_map, opts);
        set!(value_map, opts, name => self);
        set!(value_map, FIELD_NAME_SIGNATURE => self.signature().to_json_with(opts));
        if let Some(body) = self.body() {
            set!(value_map, FIELD_NAME_BODY => body.to_json_with(opts));
        }
        add_annotations!(value_map, opts, self);
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

#[inline(always)]
fn into_generator_error(e: serde_json::Error) -> Error {
    println!("{e:#?}");
    Error::GeneratorError {
        name: "JSON".into(),
        message: e.to_string(),
    }
}
