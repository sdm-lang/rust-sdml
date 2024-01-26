/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::GenerateToWriter;
use sdml_core::{
    cache::ModuleCache,
    error::Error,
    model::{
        annotations::{Annotation, AnnotationProperty, HasAnnotations},
        constraints::Constraint,
        definitions::{
            DatatypeDef, Definition, EntityDef, EnumDef, EventDef, HasMembers, HasVariants,
            PropertyDef, RdfDef, StructureDef, TypeClassDef, TypeVariant, UnionDef, ValueVariant,
        },
        identifiers::{Identifier, IdentifierReference},
        members::{HasCardinality, Member, Ordering, Uniqueness, DEFAULT_CARDINALITY},
        members::{HasType, TypeReference},
        modules::Module,
        values::{
            MappingValue, SequenceMember, SequenceOfValues, SimpleValue, Value, ValueConstructor,
        },
        HasBody, HasName, HasNameReference, HasOptionalBody,
    },
    stdlib,
};
use std::{fmt::Display, io::Write};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct RdfModelGenerator {}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum RdfRepresentation {
    NTriples,
    Turtle,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

macro_rules! write_header {
    ($module_name:expr, $name:expr, $parent_name:expr, $writer:expr; $( $super_type:expr ),+) => {
        let super_types = vec![
            $( $super_type, )+
        ].join(", ");
        $writer.write_all(
            format!(
                "{}:{}__{}\n    rdf:type {} ;\n",
                $module_name,
                $parent_name,
                $name,
                super_types,
            ).as_bytes()
        )?;
    };
    ($module_name:expr, $name:expr, $writer:expr; $( $super_type:expr ),+) => {
        let super_types = vec![
            $( $super_type, )+
        ].join(", ");
        $writer.write_all(
            format!(
                "{}:{}\n    rdf:type {} ;\n",
                $module_name,
                $name,
                super_types,
            ).as_bytes()
        )?;
    };
}

macro_rules! write_annotations {
    ($self:expr, $anns:expr, $module_name:expr, $writer:expr) => {
        for annotation in $anns {
            match &annotation {
                Annotation::Property(me) => {
                    $self.write_annotation_property(me, $module_name, $writer)?
                }
                Annotation::Constraint(me) => $self.write_constraint(me, $module_name, $writer)?,
            }
        }
    };
}

macro_rules! write_members {
    ($self:expr, $me:expr, $module_name:expr, $parent_name:expr, $writer:expr) => {
        if let Some(body) = $me.body() {
            for member in body.members() {
                $self.write_member(member, $module_name, $parent_name, $writer)?;
            }
        }
    };
    ($body:expr, $parent_name:expr, $writer:expr) => {
        if $body.has_members() {
            $writer.write_all(b"    sdml:hasMember\n")?;
            let last_member = $body.members_len() - 1;
            for (i, member) in $body.members().enumerate() {
                $writer.write_all(
                    format!(
                        "        {}__{}{}\n",
                        $parent_name,
                        member.name(),
                        if i < last_member { "," } else { " ;" },
                    )
                    .as_bytes(),
                )?;
            }
        }
    };
}

macro_rules! write_variants {
    ($body:expr, $parent_name:expr, $kind:expr, $writer:expr) => {
        if $body.has_variants() {
            $writer.write_all(format!("    {}\n", $kind).as_bytes())?;
            let last_variant = $body.variants_len() - 1;
            for (i, variant) in $body.variants().enumerate() {
                $writer.write_all(
                    format!(
                        "        {}__{}{}\n",
                        $parent_name,
                        variant.name(),
                        if i < last_variant { "," } else { " ;" },
                    )
                    .as_bytes(),
                )?;
            }
        }
    };
}

macro_rules! write_footer {
    ($module_name:expr, $name:expr, $writer:expr) => {
        $writer.write_all(
            format!(
                "    sdml:srcLabel \"{}\" ;
    rdfs:isDefinedBy {}: .

",
                $name, $module_name
            )
            .as_bytes(),
        )?;
    };
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl GenerateToWriter<RdfRepresentation> for RdfModelGenerator {
    fn write_in_format(
        &mut self,
        module: &Module,
        cache: &ModuleCache,
        writer: &mut dyn Write,
        _format: RdfRepresentation,
    ) -> Result<(), Error> {
        let module_name = module.name();

        if let Some(base) = module.base_uri() {
            writer.write_all(format!("@base <{base}> .\n").as_bytes())?;
            writer.write_all(format!("@prefix {module_name}: <{base}> .\n").as_bytes())?;
        }

        let body = module.body();
        let mut imported_modules = body.imported_modules();

        if !imported_modules.contains(&Identifier::new_unchecked(stdlib::owl::MODULE_NAME)) {
            writer.write_all(b"@prefix owl: <http://www.w3.org/2002/07/owl#> .\n")?;
        }
        if !imported_modules.contains(&Identifier::new_unchecked(stdlib::rdf::MODULE_NAME)) {
            writer.write_all(b"@prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .\n")?;
        }
        if !imported_modules.contains(&Identifier::new_unchecked(stdlib::rdfs::MODULE_NAME)) {
            writer.write_all(b"@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .\n")?;
        }
        if !imported_modules.contains(&Identifier::new_unchecked(stdlib::sdml::MODULE_NAME)) {
            writer.write_all(b"@prefix sdml: <https://sdml.io/sdml-owl.ttl#> .\n")?;
        }

        for import in &imported_modules {
            if let Some(uri) = cache.url_for_identifier(import) {
                writer.write_all(format!("@prefix {import}: <{uri}> .\n").as_bytes())?;
            }
        }

        writer.write_all(b"\n")?;

        writer.write_all(
            format!("{module_name}: rdf:type owl:Ontology, sdml:Module ;\n").as_bytes(),
        )?;
        if let Some(version_info) = module.version_info() {
            writer.write_all(format!("    owl:versionInfo {version_info:?} ;\n").as_bytes())?;
        }
        if let Some(version_uri) = module.version_uri() {
            writer.write_all(format!("    owl:versionIRI <{version_uri}> ;\n").as_bytes())?;
        }

        imported_modules.remove(&Identifier::new_unchecked(stdlib::owl::MODULE_NAME));
        imported_modules.remove(&Identifier::new_unchecked(stdlib::rdf::MODULE_NAME));
        imported_modules.remove(&Identifier::new_unchecked(stdlib::rdfs::MODULE_NAME));
        imported_modules.remove(&Identifier::new_unchecked(stdlib::xsd::MODULE_NAME));
        for import in &imported_modules {
            if let Some(url) = cache.url_for_identifier(import) {
                writer.write_all(format!("    owl:imports <{url}> ;\n").as_bytes())?;
            }
        }

        write_annotations!(self, body.annotations(), module_name, writer);

        writer.write_all(format!("    sdml:srcLabel \"{module_name}\" .\n\n").as_bytes())?;

        for definition in body.definitions() {
            match &definition {
                Definition::Datatype(v) => self.write_datatype(v, module_name, writer)?,
                Definition::Entity(v) => self.write_entity(v, module_name, writer)?,
                Definition::Enum(v) => self.write_enumeration(v, module_name, writer)?,
                Definition::Event(v) => self.write_event(v, module_name, writer)?,
                Definition::Property(v) => self.write_property(v, module_name, writer)?,
                Definition::Rdf(v) => self.write_rdf(v, module_name, writer)?,
                Definition::Structure(v) => self.write_structure(v, module_name, writer)?,
                Definition::TypeClass(v) => self.write_type_class(v, module_name, writer)?,
                Definition::Union(v) => self.write_union(v, module_name, writer)?,
            }
        }

        Ok(())
    }
}

impl RdfModelGenerator {
    fn write_datatype(
        &mut self,
        me: &DatatypeDef,
        module_name: &Identifier,
        writer: &mut dyn Write,
    ) -> Result<(), Error> {
        let name = me.name();
        let base_type = me.base_type();

        write_header!(module_name, name, writer; "rdfs:Datatype");
        writer.write_all(format!("    owl:onDatatype {base_type} ;\n").as_bytes())?;

        if let Some(body) = me.body() {
            let (facets, other): (Vec<_>, Vec<_>) = body.annotations().partition(|ann| {
                if let Annotation::Property(prop) = ann {
                    prop.is_datatype_facet()
                } else {
                    false
                }
            });

            if !facets.is_empty() {
                writer.write_all(b"    owl:withRestrictions\n")?;
                let last_facet = facets.len() - 1;
                for (i, facet) in facets.iter().enumerate() {
                    if let Some(facet) = facet.as_annotation_property() {
                        self.write_facet_property(
                            facet,
                            module_name,
                            if i < last_facet { "," } else { " ;" },
                            writer,
                        )?;
                    } else {
                        unreachable!()
                    }
                }
            }

            write_annotations!(self, other.iter(), module_name, writer);
        }

        write_footer!(module_name, name, writer);

        Ok(())
    }

    fn write_entity(
        &mut self,
        me: &EntityDef,
        module_name: &Identifier,
        writer: &mut dyn Write,
    ) -> Result<(), Error> {
        let name = me.name();

        write_header!(module_name, name, writer; "owl:Class", "sdml:Entity");

        if let Some(body) = me.body() {
            write_annotations!(self, body.annotations(), module_name, writer);

            write_members!(body, name, writer);
        }

        write_footer!(module_name, name, writer);

        write_members!(self, me, module_name, name, writer);

        Ok(())
    }

    fn write_member(
        &mut self,
        me: &Member,
        module_name: &Identifier,
        parent: &Identifier,
        writer: &mut dyn Write,
    ) -> Result<(), Error> {
        let name = me.name();

        let more = if let Some(_property) = me.as_property_reference() {
            write_header!(module_name, name, parent, writer; "rdfs:Property", "sdml:RoleReference");
            self.write_member_type(me, module_name, "rdfs:range", writer)?
        } else if let Some(def) = me.as_definition() {
            write_header!(module_name, name, parent, writer; "rdfs:Property", "sdml:Member");
            writer.write_all(format!("    rdfs:domain {module_name}:{parent} ;\n").as_bytes())?;
            let more = self.write_member_type(me, module_name, "rdfs:range", writer)?;
            if let Some(body) = def.body() {
                write_annotations!(self, body.annotations(), module_name, writer);
            }
            more
        } else {
            unreachable!();
        };

        write_footer!(module_name, name, writer);

        if !more.is_empty() {
            writer.write_all(more.as_bytes())?;
        }

        Ok(())
    }

    fn write_member_type(
        &mut self,
        me: &Member,
        module_name: &Identifier,
        property: &str,
        writer: &mut dyn Write,
    ) -> Result<String, Error> {
        let mut more = String::new();

        if let Some(def) = me.as_definition() {
            match def.target_type() {
                TypeReference::Unknown => {
                    writer.write_all(format!("    {property} sdml:Unknown ;\n").as_bytes())?;
                }
                TypeReference::Type(name) => {
                    writer.write_all(
                        format!(
                            "    {property} {} ;\n",
                            match name {
                                IdentifierReference::Identifier(id) =>
                                    format!("{module_name}:{id}"),
                                IdentifierReference::QualifiedIdentifier(qid) => qid.to_string(),
                            },
                        )
                        .as_bytes(),
                    )?;
                    let card = def.target_cardinality();
                    if card != &DEFAULT_CARDINALITY {
                        if let Some(ordering) = card.ordering() {
                            writer.write_all(
                                format!(
                                    "    sdml:isOrdered {} ;\n",
                                    matches!(ordering, Ordering::Ordered),
                                )
                                .as_bytes(),
                            )?;
                        }
                        if let Some(uniqueness) = card.uniqueness() {
                            writer.write_all(
                                format!(
                                    "    sdml:isUnique {} ;\n",
                                    matches!(uniqueness, Uniqueness::Unique),
                                )
                                .as_bytes(),
                            )?;
                        }
                        let range = card.range();
                        writer.write_all(
                            format!(
                                "    owl:minCardinality \"{}\"^^xsd:nonNegativeInteger ;\n",
                                range.min_occurs(),
                            )
                            .as_bytes(),
                        )?;
                        if let Some(max) = range.max_occurs() {
                            writer.write_all(
                                format!(
                                    "    owl:maxCardinality \"{}\"^^xsd:nonNegativeInteger ;\n",
                                    max,
                                )
                                .as_bytes(),
                            )?;
                        }
                    }
                }
                TypeReference::FeatureSet(name) => {
                    let (target_name, label) = match name {
                        IdentifierReference::Identifier(id) => (format!("{module_name}:{id}"), id),
                        IdentifierReference::QualifiedIdentifier(qid) => {
                            (qid.to_string(), qid.member())
                        }
                    };
                    writer.write_all(
                        format!("    {property} {}__Feature ;\n", target_name,).as_bytes(),
                    )?;
                    // TODO cardinality
                    more = format!("{module_name}:{label}__Feature rdf:type owl:Class, sdml:Union, sdml:FeatureSet ;
    owl:equivalentClass {target_name} ;
    sdml:srcLabel \"{label}\" ;
    rdfs:isDefinedBy {module_name}: .

");
                }
                TypeReference::MappingType(_map) => {
                    // 1. throw hands in the air, this is a mess.
                    // TODO cardinality
                }
            }
        } else if let Some(_property) = me.as_property_reference() {
            // 1. lookup `property` in cache
            // 2. find member name as `role` in property
            // 3. call self with member type of property
        } else {
            unreachable!()
        }

        Ok(more)
    }

    fn write_enumeration(
        &mut self,
        me: &EnumDef,
        module_name: &Identifier,
        writer: &mut dyn Write,
    ) -> Result<(), Error> {
        let name = me.name();

        write_header!(module_name, name, writer; "owl:Class", "sdml:Enumeration");

        if let Some(body) = me.body() {
            write_annotations!(self, body.annotations(), module_name, writer);

            write_variants!(body, name, "sdml:hasValueVariant", writer);
        }

        write_footer!(module_name, name, writer);

        if let Some(body) = me.body() {
            for variant in body.variants() {
                self.write_value_variant(variant, module_name, name, writer)?;
            }
        }

        Ok(())
    }

    fn write_value_variant(
        &mut self,
        me: &ValueVariant,
        module_name: &Identifier,
        parent: &Identifier,
        writer: &mut dyn Write,
    ) -> Result<(), Error> {
        let name = me.name();

        write_header!(
            module_name, name, parent, writer;
            "owl:NamedIndividual", "sdml:ValueVariant", format!("{module_name}:{parent}").as_str()
        );

        if let Some(body) = me.body() {
            write_annotations!(self, body.annotations(), module_name, writer);
        }

        write_footer!(module_name, name, writer);

        Ok(())
    }

    fn write_event(
        &mut self,
        me: &EventDef,
        module_name: &Identifier,
        writer: &mut dyn Write,
    ) -> Result<(), Error> {
        let name = me.name();

        write_header!(module_name, name, writer; "owl:Class", "sdml:Event");
        writer.write_all(format!("    sdml:hasSource {} ;\n", me.event_source()).as_bytes())?;

        if let Some(body) = me.body() {
            write_annotations!(self, body.annotations(), module_name, writer);

            write_members!(body, name, writer);
        }

        write_footer!(module_name, name, writer);

        write_members!(self, me, module_name, name, writer);

        Ok(())
    }

    fn write_structure(
        &mut self,
        me: &StructureDef,
        module_name: &Identifier,
        writer: &mut dyn Write,
    ) -> Result<(), Error> {
        let name = me.name();

        write_header!(module_name, name, writer; "owl:Class", "sdml:Structure");

        if let Some(body) = me.body() {
            write_annotations!(self, body.annotations(), module_name, writer);

            write_members!(body, name, writer);
        }

        write_footer!(module_name, name, writer);

        write_members!(self, me, module_name, name, writer);

        Ok(())
    }

    fn write_union(
        &mut self,
        me: &UnionDef,
        module_name: &Identifier,
        writer: &mut dyn Write,
    ) -> Result<(), Error> {
        let name = me.name();

        write_header!(module_name, name, writer; "owl:Class", "sdml:Union");

        if let Some(body) = me.body() {
            write_annotations!(self, body.annotations(), module_name, writer);

            write_variants!(body, name, "sdml:hasTypeVariant", writer);
        }

        write_footer!(module_name, name, writer);

        if let Some(body) = me.body() {
            for variant in body.variants() {
                self.write_type_variant(variant, module_name, name, writer)?;
            }
        }

        Ok(())
    }

    fn write_type_variant(
        &mut self,
        me: &TypeVariant,
        module_name: &Identifier,
        parent: &Identifier,
        writer: &mut dyn Write,
    ) -> Result<(), Error> {
        let name = me.name();

        write_header!(module_name, name, parent, writer; "sdml:TypeVariant");

        let (ty_module, ty_name) = match me.name_reference() {
            IdentifierReference::Identifier(name) => (module_name, name),
            IdentifierReference::QualifiedIdentifier(name) => (name.module(), name.member()),
        };
        writer
            .write_all(format!("    owl:equivalentClass {ty_module}:{ty_name} ;\n").as_bytes())?;

        if let Some(body) = me.body() {
            write_annotations!(self, body.annotations(), module_name, writer);
        }

        write_footer!(module_name, name, writer);

        Ok(())
    }

    fn write_property(
        &mut self,
        me: &PropertyDef,
        module_name: &Identifier,
        writer: &mut dyn Write,
    ) -> Result<(), Error> {
        let name = me.name();

        write_header!(module_name, name, writer; "owl:Class", "sdml:Property");

        if let Some(body) = me.body() {
            write_annotations!(self, body.annotations(), module_name, writer);

            // TODO: roles
        }

        write_footer!(module_name, name, writer);

        Ok(())
    }

    fn write_rdf(
        &mut self,
        me: &RdfDef,
        module_name: &Identifier,
        writer: &mut dyn Write,
    ) -> Result<(), Error> {
        let name = me.name();

        writer.write_all(format!("{module_name}:{name}\n").as_bytes())?;

        write_annotations!(self, me.body().annotations(), module_name, writer);

        write_footer!(module_name, name, writer);

        Ok(())
    }

    fn write_type_class(
        &mut self,
        _me: &TypeClassDef,
        _module_name: &Identifier,
        _writer: &mut dyn Write,
    ) -> Result<(), Error> {
        todo!()
    }

    fn write_annotation_property(
        &mut self,
        me: &AnnotationProperty,
        module_name: &Identifier,
        writer: &mut dyn Write,
    ) -> Result<(), Error> {
        let (module, name) = self.qualified_idref(module_name, me.name_reference());

        writer.write_all(format!("    {module}:{name} ").as_bytes())?;

        writer.write_all(self.value_to_string(me.value(), module_name).as_bytes())?;

        writer.write_all(b" ;\n")?;

        Ok(())
    }

    fn write_facet_property(
        &mut self,
        me: &AnnotationProperty,
        module_name: &Identifier,
        terminator: &str,
        writer: &mut dyn Write,
    ) -> Result<(), Error> {
        let (module, name) = self.qualified_idref(module_name, me.name_reference());
        let value = if let Value::Reference(IdentifierReference::Identifier(v)) = me.value() {
            format!("{module_name}:{v}")
        } else {
            me.value().to_string()
        };
        writer.write_all(format!("        {module}:{name} {value}{terminator}\n").as_bytes())?;

        Ok(())
    }

    fn write_constraint(
        &mut self,
        _me: &Constraint,
        _module_name: &Identifier,
        _writer: &mut dyn Write,
    ) -> Result<(), Error> {
        todo!();
    }

    fn qualified_idref_string(
        &self,
        module_name: &Identifier,
        idref: &IdentifierReference,
    ) -> String {
        let (module, member) = self.qualified_idref(module_name, idref);
        format!("{module}:{member}")
    }

    fn qualified_idref<'a>(
        &self,
        module_name: &'a Identifier,
        idref: &'a IdentifierReference,
    ) -> (&'a Identifier, &'a Identifier) {
        match idref {
            IdentifierReference::Identifier(name) => (module_name, name),
            IdentifierReference::QualifiedIdentifier(name) => (name.module(), name.member()),
        }
    }

    fn value_to_string(&mut self, me: &Value, module_name: &Identifier) -> String {
        match me {
            Value::Simple(v) => self.simple_value_to_string(v),
            Value::ValueConstructor(v) => self.value_constructor_to_string(v, module_name),
            Value::Mapping(v) => self.mapping_value_to_string(v, module_name),
            Value::Reference(v) => self.reference_value_to_string(v, module_name),
            Value::List(v) => self.list_value_to_string(v, module_name),
        }
    }

    fn simple_value_to_string(&mut self, me: &SimpleValue) -> String {
        match me {
            SimpleValue::Boolean(v) => v.to_string(),
            SimpleValue::Double(v) => format!("\"{v}\"^^sdml:double"),
            SimpleValue::Decimal(v) => format!("\"{v}\"^^sdml:decimal"),
            SimpleValue::Integer(v) => v.to_string(),
            SimpleValue::Unsigned(v) => format!("\"{v}\"^^sdml:unsigned"),
            SimpleValue::String(v) => format!("{v}"),
            SimpleValue::IriReference(v) => format!("<{v}>"),
            SimpleValue::Binary(_) => todo!(),
        }
    }

    fn value_constructor_to_string(
        &mut self,
        me: &ValueConstructor,
        module_name: &Identifier,
    ) -> String {
        format!(
            "\"{}\"^^{}",
            me.value(),
            self.qualified_idref_string(module_name, me.type_name())
        )
    }

    fn mapping_value_to_string(&mut self, me: &MappingValue, module_name: &Identifier) -> String {
        format!(
            "[
        sdml:hasDomainValue {} ;
        sdml:hasRangeValue {}
    ]",
            self.simple_value_to_string(me.domain()),
            self.value_to_string(me.range(), module_name)
        )
    }

    fn reference_value_to_string(
        &mut self,
        me: &IdentifierReference,
        module_name: &Identifier,
    ) -> String {
        self.qualified_idref_string(module_name, me)
    }

    fn list_value_to_string(&mut self, me: &SequenceOfValues, module_name: &Identifier) -> String {
        let mut buffer = String::from("(\n");

        for member in me.iter() {
            let value = match member {
                SequenceMember::Simple(v) => self.simple_value_to_string(v),
                SequenceMember::ValueConstructor(v) => {
                    self.value_constructor_to_string(v, module_name)
                }
                SequenceMember::Reference(v) => self.reference_value_to_string(v, module_name),
                SequenceMember::Mapping(v) => self.mapping_value_to_string(v, module_name),
            };
            buffer.push_str(&format!("        {value}\n"));
        }

        buffer.push_str("    )");
        buffer
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for RdfRepresentation {
    fn default() -> Self {
        Self::Turtle
    }
}

impl Display for RdfRepresentation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match (self, f.alternate()) {
                (Self::NTriples, false) => "NTriples",
                (Self::NTriples, true) => "nt",
                (Self::Turtle, false) => "Turtle",
                (Self::Turtle, true) => "ttl",
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
