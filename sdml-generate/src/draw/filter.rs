/*!
This module provides a capability to filter elements from a diagram, specifically the UML Class
diagram.

The intent is less for clients to use the API directly but to be able to load a diagram
*configuration* from a persisted JSON file.

# Filters

All filters are exact as *exclusions*, that is they define names to **not** show for their context.

## Import Filters

These determine which imported modules and definitions should be shown.

The first is the *module import filter* which is a **name filter** that will excluded from the diagram entirely any
matching module if the current module imports them. The JSON configuration file to hide most of the stdlib modules is
shown here:


```json
{
  "module_import_filter": {
    "named": [
      "dc",
      "dc_terms",
      "owl",
      "rdf",
      "rdfs",
      "sdml",
      "skos",
      "xsd"
    ]
  }
}
```

The following listing shows how to construct the same filter with the API. Note that as it is a
common need to hide all stdlib imports you can use the `filter_stdlib_imports` method on
`DiagramContentFilter` to apply a filter to exclude all stdlib modules.

```rust
use sdml_core::model::identifiers::Identifier;
use sdml_generate::draw::filter::{DiagramContentFilter, NameFilter};
use std::str::FromStr;

let filter = DiagramContentFilter::default()
    .with_module_import_filter(
        NameFilter::from_iter(
            vec![
                "dc", "dc_terms", "owl","rdf", "rdfs", "sdml", "skos", "xsd",
            ]
            .into_iter()
            .map(|s| Identifier::from_str(s).unwrap())
            .collect::<Vec<_>>()
        )
    );
assert!(filter.draw_module_import(&Identifier::from_str("mine").unwrap()));
assert!(!filter.draw_module_import(&Identifier::from_str("rdfs").unwrap()));
```

The second filter type is the *member import* filter which uses a **qualified name filter**, a map from module name to a *name
filter*. The following JSON shows how this map is represented.

```json
{
  "member_import_filter": {
    "xsd": {
      "matches": "^[A-Z]+$"
    },
    "sdml": "all"
  }
}
```

The following is the API example corresponding to the JSON above.

```rust
use regex::Regex;
use sdml_core::model::identifiers::Identifier;
use sdml_generate::draw::filter::{
    DiagramContentFilter, IdentifierString, NameFilter, QualifiedNameFilter,
};
use std::str::FromStr;

let filter = DiagramContentFilter::default()
    .with_member_import_filter(
        QualifiedNameFilter::from(vec![
            (IdentifierString::from_str("sdml").unwrap(), NameFilter::All,),
            (
                IdentifierString::from_str("xsd").unwrap(),
                Regex::new("^[A-Z]+$").unwrap().into(),
            )
        ])
    );
assert!(filter.draw_member_import_pair(
    &Identifier::from_str("mine").unwrap(),
    &Identifier::from_str("MyType").unwrap(),
));
assert!(!filter.draw_member_import_pair(
    &Identifier::from_str("sdml").unwrap(),
    &Identifier::from_str("string").unwrap(),
));
assert!(filter.draw_member_import_pair(
    &Identifier::from_str("xsd").unwrap(),
    &Identifier::from_str("integer").unwrap(),
));
assert!(!filter.draw_member_import_pair(
    &Identifier::from_str("xsd").unwrap(),
    &Identifier::from_str("NMTOKEN").unwrap(),
));
```
 */

use regex::Regex;
use sdml_core::model::{
    definitions::Definition,
    identifiers::{Identifier, QualifiedIdentifier},
    modules::Import,
    HasName,
};
use sdml_core::stdlib;
use sdml_errors::Error;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt::Display,
    fs::OpenOptions,
    io::{BufReader, Read, Write},
    path::Path,
    str::FromStr,
};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// A *content filter* is created to determine elements of a module to **exclude** from a diagram.
/// For example, you may wish to create a diagram with only structures and create associations
/// for only target types which are also structures.
///
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct DiagramContentFilter {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    module_import_filter: Option<NameFilter>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    member_import_filter: Option<QualifiedNameFilter>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    annotation_filter: Option<QualifiedNameFilter>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    definition_filter: Vec<DefinitionFilter>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    association_filter: Vec<DefinitionFilter>,
}

///
/// A *name filter* filters out elements that match by their name (an `Identifier`).
///
/// Name filters have three actions:
///
/// 1. Apply to `All` names in a scope.
/// 2. Apply to any name in the `Named` list.
/// 3. Apply to any name that `Matches` the provided regex.
///
/// The following demonstrates all three of these actions in a single member import filter.
///
/// ```json
/// {
///   "member_import_filter": {
///     "sdml": "all",
///     "skos": {
///       "named": [
///         "changeNote",
///         "editorialNote",
///         "historyNote",
///         "scopeNote"
///       ],
///     },
///     "xsd": {
///       "matches": "^[A-Z]+$"
///     }
///   }
/// }
/// ```
///
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum NameFilter {
    Named(Vec<IdentifierString>),
    Matches(#[serde(with = "serde_regex")] Regex),
    All,
}

///
/// This type is a wrapper around the core `Identifier` type to allow serde to serialize and deserialize names as
/// strings.
///
#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(try_from = "String", into = "String")]
pub struct IdentifierString(Identifier);

///
/// A *qualified name filter* is a map from a namespace name to a `NameFilter` to exclude members of the namespace.
///
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QualifiedNameFilter {
    #[serde(flatten)]
    name_map: HashMap<IdentifierString, NameFilter>,
}

///
/// This enumeration represents the set of `Definition` types that are present in a module.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DefinitionKind {
    Datatype,
    Entity,
    Enum,
    Event,
    Property,
    Rdf,
    Structure,
    TypeClass,
    Union,
}

///
/// A *definition filter* allows the hiding of definitions in the current module using a combination of [`DefinitionKind`]
/// and [`NameFilter`]. In a similar way to name filters there are three actions:
///
/// 1. Apply to all definitions of `Kind`.
/// 2. Apply to all definitions `Named` according to a name filter.
/// 3. Apply to all definitions where `Both` the kind *and*  name filter match.
///
/// ```json
/// {
///  "definition_filter": [
///     {
///       "both": {
///         "kind": "enum",
///         "names": {
///           "matches": "_AC$"
///         }
///       }
///     }
///   ]
/// }
/// ```
///
/// ```rust
/// use regex::Regex;
/// use sdml_generate::draw::filter::{
///     DefinitionFilter, DefinitionKind, DiagramContentFilter
/// };
///
/// let filter = DiagramContentFilter::default()
///     .with_definition_filter(
///         DefinitionFilter::Both {
///             kind: DefinitionKind::Enum,
///             names: Regex::new("_AC$").unwrap().into(),
///         }
///     );
/// ```
///
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DefinitionFilter {
    Kind {
        #[serde(flatten)]
        kind: DefinitionKind,
    },
    Named {
        #[serde(flatten)]
        names: NameFilter,
    },
    Both {
        kind: DefinitionKind,
        names: NameFilter,
    },
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl DiagramContentFilter {
    ///
    /// Builder function to set the *module import filter*.
    ///
    pub fn with_module_import_filter(self, filter: NameFilter) -> Self {
        Self {
            module_import_filter: Some(filter),
            ..self
        }
    }

    ///
    /// Builder function to set the *module import filter* to exclude all standard library modules.
    ///
    pub fn filter_stdlib_imports(self) -> Self {
        let mut self_mut = self;

        let stdlib_names: Vec<IdentifierString> = [
            stdlib::dc::MODULE_NAME,
            stdlib::dc::terms::MODULE_NAME,
            stdlib::iso_3166::MODULE_NAME,
            stdlib::iso_4217::MODULE_NAME,
            stdlib::owl::MODULE_NAME,
            stdlib::rdf::MODULE_NAME,
            stdlib::rdfs::MODULE_NAME,
            stdlib::sdml::MODULE_NAME,
            stdlib::skos::MODULE_NAME,
            stdlib::xsd::MODULE_NAME,
        ]
        .into_iter()
        .map(Identifier::new_unchecked)
        .map(IdentifierString::from)
        .collect();

        if let Some(NameFilter::Named(filter)) = &mut self_mut.module_import_filter {
            filter.extend(stdlib_names);
        } else {
            self_mut.module_import_filter = Some(NameFilter::from_iter(stdlib_names))
        }

        self_mut
    }

    ///
    /// Builder function to set the *member import filter*.
    ///
    pub fn with_member_import_filter<F>(self, filter: F) -> Self
    where
        F: Into<QualifiedNameFilter>,
    {
        Self {
            member_import_filter: Some(filter.into()),
            ..self
        }
    }

    ///
    /// Builder function to set the *annotation filter*.
    ///
    pub fn with_annotation_filter<F>(self, filter: F) -> Self
    where
        F: Into<QualifiedNameFilter>,
    {
        Self {
            annotation_filter: Some(filter.into()),
            ..self
        }
    }

    ///
    /// Builder function to set the *definition filter*.
    ///
    pub fn with_definition_filter<F>(self, filter: F) -> Self
    where
        F: Into<DefinitionFilter>,
    {
        Self {
            definition_filter: vec![filter.into()],
            ..self
        }
    }

    ///
    /// Builder function to set the *definition filter*.
    ///
    pub fn with_definition_filters(self, filter: Vec<DefinitionFilter>) -> Self {
        Self {
            definition_filter: filter,
            ..self
        }
    }

    ///
    /// Builder function to set the *association filter*.
    ///
    pub fn with_association_filter<F>(self, filter: F) -> Self
    where
        F: Into<DefinitionFilter>,
    {
        Self {
            association_filter: vec![filter.into()],
            ..self
        }
    }

    ///
    /// Builder function to set the *association filter*.
    ///
    pub fn with_association_filters(self, filter: Vec<DefinitionFilter>) -> Self {
        Self {
            association_filter: filter,
            ..self
        }
    }

    ///
    /// Returns `true` if the filter has no sub-filters, else `false`.
    ///
    pub fn is_empty(&self) -> bool {
        self.module_import_filter.is_none()
            && self.member_import_filter.is_none()
            && self.definition_filter.is_empty()
            && self.association_filter.is_empty()
    }

    pub fn draw_import(&self, id: &Import) -> bool {
        match id {
            Import::Module(v) => self.draw_module_import(v.name()),
            Import::Member(v) => self.draw_member_import(v),
        }
    }

    ///
    /// Returns `true` if the diagram should draw an import relationship to the module named
    /// `id`, else `false`.
    ///
    pub fn draw_module_import(&self, id: &Identifier) -> bool {
        !self
            .module_import_filter
            .as_ref()
            .map(|filter| filter.is_excluded(id))
            .unwrap_or_default()
    }

    ///
    /// Returns `true` if the diagram should draw an import relationship to the member named
    /// `id`, else `false`.
    ///
    pub fn draw_member_import(&self, id: &QualifiedIdentifier) -> bool {
        !self
            .member_import_filter
            .as_ref()
            .map(|filter| filter.is_excluded(id))
            .unwrap_or_default()
    }

    ///
    /// Returns `true` if the diagram should draw an import relationship to the module named
    /// `id` in module `nsid`, else `false`.
    ///
    pub fn draw_member_import_pair(&self, nsid: &Identifier, id: &Identifier) -> bool {
        !self
            .member_import_filter
            .as_ref()
            .map(|filter| filter.is_excluded_pair(nsid, id))
            .unwrap_or_default()
    }

    ///
    /// Returns `true` if the diagram should draw the definition `defn`, else `false`.
    ///
    pub fn draw_definition(&self, defn: &Definition) -> bool {
        !self
            .definition_filter
            .iter()
            .any(|filter| filter.is_excluded(defn))
    }

    pub fn draw_definition_named(
        &self,
        subject_kind: DefinitionKind,
        subject_id: &Identifier,
    ) -> bool {
        !self
            .definition_filter
            .iter()
            .any(|filter| filter.is_excluded_pair(subject_kind, subject_id))
    }

    ///
    /// Returns `true` if the diagram should draw a member as an association
    /// (or attribute) depending on it's target type, else `false`.
    ///
    pub fn draw_member_as_association(&self, defn: &Definition) -> bool {
        self.association_filter
            .iter()
            .any(|names| names.is_excluded(defn))
    }

    ///
    /// Write this filter to the file named `file` in JSON format.
    ///
    pub fn write_to_file<P>(&self, file: P) -> Result<(), Error>
    where
        P: AsRef<Path>,
    {
        self.write_to_writer(
            OpenOptions::new()
                .create_new(true)
                .write(true)
                .truncate(true)
                .open(file)?,
        )
    }

    ///
    /// Write this filter to the provided writer in JSON format.
    ///
    pub fn write_to_writer<W>(&self, writer: W) -> Result<(), Error>
    where
        W: Write,
    {
        serde_json::to_writer_pretty(writer, self).map_err(into_generator_error)
    }

    ///
    /// Read a filter from the file named `file` in JSON format.
    ///
    pub fn read_from_file<P>(file: P) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        let file = OpenOptions::new().read(true).open(file)?;
        let reader = BufReader::new(file);
        Self::read_from_reader(reader)
    }

    ///
    /// Read a filter from the provided reader in JSON format.
    ///
    pub fn read_from_reader<R>(reader: R) -> Result<Self, Error>
    where
        R: Read,
    {
        let filter: Self = serde_json::from_reader(reader).map_err(into_generator_error)?;
        Ok(filter)
    }
}

// ------------------------------------------------------------------------------------------------

impl From<&Definition> for DefinitionKind {
    fn from(value: &Definition) -> Self {
        match value {
            Definition::Datatype(_) => Self::Datatype,
            Definition::Entity(_) => Self::Entity,
            Definition::Enum(_) => Self::Enum,
            Definition::Event(_) => Self::Event,
            Definition::Property(_) => Self::Property,
            Definition::Rdf(_) => Self::Rdf,
            Definition::Structure(_) => Self::Structure,
            Definition::TypeClass(_) => Self::TypeClass,
            Definition::Union(_) => Self::Union,
        }
    }
}

impl DefinitionKind {
    ///
    /// Returns `true` if the definition is excluded by virtue of matching this specific
    /// definition kind, else `false`.
    ///
    pub fn is_excluded(&self, defn: &Definition) -> bool {
        *self == defn.into()
    }
}

// ------------------------------------------------------------------------------------------------

impl From<DefinitionKind> for DefinitionFilter {
    fn from(kind: DefinitionKind) -> Self {
        Self::Kind { kind }
    }
}

impl From<NameFilter> for DefinitionFilter {
    fn from(value: NameFilter) -> Self {
        Self::Named { names: value }
    }
}

impl From<(DefinitionKind, NameFilter)> for DefinitionFilter {
    fn from(value: (DefinitionKind, NameFilter)) -> Self {
        Self::Both {
            kind: value.0,
            names: value.1,
        }
    }
}

impl DefinitionFilter {
    ///
    /// Constructor for a filter based on a [`NameFilter`].
    ///
    pub fn exclude_named<F>(names: F) -> Self
    where
        F: Into<NameFilter>,
    {
        Self::Named {
            names: names.into(),
        }
    }

    ///
    /// Constructor for a filter that excludes all datatype definitions.
    ///
    pub fn exclude_datatypes() -> Self {
        Self::Kind {
            kind: DefinitionKind::Datatype,
        }
    }

    ///
    /// Constructor for a filter that excludes all entity definitions.
    ///
    pub fn exclude_entities() -> Self {
        Self::Kind {
            kind: DefinitionKind::Entity,
        }
    }

    ///
    /// Constructor for a filter that excludes all enum definitions.
    ///
    pub fn exclude_enums() -> Self {
        Self::Kind {
            kind: DefinitionKind::Enum,
        }
    }

    ///
    /// Constructor for a filter that excludes all event definitions.
    ///
    pub fn exclude_events() -> Self {
        Self::Kind {
            kind: DefinitionKind::Event,
        }
    }

    ///
    /// Constructor for a filter that excludes all property definitions.
    ///
    pub fn exclude_properties() -> Self {
        Self::Kind {
            kind: DefinitionKind::Property,
        }
    }

    ///
    /// Constructor for a filter that excludes all rdf definitions.
    ///
    pub fn exclude_rdfs() -> Self {
        Self::Kind {
            kind: DefinitionKind::Rdf,
        }
    }

    ///
    /// Constructor for a filter that excludes all structure definitions.
    ///
    pub fn exclude_structures() -> Self {
        Self::Kind {
            kind: DefinitionKind::Structure,
        }
    }

    ///
    /// Constructor for a filter that excludes all type-class definitions.
    ///
    pub fn exclude_typeclasses() -> Self {
        Self::Kind {
            kind: DefinitionKind::TypeClass,
        }
    }

    ///
    /// Constructor for a filter that excludes all union definitions.
    ///
    pub fn exclude_unions() -> Self {
        Self::Kind {
            kind: DefinitionKind::Union,
        }
    }

    ///
    /// Constructor for a filter that excludes any datatype definition that match the provided [`NameFilter`].
    ///
    pub fn exclude_datatypes_named_named<F>(names: F) -> Self
    where
        F: Into<NameFilter>,
    {
        Self::Both {
            kind: DefinitionKind::Datatype,
            names: names.into(),
        }
    }

    ///
    /// Constructor for a filter that excludes any entity definition that match the provided [`NameFilter`].
    ///
    pub fn exclude_entities_named<F>(names: F) -> Self
    where
        F: Into<NameFilter>,
    {
        Self::Both {
            kind: DefinitionKind::Entity,
            names: names.into(),
        }
    }

    ///
    /// Constructor for a filter that excludes any enum definition that match the provided [`NameFilter`].
    ///
    pub fn exclude_enums_named<F>(names: F) -> Self
    where
        F: Into<NameFilter>,
    {
        Self::Both {
            kind: DefinitionKind::Enum,
            names: names.into(),
        }
    }

    ///
    /// Constructor for a filter that excludes any event definition that match the provided [`NameFilter`].
    ///
    pub fn exclude_events_named<F>(names: F) -> Self
    where
        F: Into<NameFilter>,
    {
        Self::Both {
            kind: DefinitionKind::Event,
            names: names.into(),
        }
    }

    ///
    /// Constructor for a filter that excludes any property definition that match the provided [`NameFilter`].
    ///
    pub fn exclude_properties_named<F>(names: F) -> Self
    where
        F: Into<NameFilter>,
    {
        Self::Both {
            kind: DefinitionKind::Property,
            names: names.into(),
        }
    }

    ///
    /// Constructor for a filter that excludes any rdf definition that match the provided [`NameFilter`].
    ///
    pub fn exclude_rdfs_named<F>(names: F) -> Self
    where
        F: Into<NameFilter>,
    {
        Self::Both {
            kind: DefinitionKind::Rdf,
            names: names.into(),
        }
    }

    ///
    /// Constructor for a filter that excludes any structure definition that match the provided [`NameFilter`].
    ///
    pub fn exclude_structures_named<F>(names: F) -> Self
    where
        F: Into<NameFilter>,
    {
        Self::Both {
            kind: DefinitionKind::Structure,
            names: names.into(),
        }
    }

    ///
    /// Constructor for a filter that excludes any type-class definition that match the provided [`NameFilter`].
    ///
    pub fn exclude_typeclasses_named<F>(names: F) -> Self
    where
        F: Into<NameFilter>,
    {
        Self::Both {
            kind: DefinitionKind::TypeClass,
            names: names.into(),
        }
    }

    ///
    /// Constructor for a filter that excludes any union definition that match the provided [`NameFilter`].
    ///
    pub fn exclude_unions_named<F>(names: F) -> Self
    where
        F: Into<NameFilter>,
    {
        Self::Both {
            kind: DefinitionKind::Union,
            names: names.into(),
        }
    }

    ///
    /// Returns `true` if the definition `defn` is excluded, else `false`.
    ///
    pub fn is_excluded(&self, defn: &Definition) -> bool {
        self.is_excluded_pair(defn.into(), defn.name())
    }

    ///
    /// Returns `true` if a definition of `subject_kind` and name `subject_id` is excluded, else `false`.
    ///
    pub fn is_excluded_pair(&self, subject_kind: DefinitionKind, subject_id: &Identifier) -> bool {
        match self {
            Self::Kind { kind } => *kind == subject_kind,
            Self::Named { names } => names.is_excluded(subject_id),
            Self::Both { kind, names } => *kind == subject_kind || names.is_excluded(subject_id),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl From<HashMap<IdentifierString, NameFilter>> for QualifiedNameFilter {
    fn from(value: HashMap<IdentifierString, NameFilter>) -> Self {
        Self { name_map: value }
    }
}

impl From<(IdentifierString, NameFilter)> for QualifiedNameFilter {
    fn from(value: (IdentifierString, NameFilter)) -> Self {
        Self {
            name_map: HashMap::from_iter(vec![value]),
        }
    }
}

impl From<Vec<(IdentifierString, NameFilter)>> for QualifiedNameFilter {
    fn from(value: Vec<(IdentifierString, NameFilter)>) -> Self {
        Self {
            name_map: HashMap::from_iter(value),
        }
    }
}

impl QualifiedNameFilter {
    ///
    /// Returns `true` if the qualified name `qid` is excluded, else `false`.
    ///
    pub fn is_excluded(&self, qid: &QualifiedIdentifier) -> bool {
        self.is_excluded_pair(qid.module(), qid.member())
    }

    /// Returns `true` if the member `id` in namespace `nsid` is excluded, else `false`.
    pub fn is_excluded_pair(&self, nsid: &Identifier, id: &Identifier) -> bool {
        let nsid = IdentifierString::from(nsid.clone());
        self.name_map
            .get(&nsid)
            .map(|names| names.is_excluded(id))
            .unwrap_or_default()
    }
}

// ------------------------------------------------------------------------------------------------

impl From<Identifier> for NameFilter {
    fn from(value: Identifier) -> Self {
        Self::Named(vec![value.into()])
    }
}

impl From<IdentifierString> for NameFilter {
    fn from(value: IdentifierString) -> Self {
        Self::Named(vec![value])
    }
}

impl<I> FromIterator<I> for NameFilter
where
    I: Into<IdentifierString>,
{
    fn from_iter<T: IntoIterator<Item = I>>(iter: T) -> Self {
        Self::Named(Vec::from_iter(iter.into_iter().map(|i| i.into())))
    }
}

impl From<Regex> for NameFilter {
    fn from(value: Regex) -> Self {
        Self::Matches(value)
    }
}

impl NameFilter {
    ///
    /// Returns `true` if the name `id` is excluded, else `false`.
    ///
    pub fn is_excluded(&self, id: &Identifier) -> bool {
        let id = IdentifierString::from(id.clone());
        match self {
            Self::Named(names) => names.contains(&id),
            Self::Matches(regex) => regex.is_match(id.as_ref()),
            Self::All => true,
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for IdentifierString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Identifier> for IdentifierString {
    fn from(value: Identifier) -> Self {
        Self(value)
    }
}

impl From<IdentifierString> for Identifier {
    fn from(value: IdentifierString) -> Self {
        value.0
    }
}

impl From<IdentifierString> for String {
    fn from(value: IdentifierString) -> Self {
        value.0.into()
    }
}

impl FromStr for IdentifierString {
    type Err = sdml_errors::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Identifier::from_str(s)?))
    }
}

impl TryFrom<String> for IdentifierString {
    type Error = sdml_errors::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self(Identifier::from_str(&value)?))
    }
}

impl AsRef<str> for IdentifierString {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn into_generator_error(e: serde_json::Error) -> Error {
    crate::errors::into_generator_error("draw::filter", e)
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{from_str, to_string_pretty, Error};

    #[test]
    fn test_create_empty_filter_json() {
        println!(
            "{}",
            to_string_pretty(&DiagramContentFilter::default()).unwrap()
        );
    }

    #[test]
    fn test_parse_empty_filter_json() {
        let result: Result<DiagramContentFilter, Error> = from_str("{}");
        println!("{:?}", result);
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn test_create_no_stdlib_filter_json() {
        println!(
            "{}",
            to_string_pretty(&DiagramContentFilter::default().filter_stdlib_imports()).unwrap()
        );
    }

    #[test]
    fn test_parse_no_stdlib_filter_json() {
        let result: Result<DiagramContentFilter, Error> = from_str(
            r#"{
  "module_import_filter": {
    "named": [
      "dc",
      "dc_terms",
      "iso_3166",
      "iso_4217",
      "owl",
      "rdf",
      "rdfs",
      "sdml",
      "skos",
      "xsd"
    ]
  }
}"#,
        );
        println!("{:?}", result);
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_member_import_filter_json() {
        println!(
            "{}",
            to_string_pretty(&DiagramContentFilter::default().with_member_import_filter(
                QualifiedNameFilter::from(vec![
                    (IdentifierString::from_str("sdml").unwrap(), NameFilter::All,),
                    (
                        IdentifierString::from_str("xsd").unwrap(),
                        Regex::new("^[A-Z]+$").unwrap().into(),
                    )
                ])
            ))
            .unwrap()
        );
    }

    #[test]
    fn test_parse_member_import_filter_json() {
        let result: Result<DiagramContentFilter, Error> = from_str(
            r#"{
  "member_import_filter": {
    "sdml": "all",
    "xsd": {
      "matches": "^[A-Z]+$"
    }
  }
}"#,
        );
        println!("{:?}", result);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_member_import_filter_all_json() {
        let result: Result<DiagramContentFilter, Error> = from_str(
            r#"{
  "member_import_filter": {
    "sdml": "all",
    "skos": {
      "named": [
        "changeNote",
        "editorialNote",
        "historyNote",
        "scopeNote"
      ]
    },
    "xsd": {
      "matches": "^[A-Z]+$"
    }
  }
}"#,
        );
        println!("{:?}", result);
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_enum_regex_destination_filter_json() {
        println!(
            "{}",
            to_string_pretty(&DiagramContentFilter::default().with_definition_filter(
                DefinitionFilter::Both {
                    kind: DefinitionKind::Enum,
                    names: Regex::new("_AC$").unwrap().into(),
                }
            ))
            .unwrap()
        );
    }

    #[test]
    fn test_parse_enum_regex_destination_filter_json() {
        let result: Result<DiagramContentFilter, Error> = from_str(
            r#"{
  "definition_filter": [
    {
      "both": {
        "kind": "enum",
        "names": {
          "matches": "_AC$"
        }
      }
    }
  ]
}"#,
        );
        println!("{:?}", result);
        assert!(result.is_ok());
    }
}
