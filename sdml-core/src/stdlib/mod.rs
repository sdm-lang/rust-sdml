/*!
This module provides modules corresponding to the SDML standard library.
*/

use crate::model::{identifiers::Identifier, modules::Module};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

macro_rules! library_module_url {
    ($authority:expr, $path:expr) => {
        pub const MODULE_URL: &str =
            concat!("https://sdml.io/stdlib/", $authority, "/", $path, "#");
    };
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn is_library_module(name: &Identifier) -> bool {
    [
        dc::MODULE_NAME,
        dc::terms::MODULE_NAME,
        iso::i3166::MODULE_NAME,
        iso::i4217::MODULE_NAME,
        owl::MODULE_NAME,
        rdf::MODULE_NAME,
        rdfs::MODULE_NAME,
        sdml::MODULE_NAME,
        skos::MODULE_NAME,
        xsd::MODULE_NAME,
    ]
    .contains(&name.as_ref())
}

pub fn is_builtin_type_name(name: &Identifier) -> bool {
    [
        sdml::BINARY,
        sdml::BOOLEAN,
        sdml::DECIMAL,
        sdml::DOUBLE,
        sdml::INTEGER,
        sdml::IRI,
        sdml::LANGUAGE,
        sdml::STRING,
        sdml::UNSIGNED,
    ]
    .contains(&name.as_ref())
}

pub fn library_module(name: &Identifier) -> Option<Module> {
    match name.as_ref() {
        dc::MODULE_NAME => Some(dc::module()),
        dc::terms::MODULE_NAME => Some(dc::terms::module()),
        iso::i3166::MODULE_NAME => Some(iso::i3166::module()),
        iso::i4217::MODULE_NAME => Some(iso::i4217::module()),
        owl::MODULE_NAME => Some(owl::module()),
        rdf::MODULE_NAME => Some(rdf::module()),
        rdfs::MODULE_NAME => Some(rdfs::module()),
        sdml::MODULE_NAME => Some(sdml::module()),
        skos::MODULE_NAME => Some(skos::module()),
        xsd::MODULE_NAME => Some(xsd::module()),
        _ => None,
    }
}

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

macro_rules! lstr {
    ($value:literal) => {
        $crate::model::values::LanguageString::new(
            $value, None,
        )
    };
     ($value:literal @en) => {
        lstr!($value @ "en")
    };
    ($value:literal @ $lang:expr) => {
        $crate::model::values::LanguageString::new(
            $value,
            Some(crate::model::values::LanguageTag::new_unchecked($lang)),
        )
    };
}

macro_rules! id {
    ($id:expr) => {
        $crate::model::identifiers::Identifier::new_unchecked($id)
    };
}

macro_rules! qualid {
    ($module:expr, $member:expr) => {
        $crate::model::identifiers::QualifiedIdentifier::new(id!($module), id!($member))
    };
}

macro_rules! idref {
    ($module:expr, $member:expr) => {
        $crate::model::identifiers::IdentifierReference::from(qualid!($module, $member))
    };
    ($id:expr) => {
        $crate::model::identifiers::IdentifierReference::from(id!($id))
    };
}

macro_rules! import {
    ( $( $module:expr ),* ) => {
        $crate::model::modules::ImportStatement::new(
            vec![
                $(
                    $crate::model::modules::Import::from(
                        $module
                    ),
                )*
            ]
        )
    };
}

macro_rules! simple {
    ($value:expr) => {
        $crate::model::values::SimpleValue::from($value)
    };
}

macro_rules! tc {
    ($module:expr, $name:expr => $value:expr) => {
        $crate::model::values::ValueConstructor::new(idref!($module, $name), simple!($value))
    };
}

macro_rules! prop {
    ($module:expr, $name:expr; $value:expr) => {
        $crate::model::annotations::AnnotationProperty::new(qualid!($module, $name), $value)
    }; //($name:expr; $value:expr) => {
       //    $crate::model::annotations::AnnotationProperty::new(
       //        id!($name).into(),
       //        $crate::model::values::Value::from($value)
       //    )
       //};
}

macro_rules! datatype {
    ($name:expr => $module:expr, $base:expr) => {
        $crate::model::definitions::DatatypeDef::new(id!($name), qualid!($module, $base).into())
    }; //($name:expr => $base:expr) => {
       //    $crate::model::definitions::DatatypeDef::new(
       //        id!($name),
       //        id!($base).into()
       //    )
       //};
}

macro_rules! union {
    ($name:expr => $( $idref:expr ),+) => {
        $crate::model::definitions::UnionDef::new(id!($name))
            .with_body(
                $crate::model::definitions::UnionBody::default()
                    .with_variants(vec![
                        $(
                            $crate::model::definitions::TypeVariant::new(idref!($idref)),
                        )+
                    ])
            )
    };
}

macro_rules! rdf {
    // --------------------------------------------------------------------------------------------
    ($kind:ident, $id:expr, $in:expr) => {
        $crate::model::definitions::RdfDef::$kind(id!($id))
            .with_label($crate::model::values::LanguageString::from($id))
            .with_is_defined_by($in.clone())
    };
    // --------------------------------------------------------------------------------------------
    (thing $id:expr, $in:expr; $( ( $super_mod:expr, $super:expr ) ),*) => {
        rdf!(individual, $id, $in)
            $(
                .with_super_class(qualid!($super_mod, $super))
            )*
    };
    (thing $id:expr, $in:expr, $( $type:expr ),+) => {
        rdf!(individual, $id, $in)
            $(
                .with_type(id!($type))
            )*
    };
    // --------------------------------------------------------------------------------------------
    (class $id:expr, $in:expr; $( ($super_mod:expr, $super:expr) ),+) => {
        rdf!(class, $id, $in)
            $(
                .with_super_class(qualid!($super_mod, $super))
            )*
    };
    (class $id:expr, $in:expr; $( $super:expr ),+) => {
        rdf!(class, $id, $in)
            $(
                .with_super_class(id!($super))
            )*
    };
    (class $id:expr, $in:expr) => {
        rdf!(class, $id, $in)
    };
    // --------------------------------------------------------------------------------------------
    (datatype $id:expr, $in:expr; $( $super:expr ),*) => {
        rdf!(datatype, $id, $in)
            $(
                .with_super_class(id!($super))
            )*
    };
    (datatype $id:expr, $in:expr) => {
        rdf!(datatype, $id, $in)
    };
    // --------------------------------------------------------------------------------------------
    (property $id:expr, $in:expr; ( $dom_mod:expr, $dom:expr ) => ( $rge_mod:expr, $rge:expr )) => {
        rdf!(property, $id, $in)
            .with_domain(qualid!($dom_mod, $dom))
            .with_range(qualid!($rge_mod, $rge))
    };
    (property $id:expr, $in:expr; ( $dom_mod:expr, $dom:expr ) => $rge:expr) => {
        rdf!(property, $id, $in)
            .with_domain(qualid!($dom_mod, $dom))
            .with_range(id!($rge))
    };
    (property $id:expr, $in:expr; $dom:expr => ( $rge_mod:expr, $rge:expr ) ) => {
        rdf!(property, $id, $in)
            .with_domain(id!($dom))
            .with_range(qualid!($rge_mod, $rge))
    };
    (property $id:expr, $in:expr; $dom:expr => $rge:expr) => {
        rdf!(property, $id, $in)
            .with_domain(id!($dom))
            .with_range(id!($rge))
    };
    (property $id:expr, $in:expr; ( $dom_mod:expr, $dom:expr )) => {
        rdf!(property, $id, $in)
            .with_domain(qualid!($dom_mod, $dom))
    };
    (property $id:expr, $in:expr; $dom:expr) => {
        rdf!(property, $id, $in)
            .with_domain(id!($dom))
    };
   (property $id:expr, $in:expr) => {
        rdf!(property, $id, $in)
    };
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod dc;

pub mod iso;

pub mod owl;

pub mod rdf;

pub mod rdfs;

pub mod sdml;

pub mod skos;

pub mod xsd;
