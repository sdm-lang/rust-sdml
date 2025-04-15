/*!
One-line description.

TBD

# Example

TBD

 */

use crate::{
    config::{LibraryConfiguration, LibraryModule},
    model::{identifiers::Identifier, modules::ModulePath},
};
use serde_json::{self, Map, Value};
use std::{
    collections::{BTreeSet, HashMap},
    path::PathBuf,
    str::FromStr,
};
use xdirs::config_dir_for;

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
enum LibraryConfigurationError {
    ParserError(String),
    MissingKey(&'static str),
    InvalidValueType(&'static str, &'static str),
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl std::fmt::Display for LibraryConfigurationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "Library Configuration Error: {}",
            match self {
                Self::ParserError(error) => format!("JSON Parser: error: {error}"),
                Self::MissingKey(key) => format!("Missing Key: '{key}'"),
                Self::InvalidValueType(expecting, actual) => format!(
                    "Invalid JSON Value Variant: expecting '{expecting}', actual: '{actual}'"
                ),
            }
        )
    }
}

impl std::error::Error for LibraryConfigurationError {}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

const DEFAULT_DATATYPE_CONFIG: &str = include_str!("sdml_stdlib.json");
const CONFIG_LOCATION_VAR_NAME: &str = "SDML_CONFIG_HOME";
const LIBRARY_CONFIG_FILE_NAME: &str = "sdml_stdlib.json";

pub(super) fn load_library_configuration() -> LibraryConfiguration {
    match parse_configuration(get_configuration()) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("{e:?}");
            tracing::error!("{e:?}");
            LibraryConfiguration::default()
        }
    }
}

fn parse_configuration(source: String) -> Result<LibraryConfiguration, LibraryConfigurationError> {
    match serde_json::from_str(&source) {
        Ok(Value::Object(top)) => {
            if let Some(Value::Object(modules)) = top.get("modules") {
                let mut config = LibraryConfiguration {
                    modules: parse_library_modules(modules)?,
                    rdf_definition_allow: Default::default(),
                    typeclass_definition_allow: Default::default(),
                };
                if let Some(Value::Array(allow_list)) = top.get("rdf_definition_allow") {
                    config.rdf_definition_allow = parse_allow_list(allow_list)?;
                }
                if let Some(Value::Array(allow_list)) = top.get("typeclass_definition_allow") {
                    config.typeclass_definition_allow = parse_allow_list(allow_list)?;
                }
                Ok(config)
            } else {
                Err(LibraryConfigurationError::MissingKey("modules"))
            }
        }
        Ok(v) => Err(LibraryConfigurationError::InvalidValueType(
            "Object",
            value_actual_variant(&v),
        )),
        Err(e) => Err(LibraryConfigurationError::ParserError(e.to_string())),
    }
}

fn parse_library_modules(
    modules: &Map<String, Value>,
) -> Result<HashMap<ModulePath, Vec<LibraryModule>>, LibraryConfigurationError> {
    let mut lib_modules: HashMap<ModulePath, Vec<LibraryModule>> = HashMap::default();
    for (path, modules) in modules {
        if let Value::Array(modules) = modules {
            let path = ModulePath::from_str(path).unwrap();
            lib_modules.insert(path, parse_module_list(modules)?);
        } else {
            return Err(LibraryConfigurationError::InvalidValueType(
                "Array",
                value_actual_variant(modules),
            ));
        }
    }
    Ok(lib_modules)
}

fn parse_module_list(
    modules: &Vec<Value>,
) -> Result<Vec<LibraryModule>, LibraryConfigurationError> {
    let mut list = Vec::default();
    for module in modules {
        list.push(parse_module(module)?);
    }
    Ok(list)
}

fn parse_allow_list(
    allow_list_in: &Vec<Value>,
) -> Result<BTreeSet<Identifier>, LibraryConfigurationError> {
    let mut allow_list = BTreeSet::default();
    for value in allow_list_in {
        if let Value::String(name) = value {
            allow_list.insert(Identifier::new_unchecked(name));
        } else {
            return Err(LibraryConfigurationError::InvalidValueType(
                "String",
                value_actual_variant(value),
            ));
        }
    }
    Ok(allow_list)
}

#[inline]
fn value_actual_variant(v: &Value) -> &'static str {
    match v {
        Value::Null => "Null",
        Value::Bool(_) => "Bool",
        Value::Number(_) => "Number",
        Value::String(_) => "String",
        Value::Array(_) => "Array",
        Value::Object(_) => "Object",
    }
}

fn parse_module(module: &Value) -> Result<LibraryModule, LibraryConfigurationError> {
    if let Some(Value::String(name)) = module.get("name") {
        let mut lib_module = LibraryModule {
            name: Identifier::new_unchecked(name),
            compatibility_alias: None,
            builtin: false,
            base_datatypes: Default::default(),
        };
        match module.get("alias") {
            Some(Value::String(alias)) => {
                lib_module.compatibility_alias = Some(Identifier::new_unchecked(alias));
            }
            Some(v) => {
                return Err(LibraryConfigurationError::InvalidValueType(
                    "String",
                    value_actual_variant(v),
                ));
            }
            None => {}
        }
        match module.get("builtin") {
            Some(Value::Bool(builtin)) => {
                lib_module.builtin = *builtin;
            }
            Some(v) => {
                return Err(LibraryConfigurationError::InvalidValueType(
                    "Bool",
                    value_actual_variant(v),
                ));
            }
            None => {}
        }
        match module.get("datatypes") {
            Some(Value::Array(datatypes)) => {
                lib_module.base_datatypes = parse_datatype_list(datatypes)?;
            }
            Some(v) => {
                return Err(LibraryConfigurationError::InvalidValueType(
                    "Array",
                    value_actual_variant(v),
                ));
            }
            None => {}
        }
        Ok(lib_module)
    } else {
        Err(LibraryConfigurationError::MissingKey("name"))
    }
}

fn parse_datatype_list(
    datatypes: &Vec<Value>,
) -> Result<BTreeSet<Identifier>, LibraryConfigurationError> {
    let mut lib_datatypes = BTreeSet::new();
    for datatype in datatypes {
        if let Value::String(datatype) = datatype {
            lib_datatypes.insert(Identifier::new_unchecked(datatype));
        } else {
            return Err(LibraryConfigurationError::InvalidValueType(
                "String",
                value_actual_variant(datatype),
            ));
        }
    }
    Ok(lib_datatypes)
}

fn get_configuration() -> String {
    let location = if let Ok(value) = std::env::var(CONFIG_LOCATION_VAR_NAME) {
        Some(PathBuf::from(value))
    } else {
        config_dir_for("sdml")
    };
    if location
        .clone()
        .map(|mut path| {
            path.push(format!("config/{LIBRARY_CONFIG_FILE_NAME}"));
            path.exists()
        })
        .unwrap_or_default()
    {
        std::fs::read_to_string(location.unwrap()).unwrap_or(DEFAULT_DATATYPE_CONFIG.to_string())
    } else {
        DEFAULT_DATATYPE_CONFIG.to_string()
    }
}
