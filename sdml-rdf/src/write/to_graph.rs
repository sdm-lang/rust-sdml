/*!
One-line description.

More detailed description, with

# Example

 */

use crate::{
    errors::{missing_base_uri_error, module_not_loaded_error},
    write::Options,
};
use rdftk_core::model::{
    graph::{Graph, PrefixMapping},
    literal::{DataType, Literal},
    statement::{BlankNode, Statement, SubjectNode},
};
use rdftk_iri::{Iri, IriExtra, Name};
use sdml_core::{
    model::{
        annotations::{Annotation, AnnotationProperty, HasAnnotations},
        constraints::{Constraint, ConstraintBody, ControlledLanguageString, FormalConstraint},
        definitions::{
            DatatypeDef, Definition, DimensionDef, DimensionIdentity, DimensionParent, EntityDef,
            EnumDef, EventDef, PropertyDef, RdfDef, SourceEntity, StructureDef, TypeClassDef,
            TypeVariant, UnionDef, ValueVariant,
        },
        identifiers::{Identifier, IdentifierReference},
        members::{Cardinality, Member, MemberDef, MemberKind, TypeReference},
        modules::{HeaderValue, Module},
        values::{
            MappingValue, SequenceMember, SequenceOfValues, SimpleValue, Value, ValueConstructor,
        },
        HasBody, HasName, HasNameReference, HasOptionalBody, HasSourceSpan, Span,
    },
    stdlib::{owl, rdf, rdfs, sdml},
    store::ModuleStore,
};
use sdml_errors::Error as ApiError;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct Context {
    base_uri: Iri,
    subject: Vec<SubjectNode>,
    options: Options,
}

pub trait ToGraph {
    fn to_graph(&self, ctx: &mut Context, cache: &impl ModuleStore) -> Result<Graph, ApiError> {
        let mappings = if let Some(mappings) = &ctx.options.mappings {
            mappings.clone()
        } else {
            PrefixMapping::common().with_dc_elements()
        }
        .with_default(ctx.base_uri().clone())
        .with(
            Name::from_str(sdml::MODULE_NAME).expect("TODO: convert to Error"),
            Iri::parse(sdml::MODULE_URL)?,
        );
        let mut graph = Graph::default().with_mappings(mappings);
        self.add_to_graph(&mut graph, ctx, cache)?;
        Ok(graph)
    }

    fn add_to_graph(
        &self,
        graph: &mut Graph,
        ctx: &mut Context,
        cache: &impl ModuleStore,
    ) -> Result<(), ApiError>;
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

trait ToGraphAsObject {
    fn add_to_graph(
        &self,
        graph: &mut Graph,
        ctx: &mut Context,
        cache: &impl ModuleStore,
        subject: &SubjectNode,
        predicate: &Iri,
    ) -> Result<(), ApiError>;
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Context
// ------------------------------------------------------------------------------------------------

impl Context {
    pub fn new(base_uri: Iri) -> Self {
        Self {
            base_uri,
            subject: Default::default(),
            options: Default::default(),
        }
    }

    pub fn from(module: &Module) -> Result<Self, ApiError> {
        if let Some(base_uri) = module.base_uri() {
            Ok(Self::new(base_uri.value().clone()))
        } else {
            Err(missing_base_uri_error(module.name()))
        }
    }

    pub fn with_options(self, options: Options) -> Self {
        let mut self_mut = self;
        self_mut.options = options;
        self_mut
    }

    pub fn base_uri(&self) -> &Iri {
        &self.base_uri
    }

    pub fn options(&self) -> &Options {
        &self.options
    }

    fn current_subject(&self) -> &SubjectNode {
        self.subject.last().expect("Error, subject is empty (get)")
    }

    fn has_current_subject(&self) -> bool {
        !self.subject.is_empty()
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
// Implementations ❱ Span
// ------------------------------------------------------------------------------------------------

///
/// # Example
///
/// ```ttl
/// $subject
///     sdml:sourceLocation _:B01 .
/// _:B01
///     sdml:startByte 101 ;
///     sdml:endByte 105 .
/// ```
///
impl ToGraph for Span {
    fn add_to_graph(
        &self,
        graph: &mut Graph,
        ctx: &mut Context,
        _cache: &impl ModuleStore,
    ) -> Result<(), ApiError> {
        if ctx.options.include_source_location {
            let blank = BlankNode::generate();
            g_insert!(graph ; ctx =>  mkiri!(sdml:SOURCE_LOCATION), blank.clone());
            g_insert!(graph ; blank.clone(), mkiri!(sdml:START_BYTE), Literal::from(self.start() as u64));
            g_insert!(graph ; blank, mkiri!(sdml:END_BYTE), Literal::from(self.end() as u64));
        }
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Module
// ------------------------------------------------------------------------------------------------

///
/// # Example
///
/// ```ttl
/// $subject
///     sdml:sourceLocation _:B01 ; # See Span
///     rdf:type owl:Ontology, sdml:Module ;
///     sdml:srcLabel "$subject.name" ;
///     owl:versionIri <$subject.versionIri> ; # optional
///     owl:versionInfo "$subject.versionInfo" ; # optional
///     # annotations
///     # imports
///     # definitions
/// .
/// ```
///
impl ToGraph for Module {
    fn add_to_graph(
        &self,
        graph: &mut Graph,
        ctx: &mut Context,
        cache: &impl ModuleStore,
    ) -> Result<(), ApiError> {
        set_current_context!(self, ctx);
        add_source_span!(self, graph, ctx, cache);

        g_insert!(graph ; ctx => rdf:type, mkiri!(owl:ONTOLOGY));
        g_insert!(graph ; ctx => rdf:type, mkiri!(sdml:MODULE));

        g_insert!(graph ; ctx => sdml:srcLabel, self.name());

        if let Some(version_uri) = self.version_uri() {
            g_insert!(graph ; ctx => mkiri!(owl:VERSION_IRI), version_uri.value());
        }

        if let Some(version_info) = self.version_info() {
            g_insert!(graph ; ctx => mkiri!(owl:VERSION_INFO), Literal::plain(version_info.value()));
        }

        add_annotations!(self, graph, ctx, cache);

        for name_and_version in self.imported_module_versions() {
            name_and_version.add_to_graph(graph, ctx, cache)?;
        }

        for defn in self.definitions() {
            defn.add_to_graph(graph, ctx, cache)?;
        }

        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Annotations
// ------------------------------------------------------------------------------------------------

impl ToGraph for Annotation {
    fn add_to_graph(
        &self,
        graph: &mut Graph,
        ctx: &mut Context,
        cache: &impl ModuleStore,
    ) -> Result<(), ApiError> {
        match self {
            Annotation::Property(property) => property.add_to_graph(graph, ctx, cache),
            Annotation::Constraint(constraint) => constraint.add_to_graph(graph, ctx, cache),
        }
    }
}

///
/// # Example
///
/// ```ttl
/// $subject
///     <$subject.identifier_reference> # idref resolved to IRI
///     $subject.value # value in relevant representation
/// .
/// ```
///
impl ToGraph for AnnotationProperty {
    fn add_to_graph(
        &self,
        graph: &mut Graph,
        ctx: &mut Context,
        cache: &impl ModuleStore,
    ) -> Result<(), ApiError> {
        let subject = ctx.current_subject().clone();
        let predicate = identifier_reference_to_url(self.name_reference(), ctx, cache)
            .expect("Iri parse error");
        self.value()
            .add_to_graph(graph, ctx, cache, &subject, &predicate)
    }
}

///
/// # Example
///
/// ```ttl
/// $subject
///     sdml:hasConstraint _:B01 .
/// _:B01
///     sdml:sourceLocation _:B02 ; # See Span
///     sdml:srcLabel "$subject.name" ;
///     $informal_or_formal ;
/// .
/// ```
///
impl ToGraph for Constraint {
    fn add_to_graph(
        &self,
        graph: &mut Graph,
        ctx: &mut Context,
        cache: &impl ModuleStore,
    ) -> Result<(), ApiError> {
        add_source_span!(self, graph, ctx, cache);
        let subject = ctx.current_subject();
        let constraint_node = BlankNode::generate();
        g_insert!(graph ;
            subject,
            mkiri!(sdml:HAS_CONSTRAINT),
            &constraint_node
        );
        let name = Literal::from(self.name().to_string());
        g_insert!(graph ; &constraint_node, mkiri!(sdml:SRC_LABEL), name);

        ctx.push_subject(constraint_node);

        match self.body() {
            ConstraintBody::Informal(v) => v.add_to_graph(graph, ctx, cache)?,
            ConstraintBody::Formal(v) => v.add_to_graph(graph, ctx, cache)?,
        };

        ctx.pop_subject();

        Ok(())
    }
}

///
/// # Example
///
/// ```ttl
/// $subject
///     rdf:type sdml:Constraint, sdml:InformalConstraint ;
///     rdf:value "$subject.value" ;
///     sdml:controlledLanguage "$subject.language" # Optional
/// .
/// ```
///
impl ToGraph for ControlledLanguageString {
    fn add_to_graph(
        &self,
        graph: &mut Graph,
        ctx: &mut Context,
        _cache: &impl ModuleStore,
    ) -> Result<(), ApiError> {
        let subject = ctx.current_subject();
        g_insert!(graph ; ctx => rdf:type, mkiri!(sdml:CONSTRAINT));
        g_insert!(graph ; ctx => rdf:type, mkiri!(sdml:INFORMAL_CONSTRAINT));
        g_insert!(graph ; ctx => rdf:value, Literal::plain(self.value()));
        if let Some(language) = self.language() {
            g_insert!(graph ; subject, mkiri!(sdml:CONTROLLED_LANG_STRING), Literal::plain(language.to_string()));
        }
        Ok(())
    }
}

///
/// # Example
///
/// ```ttl
/// $subject
///     rdf:type sdml:Constraint, sdml:FormalConstraint ;
///     # TBD
/// .
/// ```
///
impl ToGraph for FormalConstraint {
    fn add_to_graph(
        &self,
        graph: &mut Graph,
        ctx: &mut Context,
        _cache: &impl ModuleStore,
    ) -> Result<(), ApiError> {
        g_insert!(graph ; ctx => rdf:type, mkiri!(sdml:CONSTRAINT));
        g_insert!(graph ; ctx => rdf:type, mkiri!(sdml:FORMAL_CONSTRAINT));
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Definitions
// ------------------------------------------------------------------------------------------------

impl ToGraph for Definition {
    fn add_to_graph(
        &self,
        graph: &mut Graph,
        ctx: &mut Context,
        cache: &impl ModuleStore,
    ) -> Result<(), ApiError> {
        match self {
            Definition::Datatype(v) => v.add_to_graph(graph, ctx, cache),
            Definition::Dimension(v) => v.add_to_graph(graph, ctx, cache),
            Definition::Entity(v) => v.add_to_graph(graph, ctx, cache),
            Definition::Enum(v) => v.add_to_graph(graph, ctx, cache),
            Definition::Event(v) => v.add_to_graph(graph, ctx, cache),
            Definition::Property(v) => v.add_to_graph(graph, ctx, cache),
            Definition::Rdf(v) => v.add_to_graph(graph, ctx, cache),
            Definition::Structure(v) => v.add_to_graph(graph, ctx, cache),
            Definition::TypeClass(v) => v.add_to_graph(graph, ctx, cache),
            Definition::Union(v) => v.add_to_graph(graph, ctx, cache),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Definitions ❱ Datatype
// ------------------------------------------------------------------------------------------------

///
/// # Example
///
/// ```ttl
/// $parent sdml:hasDefinition $subject .
///
/// $subject
///     sdml:sourceLocation _:B01 ; # See Span
///     rdf:type sdml:Datatype, rdfs:Datatype ;
///     sdml:srcLabel "$subject.name" ;
///     # annotations
/// .
/// ```
///
impl ToGraph for DatatypeDef {
    fn add_to_graph(
        &self,
        graph: &mut Graph,
        ctx: &mut Context,
        cache: &impl ModuleStore,
    ) -> Result<(), ApiError> {
        defn_common!(self, ctx, graph, sdml::DATATYPE);
        g_insert!(graph ;
            ctx =>
            rdf:type,
            mkiri!(rdfs:DATATYPE)
        );
        add_source_span!(self, graph, ctx, cache);

        // As a restriction on the following type...
        let base_uri =
            { identifier_reference_to_url(self.base_type(), ctx, cache).expect("Iri parse error") };
        g_insert!(graph ;
            ctx =>
            mkiri!(owl:ON_DATATYPE),
            base_uri
        );

        // With these facet restrictions...
        // TODO: defn.restrictions()

        if let Some(body) = self.body() {
            add_annotations!(body, graph, ctx, cache);
        }

        ctx.pop_subject();
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Definitions ❱ Dimension
// ------------------------------------------------------------------------------------------------

///
/// # Example
///
/// ```ttl
/// $parent sdml:hasDefinition $subject .
///
/// $subject
///     sdml:sourceLocation _:B01 ; # See Span
///     rdf:type sdml:Dimension ;
///     sdml:srcLabel "$subject.name" ;
///     # annotations
///     # source entity
///     # parents
///     # members
/// .
/// ```
///
impl ToGraph for DimensionDef {
    fn add_to_graph(
        &self,
        graph: &mut Graph,
        ctx: &mut Context,
        cache: &impl ModuleStore,
    ) -> Result<(), ApiError> {
        defn_common!(self, ctx, graph, sdml::DIMENSION);
        add_source_span!(self, graph, ctx, cache);

        if let Some(body) = self.body() {
            add_annotations!(body, graph, ctx, cache);

            match body.identity() {
                DimensionIdentity::Source(v) => v.add_to_graph(graph, ctx, cache)?,
                DimensionIdentity::Identity(v) => v.add_to_graph(graph, ctx, cache)?,
            }

            for parent in body.parents() {
                parent.add_to_graph(graph, ctx, cache)?;
            }

            for member in body.members() {
                member.add_to_graph(graph, ctx, cache)?;
            }
        }

        ctx.pop_subject();
        Ok(())
    }
}

impl ToGraph for DimensionParent {
    fn add_to_graph(
        &self,
        _graph: &mut Graph,
        _ctx: &mut Context,
        _cache: &impl ModuleStore,
    ) -> Result<(), ApiError> {
        todo!()
    }
}

impl ToGraph for SourceEntity {
    fn add_to_graph(
        &self,
        _graph: &mut Graph,
        _ctx: &mut Context,
        _cache: &impl ModuleStore,
    ) -> Result<(), ApiError> {
        todo!()
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Definitions ❱ Entity
// ------------------------------------------------------------------------------------------------

///
/// # Example
///
/// ```ttl
/// $parent sdml:hasDefinition $subject .
///
/// $subject
///     sdml:sourceLocation _:B01 ; # See Span
///     rdf:type sdml:Entity ;
///     sdml:srcLabel "$subject.name" ;
///     # annotations
///     # identity
///     # members
/// .
/// ```
///
impl ToGraph for EntityDef {
    fn add_to_graph(
        &self,
        graph: &mut Graph,
        ctx: &mut Context,
        cache: &impl ModuleStore,
    ) -> Result<(), ApiError> {
        defn_common!(self, ctx, graph, sdml::ENTITY);
        add_source_span!(self, graph, ctx, cache);

        if let Some(body) = self.body() {
            add_annotations!(body, graph, ctx, cache);

            body.identity().add_to_graph(graph, ctx, cache)?;

            for member in body.members() {
                member.add_to_graph(graph, ctx, cache)?;
            }
        }

        ctx.pop_subject();
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Definitions ❱ Enum
// ------------------------------------------------------------------------------------------------

///
/// # Example
///
/// ```ttl
/// $parent sdml:hasDefinition $subject .
///
/// $subject
///     sdml:sourceLocation _:B01 ; # See Span
///     rdf:type sdml:Enum ;
///     sdml:srcLabel "$subject.name" ;
///     # annotations
///     # variants -- see ValueVariant
/// .
/// ```
///
impl ToGraph for EnumDef {
    fn add_to_graph(
        &self,
        graph: &mut Graph,
        ctx: &mut Context,
        cache: &impl ModuleStore,
    ) -> Result<(), ApiError> {
        defn_common!(self, ctx, graph, sdml::ENUMERATION);
        add_source_span!(self, graph, ctx, cache);

        if let Some(body) = self.body() {
            add_annotations!(body, graph, ctx, cache);

            for variant in body.variants() {
                variant.add_to_graph(graph, ctx, cache)?;
            }
        }

        ctx.pop_subject();
        Ok(())
    }
}

///
/// # Example
///
/// ```ttl
/// $parent sdml:hasValueVariant $subject .
///
/// $subject
///     sdml:sourceLocation _:B01 ; # See Span
///     rdf:type sdml:ValueVariant ;
///     sdml:srcLabel "$subject.name" ;
///     # annotations
/// .
/// ```
///
impl ToGraph for ValueVariant {
    fn add_to_graph(
        &self,
        graph: &mut Graph,
        ctx: &mut Context,
        cache: &impl ModuleStore,
    ) -> Result<(), ApiError> {
        defn_common!(
            self,
            ctx,
            graph,
            sdml::VALUE_VARIANT,
            sdml::HAS_VALUE_VARIANT
        );
        add_source_span!(self, graph, ctx, cache);

        if let Some(body) = self.body() {
            add_annotations!(body, graph, ctx, cache);
        }

        ctx.pop_subject();
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Definitions ❱ Event
// ------------------------------------------------------------------------------------------------

///
/// # Example
///
/// ```ttl
/// $parent sdml:hasDefinition $subject .
///
/// $subject
///     sdml:sourceLocation _:B01 ; # See Span
///     rdf:type sdml:Event ;
///     sdml:srcLabel "$subject.name" ;
///     # annotations
///     # source entity
///     # members
/// .
/// ```
///
impl ToGraph for EventDef {
    fn add_to_graph(
        &self,
        graph: &mut Graph,
        ctx: &mut Context,
        cache: &impl ModuleStore,
    ) -> Result<(), ApiError> {
        defn_common!(self, ctx, graph, sdml::EVENT);
        add_source_span!(self, graph, ctx, cache);

        if let Some(body) = self.body() {
            add_annotations!(body, graph, ctx, cache);

            body.source_entity().add_to_graph(graph, ctx, cache)?;

            for member in body.members() {
                member.add_to_graph(graph, ctx, cache)?;
            }
        }

        ctx.pop_subject();
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Definitions ❱ Property
// ------------------------------------------------------------------------------------------------

///
/// # Example
///
/// ```ttl
/// $parent sdml:hasDefinition $subject .
///
/// $subject
///     sdml:sourceLocation _:B01 ; # See Span
///     rdf:type sdml:Property ;
///     sdml:srcLabel "$subject.name" ;
///     # annotations
///     # member
/// .
/// ```
///
impl ToGraph for PropertyDef {
    fn add_to_graph(
        &self,
        graph: &mut Graph,
        ctx: &mut Context,
        cache: &impl ModuleStore,
    ) -> Result<(), ApiError> {
        defn_common!(self, ctx, graph, sdml::PROPERTY);
        add_source_span!(self, graph, ctx, cache);

        self.member_def().add_to_graph(graph, ctx, cache)?;

        ctx.pop_subject();
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Definitions ❱ RDF
// ------------------------------------------------------------------------------------------------

///
/// # Example
///
/// ```ttl
/// $parent sdml:hasDefinition $subject .
///
/// $subject
///     sdml:sourceLocation _:B01 ; # See Span
///     rdf:type sdml:Rdf ;
///     sdml:srcLabel "$subject.name" ;
///     # annotations
/// .
/// ```
///
impl ToGraph for RdfDef {
    fn add_to_graph(
        &self,
        graph: &mut Graph,
        ctx: &mut Context,
        cache: &impl ModuleStore,
    ) -> Result<(), ApiError> {
        defn_common!(self, ctx, graph, sdml::RDF);
        add_source_span!(self, graph, ctx, cache);

        if let Some(body) = self.body() {
            add_annotations!(body, graph, ctx, cache);
        }

        ctx.pop_subject();
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Definitions ❱ Structure
// ------------------------------------------------------------------------------------------------

///
/// # Example
///
/// ```ttl
/// $parent sdml:hasDefinition $subject .
///
/// $subject
///     sdml:sourceLocation _:B01 ; # See Span
///     rdf:type sdml:Structure ;
///     sdml:srcLabel "$subject.name" ;
///     # annotations
///     # member
/// .
/// ```
///
impl ToGraph for StructureDef {
    fn add_to_graph(
        &self,
        graph: &mut Graph,
        ctx: &mut Context,
        cache: &impl ModuleStore,
    ) -> Result<(), ApiError> {
        defn_common!(self, ctx, graph, sdml::STRUCTURE);
        add_source_span!(self, graph, ctx, cache);

        if let Some(body) = self.body() {
            add_annotations!(body, graph, ctx, cache);

            for member in body.members() {
                member.add_to_graph(graph, ctx, cache)?;
            }
        }

        ctx.pop_subject();
        Ok(())
    }
}

///
/// # Example
///
/// ```ttl
/// $parent sdml:hasMember $subject .
///
/// # either a Property Reference:
///
/// $subject
///     sdml:sourceLocation _:B01 ; # See Span
///     rdf:type sdml:PropertyRef ;
///     sdml:reference "$subject" ;
/// .
///
/// # or MemberDef
/// ```
///
impl ToGraph for Member {
    fn add_to_graph(
        &self,
        graph: &mut Graph,
        ctx: &mut Context,
        cache: &impl ModuleStore,
    ) -> Result<(), ApiError> {
        defn_common!(self, ctx, graph, sdml::MEMBER, sdml::HAS_MEMBER);
        add_source_span!(self, graph, ctx, cache);

        let current_subject = ctx.current_subject().clone();
        match self.kind() {
            MemberKind::Reference(v) => {
                g_insert!(graph ; &current_subject, rdf:type, mkiri!(sdml:PROPERTY_REF));
                g_insert!(graph ;
                    &current_subject,
                    mkiri!(sdml:IDENTIFIER_REFERENCE),
                    identifier_reference_to_url(v, ctx, cache)?
                )
            }
            MemberKind::Definition(defn) => {
                defn.add_to_graph(graph, ctx, cache)?;
            }
        }

        ctx.pop_subject();
        Ok(())
    }
}

///
/// # Example
///
/// ```ttl
/// $parent sdml:hasDefinition $subject .
///
/// $subject
///     sdml:sourceLocation _:B01 ; # See Span
///     rdf:type sdml:Member ;
///     sdml:srcLabel "$subject.name" ;
///     # annotations
///     # cardinality
///     # target type
/// .
/// ```
///
impl ToGraph for MemberDef {
    fn add_to_graph(
        &self,
        graph: &mut Graph,
        ctx: &mut Context,
        cache: &impl ModuleStore,
    ) -> Result<(), ApiError> {
        add_source_span!(self, graph, ctx, cache);

        let current_subject = ctx.current_subject().clone();
        g_insert!(graph ; &current_subject, rdf:type, mkiri!(sdml:MEMBER));

        self.target_cardinality().add_to_graph(graph, ctx, cache)?;

        add_type_reference(self.target_type(), mkiri!(sdml:HAS_TYPE), graph, ctx, cache)?;

        if let Some(body) = self.body() {
            add_annotations!(body, graph, ctx, cache);
        }

        Ok(())
    }
}

impl ToGraph for Cardinality {
    fn add_to_graph(
        &self,
        graph: &mut Graph,
        ctx: &mut Context,
        _cache: &impl ModuleStore,
    ) -> Result<(), ApiError> {
        let parent_subject = ctx.current_subject().clone();
        let subject = BlankNode::generate();
        g_insert!(graph ; &parent_subject, mkiri!(sdml:HAS_CARDINALITY), subject.clone());

        g_insert!(graph ; &subject, rdf:type, mkiri!(sdml:CARDINALITY));

        if let Some(ordering) = self.ordering() {
            g_insert!(
                graph ;
                &subject,
                mkiri!(sdml::MODULE_URL, sdml::ELEMENT_ORDERING),
                mkiri!(sdml::MODULE_URL, ordering.to_string())
            );
        }

        if let Some(uniqueness) = self.uniqueness() {
            g_insert!(
                graph ;
                &subject,
                mkiri!(sdml::MODULE_URL, sdml::ELEMENT_UNIQUENESS),
                mkiri!(sdml::MODULE_URL, uniqueness.to_string())
            );
        }

        g_insert!(
            graph ;
            &subject,
            mkiri!(sdml::MODULE_URL, sdml::MIN_OCCURS),
            Literal::from(self.min_occurs())
        );

        if let Some(max_occurs) = self.max_occurs() {
            g_insert!(
                graph ;
                &subject,
                mkiri!(sdml::MODULE_URL, sdml::MAX_OCCURS),
                Literal::from(max_occurs)
            );
        }

        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Definitions ❱ Type Class
// ------------------------------------------------------------------------------------------------

///
/// # Example
///
/// ```ttl
/// $parent sdml:hasDefinition $subject .
///
/// $subject
///     sdml:sourceLocation _:B01 ; # See Span
///     rdf:type sdml:TypeClass ;
///     sdml:srcLabel "$subject.name" ;
///     # annotations
///     # methods
/// .
/// ```
///
impl ToGraph for TypeClassDef {
    fn add_to_graph(
        &self,
        graph: &mut Graph,
        ctx: &mut Context,
        cache: &impl ModuleStore,
    ) -> Result<(), ApiError> {
        defn_common!(self, ctx, graph, sdml::TYPE_CLASS);
        add_source_span!(self, graph, ctx, cache);

        if let Some(body) = self.body() {
            add_annotations!(body, graph, ctx, cache);
        }

        ctx.pop_subject();
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Definitions ❱ Union
// ------------------------------------------------------------------------------------------------

///
/// # Example
///
/// ```ttl
/// $parent sdml:hasDefinition $subject .
///
/// $subject
///     sdml:sourceLocation _:B01 ; # See Span
///     rdf:type sdml:Union ;
///     sdml:srcLabel "$subject.name" ;
///     # annotations
///     # variants -- see TypeVariant
/// .
/// ```
///
impl ToGraph for UnionDef {
    fn add_to_graph(
        &self,
        graph: &mut Graph,
        ctx: &mut Context,
        cache: &impl ModuleStore,
    ) -> Result<(), ApiError> {
        defn_common!(self, ctx, graph, sdml::UNION);
        add_source_span!(self, graph, ctx, cache);

        if let Some(body) = self.body() {
            add_annotations!(body, graph, ctx, cache);

            for variant in body.variants() {
                variant.add_to_graph(graph, ctx, cache)?;
            }
        }

        ctx.pop_subject();
        Ok(())
    }
}

///
/// # Example
///
/// ```ttl
/// $parent sdml:hasType $subject .
///
/// $subject
///     sdml:sourceLocation _:B01 ;     # See Span
///     rdf:type sdml:TypeVariant ;
///     sdml:srcLabel "$subject.name" ;
///     sdml:rename "$subject.rename" ; # Optional
///     # annotations
/// .
/// ```
///
impl ToGraph for TypeVariant {
    fn add_to_graph(
        &self,
        graph: &mut Graph,
        ctx: &mut Context,
        cache: &impl ModuleStore,
    ) -> Result<(), ApiError> {
        defn_common!(self, ctx, graph, sdml::TYPE_VARIANT, sdml::HAS_TYPE_VARIANT);
        add_source_span!(self, graph, ctx, cache);

        if let Some(rename) = self.rename() {
            g_insert!(graph ; ctx => mkiri!(sdml:RENAME), Literal::from(rename.to_string()));
        }

        if let Some(body) = self.body() {
            add_annotations!(body, graph, ctx, cache);
        }

        ctx.pop_subject();
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Private Implementations ❱ Modules & Imports
// ------------------------------------------------------------------------------------------------

impl ToGraph for (&Identifier, Option<&HeaderValue<Iri>>) {
    fn add_to_graph(
        &self,
        graph: &mut Graph,
        ctx: &mut Context,
        cache: &impl ModuleStore,
    ) -> Result<(), ApiError> {
        let (name, version) = self;
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
}

// ------------------------------------------------------------------------------------------------
// Private Implementations ❱ Values
// ------------------------------------------------------------------------------------------------

impl ToGraphAsObject for Value {
    fn add_to_graph(
        &self,
        graph: &mut Graph,
        ctx: &mut Context,
        cache: &impl ModuleStore,
        subject: &SubjectNode,
        predicate: &Iri,
    ) -> Result<(), ApiError> {
        match self {
            Value::Simple(v) => v.add_to_graph(graph, ctx, cache, &subject, &predicate)?,
            Value::ValueConstructor(v) => {
                v.add_to_graph(graph, ctx, cache, &subject, &predicate)?
            }
            Value::Mapping(v) => v.add_to_graph(graph, ctx, cache, &subject, &predicate)?,
            Value::Reference(v) => v.add_to_graph(graph, ctx, cache, &subject, &predicate)?,
            Value::Sequence(v) => v.add_to_graph(graph, ctx, cache, &subject, &predicate)?,
        }
        Ok(())
    }
}

impl ToGraphAsObject for SequenceOfValues {
    fn add_to_graph(
        &self,
        graph: &mut Graph,
        ctx: &mut Context,
        cache: &impl ModuleStore,
        subject: &SubjectNode,
        predicate: &Iri,
    ) -> Result<(), ApiError> {
        let sequence = BlankNode::generate();
        g_insert!(graph ; subject.clone(), predicate.clone(), &sequence);
        g_insert!(graph ; &sequence, rdf:type, mkiri!(sdml:SEQUENCE));

        if let Some(ordering) = self.ordering() {
            g_insert!(
                graph ;
                &sequence,
                mkiri!(sdml::MODULE_URL, sdml::ELEMENT_ORDERING),
                mkiri!(sdml::MODULE_URL, ordering.to_string())
            );
        }

        if let Some(uniqueness) = self.uniqueness() {
            g_insert!(
                graph ;
                &sequence,
                mkiri!(sdml::MODULE_URL, sdml::ELEMENT_UNIQUENESS),
                mkiri!(sdml::MODULE_URL, uniqueness.to_string())
            );
        }

        let sequence = SubjectNode::from(&sequence);

        for (i, value) in self.iter().enumerate() {
            let predicate = mkiri!(rdf::MODULE_URL, format!("_{}", i + 1));
            match value {
                SequenceMember::Simple(v) => {
                    v.add_to_graph(graph, ctx, cache, &sequence, &predicate)?;
                }
                SequenceMember::ValueConstructor(v) => {
                    v.add_to_graph(graph, ctx, cache, &sequence, &predicate)?;
                }
                SequenceMember::Reference(v) => {
                    v.add_to_graph(graph, ctx, cache, &sequence, &predicate)?;
                }
                SequenceMember::Mapping(v) => {
                    v.add_to_graph(graph, ctx, cache, &sequence, &predicate)?;
                }
            };
        }

        Ok(())
    }
}

impl ToGraphAsObject for ValueConstructor {
    fn add_to_graph(
        &self,
        graph: &mut Graph,
        ctx: &mut Context,
        cache: &impl ModuleStore,
        subject: &SubjectNode,
        predicate: &Iri,
    ) -> Result<(), ApiError> {
        let lexical_form = match self.value() {
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
            identifier_reference_to_url(self.type_name(), ctx, cache).expect("Iri parse error");
        g_insert!(graph ;
            subject.clone(),
            predicate.clone(),
            Literal::with_data_type(lexical_form, DataType::Other(data_type))
        );
        Ok(())
    }
}

impl ToGraphAsObject for IdentifierReference {
    fn add_to_graph(
        &self,
        graph: &mut Graph,
        ctx: &mut Context,
        cache: &impl ModuleStore,
        subject: &SubjectNode,
        predicate: &Iri,
    ) -> Result<(), ApiError> {
        g_insert!(graph ;
            subject.clone(),
            predicate.clone(),
            identifier_reference_to_url(self, ctx, cache).expect("Iri parse error")
        );
        Ok(())
    }
}

///
/// # Example
///
/// ```ttl
/// $subject $predicate _:B01 .
/// _:B01
///     sdml:domainValue $object.domain ;
///     sdml:rangeValue $object.range .
/// ```
///
impl ToGraphAsObject for MappingValue {
    fn add_to_graph(
        &self,
        graph: &mut Graph,
        ctx: &mut Context,
        cache: &impl ModuleStore,
        subject: &SubjectNode,
        predicate: &Iri,
    ) -> Result<(), ApiError> {
        let mapping = BlankNode::generate();
        graph.insert(Statement::new(subject.clone(), predicate.clone(), &mapping));
        g_insert!(graph ; &mapping, rdf:type, mkiri!(sdml:MAP_TYPE));

        let mapping = SubjectNode::from(mapping);

        self.domain()
            .add_to_graph(graph, ctx, cache, &mapping, &mkiri!(sdml:DOMAIN_VALUE))?;
        self.range()
            .add_to_graph(graph, ctx, cache, &mapping, &mkiri!(sdml:RANGE_VALUE))?;

        Ok(())
    }
}

impl ToGraphAsObject for SimpleValue {
    fn add_to_graph(
        &self,
        graph: &mut Graph,
        _ctx: &mut Context,
        _cache: &impl ModuleStore,
        subject: &SubjectNode,
        predicate: &Iri,
    ) -> Result<(), ApiError> {
        let object = match self {
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
}

// ------------------------------------------------------------------------------------------------
// Private Functions
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

///
/// # Example
///
/// ```ttl
/// $subject $predicate sdml:unknown .
///
/// $subject $predicate <$object> .
///
/// $subject $predicate _:B01 .
/// _:B01
///     sdml:domainType $object.domain ;
///     sdml:rangeType $object.range .
/// ```
///
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
