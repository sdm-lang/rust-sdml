/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::model::{identifiers::Identifier, modules::Module};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn is_library_module(name: &Identifier) -> bool {
    [
        dc::MODULE_NAME,
        dc::terms::MODULE_NAME,
        owl::MODULE_NAME,
        rdf::MODULE_NAME,
        rdfs::MODULE_NAME,
        sdml::MODULE_NAME,
        skos::MODULE_NAME,
        xsd::MODULE_NAME,
    ]
    .contains(&name.as_ref())
}

pub fn library_module(name: &Identifier) -> Option<Module> {
    match name.as_ref() {
        dc::MODULE_NAME => Some(dc::module()),
        dc::terms::MODULE_NAME => Some(dc::terms::module()),
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
    //    ( ($module:expr, $member:expr) ) => {
    //        $crate::model::identifiers::IdentifierReference::from(qualid!($module, $member))
    //    };
    //    ($module:expr, $member:expr) => {
    //        $crate::model::identifiers::IdentifierReference::from(qualid!($module, $member))
    //    };
    ($id:expr) => {
        $crate::model::identifiers::IdentifierReference::from(id!($id))
    };
}

macro_rules! seq {
    ($( $member:expr ),*) => {
        $crate::model::values::SequenceOfValues::from(
            vec![
                $(
                     $crate::model::values::SequenceMember::from($member),
                )*
            ]
        )
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
    (property $id:expr, $in:expr => $rge:expr) => {
        rdf!(property, $id, $in)
            .with_range(id!($rge))
    };
    (property $id:expr, $in:expr, $( ($super_mod:expr, $super:expr) ),+ ) => {
        rdf!(property, $id, $in)
            $(
                .with_super_property(qualid!($super_mod, $super))
            )+
    };
    (property $id:expr, $in:expr, $( $super:expr ),+ ) => {
        rdf!(property, $id, $in)
            $(
                .with_super_property(id!($super))
            )+
    };
    (property $id:expr, $in:expr, $( $super:expr ),+; $dom:expr => $rge:expr ) => {
        rdf!(property, $id, $in)
            $(
                .with_super_property(id!($super))
            )+
            .with_domain(id!($dom))
            .with_range(id!($rge))
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

pub mod owl;

pub mod rdf;

pub mod rdfs;

pub mod sdml;

pub mod skos;

pub mod xsd;
