/*!
One-line description.

More detailed description, with

# Example

 */

use crate::errors::{missing_base_uri_error, module_not_loaded_error};
use rdftk_core::model::{
    graph::{Graph, PrefixMapping},
    literal::{DataType, Literal},
    statement::{BlankNode, Statement, SubjectNode},
};
use rdftk_io::{
    self, nq::NQuadWriter, nt::NTripleWriter, nt::NTripleWriterOptions, turtle::TurtleWriter,
    HasOptions, ObjectWriter,
};
use rdftk_iri::{Iri, IriExtra, Name};
use sdml_core::{
    model::{
        annotations::{Annotation, AnnotationProperty, HasAnnotations},
        constraints::{Constraint, ConstraintBody, ControlledLanguageString, FormalConstraint},
        definitions::{
            DatatypeDef, Definition, DimensionDef, EntityDef, EnumDef, EventDef, PropertyDef,
            RdfDef, SourceEntity, StructureBody, StructureDef, TypeClassDef, TypeVariant, UnionDef,
            ValueVariant,
        },
        identifiers::{Identifier, IdentifierReference},
        members::{Member, MemberDef, MemberKind, TypeReference},
        modules::{HeaderValue, Module},
        values::{
            MappingValue, SequenceMember, SequenceOfValues, SimpleValue, Value, ValueConstructor,
        },
        HasBody, HasName, HasNameReference, HasOptionalBody, HasSourceSpan, Span,
    },
    repr::RepresentationWriter,
    stdlib::{owl, rdf, rdfs, sdml},
    store::{InMemoryModuleCache, ModuleStore},
};
use sdml_errors::Error as ApiError;
use std::{fmt::Display, io::Write, str::FromStr};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, Default)]
pub struct Options {
    include_source_location: bool,
    mappings: Option<PrefixMapping>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Dialect {
    NTriples,
    #[default]
    Turtle,
    NQuads,
    TriX,
}

#[derive(Clone, Debug, Default)]
pub struct WriterOptions {
    dialect: Dialect,
    options: Options,
}

#[derive(Clone, Debug, Default)]
pub struct Writer;

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

macro_rules! mkiri {
    ($module:ident : $name:ident) => {
        mkiri!($module::MODULE_URL, $module::$name)
    };
    ($base:expr, $name:expr) => {
        Iri::from_str(&format!("{}{}", $base, $name)).unwrap()
    };
}

macro_rules! g_insert {
    ($graph:expr ; $subject:expr, rdf:type, $object:expr) => {
        g_insert!($graph ; $subject, mkiri!(rdf:TYPE), $object)
    };
    ($graph:expr ; $subject:expr, sdml:name, $object:expr) => {
        g_insert!($graph ; $subject, mkiri!(sdml:NAME), $object)
    };
    ($graph:expr ; $subject:expr, $predicate:expr, $object:expr) => {
        $graph.insert(Statement::new($subject, $predicate, $object))
    };
    ($graph:expr ; $ctx:expr => rdf:type, $object:expr) => {
        g_insert!(
            $graph ;
            $ctx.current_subject(),
            mkiri!(rdf:TYPE),
            $object
        )
    };
    ($graph:expr ; $ctx:expr => rdf:value, $object:expr) => {
        g_insert!(
            $graph ;
            $ctx.current_subject(),
            mkiri!(rdf:VALUE),
            $object
        )
    };
    ($graph:expr ; $ctx:expr => sdml:name, $name:expr) => {
        g_insert!(
            $graph ;
            $ctx.current_subject(),
            mkiri!(sdml:NAME),
            Literal::with_data_type_iri($name.to_string(), mkiri!(sdml:IDENTIFIER))
        )
    };
    ($graph:expr ; $ctx:expr => sdml:srcLabel, $name:expr) => {
        g_insert!(
            $graph ;
            $ctx.current_subject(),
            mkiri!(sdml:SRC_LABEL),
            Literal::plain($name.to_string())
        )
    };
    ($graph:expr ; $ctx:expr => $predicate:expr, $object:expr) => {
        g_insert!(
            $graph ;
            $ctx.current_subject(),
            $predicate,
            $object
        )
    };
    ($graph:expr ; $subject:expr, $predicate:expr => $ctx:expr) => {
        g_insert!(
            $graph ;
            $subject,
            $predicate,
            $ctx.current_subject().to_object()
        )
    };
}

macro_rules! set_current_context {
    ($named:expr, $ctx:expr) => {{
        let new_name = rdftk_iri::Name::from_str($named.name().as_ref()).unwrap();
        let new_subject = $ctx.base_uri.make_name(new_name).unwrap();
        $ctx.push_subject(new_subject);
    }};
}

macro_rules! defn_common {
    ($defn:expr, $ctx:expr, $graph:expr, $type_name:expr) => {
        defn_common!($defn, $ctx, $graph, $type_name, sdml::HAS_DEFINITION);
    };
    ($defn:expr, $ctx:expr, $graph:expr, $type_name:expr, $member_name:expr) => {
        let outer_subject = $ctx.current_subject().clone();
        set_current_context!($defn, $ctx);

        g_insert!($graph ; outer_subject, mkiri!(sdml::MODULE_URL, $member_name) => $ctx);

        g_insert!($graph ; $ctx => rdf:type, mkiri!(sdml::MODULE_URL, $type_name));
        g_insert!($graph ; $ctx => sdml:srcLabel, $defn.name());
    };
}

macro_rules! add_source_span {
    ($owner:expr, $graph:expr, $ctx:expr) => {
        if let Some(span) = $owner.source_span() {
            add_source_span(span, $graph, $ctx)?;
        }
    };
}

macro_rules! add_annotations {
    ($owner:expr, $graph:expr, $ctx:expr, $cache:expr) => {
        for annotation in $owner.annotations() {
            add_annotation(annotation, $graph, $ctx, $cache)?;
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn module_to_graph(
    module: &Module,
    cache: &impl ModuleStore,
    options: &Options,
) -> Result<Graph, ApiError> {
    let mappings = if let Some(mappings) = &options.mappings {
        mappings.clone()
    } else {
        PrefixMapping::common().with_dc_elements()
    };
    let mappings = if let Some(base_uri) = module.base_uri() {
        mappings.with_default(base_uri.value().clone())
    } else {
        mappings
    };
    let mappings = mappings.with(
        Name::from_str(sdml::MODULE_NAME).expect("TODO: convert to Error"),
        Iri::parse(sdml::MODULE_URL)?,
    );
    let mut graph = Graph::default().with_mappings(mappings);

    add_module_to_graph(module, &mut graph, cache, options)?;

    Ok(graph)
}

pub fn add_module_to_graph(
    module: &Module,
    graph: &mut Graph,
    cache: &impl ModuleStore,
    options: &Options,
) -> Result<(), ApiError> {
    if let Some(base_uri) = module.base_uri() {
        let mut ctx = Context {
            module_name: module.name().clone(),
            base_uri: base_uri.value().clone(),
            subject: vec![base_uri.value().join(module.name().as_ref())?.into()],
            options: options.clone(),
        };

        add_source_span!(module, graph, &mut ctx);

        g_insert!(graph ; ctx => rdf:type, mkiri!(owl:ONTOLOGY));
        g_insert!(graph ; ctx => rdf:type, mkiri!(sdml:MODULE));

        g_insert!(graph ; ctx => sdml:srcLabel, module.name());

        if let Some(version_uri) = module.version_uri() {
            g_insert!(graph ; ctx => mkiri!(owl:VERSION_IRI), version_uri.value());
        }

        if let Some(version_info) = module.version_info() {
            g_insert!(graph ; ctx => mkiri!(owl:VERSION_INFO), Literal::plain(version_info.value()));
        }

        add_annotations!(module, graph, &mut ctx, cache);

        for name_and_version in module.imported_module_versions() {
            add_an_import(name_and_version, graph, &mut ctx, cache)?;
        }

        for defn in module.definitions() {
            add_definition(defn, graph, &mut ctx, cache)?;
        }

        Ok(())
    } else {
        Err(missing_base_uri_error(module.name()))
    }
}

// ------------------------------------------------------------------------------------------------
// Public Implementations ❱ Writer
// ------------------------------------------------------------------------------------------------

impl RepresentationWriter for Writer {
    type Object = Module;
    type Cache = InMemoryModuleCache;
    type Options = WriterOptions;

    fn write_with<W>(
        &self,
        w: &mut W,
        module: &Self::Object,
        cache: Option<&Self::Cache>,
        options: &Self::Options,
    ) -> Result<(), ApiError>
    where
        W: Write,
    {
        let graph = module_to_graph(module, cache.unwrap(), &options.options)?;
        match options.dialect {
            Dialect::NTriples => NTripleWriter::default()
                .with_options(NTripleWriterOptions::default().force_string_literals(true))
                .write(w, &graph),
            Dialect::Turtle => TurtleWriter::default().write(w, &graph),
            Dialect::NQuads => NQuadWriter::default().write(w, &graph),
            Dialect::TriX => todo!(),
        }
        .map_err(|e| ApiError::GeneratorError {
            name: options.dialect.to_string(),
            message: e.to_string(),
        })
    }
}

// ------------------------------------------------------------------------------------------------
// Public Implementations ❱ Dialect
// ------------------------------------------------------------------------------------------------

impl Display for Dialect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::NTriples => "RDF NTriples",
                Self::Turtle => "RDF Turtle",
                Self::NQuads => "RDF NQuads",
                Self::TriX => "RDF TRiX",
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------
// Public Implementations ❱ Options
// ------------------------------------------------------------------------------------------------

impl Options {
    pub fn with_include_source_location(self, include_source_location: bool) -> Self {
        let mut self_mut = self;
        self_mut.set_include_source_location(include_source_location);
        self_mut
    }

    pub fn include_source_location(&self) -> bool {
        self.include_source_location
    }

    pub fn set_include_source_location(&mut self, include_source_location: bool) {
        self.include_source_location = include_source_location
    }

    pub fn with_mappings(self, mappings: PrefixMapping) -> Self {
        let mut self_mut = self;
        self_mut.set_mappings(mappings);
        self_mut
    }

    pub fn mappings(&self) -> Option<&PrefixMapping> {
        self.mappings.as_ref()
    }

    pub fn set_mappings(&mut self, mappings: PrefixMapping) {
        self.mappings = Some(mappings);
    }

    pub fn unset_mappings(&mut self) {
        self.mappings = None;
    }
}

// ------------------------------------------------------------------------------------------------
// Public Implementations ❱ Context
// ------------------------------------------------------------------------------------------------

impl Context {
    fn current_subject(&self) -> &SubjectNode {
        self.subject.last().expect("Error, subject is empty (get)")
    }

    #[allow(dead_code)] // TODO: audit this
    fn push_subject<S>(&mut self, new_subject: S)
    where
        S: Into<SubjectNode>,
    {
        self.subject.push(new_subject.into());
    }

    #[allow(dead_code)] // TODO: audit this
    fn pop_subject(&mut self) -> SubjectNode {
        self.subject.pop().expect("Error, subject is empty (pop)")
    }
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[allow(dead_code)] // TODO: audit this
#[derive(Debug)]
struct Context {
    module_name: Identifier,
    base_uri: Iri,
    subject: Vec<SubjectNode>,
    options: Options,
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private functions type for transformer functions:
//
// | type AddToGraphFn<T> = fn(
// |     model_element: &T,
// |     graph: &mut Graph,
// |     ctx: &mut Context,
// |     cache: Box<&dyn ModuleStore>,
// | ) -> Result<(), ApiError>;
//
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions ❱ Modules & Imports
// ------------------------------------------------------------------------------------------------

fn add_an_import(
    name_and_version: (&Identifier, Option<&HeaderValue<Iri>>),
    graph: &mut Graph,
    ctx: &mut Context,
    cache: &impl ModuleStore,
) -> Result<(), ApiError> {
    let (name, version) = name_and_version;
    if let Some(actual) = cache.get(name) {
        if let Some(version_uri) = version {
            g_insert!(graph ; ctx => mkiri!(owl:IMPORTS), version_uri.value());
        } else if let Some(base_uri) = actual.base_uri() {
            g_insert!(graph ; ctx => mkiri!(owl:IMPORTS), base_uri.value());
        } else {
            panic!("missing_base_uri");
        }
    } else {
        panic!("module_not_loaded: {name}");
    }
    Ok(())
}

// ------------------------------------------------------------------------------------------------
// Private Functions ❱ Spans
// ------------------------------------------------------------------------------------------------

fn add_source_span(span: &Span, graph: &mut Graph, ctx: &mut Context) -> Result<(), ApiError> {
    if ctx.options.include_source_location {
        let blank = BlankNode::generate();
        g_insert!(graph ; ctx =>  mkiri!(sdml:SOURCE_LOCATION), blank.clone());
        g_insert!(graph ; blank.clone(), mkiri!(sdml:START_BYTE), Literal::from(span.start() as u64));
        g_insert!(graph ; blank, mkiri!(sdml:END_BYTE), Literal::from(span.end() as u64));
    }
    Ok(())
}

// ------------------------------------------------------------------------------------------------
// Private Functions ❱ Names & Values
// ------------------------------------------------------------------------------------------------

fn identifier_reference_to_url(
    id_ref: &IdentifierReference,
    ctx: &mut Context,
    cache: &impl ModuleStore,
) -> Result<Iri, ApiError> {
    match id_ref {
        IdentifierReference::Identifier(ident) => Ok(ctx.base_uri.join(ident.as_ref())?),
        IdentifierReference::QualifiedIdentifier(ident) => {
            if let Some(other_base_uri) = cache.module_name_to_uri(ident.module()) {
                let new_name = rdftk_iri::Name::from_str(ident.member().as_ref()).unwrap();
                let url = other_base_uri.make_name(new_name).unwrap();
                Ok(url)
            } else {
                Err(module_not_loaded_error(ident.module()))
            }
        }
    }
}

fn add_values(
    subject: &SubjectNode,
    predicate: &Iri,
    value: &Value,
    graph: &mut Graph,
    ctx: &mut Context,
    cache: &impl ModuleStore,
) -> Result<(), ApiError> {
    match value {
        Value::Simple(v) => add_simple_value(subject, predicate, v, graph),
        Value::ValueConstructor(v) => {
            add_value_constructor_value(subject, predicate, v, graph, ctx, cache)
        }
        Value::Mapping(v) => add_mapping_value(subject, predicate, v, graph, ctx, cache),
        Value::Reference(v) => add_name_reference_value(subject, predicate, v, graph, ctx, cache),
        Value::Sequence(vs) => add_sequence_value(subject, predicate, vs, graph, ctx, cache),
    }?;

    Ok(())
}

fn add_sequence_value(
    subject: &SubjectNode,
    predicate: &Iri,
    value: &SequenceOfValues,
    graph: &mut Graph,
    ctx: &mut Context,
    cache: &impl ModuleStore,
) -> Result<(), ApiError> {
    let sequence = BlankNode::generate();
    g_insert!(graph ; subject.clone(), predicate.clone(), &sequence);
    g_insert!(graph ; &sequence, rdf:type, mkiri!(sdml:SEQUENCE));

    if let Some(ordering) = value.ordering() {
        g_insert!(graph ; &sequence, mkiri!(sdml::MODULE_URL, sdml::ORDERING), mkiri!(sdml::MODULE_URL, ordering.to_string()));
    }

    if let Some(uniqueness) = value.uniqueness() {
        g_insert!(graph ; &sequence, mkiri!(sdml::MODULE_URL, sdml::UNIQUENESS), mkiri!(sdml::MODULE_URL, uniqueness.to_string()));
    }

    if let Some(uniqueness) = value.uniqueness() {}

    let sequence = SubjectNode::from(&sequence);

    for (i, value) in value.iter().enumerate() {
        let predicate = mkiri!(rdf::MODULE_URL, format!("_{}", i + 1));
        match value {
            SequenceMember::Simple(v) => add_simple_value(&sequence, &predicate, v, graph)?,
            SequenceMember::ValueConstructor(v) => {
                add_value_constructor_value(&sequence, &predicate, v, graph, ctx, cache)?
            }
            SequenceMember::Reference(v) => {
                add_name_reference_value(&sequence, &predicate, v, graph, ctx, cache)?
            }
            SequenceMember::Mapping(v) => {
                add_mapping_value(&sequence, &predicate, v, graph, ctx, cache)?
            }
        };
    }

    Ok(())
}

fn add_value_constructor_value(
    subject: &SubjectNode,
    predicate: &Iri,
    value: &ValueConstructor,
    graph: &mut Graph,
    ctx: &mut Context,
    cache: &impl ModuleStore,
) -> Result<(), ApiError> {
    let lexical_form = match value.value() {
        SimpleValue::Boolean(v) => v.to_string(),
        SimpleValue::Double(v) => v.to_string(),
        SimpleValue::Decimal(v) => v.to_string(),
        SimpleValue::Integer(v) => v.to_string(),
        SimpleValue::Unsigned(v) => v.to_string(),
        SimpleValue::String(v) => v.to_string(),
        SimpleValue::IriReference(v) => v.to_string(),
        SimpleValue::Binary(v) => Literal::hex_encoded(v.as_bytes()).lexical_form().clone(),
    };
    let data_type =
        identifier_reference_to_url(value.type_name(), ctx, cache).expect("Iri parse error");
    g_insert!(graph ;
        subject.clone(),
        predicate.clone(),
        Literal::with_data_type(lexical_form, DataType::Other(data_type))
    );
    Ok(())
}

fn add_name_reference_value(
    subject: &SubjectNode,
    predicate: &Iri,
    value: &IdentifierReference,
    graph: &mut Graph,
    ctx: &mut Context,
    cache: &impl ModuleStore,
) -> Result<(), ApiError> {
    g_insert!(graph ;
        subject.clone(),
        predicate.clone(),
        identifier_reference_to_url(value, ctx, cache).expect("Iri parse error")
    );
    Ok(())
}

fn add_mapping_value(
    subject: &SubjectNode,
    predicate: &Iri,
    value: &MappingValue,
    graph: &mut Graph,
    ctx: &mut Context,
    cache: &impl ModuleStore,
) -> Result<(), ApiError> {
    let mapping = BlankNode::generate();
    graph.insert(Statement::new(subject.clone(), predicate.clone(), &mapping));
    g_insert!(graph ; &mapping, rdf:type, mkiri!(sdml:MAP_TYPE));

    let mapping = SubjectNode::from(mapping);

    add_simple_value(&mapping, &mkiri!(sdml:DOMAIN_VALUE), value.domain(), graph)?;
    add_values(
        &mapping,
        &mkiri!(sdml:RANGE_VALUE),
        value.range(),
        graph,
        ctx,
        cache,
    )?;

    Ok(())
}

fn add_simple_value(
    subject: &SubjectNode,
    predicate: &Iri,
    value: &SimpleValue,
    graph: &mut Graph,
) -> Result<(), ApiError> {
    let object = match value {
        SimpleValue::Boolean(v) => Literal::from(*v),
        SimpleValue::Double(v) => Literal::from(v.into_inner()),
        SimpleValue::Decimal(v) => Literal::from(*v),
        SimpleValue::Integer(v) => Literal::from(*v),
        SimpleValue::Unsigned(v) => Literal::from(*v),
        SimpleValue::String(v) => {
            if let Some(language) = v.language() {
                Literal::with_language(v.value(), language.inner().clone())
            } else {
                Literal::plain(v.value())
            }
        }
        SimpleValue::IriReference(v) => Literal::from(v.clone()),
        SimpleValue::Binary(v) => {
            println!("BYTES: {:#?}", v.as_bytes());
            Literal::hex_encoded(v.as_bytes())
        }
    };

    g_insert!(graph ; subject.clone(), predicate.clone(), object);

    Ok(())
}

// ------------------------------------------------------------------------------------------------
// Private Functions ❱ Annotations
// ------------------------------------------------------------------------------------------------

fn add_annotation(
    me: &Annotation,
    graph: &mut Graph,
    ctx: &mut Context,
    cache: &impl ModuleStore,
) -> Result<(), ApiError> {
    match me {
        Annotation::Property(me) => add_annotation_property(me, graph, ctx, cache)?,
        Annotation::Constraint(me) => add_annotation_constraint(me, graph, ctx, cache)?,
    }
    Ok(())
}

fn add_annotation_property(
    property: &AnnotationProperty,
    graph: &mut Graph,
    ctx: &mut Context,
    cache: &impl ModuleStore,
) -> Result<(), ApiError> {
    //add_source_span!(property, graph, ctx);
    let subject = ctx.current_subject().clone();
    let predicate = identifier_reference_to_url(property.name_reference(), ctx, cache)
        .expect("Iri parse error");
    add_values(&subject, &predicate, property.value(), graph, ctx, cache)
}

fn add_annotation_constraint(
    constraint: &Constraint,
    graph: &mut Graph,
    ctx: &mut Context,
    cache: &impl ModuleStore,
) -> Result<(), ApiError> {
    let subject = ctx.current_subject();
    let constraint_node = BlankNode::generate();
    g_insert!(graph ;
        subject,
        mkiri!(sdml:HAS_CONSTRAINT),
        &constraint_node
    );
    let name = Literal::from(constraint.name().to_string());
    g_insert!(graph ; &constraint_node, sdml:name, name);

    ctx.push_subject(constraint_node);

    match constraint.body() {
        ConstraintBody::Informal(v) => add_informal_constraint(v, graph, ctx, cache)?,
        ConstraintBody::Formal(v) => add_formal_constraint(v, graph, ctx, cache)?,
    };

    ctx.pop_subject();

    Ok(())
}

fn add_informal_constraint(
    constraint: &ControlledLanguageString,
    graph: &mut Graph,
    ctx: &mut Context,
    _cache: &impl ModuleStore,
) -> Result<(), ApiError> {
    let subject = ctx.current_subject();
    g_insert!(graph ; ctx => rdf:type, mkiri!(sdml:CONSTRAINT));
    g_insert!(graph ; ctx => rdf:type, mkiri!(sdml:INFORMAL_CONSTRAINT));
    g_insert!(graph ; ctx => rdf:value, Literal::plain(constraint.value()));
    if let Some(language) = constraint.language() {
        g_insert!(graph ; subject, mkiri!(sdml:CONTROLLED_LANG_STRING), Literal::plain(language.to_string()));
    }
    Ok(())
}

fn add_formal_constraint(
    _constraint: &FormalConstraint,
    graph: &mut Graph,
    ctx: &mut Context,
    _cache: &impl ModuleStore,
) -> Result<(), ApiError> {
    g_insert!(graph ; ctx => rdf:type, mkiri!(sdml:CONSTRAINT));
    g_insert!(graph ; ctx => rdf:type, mkiri!(sdml:FORMAL_CONSTRAINT));
    Ok(())
}

// ------------------------------------------------------------------------------------------------
// Private Functions ❱ Definitions
// ------------------------------------------------------------------------------------------------

fn add_definition(
    definition: &Definition,
    graph: &mut Graph,
    ctx: &mut Context,
    cache: &impl ModuleStore,
) -> Result<(), ApiError> {
    match definition {
        Definition::Datatype(v) => add_datatype_def(v, graph, ctx, cache),
        Definition::Dimension(v) => add_dimension_def(v, graph, ctx, cache),
        Definition::Entity(v) => add_entity_def(v, graph, ctx, cache),
        Definition::Enum(v) => add_enum_def(v, graph, ctx, cache),
        Definition::Event(v) => add_event_def(v, graph, ctx, cache),
        Definition::Property(v) => add_property_def(v, graph, ctx, cache),
        Definition::Rdf(v) => add_rdf_def(v, graph, ctx, cache),
        Definition::Structure(v) => add_structure_def(v, graph, ctx, cache),
        Definition::TypeClass(v) => add_type_class_def(v, graph, ctx, cache),
        Definition::Union(v) => add_union_def(v, graph, ctx, cache),
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions ❱ Definitions ❱ Datatype
// ------------------------------------------------------------------------------------------------

fn add_datatype_def(
    defn: &DatatypeDef,
    graph: &mut Graph,
    ctx: &mut Context,
    cache: &impl ModuleStore,
) -> Result<(), ApiError> {
    defn_common!(defn, ctx, graph, sdml::DATATYPE);

    // Also, make this resource an RDF-Schema Datatype
    g_insert!(graph ;
        ctx =>
        rdf:type,
        mkiri!(rdfs:DATATYPE)
    );

    // As a restriction on the following type...
    let base_uri =
        { identifier_reference_to_url(defn.base_type(), ctx, cache).expect("Iri parse error") };
    g_insert!(graph ;
        ctx =>
        mkiri!(owl:ON_DATATYPE),
        base_uri
    );

    // With these facet restrictions...
    // TODO: defn.restrictions()

    if let Some(body) = defn.body() {
        add_annotations!(body, graph, ctx, cache);
    }

    ctx.pop_subject();
    Ok(())
}

// ------------------------------------------------------------------------------------------------
// Private Functions ❱ Definitions ❱ Dimension
// ------------------------------------------------------------------------------------------------

fn add_dimension_def(
    defn: &DimensionDef,
    graph: &mut Graph,
    ctx: &mut Context,
    cache: &impl ModuleStore,
) -> Result<(), ApiError> {
    defn_common!(defn, ctx, graph, sdml::DIMENSION);

    if let Some(body) = defn.body() {
        add_annotations!(body, graph, ctx, cache);
    }

    ctx.pop_subject();
    Ok(())
}

fn add_entity_def(
    defn: &EntityDef,
    graph: &mut Graph,
    ctx: &mut Context,
    cache: &impl ModuleStore,
) -> Result<(), ApiError> {
    defn_common!(defn, ctx, graph, sdml::ENTITY);

    if let Some(body) = defn.body() {
        add_annotations!(body, graph, ctx, cache);
    }

    ctx.pop_subject();
    Ok(())
}

// ------------------------------------------------------------------------------------------------
// Private Functions ❱ Definitions ❱ Enum
// ------------------------------------------------------------------------------------------------

fn add_enum_def(
    defn: &EnumDef,
    graph: &mut Graph,
    ctx: &mut Context,
    cache: &impl ModuleStore,
) -> Result<(), ApiError> {
    defn_common!(defn, ctx, graph, sdml::ENUMERATION);

    if let Some(body) = defn.body() {
        add_annotations!(body, graph, ctx, cache);

        for variant in body.variants() {
            add_value_variant(variant, graph, ctx, cache)?;
        }
    }

    ctx.pop_subject();
    Ok(())
}

fn add_value_variant(
    defn: &ValueVariant,
    graph: &mut Graph,
    ctx: &mut Context,
    cache: &impl ModuleStore,
) -> Result<(), ApiError> {
    defn_common!(
        defn,
        ctx,
        graph,
        sdml::VALUE_VARIANT,
        sdml::HAS_VALUE_VARIANT
    );

    if let Some(body) = defn.body() {
        add_annotations!(body, graph, ctx, cache);
    }

    ctx.pop_subject();
    Ok(())
}

// ------------------------------------------------------------------------------------------------
// Private Functions ❱ Definitions ❱ Event
// ------------------------------------------------------------------------------------------------

fn add_event_def(
    defn: &EventDef,
    graph: &mut Graph,
    ctx: &mut Context,
    cache: &impl ModuleStore,
) -> Result<(), ApiError> {
    defn_common!(defn, ctx, graph, sdml::EVENT);

    if let Some(body) = defn.body() {
        add_annotations!(body, graph, ctx, cache);

        add_source_entity(body.source_entity(), graph, ctx, cache)?;

        for member in body.members() {
            add_member(member, graph, ctx, cache)?;
        }
    }

    ctx.pop_subject();
    Ok(())
}

fn add_source_entity(
    _defn: &SourceEntity,
    _graph: &mut Graph,
    _ctx: &mut Context,
    _cache: &impl ModuleStore,
) -> Result<(), ApiError> {
    todo!()
}

// ------------------------------------------------------------------------------------------------
// Private Functions ❱ Definitions ❱ Property
// ------------------------------------------------------------------------------------------------

fn add_property_def(
    defn: &PropertyDef,
    graph: &mut Graph,
    ctx: &mut Context,
    cache: &impl ModuleStore,
) -> Result<(), ApiError> {
    defn_common!(defn, ctx, graph, sdml::PROPERTY);

    add_member_def(defn.member_def(), graph, ctx, cache)?;

    ctx.pop_subject();
    Ok(())
}

// ------------------------------------------------------------------------------------------------
// Private Functions ❱ Definitions ❱ RDF
// ------------------------------------------------------------------------------------------------

fn add_rdf_def(
    defn: &RdfDef,
    graph: &mut Graph,
    ctx: &mut Context,
    cache: &impl ModuleStore,
) -> Result<(), ApiError> {
    defn_common!(defn, ctx, graph, sdml::RDF);

    add_annotations!(defn.body(), graph, ctx, cache);

    ctx.pop_subject();
    Ok(())
}

// ------------------------------------------------------------------------------------------------
// Private Functions ❱ Definitions ❱ Structure
// ------------------------------------------------------------------------------------------------

fn add_structure_def(
    defn: &StructureDef,
    graph: &mut Graph,
    ctx: &mut Context,
    cache: &impl ModuleStore,
) -> Result<(), ApiError> {
    defn_common!(defn, ctx, graph, sdml::STRUCTURE);

    if let Some(body) = defn.body() {
        add_annotations!(body, graph, ctx, cache);

        add_structure_body(body, graph, ctx, cache)?;
    }

    ctx.pop_subject();
    Ok(())
}

fn add_structure_body(
    body: &StructureBody,
    graph: &mut Graph,
    ctx: &mut Context,
    cache: &impl ModuleStore,
) -> Result<(), ApiError> {
    add_annotations!(body, graph, ctx, cache);

    for member in body.members() {
        add_member(member, graph, ctx, cache)?;
    }

    Ok(())
}

fn add_member(
    member: &Member,
    graph: &mut Graph,
    ctx: &mut Context,
    cache: &impl ModuleStore,
) -> Result<(), ApiError> {
    defn_common!(member, ctx, graph, sdml::MEMBER, sdml::HAS_MEMBER);

    let current_subject = ctx.current_subject().clone();
    match member.kind() {
        MemberKind::Reference(v) => g_insert!(graph ;
            &current_subject,
            mkiri!(sdml:IDENTIFIER_REFERENCE),
            identifier_reference_to_url(v, ctx, cache)?
        ),
        MemberKind::Definition(v) => {
            add_member_def(v, graph, ctx, cache)?;
        }
    }

    ctx.pop_subject();
    Ok(())
}

fn add_member_def(
    member: &MemberDef,
    graph: &mut Graph,
    ctx: &mut Context,
    cache: &impl ModuleStore,
) -> Result<(), ApiError> {
    let current_subject = ctx.current_subject().clone();
    g_insert!(graph ; &current_subject, rdf:type, mkiri!(sdml:MEMBER));
    let name = Literal::from(member.name().to_string());
    g_insert!(graph ; &current_subject, sdml:name, name);

    // TODO: member.target_cardinality();

    add_type_reference(
        member.target_type(),
        mkiri!(sdml:HAS_TYPE),
        graph,
        ctx,
        cache,
    )?;

    if let Some(body) = member.body() {
        add_annotations!(body, graph, ctx, cache);
    }

    Ok(())
}

fn add_type_reference(
    type_ref: &TypeReference,
    predicate: Iri,
    graph: &mut Graph,
    ctx: &mut Context,
    cache: &impl ModuleStore,
) -> Result<(), ApiError> {
    let current_subject = ctx.current_subject().clone();

    match type_ref {
        TypeReference::Unknown => {
            g_insert!(graph ;
                &current_subject,
                predicate,
                mkiri!(sdml:UNKNOWN)
            );
        }
        TypeReference::Type(v) => {
            g_insert!(graph ;
                &current_subject,
                predicate,
                identifier_reference_to_url(v, ctx, cache)?
            );
        }
        TypeReference::MappingType(v) => {
            let type_node = BlankNode::generate();
            g_insert!(graph ; &current_subject, predicate, &type_node);

            ctx.push_subject(&type_node);
            add_type_reference(v.domain(), mkiri!(sdml:DOMAIN_TYPE), graph, ctx, cache)?;
            add_type_reference(v.range(), mkiri!(sdml:RANGE_TYPE), graph, ctx, cache)?;
            ctx.pop_subject();
        }
    }

    Ok(())
}

// ------------------------------------------------------------------------------------------------
// Private Functions ❱ Definitions ❱ Type Class
// ------------------------------------------------------------------------------------------------

fn add_type_class_def(
    defn: &TypeClassDef,
    graph: &mut Graph,
    ctx: &mut Context,
    cache: &impl ModuleStore,
) -> Result<(), ApiError> {
    defn_common!(defn, ctx, graph, sdml::TYPE_CLASS);

    if let Some(body) = defn.body() {
        add_annotations!(body, graph, ctx, cache);
    }

    ctx.pop_subject();
    Ok(())
}

// ------------------------------------------------------------------------------------------------
// Private Functions ❱ Definitions ❱ Union
// ------------------------------------------------------------------------------------------------

fn add_union_def(
    defn: &UnionDef,
    graph: &mut Graph,
    ctx: &mut Context,
    cache: &impl ModuleStore,
) -> Result<(), ApiError> {
    defn_common!(defn, ctx, graph, sdml::UNION);

    if let Some(body) = defn.body() {
        add_annotations!(body, graph, ctx, cache);

        for variant in body.variants() {
            add_type_variant(variant, graph, ctx, cache)?;
        }
    }

    ctx.pop_subject();
    Ok(())
}

fn add_type_variant(
    defn: &TypeVariant,
    graph: &mut Graph,
    ctx: &mut Context,
    cache: &impl ModuleStore,
) -> Result<(), ApiError> {
    defn_common!(defn, ctx, graph, sdml::TYPE_VARIANT, sdml::HAS_TYPE_VARIANT);

    if let Some(rename) = defn.rename() {
        // TODO: add RDF
    }

    if let Some(body) = defn.body() {
        add_annotations!(body, graph, ctx, cache);
    }

    ctx.pop_subject();
    Ok(())
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

//        fn write_entity(
//            &mut self,
//            me: &EntityDef,
//            module_name: &Identifier,
//            writer: &mut dyn Write,
//        ) -> Result<(), Error> {
//            let name = me.name();
//
//            writer.write_all(type_subject(module_name, name).as_bytes())?;
//            writer.write_all(
//                predicate_with_value_list(
//                    stdlib::rdf::MODULE_NAME,
//                    stdlib::rdf::TYPE,
//                    &[
//                        type_ref_qname(stdlib::owl::MODULE_NAME, stdlib::owl::CLASS),
//                        type_ref_qname(stdlib::sdml::MODULE_NAME, stdlib::sdml::ENTITY),
//                    ],
//                    Separator::Predicate,
//                )
//                .as_bytes(),
//            )?;
//
//            if let Some(body) = me.body() {
//                write_annotations!(self, body.annotations(), module_name, writer);
//
//                if body.has_members() {
//                    let member_list = body
//                        .members()
//                        .map(|m| thing_qname(module_name, mv_name(name, m.name())))
//                        .collect::<Vec<_>>();
//                    writer.write_all(
//                        predicate_with_value_list(
//                            stdlib::sdml::MODULE_NAME,
//                            stdlib::sdml::HAS_MEMBER,
//                            &member_list,
//                            Separator::Predicate,
//                        )
//                        .as_bytes(),
//                    )?;
//                }
//            }
//
//            self.write_defn_end(module_name, name, writer)?;
//
//            if let Some(body) = me.body() {
//                for member in body.members() {
//                    self.write_member(member, module_name, name, writer)?;
//                }
//            }
//
//            Ok(())
//        }
//
//        fn write_member(
//            &mut self,
//            me: &Member,
//            module_name: &Identifier,
//            parent: &Identifier,
//            writer: &mut dyn Write,
//        ) -> Result<(), Error> {
//            let name = me.name();
//
//            writer.write_all(property_subject(module_name, name).as_bytes())?;
//
//            let more = if let Some(_property) = me.as_property_reference() {
//                writer.write_all(
//                    predicate_with_value_list(
//                        stdlib::rdf::MODULE_NAME,
//                        stdlib::rdf::TYPE,
//                        &[
//                            type_ref_qname(stdlib::rdf::MODULE_NAME, stdlib::rdf::PROPERTY),
//                            type_ref_qname(stdlib::sdml::MODULE_NAME, stdlib::sdml::ROLE_REFERENCE),
//                        ],
//                        Separator::Predicate,
//                    )
//                    .as_bytes(),
//                )?;
//
//                self.write_member_type(me, module_name, writer)?
//            } else if let Some(def) = me.as_definition() {
//                writer.write_all(
//                    predicate_with_value_list(
//                        stdlib::rdf::MODULE_NAME,
//                        stdlib::rdf::TYPE,
//                        &[
//                            type_ref_qname(stdlib::rdf::MODULE_NAME, stdlib::rdf::PROPERTY),
//                            type_ref_qname(stdlib::sdml::MODULE_NAME, stdlib::sdml::MEMBER),
//                        ],
//                        Separator::Predicate,
//                    )
//                    .as_bytes(),
//                )?;
//
//                writer.write_all(
//                    predicate_with_value(
//                        stdlib::rdfs::MODULE_NAME,
//                        stdlib::rdfs::DOMAIN,
//                        type_ref_qname(module_name, parent),
//                        Separator::Predicate,
//                    )
//                    .as_bytes(),
//                )?;
//                let more = self.write_member_type(me, module_name, writer)?;
//
//                if let Some(body) = def.body() {
//                    write_annotations!(self, body.annotations(), module_name, writer);
//                }
//
//                more
//            } else {
//                unreachable!();
//            };
//
//            self.write_defn_end(module_name, name, writer)?;
//
//            if !more.is_empty() {
//                writer.write_all(more.as_bytes())?;
//            }
//
//            Ok(())
//        }
//
//        fn write_member_type(
//            &mut self,
//            me: &Member,
//            module_name: &Identifier,
//            writer: &mut dyn Write,
//        ) -> Result<String, Error> {
//            let more = String::new();
//
//            if let Some(def) = me.as_definition() {
//                match def.target_type() {
//                    TypeReference::Unknown => {
//                        writer.write_all(
//                            predicate_with_value(
//                                stdlib::rdfs::MODULE_NAME,
//                                stdlib::rdfs::RANGE,
//                                type_ref_qname(stdlib::sdml::MODULE_NAME, stdlib::sdml::UNKNOWN),
//                                Separator::Predicate,
//                            )
//                            .as_bytes(),
//                        )?;
//                    }
//                    TypeReference::Type(name) => {
//                        let (ty_module, ty_name) = self.qualified_idref(module_name, name);
//                        writer.write_all(
//                            predicate_with_value(
//                                stdlib::rdfs::MODULE_NAME,
//                                stdlib::rdfs::RANGE,
//                                type_ref_qname(ty_module, ty_name),
//                                Separator::Predicate,
//                            )
//                            .as_bytes(),
//                        )?;
//                        let card = def.target_cardinality();
//                        if card != &DEFAULT_CARDINALITY {
//                            if let Some(ordering) = card.ordering() {
//                                writer.write_all(
//                                    predicate_with_value(
//                                        stdlib::sdml::MODULE_NAME,
//                                        stdlib::sdml::ORDERING,
//                                        if ordering == Ordering::Ordered {
//                                            thing_qname(
//                                                stdlib::sdml::MODULE_NAME,
//                                                stdlib::sdml::ORDERED,
//                                            )
//                                        } else {
//                                            thing_qname(
//                                                stdlib::sdml::MODULE_NAME,
//                                                stdlib::sdml::UNORDERED,
//                                            )
//                                        },
//                                        Separator::Predicate,
//                                    )
//                                    .as_bytes(),
//                                )?;
//                            }
//                            if let Some(uniqueness) = card.uniqueness() {
//                                writer.write_all(
//                                    predicate_with_value(
//                                        stdlib::sdml::MODULE_NAME,
//                                        stdlib::sdml::UNIQUENESS,
//                                        if uniqueness == Uniqueness::Unique {
//                                            thing_qname(
//                                                stdlib::sdml::MODULE_NAME,
//                                                stdlib::sdml::UNIQUE,
//                                            )
//                                        } else {
//                                            thing_qname(
//                                                stdlib::sdml::MODULE_NAME,
//                                                stdlib::sdml::NONUNIQUE,
//                                            )
//                                        },
//                                        Separator::Predicate,
//                                    )
//                                    .as_bytes(),
//                                )?;
//                            }
//                            let range = card.range();
//                            writer.write_all(
//                                predicate_with_value(
//                                    stdlib::owl::MODULE_NAME,
//                                    stdlib::owl::MIN_CARDINALITY,
//                                    format_type_constructor(
//                                        stdlib::xsd::MODULE_NAME,
//                                        stdlib::xsd::NONNEGATIVE_INTEGER,
//                                        range.min_occurs().to_string(),
//                                    ),
//                                    Separator::Predicate,
//                                )
//                                .as_bytes(),
//                            )?;
//                            if let Some(max) = range.max_occurs() {
//                                writer.write_all(
//                                    predicate_with_value(
//                                        stdlib::owl::MODULE_NAME,
//                                        stdlib::owl::MAX_CARDINALITY,
//                                        format_type_constructor(
//                                            stdlib::xsd::MODULE_NAME,
//                                            stdlib::xsd::NONNEGATIVE_INTEGER,
//                                            max.to_string(),
//                                        ),
//                                        Separator::Predicate,
//                                    )
//                                    .as_bytes(),
//                                )?;
//                            }
//                        }
//                    }
//                    TypeReference::MappingType(_map) => {
//                        // 1. throw hands in the air, this is a mess.
//                        // TODO cardinality
//                    }
//                }
//            } else if let Some(_property) = me.as_property_reference() {
//                // 1. lookup `property` in cache
//                // 2. find member name as `role` in property
//                // 3. call self with member type of property
//            } else {
//                unreachable!()
//            }
//
//            Ok(more)
//        }
//
//        fn write_enumeration(
//            &mut self,
//            me: &EnumDef,
//            module_name: &Identifier,
//            writer: &mut dyn Write,
//        ) -> Result<(), Error> {
//            let name = me.name();
//            /*
//            TODO: better RDF
//
//            :enumName
//                :rdfs: subClassOf sdml:Union
//            */
//
//            writer.write_all(type_subject(module_name, name).as_bytes())?;
//            writer.write_all(
//                predicate_with_value_list(
//                    stdlib::rdf::MODULE_NAME,
//                    stdlib::rdf::TYPE,
//                    &[
//                        type_ref_qname(stdlib::owl::MODULE_NAME, stdlib::owl::CLASS),
//                        type_ref_qname(stdlib::sdml::MODULE_NAME, stdlib::sdml::ENUMERATION),
//                    ],
//                    Separator::Predicate,
//                )
//                .as_bytes(),
//            )?;
//
//            if let Some(body) = me.body() {
//                write_annotations!(self, body.annotations(), module_name, writer);
//
//                if body.has_variants() {
//                    let variant_list = body
//                        .variants()
//                        .map(|v| thing_qname(module_name, mv_name(name, v.name())))
//                        .collect::<Vec<_>>();
//                    writer.write_all(
//                        predicate_with_value_list(
//                            stdlib::sdml::MODULE_NAME,
//                            stdlib::sdml::HAS_VALUE_VARIANT,
//                            &variant_list,
//                            Separator::Predicate,
//                        )
//                        .as_bytes(),
//                    )?;
//                }
//            }
//
//            self.write_defn_end(module_name, name, writer)?;
//
//            if let Some(body) = me.body() {
//                for variant in body.variants() {
//                    self.write_value_variant(variant, module_name, name, writer)?;
//                }
//            }
//
//            Ok(())
//        }
//
//        fn write_value_variant(
//            &mut self,
//            me: &ValueVariant,
//            module_name: &Identifier,
//            parent: &Identifier,
//            writer: &mut dyn Write,
//        ) -> Result<(), Error> {
//            let name = mv_name(parent, me.name());
//
//            writer.write_all(thing_subject(module_name, name.clone()).as_bytes())?;
//            writer.write_all(
//                predicate_with_value_list(
//                    stdlib::rdf::MODULE_NAME,
//                    stdlib::rdf::TYPE,
//                    &[
//                        type_ref_qname(stdlib::owl::MODULE_NAME, stdlib::owl::NAMED_INDIVIDUAL),
//                        type_ref_qname(stdlib::sdml::MODULE_NAME, stdlib::sdml::VALUE_VARIANT),
//                        type_ref_qname(module_name, parent),
//                    ],
//                    Separator::Predicate,
//                )
//                .as_bytes(),
//            )?;
//
//            if let Some(body) = me.body() {
//                write_annotations!(self, body.annotations(), module_name, writer);
//            }
//
//            self.write_defn_end(module_name, name, writer)?;
//
//            Ok(())
//        }
//
//        fn write_event(
//            &mut self,
//            me: &EventDef,
//            module_name: &Identifier,
//            writer: &mut dyn Write,
//        ) -> Result<(), Error> {
//            let name = me.name();
//
//            writer.write_all(type_subject(module_name, name).as_bytes())?;
//            writer.write_all(
//                predicate_with_value_list(
//                    stdlib::rdf::MODULE_NAME,
//                    stdlib::rdf::TYPE,
//                    &[
//                        type_ref_qname(stdlib::owl::MODULE_NAME, stdlib::owl::CLASS),
//                        type_ref_qname(stdlib::sdml::MODULE_NAME, stdlib::sdml::EVENT),
//                    ],
//                    Separator::Predicate,
//                )
//                .as_bytes(),
//            )?;
//
//            let (source_module, source_name) = self.qualified_idref(module_name, me.event_source());
//            writer.write_all(
//                predicate_with_value(
//                    stdlib::sdml::MODULE_NAME,
//                    stdlib::sdml::HAS_SOURCE_ENTITY,
//                    predicate_qname(source_module, source_name),
//                    Separator::Predicate,
//                )
//                .as_bytes(),
//            )?;
//
//            if let Some(body) = me.body() {
//                write_annotations!(self, body.annotations(), module_name, writer);
//
//                if body.has_members() {
//                    let member_list = body
//                        .members()
//                        .map(|m| thing_qname(module_name, mv_name(name, m.name())))
//                        .collect::<Vec<_>>();
//                    writer.write_all(
//                        predicate_with_value_list(
//                            stdlib::sdml::MODULE_NAME,
//                            stdlib::sdml::HAS_MEMBER,
//                            &member_list,
//                            Separator::Predicate,
//                        )
//                        .as_bytes(),
//                    )?;
//                }
//            }
//
//            self.write_defn_end(module_name, name, writer)?;
//
//            if let Some(body) = me.body() {
//                for member in body.members() {
//                    self.write_member(member, module_name, name, writer)?;
//                }
//            }
//
//            Ok(())
//        }
//
//        fn write_structure(
//            &mut self,
//            me: &StructureDef,
//            module_name: &Identifier,
//            writer: &mut dyn Write,
//        ) -> Result<(), Error> {
//            let name = me.name();
//
//            writer.write_all(type_subject(module_name, name).as_bytes())?;
//            writer.write_all(
//                predicate_with_value_list(
//                    stdlib::rdf::MODULE_NAME,
//                    stdlib::rdf::TYPE,
//                    &[
//                        type_ref_qname(stdlib::owl::MODULE_NAME, stdlib::owl::CLASS),
//                        type_ref_qname(stdlib::sdml::MODULE_NAME, stdlib::sdml::STRUCTURE),
//                    ],
//                    Separator::Predicate,
//                )
//                .as_bytes(),
//            )?;
//
//            if let Some(body) = me.body() {
//                write_annotations!(self, body.annotations(), module_name, writer);
//
//                if body.has_members() {
//                    let member_list = body
//                        .members()
//                        .map(|m| thing_qname(module_name, mv_name(name, m.name())))
//                        .collect::<Vec<_>>();
//                    writer.write_all(
//                        predicate_with_value_list(
//                            stdlib::sdml::MODULE_NAME,
//                            stdlib::sdml::HAS_MEMBER,
//                            &member_list,
//                            Separator::Predicate,
//                        )
//                        .as_bytes(),
//                    )?;
//                }
//            }
//
//            self.write_defn_end(module_name, name, writer)?;
//
//            if let Some(body) = me.body() {
//                for member in body.members() {
//                    self.write_member(member, module_name, name, writer)?;
//                }
//            }
//
//            Ok(())
//        }
//
//        fn write_union(
//            &mut self,
//            me: &UnionDef,
//            module_name: &Identifier,
//            writer: &mut dyn Write,
//        ) -> Result<(), Error> {
//            let name = me.name();
//
//            /*
//            TODO: better RDF:
//
//            :unionName
//                owl:unionOf
//                    :variantName, ... .
//
//            :variantRename owl:equivalentClass :variantName .
//             */
//
//            writer.write_all(type_subject(module_name, name).as_bytes())?;
//            writer.write_all(
//                predicate_with_value_list(
//                    stdlib::rdf::MODULE_NAME,
//                    stdlib::rdf::TYPE,
//                    &[
//                        type_ref_qname(stdlib::owl::MODULE_NAME, stdlib::owl::CLASS),
//                        type_ref_qname(stdlib::sdml::MODULE_NAME, stdlib::sdml::UNION),
//                    ],
//                    Separator::Predicate,
//                )
//                .as_bytes(),
//            )?;
//
//            if let Some(body) = me.body() {
//                write_annotations!(self, body.annotations(), module_name, writer);
//
//                if body.has_variants() {
//                    let variant_list = body
//                        .variants()
//                        .map(|v| thing_qname(module_name, mv_name(name, v.name())))
//                        .collect::<Vec<_>>();
//                    writer.write_all(
//                        predicate_with_value_list(
//                            stdlib::sdml::MODULE_NAME,
//                            stdlib::sdml::HAS_TYPE_VARIANT,
//                            &variant_list,
//                            Separator::Predicate,
//                        )
//                        .as_bytes(),
//                    )?;
//                }
//            }
//
//            self.write_defn_end(module_name, name, writer)?;
//
//            if let Some(body) = me.body() {
//                for variant in body.variants() {
//                    self.write_type_variant(variant, module_name, name, writer)?;
//                }
//            }
//
//            Ok(())
//        }
//
//        fn write_type_variant(
//            &mut self,
//            me: &TypeVariant,
//            module_name: &Identifier,
//            parent: &Identifier,
//            writer: &mut dyn Write,
//        ) -> Result<(), Error> {
//            let name = format!("{parent}__{}", me.name());
//
//            writer.write_all(type_subject(module_name, name.clone()).as_bytes())?;
//            writer.write_all(
//                predicate_with_value_list(
//                    stdlib::rdf::MODULE_NAME,
//                    stdlib::rdf::TYPE,
//                    &[
//                        type_ref_qname(stdlib::owl::MODULE_NAME, stdlib::owl::CLASS),
//                        type_ref_qname(stdlib::sdml::MODULE_NAME, stdlib::sdml::TYPE_VARIANT),
//                    ],
//                    Separator::Predicate,
//                )
//                .as_bytes(),
//            )?;
//            writer.write_all(
//                predicate_with_value(
//                    stdlib::rdfs::MODULE_NAME,
//                    stdlib::rdfs::SUB_CLASS_OF,
//                    type_ref_qname(module_name, parent),
//                    Separator::Predicate,
//                )
//                .as_bytes(),
//            )?;
//
//            let (ty_module, ty_name) = self.qualified_idref(module_name, me.name_reference());
//            writer.write_all(
//                predicate_with_value(
//                    stdlib::owl::MODULE_NAME,
//                    stdlib::owl::EQUIVALENT_CLASS,
//                    predicate_qname(ty_module, ty_name),
//                    Separator::Predicate,
//                )
//                .as_bytes(),
//            )?;
//
//            if let Some(body) = me.body() {
//                write_annotations!(self, body.annotations(), module_name, writer);
//            }
//
//            self.write_defn_end(module_name, name, writer)?;
//
//            Ok(())
//        }
//
//        fn write_property(
//            &mut self,
//            me: &PropertyDef,
//            module_name: &Identifier,
//            writer: &mut dyn Write,
//        ) -> Result<(), Error> {
//            let name = me.name();
//
//            writer.write_all(type_subject(module_name, name).as_bytes())?;
//            writer.write_all(
//                predicate_with_value_list(
//                    stdlib::rdf::MODULE_NAME,
//                    stdlib::rdf::TYPE,
//                    &[
//                        type_ref_qname(stdlib::owl::MODULE_NAME, stdlib::owl::CLASS),
//                        type_ref_qname(stdlib::sdml::MODULE_NAME, stdlib::sdml::PROPERTY),
//                    ],
//                    Separator::Predicate,
//                )
//                .as_bytes(),
//            )?;
//
//            // TODO: MemberDef
//
//            self.write_defn_end(module_name, name, writer)?;
//
//            Ok(())
//        }
//
//        fn write_rdf(
//            &mut self,
//            me: &RdfDef,
//            module_name: &Identifier,
//            writer: &mut dyn Write,
//        ) -> Result<(), Error> {
//            let name = me.name();
//
//            writer.write_all(thing_subject(module_name, name).as_bytes())?;
//
//            write_annotations!(self, me.body().annotations(), module_name, writer);
//
//            self.write_defn_end(module_name, name, writer)?;
//
//            Ok(())
//        }
//
//        fn write_type_class(
//            &mut self,
//            me: &TypeClassDef,
//            module_name: &Identifier,
//            writer: &mut dyn Write,
//        ) -> Result<(), Error> {
//            let name = me.name();
//            writer.write_all(type_subject(module_name, name).as_bytes())?;
//            writer.write_all(
//                predicate_with_value_list(
//                    stdlib::rdf::MODULE_NAME,
//                    stdlib::rdf::TYPE,
//                    &[
//                        type_ref_qname(stdlib::owl::MODULE_NAME, stdlib::owl::CLASS),
//                        type_ref_qname(stdlib::sdml::MODULE_NAME, stdlib::sdml::TYPE_CLASS),
//                    ],
//                    Separator::Predicate,
//                )
//                .as_bytes(),
//            )?;
//
//            self.write_defn_end(module_name, name, writer)?;
//
//            Ok(())
//        }
//
//        fn write_defn_end<S>(
//            &mut self,
//            module_name: &Identifier,
//            name: S,
//            writer: &mut dyn Write,
//        ) -> Result<(), Error>
//        where
//            S: AsRef<str>,
//        {
//            writer.write_all(
//                predicate_with_value(
//                    stdlib::sdml::MODULE_NAME,
//                    stdlib::sdml::SRC_LABEL,
//                    format_str(format!("{:?}", name.as_ref())),
//                    Separator::Predicate,
//                )
//                .as_bytes(),
//            )?;
//            writer.write_all(
//                predicate_with_value(
//                    stdlib::rdfs::MODULE_NAME,
//                    stdlib::rdfs::IS_DEFINED_BY,
//                    module_ref_qname(module_name),
//                    Separator::Statement,
//                )
//                .as_bytes(),
//            )?;
//            writer.write_all(b"\n")?;
//            Ok(())
//        }
//
//        fn write_annotation_property(
//            &mut self,
//            me: &AnnotationProperty,
//            module_name: &Identifier,
//            writer: &mut dyn Write,
//        ) -> Result<(), Error> {
//            let (module, name) = self.qualified_idref(module_name, me.name_reference());
//
//            writer.write_all(
//                predicate_with_value(
//                    module,
//                    name,
//                    self.value_to_string(me.value(), module_name),
//                    Separator::Predicate,
//                )
//                .as_bytes(),
//            )?;
//
//            Ok(())
//        }
//
//        fn write_facet_property(
//            &mut self,
//            me: &AnnotationProperty,
//            module_name: &Identifier,
//            sep: Separator,
//            writer: &mut dyn Write,
//        ) -> Result<(), Error> {
//            let (module, name) = self.qualified_idref(module_name, me.name_reference());
//            let value = self.value_to_string(me.value(), module_name);
//            writer.write_all(bnode_predicate_with_value(module, name, value, sep).as_bytes())?;
//
//            Ok(())
//        }
//
//        fn write_constraint(
//            &mut self,
//            _me: &Constraint,
//            _module_name: &Identifier,
//            _writer: &mut dyn Write,
//        ) -> Result<(), Error> {
//            todo!();
//        }
//
//        fn qualified_idref_string(
//            &self,
//            module_name: &Identifier,
//            idref: &IdentifierReference,
//        ) -> String {
//            let (module, ty_name) = self.qualified_idref(module_name, idref);
//            color::type_ref_qname(module, ty_name)
//        }
//
//        fn qualified_idref<'a>(
//            &self,
//            module_name: &'a Identifier,
//            idref: &'a IdentifierReference,
//        ) -> (&'a Identifier, &'a Identifier) {
//            match idref {
//                IdentifierReference::Identifier(name) => (module_name, name),
//                IdentifierReference::QualifiedIdentifier(name) => (name.module(), name.member()),
//            }
//        }
//
//        fn value_to_string(&mut self, me: &Value, module_name: &Identifier) -> String {
//            match me {
//                Value::Simple(v) => self.simple_value_to_string(v),
//                Value::ValueConstructor(v) => self.value_constructor_to_string(v, module_name),
//                Value::Mapping(v) => self.mapping_value_to_string(v, module_name),
//                Value::Reference(v) => self.type_reference_to_string(v, module_name),
//                Value::List(v) => self.list_value_to_string(v, module_name),
//            }
//        }
//
//        fn simple_value_to_string(&mut self, me: &SimpleValue) -> String {
//            match me {
//                SimpleValue::Boolean(v) => v.to_string(),
//                SimpleValue::Double(v) => color::format_type_constructor(
//                    stdlib::sdml::MODULE_NAME,
//                    stdlib::sdml::DOUBLE,
//                    v.to_string(),
//                ),
//                SimpleValue::Decimal(v) => color::format_type_constructor(
//                    stdlib::sdml::MODULE_NAME,
//                    stdlib::sdml::DECIMAL,
//                    v.to_string(),
//                ),
//                SimpleValue::Integer(v) => color::format_number(v.to_string()),
//                SimpleValue::Unsigned(v) => color::format_type_constructor(
//                    stdlib::sdml::MODULE_NAME,
//                    stdlib::sdml::UNSIGNED,
//                    v.to_string(),
//                ),
//                SimpleValue::String(v) => color::format_lang_str(v),
//                SimpleValue::IriReference(v) => color::format_url(v),
//                SimpleValue::Binary(_) => todo!(),
//            }
//        }
//
//        fn value_constructor_to_string(
//            &mut self,
//            me: &ValueConstructor,
//            module_name: &Identifier,
//        ) -> String {
//            let (module_name, ty_name) = self.qualified_idref(module_name, me.type_name());
//            color::format_type_constructor(module_name, ty_name, me.value().to_string())
//        }
//
//        fn mapping_value_to_string(
//            &mut self,
//            me: &MappingValue,
//            module_name: &Identifier,
//        ) -> String {
//            format!(
//                "{INDENT_PREDICATE}{}
//{}{}{}
//{INDENT_PREDICATE}{}",
//                start_bnode(),
//                collection_element(predicate_with_value(
//                    stdlib::rdf::MODULE_NAME,
//                    stdlib::rdf::TYPE,
//                    type_ref_qname(stdlib::sdml::MODULE_NAME, stdlib::sdml::CLASS_MAP_TYPE_NAME,),
//                    Separator::Predicate,
//                )),
//                collection_element(predicate_with_value(
//                    stdlib::sdml::MODULE_NAME,
//                    stdlib::sdml::HAS_DOMAIN_VALUE,
//                    self.simple_value_to_string(me.domain()),
//                    Separator::Predicate,
//                )),
//                collection_element(predicate_with_value(
//                    stdlib::sdml::MODULE_NAME,
//                    stdlib::sdml::HAS_RANGE_VALUE,
//                    self.value_to_string(me.range(), module_name),
//                    Separator::None,
//                )),
//                end_bnode(),
//            )
//        }
//
//        fn type_reference_to_string(
//            &mut self,
//            me: &IdentifierReference,
//            module_name: &Identifier,
//        ) -> String {
//            self.qualified_idref_string(module_name, me)
//        }
//
//        fn list_value_to_string(
//            &mut self,
//            me: &SequenceOfValues,
//            module_name: &Identifier,
//        ) -> String {
//            let mut buffer = format!(" {}\n", start_collection());
//
//            for member in me.iter() {
//                let value = match member {
//                    SequenceMember::Simple(v) => self.simple_value_to_string(v),
//                    SequenceMember::ValueConstructor(v) => {
//                        self.value_constructor_to_string(v, module_name)
//                    }
//                    SequenceMember::Reference(v) => self.type_reference_to_string(v, module_name),
//                    SequenceMember::Mapping(v) => self.mapping_value_to_string(v, module_name),
//                };
//                buffer.push_str(&collection_element(value));
//            }
//
//            buffer.push_str(INDENT_PREDICATE);
//            buffer.push_str(&end_collection());
//            buffer
//        }
//    }
//
