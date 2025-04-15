/*!
One-line description.

More detailed description, with

# Example

End of file during parsingSymbol’s value as variable is void: rustEnd of file during parsing

 */

use crate::model::{identifiers::Identifier, modules::ModulePath};
use crate::stdlib::{self, owl, rdf, rdfs, sdml, skos, xsd};
use crate::store::InMemoryModuleCache;
use std::{
    collections::{BTreeSet, HashMap},
    sync::LazyLock,
};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LibraryModule {
    name: Identifier,
    compatibility_alias: Option<Identifier>,
    builtin: bool,
    base_datatypes: BTreeSet<Identifier>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LibraryConfiguration {
    modules: HashMap<ModulePath, Vec<LibraryModule>>,
    rdf_definition_allow: BTreeSet<Identifier>,
    typeclass_definition_allow: BTreeSet<Identifier>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

fn library_configuration() -> &'static LibraryConfiguration {
    static CONFIG: LazyLock<LibraryConfiguration> = LazyLock::new(load_library_configuration);
    &CONFIG
}

#[inline(always)]
pub fn library_module_configuration() -> &'static HashMap<ModulePath, Vec<LibraryModule>> {
    &library_configuration().modules
}

pub fn is_library_module(name: &Identifier) -> bool {
    is_library_module_str(name.as_ref())
}

pub fn is_library_module_str(name: &str) -> bool {
    static MODULE_NAMES: LazyLock<BTreeSet<&'static str>> = LazyLock::new(config_to_library_names);
    MODULE_NAMES.contains(name)
}

pub fn is_builtin_type_name(name: &Identifier) -> bool {
    is_builtin_type_name_str(name.as_ref())
}

pub fn is_builtin_type_name_str(name: &str) -> bool {
    static BUILTIN_TYPE_NAMES: LazyLock<BTreeSet<&'static str>> =
        LazyLock::new(config_to_type_names);
    BUILTIN_TYPE_NAMES.contains(name)
}

pub fn is_rdf_definition_allowed_in_module(name: &Identifier) -> bool {
    is_rdf_definition_allowed_in_module_str(name.as_ref())
}

pub fn is_rdf_definition_allowed_in_module_str(name: &str) -> bool {
    static MODULE_NAMES: LazyLock<BTreeSet<&'static str>> = LazyLock::new(config_to_rdf_allow_list);
    MODULE_NAMES.contains(name) || is_library_module_str(name)
}

pub fn is_typeclass_definition_allowed_in_module(name: &Identifier) -> bool {
    is_typeclass_definition_allowed_in_module_str(name.as_ref())
}

pub fn is_typeclass_definition_allowed_in_module_str(name: &str) -> bool {
    static MODULE_NAMES: LazyLock<BTreeSet<&'static str>> =
        LazyLock::new(config_to_typeclass_allow_list);
    MODULE_NAMES.contains(name) || is_library_module_str(name)
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl LibraryModule {
    fn named(name: &str) -> Self {
        Self {
            name: Identifier::new_unchecked(name),
            compatibility_alias: None,
            builtin: false,
            base_datatypes: BTreeSet::default(),
        }
    }
    fn builtin(self) -> Self {
        let mut self_mut = self;
        self_mut.builtin = true;
        self_mut
    }
    fn aliased_as_is(self) -> Self {
        let mut self_mut = self;
        self_mut.compatibility_alias = Some(self_mut.name.clone());
        self_mut
    }
    fn alias(self, alias: &str) -> Self {
        let mut self_mut = self;
        self_mut.compatibility_alias = Some(Identifier::new_unchecked(alias));
        self_mut
    }
    fn with_datatypes(self, datatypes: &[&str]) -> Self {
        let mut self_mut = self;
        self_mut.base_datatypes =
            BTreeSet::from_iter(datatypes.iter().map(|id| Identifier::new_unchecked(id)));
        self_mut
    }
}

// ------------------------------------------------------------------------------------------------

pub fn builtin_library_cache() -> InMemoryModuleCache {
    // TODO: read from metadata...
    InMemoryModuleCache::empty()
        .with(sdml::module().clone())
        .with(stdlib::dc::module().clone())
        .with(stdlib::dcterms::module().clone())
        .with(owl::module().clone())
        .with(rdf::module().clone())
        .with(rdfs::module().clone())
        .with(skos::module().clone())
        .with(xsd::module().clone())
}

impl Default for LibraryConfiguration {
    fn default() -> Self {
        Self {
            modules: HashMap::from_iter(vec![
                (
                    ModulePath::new_unchecked(true, &[stdlib::PATH_ROOT_SEGMENT_IO]),
                    vec![LibraryModule::named(sdml::MODULE_NAME)
                        .aliased_as_is()
                        .builtin()
                        .with_datatypes(&[sdml::BINARY, sdml::IRI, sdml::UNSIGNED])],
                ),
                (
                    ModulePath::new_unchecked(
                        true,
                        &[
                            stdlib::PATH_ROOT_SEGMENT_ORG,
                            stdlib::PATH_STDORG_SEGMENT_GS1,
                        ],
                    ),
                    vec![LibraryModule::named("gln"), LibraryModule::named("gtin")],
                ),
                (
                    ModulePath::new_unchecked(
                        true,
                        &[
                            stdlib::PATH_ROOT_SEGMENT_ORG,
                            stdlib::PATH_STDORG_SEGMENT_ISO,
                        ],
                    ),
                    vec![
                        LibraryModule::named("iso_17442"),
                        LibraryModule::named("iso_3166"),
                        LibraryModule::named("iso_4217"),
                        LibraryModule::named("iso_639_1"),
                        LibraryModule::named("iso_9362"),
                    ],
                ),
                (
                    ModulePath::new_unchecked(
                        true,
                        &[
                            stdlib::PATH_ROOT_SEGMENT_ORG,
                            stdlib::PATH_ORG_SEGMENT_PURL,
                            stdlib::PATH_STDORG_SEGMENT_DC,
                        ],
                    ),
                    vec![
                        LibraryModule::named("elements").alias("dc").builtin(),
                        LibraryModule::named("terms").alias("dcterms").builtin(),
                        LibraryModule::named("dcam").aliased_as_is(),
                        LibraryModule::named("dcmitype").aliased_as_is(),
                    ],
                ),
                (
                    ModulePath::new_unchecked(
                        true,
                        &[
                            stdlib::PATH_ROOT_SEGMENT_ORG,
                            stdlib::PATH_STDORG_SEGMENT_W3C,
                        ],
                    ),
                    vec![
                        LibraryModule::named(stdlib::owl::MODULE_NAME)
                            .aliased_as_is()
                            .builtin()
                            .with_datatypes(&[owl::RATIONAL, owl::REAL]),
                        LibraryModule::named(rdf::MODULE_NAME)
                            .aliased_as_is()
                            .builtin(),
                        LibraryModule::named(rdfs::MODULE_NAME)
                            .aliased_as_is()
                            .builtin(),
                        LibraryModule::named(skos::MODULE_NAME)
                            .aliased_as_is()
                            .builtin(),
                        LibraryModule::named(xsd::MODULE_NAME)
                            .aliased_as_is()
                            .builtin()
                            .with_datatypes(&[
                                xsd::ANY_URI,
                                xsd::BASE64_BINARY,
                                xsd::BOOLEAN,
                                xsd::DATE,
                                xsd::DATETIME,
                                xsd::DECIMAL,
                                xsd::DOUBLE,
                                xsd::DURATION,
                                xsd::FLOAT,
                                xsd::GDAY,
                                xsd::GMONTH,
                                xsd::GMONTH_DAY,
                                xsd::GYEAR_MONTH,
                                xsd::GYEAR,
                                xsd::HEX_BINARY,
                                xsd::STRING,
                                xsd::TIME,
                                xsd::DATETIME_STAMP,
                                xsd::DAYTIME_DURATION,
                                xsd::YEARMONTH_DURATION,
                                xsd::INTEGER,
                                xsd::LONG,
                                xsd::INT,
                                xsd::SHORT,
                                xsd::BYTE,
                                xsd::NONNEGATIVE_INTEGER,
                                xsd::POSITIVE_INTEGER,
                                xsd::UNSIGNED_LONG,
                                xsd::UNSIGNED_INT,
                                xsd::UNSIGNED_SHORT,
                                xsd::UNSIGNED_BYTE,
                                xsd::NONPOSITIVE_INTEGER,
                                xsd::NEGATIVE_INTEGER,
                                xsd::NORMALIZED_STRING,
                                xsd::TOKEN,
                                xsd::LANGUAGE,
                            ]),
                    ],
                ),
            ]),
            rdf_definition_allow: BTreeSet::from_iter(vec![Identifier::new_unchecked(
                "example_rdf_defs",
            )]),
            typeclass_definition_allow: BTreeSet::from_iter(vec![Identifier::new_unchecked(
                "example_typeclass_defs",
            )]),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn config_to_library_names() -> BTreeSet<&'static str> {
    library_module_configuration()
        .values()
        .map(|vs| vs.iter().map(|v| v.name.as_ref()))
        .flatten()
        .chain(
            library_module_configuration()
                .values()
                .map(|vs| {
                    vs.iter().filter_map(|v| {
                        if let Some(name) = &v.compatibility_alias {
                            Some(name.as_ref())
                        } else {
                            None
                        }
                    })
                })
                .flatten(),
        )
        .collect()
}

fn config_to_type_names() -> BTreeSet<&'static str> {
    library_module_configuration()
        .values()
        .map(|vs| {
            vs.iter()
                .map(|v| v.base_datatypes.iter().map(|name| name.as_ref()))
        })
        .flatten()
        .flatten()
        .collect()
}

fn config_to_rdf_allow_list() -> BTreeSet<&'static str> {
    library_configuration()
        .rdf_definition_allow
        .iter()
        .map(|s| s.as_ref())
        .collect()
}

fn config_to_typeclass_allow_list() -> BTreeSet<&'static str> {
    library_configuration()
        .typeclass_definition_allow
        .iter()
        .map(|s| s.as_ref())
        .collect()
}

#[cfg(not(feature = "stdlib-ext-config"))]
pub fn load_library_configuration() -> LibraryConfiguration {
    LibraryConfiguration::default()
}

#[cfg(feature = "stdlib-ext-config")]
mod external;
#[cfg(feature = "stdlib-ext-config")]
use external::load_library_configuration;

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library_config_file_loads() {
        let library_modules = library_module_configuration();
        assert_eq!(library_modules.len(), 6);
    }

    #[test]
    fn test_is_library_module() {
        assert!(is_library_module_str("dc"));
        assert!(is_library_module_str("dcam"));
        assert!(is_library_module_str("dcmitype"));
        assert!(is_library_module_str("dcterms"));
        assert!(is_library_module_str("elements"));
        assert!(is_library_module_str("terms"));
        assert!(is_library_module_str("iso_17442"));
        assert!(is_library_module_str("iso_3166"));
        assert!(is_library_module_str("iso_4217"));
        assert!(is_library_module_str("iso_639_1"));
        assert!(is_library_module_str("iso_9362"));
        assert!(is_library_module_str("owl"));
        assert!(is_library_module_str("rdf"));
        assert!(is_library_module_str("rdfs"));
        assert!(is_library_module_str("skos"));
        assert!(is_library_module_str("sdml"));
        assert!(is_library_module_str("xsd"));
    }

    #[test]
    fn test_is_builtin_type_name() {
        // SDML
        assert!(is_builtin_type_name_str("binary"));
        assert!(is_builtin_type_name_str("iri"));
        assert!(is_builtin_type_name_str("unsigned"));
        // OWL
        assert!(is_builtin_type_name_str("rational"));
        assert!(is_builtin_type_name_str("real"));
        // XSD
        assert!(is_builtin_type_name_str("anyURI"));
        assert!(is_builtin_type_name_str("base64Binary"));
        assert!(is_builtin_type_name_str("boolean"));
        assert!(is_builtin_type_name_str("date"));
        assert!(is_builtin_type_name_str("dateTime"));
        assert!(is_builtin_type_name_str("decimal"));
        assert!(is_builtin_type_name_str("double"));
        assert!(is_builtin_type_name_str("duration"));
        assert!(is_builtin_type_name_str("float"));
        assert!(is_builtin_type_name_str("gDay"));
        assert!(is_builtin_type_name_str("gMonth"));
        assert!(is_builtin_type_name_str("gMonthDay"));
        assert!(is_builtin_type_name_str("gYearMonth"));
        assert!(is_builtin_type_name_str("gYear"));
        assert!(is_builtin_type_name_str("hexBinary"));
        assert!(is_builtin_type_name_str("string"));
        assert!(is_builtin_type_name_str("time"));
        assert!(is_builtin_type_name_str("dateTimeStamp"));
        assert!(is_builtin_type_name_str("dayTimeDuration"));
        assert!(is_builtin_type_name_str("yearMonthDuration"));
        assert!(is_builtin_type_name_str("integer"));
        assert!(is_builtin_type_name_str("long"));
        assert!(is_builtin_type_name_str("int"));
        assert!(is_builtin_type_name_str("short"));
        assert!(is_builtin_type_name_str("byte"));
        assert!(is_builtin_type_name_str("nonNegativeInteger"));
        assert!(is_builtin_type_name_str("positiveInteger"));
        assert!(is_builtin_type_name_str("unsignedLong"));
        assert!(is_builtin_type_name_str("unsignedInt"));
        assert!(is_builtin_type_name_str("unsignedShort"));
        assert!(is_builtin_type_name_str("unsignedByte"));
        assert!(is_builtin_type_name_str("nonPositiveInteger"));
        assert!(is_builtin_type_name_str("negativeInteger"));
        assert!(is_builtin_type_name_str("normalizedString"));
        assert!(is_builtin_type_name_str("token"));
        assert!(is_builtin_type_name_str("language"));
    }

    #[test]
    fn test_is_rdf_definition_allowed_in_module() {
        assert!(is_rdf_definition_allowed_in_module_str("dc"));
        assert!(is_rdf_definition_allowed_in_module_str("dcam"));
        assert!(is_rdf_definition_allowed_in_module_str("dcmitype"));
        assert!(is_rdf_definition_allowed_in_module_str("dcterms"));
        assert!(is_rdf_definition_allowed_in_module_str("elements"));
        assert!(is_rdf_definition_allowed_in_module_str("terms"));
        assert!(is_rdf_definition_allowed_in_module_str("iso_17442"));
        assert!(is_rdf_definition_allowed_in_module_str("iso_3166"));
        assert!(is_rdf_definition_allowed_in_module_str("iso_4217"));
        assert!(is_rdf_definition_allowed_in_module_str("iso_639_1"));
        assert!(is_rdf_definition_allowed_in_module_str("iso_9362"));
        assert!(is_rdf_definition_allowed_in_module_str("owl"));
        assert!(is_rdf_definition_allowed_in_module_str("rdf"));
        assert!(is_rdf_definition_allowed_in_module_str("rdfs"));
        assert!(is_rdf_definition_allowed_in_module_str("skos"));
        assert!(is_rdf_definition_allowed_in_module_str("sdml"));
        assert!(is_rdf_definition_allowed_in_module_str("xsd"));

        assert!(is_rdf_definition_allowed_in_module_str("example_rdf_defs"));
        assert!(!is_rdf_definition_allowed_in_module_str(
            "example_typeclass_defs"
        ));
    }

    #[test]
    fn test_is_typeclass_definition_allowed_in_module() {
        assert!(is_typeclass_definition_allowed_in_module_str("dc"));
        assert!(is_typeclass_definition_allowed_in_module_str("dcam"));
        assert!(is_typeclass_definition_allowed_in_module_str("dcmitype"));
        assert!(is_typeclass_definition_allowed_in_module_str("dcterms"));
        assert!(is_typeclass_definition_allowed_in_module_str("elements"));
        assert!(is_typeclass_definition_allowed_in_module_str("terms"));
        assert!(is_typeclass_definition_allowed_in_module_str("iso_17442"));
        assert!(is_typeclass_definition_allowed_in_module_str("iso_3166"));
        assert!(is_typeclass_definition_allowed_in_module_str("iso_4217"));
        assert!(is_typeclass_definition_allowed_in_module_str("iso_639_1"));
        assert!(is_typeclass_definition_allowed_in_module_str("iso_9362"));
        assert!(is_typeclass_definition_allowed_in_module_str("owl"));
        assert!(is_typeclass_definition_allowed_in_module_str("rdf"));
        assert!(is_typeclass_definition_allowed_in_module_str("rdfs"));
        assert!(is_typeclass_definition_allowed_in_module_str("skos"));
        assert!(is_typeclass_definition_allowed_in_module_str("sdml"));
        assert!(is_typeclass_definition_allowed_in_module_str("xsd"));

        assert!(is_typeclass_definition_allowed_in_module_str(
            "example_typeclass_defs"
        ));
        assert!(!is_typeclass_definition_allowed_in_module_str(
            "example_rdf_defs"
        ));
    }
}
