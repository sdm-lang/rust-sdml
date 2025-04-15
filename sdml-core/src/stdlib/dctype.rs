/*!
This Rust module contains the SDML model of the SDML library module `dctype` or `dcmitype`.
*/

use crate::model::annotations::{AnnotationOnlyBody, HasAnnotations};
use crate::model::modules::Module;
use crate::model::HasBody;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub const MODULE_NAME: &str = "dctype";
pub const MODULE_URL: &str = "http://purl.org/dc/dcmitype/";

pub const COLLECTION: &str = "Collection";
pub const DATASET: &str = "Dataset";
pub const EVENT: &str = "Event";
pub const IMAGE: &str = "Image";
pub const INTERACTIVE_RESOURCE: &str = "InteractiveResource";
pub const MOVING_IMAGE: &str = "MovingImage";
pub const PHYSICAL_OBJECT: &str = "PhysicalObject";
pub const SERVICE: &str = "Service";
pub const SOFTWARE: &str = "Software";
pub const SOUND: &str = "Sound";
pub const STILL_IMAGE: &str = "StillImage";
pub const TEXT: &str = "Text";

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

module_function!(|| {
    let module_uri: url::Url = url::Url::parse(MODULE_URL).unwrap();

    module!(
        id!(unchecked dctype), module_uri ; call |module: Module|
        module.with_imports([import_statement!(
            id!(unchecked dct),
            id!(unchecked rdf),
            id!(unchecked rdfs),
        )])
            .with_annotations([
                 annotation!(id!(unchecked dct:modified), v!(id!(unchecked xsd:date), "2012-06-14")),
                 annotation!(id!(unchecked publisher), url!("http://purl.org/dc/aboutdcmi#DCMI")),
                 annotation!(id!(unchecked title), v!("DCMI Type Vocabulary")),
            ])
            .with_definitions([
                rdf!(
                    id!(unchecked Collection) ; class ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked memberOf), id!(unchecked dct:DCMIType)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dctype)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Collection"@en)),
                        annotation!(id!(unchecked dct:issued), v!(id!(unchecked xsd:date), "2000-07-11")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("An aggregation of resources."@en)),
                        annotation!(id!(unchecked dct:description), rdf_str!("A collection is described as a group; its parts may also be separately described."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked Dataset) ; class ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked memberOf), id!(unchecked dct:DCMIType)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dctype)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Dataset"@en)),
                        annotation!(id!(unchecked dct:issued), v!(id!(unchecked xsd:date), "2000-07-11")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("Data encoded in a defined structure."@en)),
                        annotation!(id!(unchecked dct:description), rdf_str!("A collection is described as a group; its parts may also be separately described."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked Event) ; class ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked memberOf), id!(unchecked dct:DCMIType)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dctype)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Event"@en)),
                        annotation!(id!(unchecked dct:issued), v!(id!(unchecked xsd:date), "2000-07-11")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("A non-persistent, time-based occurrence."@en)),
                        annotation!(id!(unchecked dct:description), rdf_str!("Metadata for an event provides descriptive information that is the basis for discovery of the purpose, location, duration, and responsible agents associated with an event. Examples include an exhibition, webcast, conference, workshop, open day, performance, battle, trial, wedding, tea party, conflagration."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked Image) ; class ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked memberOf), id!(unchecked dct:DCMIType)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dctype)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Image"@en)),
                        annotation!(id!(unchecked dct:issued), v!(id!(unchecked xsd:date), "2000-07-11")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("A visual representation other than text."@en)),
                        annotation!(id!(unchecked dct:description), rdf_str!("Examples include images and photographs of physical objects, paintings, prints, drawings, other images and graphics, animations and moving pictures, film, diagrams, maps, musical notation.  Note that Image may include both electronic and physical representations."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked InteractiveResource) ; class ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked memberOf), id!(unchecked dct:DCMIType)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dctype)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Interactive Resource"@en)),
                        annotation!(id!(unchecked dct:issued), v!(id!(unchecked xsd:date), "2000-07-11")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("A resource requiring interaction from the user to be understood, executed, or experienced."@en)),
                        annotation!(id!(unchecked dct:description), rdf_str!("Examples include forms on Web pages, applets, multimedia learning objects, chat services, or virtual reality environments."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked MovingImage) ; class id!(unchecked Image) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked memberOf), id!(unchecked dct:DCMIType)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dctype)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Moving Image"@en)),
                        annotation!(id!(unchecked dct:issued), v!(id!(unchecked xsd:date), "2003-11-18")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("A series of visual representations imparting an impression of motion when shown in succession."@en)),
                        annotation!(id!(unchecked dct:description), rdf_str!("Examples include animations, movies, television programs, videos, zoetropes, or visual output from a simulation.  Instances of the type Moving Image must also be describable as instances of the broader type Image."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked PhysicalObject) ; class ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked memberOf), id!(unchecked dct:DCMIType)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dctype)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Physical Object"@en)),
                        annotation!(id!(unchecked dct:issued), v!(id!(unchecked xsd:date), "2002-07-13")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("An inanimate, three-dimensional object or substance."@en)),
                        annotation!(id!(unchecked dct:description), rdf_str!("Note that digital representations of, or surrogates for, these objects should use Image, Text or one of the other types."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked Service) ; class ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked memberOf), id!(unchecked dct:DCMIType)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dctype)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Service"@en)),
                        annotation!(id!(unchecked dct:issued), v!(id!(unchecked xsd:date), "2000-07-11")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("A system that provides one or more functions."@en)),
                        annotation!(id!(unchecked dct:description), rdf_str!("Examples include a photocopying service, a banking service, an authentication service, interlibrary loans, a Z39.50 or Web server."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked Software) ; class ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked memberOf), id!(unchecked dct:DCMIType)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dctype)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Software"@en)),
                        annotation!(id!(unchecked dct:issued), v!(id!(unchecked xsd:date), "2000-07-11")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("A computer program in source or compiled form."@en)),
                        annotation!(id!(unchecked dct:description), rdf_str!("Examples include a C source file, MS-Windows .exe executable, or Perl script."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked Sound) ; class id!(unchecked Image) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked memberOf), id!(unchecked dct:DCMIType)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dctype)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Sound"@en)),
                        annotation!(id!(unchecked dct:issued), v!(id!(unchecked xsd:date), "2000-07-11")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("A resource primarily intended to be heard."@en)),
                        annotation!(id!(unchecked dct:description), rdf_str!("Examples include a music playback file format, an audio compact disc, and recorded speech or sounds."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked StillImage) ; class id!(unchecked Image) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked memberOf), id!(unchecked dct:DCMIType)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dctype)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Still Image"@en)),
                        annotation!(id!(unchecked dct:issued), v!(id!(unchecked xsd:date), "2003-11-18")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("A static visual representation."@en)),
                        annotation!(id!(unchecked dct:description), rdf_str!("Examples include paintings, drawings, graphic designs, plans and maps. Recommended best practice is to assign the type Text to images of textual materials. Instances of the type Still Image must also be describable as instances of the broader type Image."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked Text) ; class ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked memberOf), id!(unchecked dct:DCMIType)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dctype)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Text"@en)),
                        annotation!(id!(unchecked dct:issued), v!(id!(unchecked xsd:date), "2000-07-11")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("A resource consisting primarily of words for reading."@en)),
                        annotation!(id!(unchecked dct:description), rdf_str!("Examples include books, letters, dissertations, poems, newspapers, articles, archives of mailing lists. Note that facsimiles or images of texts are still of the genre Text."@en)),
                    ])).into(),
            ])
    )
});
