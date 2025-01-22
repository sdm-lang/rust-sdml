/*!
One-line description.

More detailed description, with

# Example


 */

use rdftk_core::model::graph::{Graph, PrefixMapping};
use rdftk_core::model::literal::{DataType, Literal};
use rdftk_core::model::statement::{BlankNode, Statement, SubjectNode};
use rdftk_iri::Name;
use sdml_core::model::annotations::{Annotation, AnnotationProperty, HasAnnotations};
use sdml_core::model::constraints::Constraint;
use sdml_core::model::definitions::{
    DatatypeDef, Definition, EntityDef, EnumDef, EventDef, PropertyDef, RdfDef, StructureDef,
    TypeClassDef, UnionDef,
};
use sdml_core::model::identifiers::{Identifier, IdentifierReference};
use sdml_core::model::modules::{HeaderValue, Module, ModuleBody};
use sdml_core::model::values::{
    MappingValue, SequenceMember, SequenceOfValues, SimpleValue, Value, ValueConstructor,
};
use sdml_core::model::{HasBody, HasName, HasNameReference, HasSourceSpan};
use sdml_core::stdlib;
use sdml_core::stdlib::sdml::{
    HAS_DOMAIN_VALUE, HAS_RANGE_VALUE, HAS_SOURCE_LOCATION, LOCATION_END_BYTE, LOCATION_START_BYTE,
    MODULE_NAME, MODULE_URL,
};
use sdml_core::store::ModuleStore;
use sdml_errors::Error as ApiError;
use std::str::FromStr;
use url::Url;

use crate::errors::{missing_base_uri_error, module_not_loaded_error};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Default)]
pub struct RdfGeneratorOptions {
    include_source_location: bool,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

#[inline]
pub fn module_to_graph(module: &Module, cache: &impl ModuleStore) -> Result<Graph, ApiError> {
    module_to_graph_with_options(module, cache, Default::default())
}

pub fn module_to_graph_with_options(
    module: &Module,
    cache: &impl ModuleStore,
    options: RdfGeneratorOptions,
) -> Result<Graph, ApiError> {
    println!("{module:#?}");
    let mappings = PrefixMapping::common().with(
        Name::from_str(MODULE_NAME).unwrap(),
        Url::from_str(MODULE_URL)?,
    );
    let mut graph = Graph::default().with_mappings(mappings);

    add_module_to_graph_with_options(module, cache, &mut graph, options)?;

    Ok(graph)
}

pub fn add_module_to_graph(
    module: &Module,
    cache: &impl ModuleStore,
    graph: &mut Graph,
) -> Result<(), ApiError> {
    add_module_to_graph_with_options(module, cache, graph, Default::default())
}

pub fn add_module_to_graph_with_options(
    module: &Module,
    cache: &impl ModuleStore,
    graph: &mut Graph,
    options: RdfGeneratorOptions,
) -> Result<(), ApiError> {
    if let Some(base_uri) = module.base_uri() {
        let context = Context {
            module_name: module.name().clone(),
            base_uri: base_uri.value().clone(),
            subject_uri: vec![base_uri.value().join(module.name().as_ref())?],
            options,
        };

        graph.insert(Statement::new(
            context.current_subject(),
            rdf_url(stdlib::rdf::TYPE),
            owl_url(stdlib::owl::ONTOLOGY),
        ));
        graph.insert(Statement::new(
            context.current_subject(),
            rdf_url(stdlib::rdf::TYPE),
            sdml_url(stdlib::sdml::MODULE),
        ));

        if let Some(version_uri) = module.version_uri() {
            graph.insert(Statement::new(
                context.current_subject(),
                owl_url(stdlib::owl::VERSION_IRI),
                version_uri.value(),
            ));
        }

        if let Some(version_info) = module.version_info() {
            graph.insert(Statement::new(
                context.current_subject(),
                owl_url(stdlib::owl::VERSION_INFO),
                Literal::plain(version_info.value()),
            ));
        }

        add_source_span(module, &context, cache, graph)?;

        add_module_body(module.body(), &context, cache, graph)
    } else {
        Err(missing_base_uri_error(module.name()))
    }
}

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[allow(dead_code)] // TODO: audit this
#[derive(Debug)]
struct Context {
    module_name: Identifier,
    base_uri: Url,
    subject_uri: Vec<Url>,
    options: RdfGeneratorOptions,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Context {
    fn current_subject(&self) -> Url {
        self.subject_uri
            .first()
            .expect("Error, subject_uri is empty (get)")
            .clone()
    }

    #[allow(dead_code)] // TODO: audit this
    fn push_subject(&mut self, new_subject: Url) {
        self.subject_uri.push(new_subject);
    }

    #[allow(dead_code)] // TODO: audit this
    fn pop_subject(&mut self) -> Url {
        self.subject_uri
            .pop()
            .expect("Error, subject_uri is empty (pop)")
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

#[inline(always)]
fn make_url(base: &str, name: &str) -> Url {
    Url::from_str(&format!("{base}{name}")).unwrap()
}

#[inline(always)]
fn sdml_url(name: &str) -> Url {
    make_url(MODULE_URL, name)
}

#[inline(always)]
fn rdf_url(name: &str) -> Url {
    make_url(stdlib::rdf::MODULE_URL, name)
}

#[inline(always)]
fn owl_url(name: &str) -> Url {
    make_url(stdlib::owl::MODULE_URL, name)
}

fn add_source_span(
    has_span: &impl HasSourceSpan,
    ctx: &Context,
    _: &impl ModuleStore,
    graph: &mut Graph,
) -> Result<(), ApiError> {
    if ctx.options.include_source_location {
        if let Some(span) = has_span.source_span() {
            let blank = BlankNode::generate();
            graph.insert(Statement::new(
                ctx.current_subject(),
                sdml_url(HAS_SOURCE_LOCATION),
                blank.clone(),
            ));
            graph.insert(Statement::new(
                blank.clone(),
                sdml_url(LOCATION_START_BYTE),
                Literal::from(span.start().byte() as u64),
            ));
            graph.insert(Statement::new(
                blank,
                sdml_url(LOCATION_END_BYTE),
                Literal::from(span.end().byte() as u64),
            ));
        }
    }
    Ok(())
}

fn add_module_body(
    body: &ModuleBody,
    ctx: &Context,
    cache: &impl ModuleStore,
    graph: &mut Graph,
) -> Result<(), ApiError> {
    if body.has_annotations() {
        add_annotations(body, ctx, cache, graph)?;
    }

    if body.has_imports() {
        add_imports(body, ctx, cache, graph)?;
    }

    if body.has_definitions() {
        add_definitions(body, ctx, cache, graph)?;
    }

    Ok(())
}

fn add_annotations(
    thing: &impl HasAnnotations,
    ctx: &Context,
    cache: &impl ModuleStore,
    graph: &mut Graph,
) -> Result<(), ApiError> {
    for annotation in thing.annotations() {
        match annotation {
            Annotation::Property(v) => add_annotation_property(v, ctx, cache, graph)?,
            Annotation::Constraint(v) => add_annotation_constraint(v, ctx, cache, graph)?,
        }
    }
    Ok(())
}

fn add_annotation_property(
    property: &AnnotationProperty,
    ctx: &Context,
    cache: &impl ModuleStore,
    graph: &mut Graph,
) -> Result<(), ApiError> {
    if property.has_source_span() {
        add_source_span(property, ctx, cache, graph)?;
    }
    let predicate = identifier_reference_to_url(property.name_reference(), ctx, cache)
        .expect("Url parse error");
    add_values(
        &(ctx.current_subject().into()),
        &predicate,
        property.value(),
        ctx,
        cache,
        graph,
    )
}

fn identifier_reference_to_url(
    name_reference: &IdentifierReference,
    ctx: &Context,
    cache: &impl ModuleStore,
) -> Result<Url, ApiError> {
    match name_reference {
        IdentifierReference::Identifier(ident) => Ok(ctx.base_uri.join(ident.as_ref())?),
        IdentifierReference::QualifiedIdentifier(ident) => {
            if let Some(other_base_uri) = cache.module_name_to_uri(ident.module()) {
                Ok(other_base_uri.join(ident.member().as_ref())?)
            } else {
                Err(module_not_loaded_error(ident.module()))
            }
        }
    }
}

fn add_values(
    subject: &SubjectNode,
    predicate: &Url,
    value: &Value,
    ctx: &Context,
    cache: &impl ModuleStore,
    graph: &mut Graph,
) -> Result<(), ApiError> {
    match value {
        Value::Simple(v) => add_simple_value(subject, predicate, v, graph),
        Value::ValueConstructor(v) => {
            add_value_constructor_value(subject, predicate, v, ctx, cache, graph)
        }
        Value::Mapping(v) => add_mapping_value(subject, predicate, v, ctx, cache, graph),
        Value::Reference(v) => add_name_reference_value(subject, predicate, v, ctx, cache, graph),
        Value::List(vs) => add_sequence_value(subject, predicate, vs, ctx, cache, graph),
    }?;

    Ok(())
}

fn add_sequence_value(
    subject: &SubjectNode,
    predicate: &Url,
    value: &SequenceOfValues,
    ctx: &Context,
    cache: &impl ModuleStore,
    graph: &mut Graph,
) -> Result<(), ApiError> {
    let sequence = BlankNode::generate();
    graph.insert(Statement::new(
        subject.clone(),
        predicate.clone(),
        &sequence,
    ));
    graph.insert(Statement::new(
        &sequence,
        rdf_url(stdlib::rdf::TYPE),
        &sequence,
    ));

    let sequence = SubjectNode::from(sequence);

    for value in value.iter() {
        match value {
            SequenceMember::Simple(v) => add_simple_value(&sequence, predicate, v, graph)?,
            SequenceMember::ValueConstructor(v) => {
                add_value_constructor_value(&sequence, predicate, v, ctx, cache, graph)?
            }
            SequenceMember::Reference(v) => {
                add_name_reference_value(&sequence, predicate, v, ctx, cache, graph)?
            }
            SequenceMember::Mapping(v) => {
                add_mapping_value(&sequence, predicate, v, ctx, cache, graph)?
            }
        };
    }

    Ok(())
}

fn add_value_constructor_value(
    subject: &SubjectNode,
    predicate: &Url,
    value: &ValueConstructor,
    ctx: &Context,
    cache: &impl ModuleStore,
    graph: &mut Graph,
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
        identifier_reference_to_url(value.type_name(), ctx, cache).expect("Url parse error");
    graph.insert(Statement::new(
        subject.clone(),
        predicate.clone(),
        Literal::with_data_type(lexical_form, DataType::Other(data_type)),
    ));
    Ok(())
}

fn add_name_reference_value(
    subject: &SubjectNode,
    predicate: &Url,
    value: &IdentifierReference,
    ctx: &Context,
    cache: &impl ModuleStore,
    graph: &mut Graph,
) -> Result<(), ApiError> {
    graph.insert(Statement::new(
        subject.clone(),
        predicate.clone(),
        identifier_reference_to_url(value, ctx, cache).expect("Url parse error"),
    ));
    Ok(())
}

fn add_mapping_value(
    subject: &SubjectNode,
    predicate: &Url,
    value: &MappingValue,
    ctx: &Context,
    cache: &impl ModuleStore,
    graph: &mut Graph,
) -> Result<(), ApiError> {
    let mapping = BlankNode::generate();
    graph.insert(Statement::new(subject.clone(), predicate.clone(), &mapping));
    graph.insert(Statement::new(
        &mapping,
        rdf_url(stdlib::rdf::TYPE),
        sdml_url(stdlib::sdml::CLASS_MAP_TYPE_NAME),
    ));

    let mapping = SubjectNode::from(mapping);

    add_simple_value(&mapping, &sdml_url(HAS_DOMAIN_VALUE), value.domain(), graph)?;
    add_values(
        &mapping,
        &sdml_url(HAS_RANGE_VALUE),
        value.range(),
        ctx,
        cache,
        graph,
    )?;

    Ok(())
}

fn add_simple_value(
    subject: &SubjectNode,
    predicate: &Url,
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
        SimpleValue::Binary(v) => Literal::hex_encoded(v.as_bytes()),
    };

    graph.insert(Statement::new(subject.clone(), predicate.clone(), object));

    Ok(())
}

fn add_annotation_constraint(
    _constraint: &Constraint,
    _ctx: &Context,
    _cache: &impl ModuleStore,
    _graph: &mut Graph,
) -> Result<(), ApiError> {
    Ok(())
}

fn add_imports(
    module: &ModuleBody,
    ctx: &Context,
    cache: &impl ModuleStore,
    graph: &mut Graph,
) -> Result<(), ApiError> {
    for (name, version) in module.imported_module_versions() {
        add_an_import(name, version, ctx, cache, graph)?;
    }
    Ok(())
}

fn add_an_import(
    name: &Identifier,
    version: Option<&HeaderValue<Url>>,
    ctx: &Context,
    cache: &impl ModuleStore,
    graph: &mut Graph,
) -> Result<(), ApiError> {
    if let Some(actual) = cache.get(name) {
        if let Some(version_uri) = version {
            graph.insert(Statement::new(
                ctx.current_subject(),
                owl_url(stdlib::owl::IMPORTS),
                version_uri.value(),
            ));
        } else if let Some(base_uri) = actual.base_uri() {
            graph.insert(Statement::new(
                ctx.current_subject(),
                owl_url(stdlib::owl::IMPORTS),
                base_uri.value(),
            ));
        } else {
            panic!("missing_base_uri");
        }
    } else {
        panic!("module_not_loaded: {name}");
    }
    Ok(())
}

fn add_definitions(
    module: &ModuleBody,
    ctx: &Context,
    cache: &impl ModuleStore,
    graph: &mut Graph,
) -> Result<(), ApiError> {
    for definition in module.definitions() {
        match definition {
            Definition::Datatype(v) => add_datatype_def(v, ctx, cache, graph),
            Definition::Entity(v) => add_entity_def(v, ctx, cache, graph),
            Definition::Enum(v) => add_enum_def(v, ctx, cache, graph),
            Definition::Event(v) => add_event_def(v, ctx, cache, graph),
            Definition::Property(v) => add_property_def(v, ctx, cache, graph),
            Definition::Rdf(v) => add_rdf_def(v, ctx, cache, graph),
            Definition::Structure(v) => add_structure_def(v, ctx, cache, graph),
            Definition::TypeClass(v) => add_type_class_def(v, ctx, cache, graph),
            Definition::Union(v) => add_union_def(v, ctx, cache, graph),
        }?;
    }
    Ok(())
}

fn add_datatype_def(
    _defn: &DatatypeDef,
    _ctx: &Context,
    _cache: &impl ModuleStore,
    _graph: &mut Graph,
) -> Result<(), ApiError> {
    Ok(())
}

fn add_entity_def(
    _defn: &EntityDef,
    _ctx: &Context,
    _cache: &impl ModuleStore,
    _graph: &mut Graph,
) -> Result<(), ApiError> {
    Ok(())
}

fn add_enum_def(
    _defn: &EnumDef,
    _ctx: &Context,
    _cache: &impl ModuleStore,
    _graph: &mut Graph,
) -> Result<(), ApiError> {
    Ok(())
}

fn add_event_def(
    _defn: &EventDef,
    _ctx: &Context,
    _cache: &impl ModuleStore,
    _graph: &mut Graph,
) -> Result<(), ApiError> {
    Ok(())
}

fn add_property_def(
    _defn: &PropertyDef,
    _ctx: &Context,
    _cache: &impl ModuleStore,
    _graph: &mut Graph,
) -> Result<(), ApiError> {
    Ok(())
}

fn add_rdf_def(
    _defn: &RdfDef,
    _ctx: &Context,
    _cache: &impl ModuleStore,
    _graph: &mut Graph,
) -> Result<(), ApiError> {
    Ok(())
}

fn add_structure_def(
    _defn: &StructureDef,
    _ctx: &Context,
    _cache: &impl ModuleStore,
    _graph: &mut Graph,
) -> Result<(), ApiError> {
    Ok(())
}

fn add_type_class_def(
    _defn: &TypeClassDef,
    _ctx: &Context,
    _cache: &impl ModuleStore,
    _graph: &mut Graph,
) -> Result<(), ApiError> {
    Ok(())
}

fn add_union_def(
    _defn: &UnionDef,
    _ctx: &Context,
    _cache: &impl ModuleStore,
    _graph: &mut Graph,
) -> Result<(), ApiError> {
    Ok(())
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

//mod old_gen {
//    /*!
//    This module provides a generator that creates the RDF representation of a module given its
//    in-memory representation.
//
//    */
//
//    use crate::{
//        color::rdf::{
//            self as color, bnode_predicate_with_value, collection_element, end_bnode,
//            end_collection, format_str, format_type_constructor, format_url, module_ref_qname,
//            module_subject, mv_name, predicate_no_value, predicate_qname, predicate_with_value,
//            predicate_with_value_list, property_subject, start_bnode, start_collection,
//            thing_qname, thing_subject, type_ref_qname, type_subject, Separator, INDENT_PREDICATE,
//        },
//        Generator,
//    };
//    use sdml_core::{
//        error::Error,
//        model::{
//            annotations::{Annotation, AnnotationProperty, HasAnnotations},
//            constraints::Constraint,
//            definitions::{
//                DatatypeDef, Definition, EntityDef, EnumDef, EventDef, HasMembers, HasVariants,
//                PropertyDef, RdfDef, StructureDef, TypeClassDef, TypeVariant, UnionDef,
//                ValueVariant,
//            },
//            identifiers::{Identifier, IdentifierReference},
//            members::TypeReference,
//            members::{Member, Ordering, Uniqueness, DEFAULT_CARDINALITY},
//            modules::Module,
//            values::{
//                MappingValue, SequenceMember, SequenceOfValues, SimpleValue, Value,
//                ValueConstructor,
//            },
//            HasBody, HasName, HasNameReference, HasOptionalBody,
//        },
//        stdlib,
//        store::ModuleStore,
//    };
//    use std::{fmt::Display, io::Write, path::PathBuf};
//    use tracing::info;
//
//    // ------------------------------------------------------------------------------------------------
//    // Public Types
//    // ------------------------------------------------------------------------------------------------
//
//    ///
//    /// Generator for the formal RDF representation of a module.
//    ///
//    #[derive(Debug, Default)]
//    pub struct RdfModelGenerator {
//        options: RdfModelOptions,
//    }
//    #[derive(Debug, Copy, Clone, Default)]
//    pub struct RdfModelOptions {
//        repr: RdfRepresentation,
//    }
//
//    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
//    pub enum RdfRepresentation {
//        NTriples,
//        Turtle,
//    }
//
//    // ------------------------------------------------------------------------------------------------
//    // Private Macros
//    // ------------------------------------------------------------------------------------------------
//
//    macro_rules! write_annotations {
//        ($self:expr, $anns:expr, $module_name:expr, $writer:expr) => {
//            for annotation in $anns {
//                match &annotation {
//                    Annotation::Property(me) => {
//                        $self.write_annotation_property(me, $module_name, $writer)?
//                    }
//                    Annotation::Constraint(me) => {
//                        $self.write_constraint(me, $module_name, $writer)?
//                    }
//                }
//            }
//        };
//    }
//
//    // ------------------------------------------------------------------------------------------------
//    // Implementations
//    // ------------------------------------------------------------------------------------------------
//
//    impl Generator for RdfModelGenerator {
//        type Options = RdfModelOptions;
//
//        fn generate_with_options<W>(
//            &mut self,
//            module: &Module,
//            cache: &impl ModuleStore,
//            options: Self::Options,
//            _: Option<PathBuf>,
//            writer: &mut W,
//        ) -> Result<(), Error>
//        where
//            W: Write + Sized,
//        {
//            self.options = options;
//            info!("Generating RDF {}", self.options.repr);
//
//            let module_name = module.name();
//
//            if let Some(base) = module.base_uri() {
//                writer.write_all(color::base_directive(base.as_ref().as_str()).as_bytes())?;
//                writer.write_all(
//                    color::prefix_directive(module_name.as_ref(), base.as_ref().as_str())
//                        .as_bytes(),
//                )?;
//            }
//
//            let body = module.body();
//            let mut imported_modules = body.imported_modules();
//
//            if !imported_modules.contains(&Identifier::new_unchecked(stdlib::owl::MODULE_NAME)) {
//                writer.write_all(
//                    color::prefix_directive(stdlib::owl::MODULE_NAME, stdlib::owl::MODULE_URL)
//                        .as_bytes(),
//                )?;
//            }
//            if !imported_modules.contains(&Identifier::new_unchecked(stdlib::rdf::MODULE_NAME)) {
//                writer.write_all(
//                    color::prefix_directive(stdlib::rdf::MODULE_NAME, stdlib::rdf::MODULE_URL)
//                        .as_bytes(),
//                )?;
//            }
//            if !imported_modules.contains(&Identifier::new_unchecked(stdlib::rdfs::MODULE_NAME)) {
//                writer.write_all(
//                    color::prefix_directive(stdlib::rdfs::MODULE_NAME, stdlib::rdfs::MODULE_URL)
//                        .as_bytes(),
//                )?;
//            }
//            if !imported_modules.contains(&Identifier::new_unchecked(stdlib::sdml::MODULE_NAME)) {
//                writer.write_all(
//                    color::prefix_directive(stdlib::sdml::MODULE_NAME, stdlib::sdml::MODULE_URL)
//                        .as_bytes(),
//                )?;
//            }
//
//            for import in &imported_modules {
//                if let Some(uri) = cache.module_name_to_uri(import) {
//                    writer.write_all(
//                        color::prefix_directive(import.as_ref(), uri.as_str()).as_bytes(),
//                    )?;
//                }
//            }
//
//            writer.write_all(b"\n")?;
//
//            writer.write_all(module_subject(module_name).as_bytes())?;
//            writer.write_all(
//                predicate_with_value_list(
//                    stdlib::rdf::MODULE_NAME,
//                    stdlib::rdf::TYPE,
//                    &[
//                        color::type_ref_qname(stdlib::owl::MODULE_NAME, stdlib::owl::ONTOLOGY),
//                        color::type_ref_qname(stdlib::sdml::MODULE_NAME, stdlib::sdml::MODULE),
//                    ],
//                    Separator::Predicate,
//                )
//                .as_bytes(),
//            )?;
//            if let Some(version_info) = module.version_info() {
//                writer.write_all(
//                    color::predicate_with_value(
//                        stdlib::owl::MODULE_NAME,
//                        stdlib::owl::VERSION_INFO,
//                        format_str(format!("{:?}", version_info.as_ref())),
//                        Separator::Predicate,
//                    )
//                    .as_bytes(),
//                )?;
//            }
//            if let Some(version_uri) = module.version_uri() {
//                writer.write_all(
//                    color::predicate_with_value(
//                        stdlib::owl::MODULE_NAME,
//                        stdlib::owl::VERSION_IRI,
//                        format_url(version_uri.as_ref()),
//                        Separator::Predicate,
//                    )
//                    .as_bytes(),
//                )?;
//            }
//
//            imported_modules.remove(&Identifier::new_unchecked(stdlib::owl::MODULE_NAME));
//            imported_modules.remove(&Identifier::new_unchecked(stdlib::rdf::MODULE_NAME));
//            imported_modules.remove(&Identifier::new_unchecked(stdlib::rdfs::MODULE_NAME));
//            imported_modules.remove(&Identifier::new_unchecked(stdlib::xsd::MODULE_NAME));
//            for import in &imported_modules {
//                if let Some(url) = cache.module_name_to_uri(import) {
//                    writer.write_all(
//                        color::predicate_with_value(
//                            stdlib::owl::MODULE_NAME,
//                            stdlib::owl::IMPORTS,
//                            format_url(url),
//                            Separator::Predicate,
//                        )
//                        .as_bytes(),
//                    )?;
//                }
//            }
//
//            write_annotations!(self, body.annotations(), module_name, writer);
//
//            writer.write_all(
//                predicate_with_value(
//                    stdlib::sdml::MODULE_NAME,
//                    stdlib::sdml::SRC_LABEL,
//                    color::format_str(format!("{:?}", module_name.as_ref())),
//                    Separator::Statement,
//                )
//                .as_bytes(),
//            )?;
//            writer.write_all(b"\n")?;
//
//            for definition in body.definitions() {
//                match &definition {
//                    Definition::Datatype(v) => self.write_datatype(v, module_name, writer)?,
//                    Definition::Entity(v) => self.write_entity(v, module_name, writer)?,
//                    Definition::Enum(v) => self.write_enumeration(v, module_name, writer)?,
//                    Definition::Event(v) => self.write_event(v, module_name, writer)?,
//                    Definition::Property(v) => self.write_property(v, module_name, writer)?,
//                    Definition::Rdf(v) => self.write_rdf(v, module_name, writer)?,
//                    Definition::Structure(v) => self.write_structure(v, module_name, writer)?,
//                    Definition::TypeClass(v) => self.write_type_class(v, module_name, writer)?,
//                    Definition::Union(v) => self.write_union(v, module_name, writer)?,
//                }
//            }
//
//            Ok(())
//        }
//    }
//
//    impl RdfModelGenerator {
//        fn write_datatype(
//            &mut self,
//            me: &DatatypeDef,
//            module_name: &Identifier,
//            writer: &mut dyn Write,
//        ) -> Result<(), Error> {
//            let name = me.name();
//
//            writer.write_all(type_subject(module_name, name).as_bytes())?;
//            writer.write_all(
//                predicate_with_value(
//                    stdlib::rdf::MODULE_NAME,
//                    stdlib::rdf::TYPE,
//                    type_ref_qname(stdlib::rdfs::MODULE_NAME, stdlib::rdfs::DATATYPE),
//                    Separator::Predicate,
//                )
//                .as_bytes(),
//            )?;
//
//            let (base_module, base_type) = self.qualified_idref(module_name, me.base_type());
//            writer.write_all(
//                predicate_with_value(
//                    stdlib::owl::MODULE_NAME,
//                    stdlib::owl::ON_DATATYPE,
//                    type_ref_qname(base_module, base_type),
//                    Separator::Predicate,
//                )
//                .as_bytes(),
//            )?;
//
//            if let Some(body) = me.body() {
//                let (facets, other): (Vec<_>, Vec<_>) = body.annotations().partition(|ann| {
//                    if let Annotation::Property(prop) = ann {
//                        prop.is_datatype_facet()
//                    } else {
//                        false
//                    }
//                });
//
//                if !facets.is_empty() {
//                    writer.write_all(
//                        format!(
//                            "{} {}{}",
//                            predicate_no_value(
//                                stdlib::owl::MODULE_NAME,
//                                stdlib::owl::WITH_RESTRICTIONS,
//                                Separator::InlineNone,
//                            ),
//                            start_collection(),
//                            Separator::None
//                        )
//                        .as_bytes(),
//                    )?;
//                    let last_facet = facets.len() - 1;
//                    for (i, facet) in facets.iter().enumerate() {
//                        if let Some(facet) = facet.as_annotation_property() {
//                            self.write_facet_property(
//                                facet,
//                                module_name,
//                                if i < last_facet {
//                                    Separator::Object
//                                } else {
//                                    Separator::None
//                                },
//                                writer,
//                            )?;
//                        } else {
//                            unreachable!()
//                        }
//                    }
//                    writer.write_all(
//                        format!(
//                            "{INDENT_PREDICATE}{}{}",
//                            end_collection(),
//                            Separator::Predicate
//                        )
//                        .as_bytes(),
//                    )?;
//                }
//
//                write_annotations!(self, other.iter(), module_name, writer);
//            }
//
//            self.write_defn_end(module_name, name, writer)?;
//
//            Ok(())
//        }
//
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
//    // ------------------------------------------------------------------------------------------------
//
//    impl RdfModelOptions {
//        pub fn as_representation(self, repr: RdfRepresentation) -> Self {
//            Self { repr }
//        }
//
//        pub fn as_ntriples(self) -> Self {
//            self.as_representation(RdfRepresentation::NTriples)
//        }
//
//        pub fn as_turtle(self) -> Self {
//            self.as_representation(RdfRepresentation::Turtle)
//        }
//    }
//
//    // ------------------------------------------------------------------------------------------------
//
//    impl Default for RdfRepresentation {
//        fn default() -> Self {
//            Self::Turtle
//        }
//    }
//
//    impl Display for RdfRepresentation {
//        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//            write!(
//                f,
//                "{}",
//                match (self, f.alternate()) {
//                    (Self::NTriples, false) => "NTriples",
//                    (Self::NTriples, true) => "nt",
//                    (Self::Turtle, false) => "Turtle",
//                    (Self::Turtle, true) => "ttl",
//                }
//            )
//        }
//    }
//}
