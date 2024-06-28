/*!
This module provides generators that create documentation of a module given its in-memory representation.

More detailed description, with

# Example

End of file during parsingSymbol’s value as variable is void: rustEnd of file during parsing

 */

use crate::convert::doc::common::ArgumentType;
use crate::convert::doc::common::Formatter;
use crate::errors::into_generator_error;
use sdml_core::cache::ModuleCache;
use sdml_core::error::Error;
use sdml_core::load::ModuleLoader;
use sdml_core::model::identifiers::{Identifier, IdentifierReference};
use sdml_core::model::modules::Module;
use serde::de::Visitor;
use serde::Deserializer;
use serde::Serializer;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::fs::read_to_string;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use tracing::error;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This structure models a heading which has a level and a string title. The level is unsigned but
/// a value of `0` denotes a /pseudo-heading/ (see [common::BlockFormat]). Also the notion of a
/// document title is separately specified at the page level (see [common::PageFormat]).
///
/// A heading may also include a label string that acts as an identifier, or anchor, for the
/// heading.
///
#[derive(Clone, Debug)]
pub struct Heading {
    level: u8,
    title: String,
    label: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AnnotationCategories {
    labels: Vec<IdentifierReference>,
    definitions: Vec<IdentifierReference>,
    descriptions: Vec<IdentifierReference>,
    comments: Vec<IdentifierReference>,
    references: Vec<IdentifierReference>,
}

pub trait DocumentationWriter<TInclude, TSource, F>
where
    TInclude: ArgumentType,
    TSource: ArgumentType,
    F: Formatter<TInclude, TSource>,
{
    fn formatter() -> &'static F;

    fn write_book<T>(
        &mut self,
        loader: &mut T,
        cache: &mut ModuleCache,
        book_config: BookConfig,
    ) -> Result<(), Error>
    where
        T: ModuleLoader;

    fn write_preamble<W>(
        &mut self,
        title: &str,
        language: &str,
        include_toc: bool,
        writer: &mut W,
    ) -> Result<(), Error>
    where
        W: Write + Sized;

    fn write_module_uml_overview<W>(
        &mut self,
        module: &Module,
        cache: &ModuleCache,
        writer: &mut W,
    ) -> Result<(), Error>
    where
        W: Write + Sized;

    fn write_module_description<W>(
        &mut self,
        heading: Heading,
        module: &Module,
        cache: &ModuleCache,
        writer: &mut W,
    ) -> Result<(), Error>
    where
        W: Write + Sized;

    fn write_module_definitions<W>(
        &mut self,
        heading_level: u8,
        module: &Module,
        cache: &ModuleCache,
        writer: &mut W,
    ) -> Result<(), Error>
    where
        W: Write + Sized;

    fn write_module_sdml_listing<W>(
        &mut self,
        heading: Heading,
        module: &Module,
        writer: &mut W,
    ) -> Result<(), Error>
    where
        W: Write + Sized;

    fn write_module_rdf_listing<W>(
        &mut self,
        heading: Heading,
        module: &Module,
        cache: &ModuleCache,
        writer: &mut W,
    ) -> Result<(), Error>
    where
        W: Write + Sized;

    fn write_module_dependency_table<W>(
        &mut self,
        module: &Module,
        cache: &ModuleCache,
        writer: &mut W,
    ) -> Result<(), Error>
    where
        W: Write + Sized;

    fn write_module_dependency_graph<W>(
        &mut self,
        heading: Heading,
        module: &Module,
        cache: &ModuleCache,
        writer: &mut W,
    ) -> Result<(), Error>
    where
        W: Write + Sized;
}

/// A content section names a list of [`ContentItem`]s.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ContentSection {
    /// The heading to display for this section.
    heading: String,
    /// The list of items contained in this section, including sub-sections.
    #[serde(default)]
    items: Vec<ContentItem>,
}

/// An item within a [`ContentSection`].
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ContentItem {
    /// Include a module by file path.
    SourceFile { module_path: PathBuf },
    /// Include a module by module name.
    SourceModule {
        #[serde(serialize_with = "identifier_to_string")]
        #[serde(deserialize_with = "identifier_from_string")]
        module: Identifier,
    },
    /// Include a file's content by file path.
    Include { include_file_path: PathBuf },
    /// Include a sub-section.
    Section { sub_section: ContentSection },
}

/// The generated document format.
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub enum DocumentFormat {
    /// Emacs Org-Mode
    #[default]
    OrgMode,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BookConfig {
    /// Title to generate for the book as a whole.
    title: String,
    /// An optional file to include between the page front-matter and the first content.
    #[serde(skip_serializing_if = "Option::is_none")]
    introduction: Option<PathBuf>,
    /// The first content section, this
    content: ContentSection,
    /// The format of the document itself, usually a markup language. The default value is
    /// [`DocumentFormat::OrgMode`].
    #[serde(default)]
    format: DocumentFormat,
    /// The name of the root document. Default is `"index.org"`
    output_file: PathBuf,
    /// If `true`, include a Table of Contents in the root document. Default is `true`
    #[serde(default = "default_to_true")]
    include_toc: bool,
    /// If `true`, attempt to construct the root document as a link-only file. Default is `true`
    #[serde(default = "default_to_true")]
    multi_part: bool,
    /// If `true`, attempt to copy any included file into the directory of the root file. Default is `false`.
    #[serde(default)]
    copy_includes: bool,
    /// A BCP-47 language-tag to identify the output language.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    language: Option<String>,
    /// Mapping from a category to a set of annotation Identifier references.
    #[serde(skip)]
    annotation_categories: AnnotationCategories,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

#[inline(always)]
fn default_to_true() -> bool {
    true
}

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for AnnotationCategories {
    fn default() -> Self {
        Self {
            labels: vec![
                // prefLabel is required.
                IdentifierReference::from_str("skos:altLabel").unwrap(),
                IdentifierReference::from_str("rdfs:label").unwrap(),
                IdentifierReference::from_str("sdml:srcLabel").unwrap(),
            ],
            definitions: vec![IdentifierReference::from_str("skos:definition").unwrap()],
            descriptions: vec![
                IdentifierReference::from_str("dc:description").unwrap(),
                IdentifierReference::from_str("dc_terms:description").unwrap(),
            ],
            comments: vec![
                IdentifierReference::from_str("rdfs:comment").unwrap(),
                IdentifierReference::from_str("skos:note").unwrap(),
                IdentifierReference::from_str("skos:changeNote").unwrap(),
                IdentifierReference::from_str("skos:editorialNote").unwrap(),
                IdentifierReference::from_str("skos:historyNote").unwrap(),
                IdentifierReference::from_str("skos:scopeNote").unwrap(),
                IdentifierReference::from_str("skos:example").unwrap(),
            ],
            references: vec![
                IdentifierReference::from_str("rdfs:seeAlso").unwrap(),
                IdentifierReference::from_str("rdfs:isDefinedBy").unwrap(),
                IdentifierReference::from_str("dc_terms:alternative").unwrap(),
                IdentifierReference::from_str("dc_terms:isFormatOf").unwrap(),
                IdentifierReference::from_str("dc_terms:isPartOf").unwrap(),
                IdentifierReference::from_str("dc_terms:isReferencedBy").unwrap(),
                IdentifierReference::from_str("dc_terms:isReplacedBy").unwrap(),
                IdentifierReference::from_str("dc_terms:isRequiredBy").unwrap(),
                IdentifierReference::from_str("dc_terms:isRequiredBy").unwrap(),
                IdentifierReference::from_str("dc_terms:isVersionOf").unwrap(),
            ],
        }
    }
}

impl AnnotationCategories {
    pub fn label_properties(&self) -> &Vec<IdentifierReference> {
        &self.labels
    }

    pub fn definition_properties(&self) -> &Vec<IdentifierReference> {
        &self.definitions
    }

    pub fn description_properties(&self) -> &Vec<IdentifierReference> {
        &self.descriptions
    }

    pub fn comment_properties(&self) -> &Vec<IdentifierReference> {
        &self.comments
    }

    pub fn reference_properties(&self) -> &Vec<IdentifierReference> {
        &self.references
    }
}

// ------------------------------------------------------------------------------------------------

impl Heading {
    const LEVEL_SECTION: u8 = 1;
    const LEVEL_SUBSECTION: u8 = 2;
    const LEVEL_SUBSUBSECTION: u8 = 3;

    pub fn new<S>(level: u8, title: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            level,
            title: title.into(),
            label: Default::default(),
        }
    }

    pub fn with_label<S>(mut self, label: S) -> Self
    where
        S: Into<String>,
    {
        self.label = Some(label.into());
        self
    }

    pub fn new_section<S>(title: S) -> Self
    where
        S: Into<String>,
    {
        Self::new(Self::LEVEL_SECTION, title)
    }

    pub fn new_subsection<S>(title: S) -> Self
    where
        S: Into<String>,
    {
        Self::new(Self::LEVEL_SUBSECTION, title)
    }

    pub fn new_subsubsection<S>(title: S) -> Self
    where
        S: Into<String>,
    {
        Self::new(Self::LEVEL_SUBSUBSECTION, title)
    }

    pub fn level(&self) -> u8 {
        self.level
    }

    pub fn is_section(&self) -> bool {
        self.level == Self::LEVEL_SECTION
    }

    pub fn is_subsection(&self) -> bool {
        self.level == Self::LEVEL_SUBSECTION
    }

    pub fn is_subsubsection(&self) -> bool {
        self.level == Self::LEVEL_SUBSUBSECTION
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn label(&self) -> Option<&String> {
        self.label.as_ref()
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for BookConfig {
    fn default() -> Self {
        Self {
            title: Default::default(),
            introduction: Default::default(),
            content: Default::default(),
            format: DocumentFormat::OrgMode,
            output_file: "index.org".into(),
            include_toc: true,
            multi_part: false,
            copy_includes: false,
            language: Default::default(),
            annotation_categories: Default::default(),
        }
    }
}

impl FromStr for BookConfig {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match serde_json::from_str(s) {
            Ok(config) => Ok(config),
            Err(err) => {
                error!(
                    ?s,
                    ?err,
                    "Could not de-serialize BookConfig from provided string"
                );
                Err(into_generator_error("doc-book", err))
            }
        }
    }
}

impl BookConfig {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let content = read_to_string(path)?;
        Self::from_str(&content)
    }

    pub fn item_count(&self) -> usize {
        self.introduction.as_ref().map(|_| 1).unwrap_or_default() + self.content.item_count()
    }

    pub fn with_title<S>(self, title: S) -> Self
    where
        S: Into<String>,
    {
        let mut self_mut = self;
        self_mut.title = title.into();
        self_mut
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn with_introduction<P>(self, introduction_file: P) -> Self
    where
        P: Into<PathBuf>,
    {
        let mut self_mut = self;
        self_mut.introduction = Some(introduction_file.into());
        self_mut
    }

    pub fn introduction(&self) -> Option<&PathBuf> {
        self.introduction.as_ref()
    }

    pub fn with_content(self, content: ContentSection) -> Self {
        let mut self_mut = self;
        self_mut.content = content;
        self_mut
    }

    pub fn content(&self) -> &ContentSection {
        &self.content
    }

    pub fn with_format<S>(self, format: DocumentFormat) -> Self {
        let mut self_mut = self;
        self_mut.format = format;
        self_mut
    }

    pub fn format(&self) -> DocumentFormat {
        self.format
    }

    pub fn with_output_file<P>(self, output_file: P) -> Self
    where
        P: Into<PathBuf>,
    {
        let mut self_mut = self;
        self_mut.output_file = output_file.into();
        self_mut
    }

    pub fn with_toc(self, include_toc: bool) -> Self {
        let mut self_mut = self;
        self_mut.include_toc = include_toc;
        self_mut
    }

    pub fn include_toc(&self) -> bool {
        self.include_toc
    }

    pub fn with_multi_part(self, multi_part: bool) -> Self {
        let mut self_mut = self;
        self_mut.multi_part = multi_part;
        self_mut
    }

    pub fn multi_part(&self) -> bool {
        self.multi_part
    }

    pub fn with_copy_includes(self, copy_includes: bool) -> Self {
        let mut self_mut = self;
        self_mut.copy_includes = copy_includes;
        self_mut
    }

    pub fn copy_includes(&self) -> bool {
        self.copy_includes
    }

    pub fn with_language<S>(self, language: S) -> Self
    where
        S: Into<String>,
    {
        let mut self_mut = self;
        self_mut.language = Some(language.into());
        self_mut
    }

    pub fn language(&self) -> Option<&String> {
        self.language.as_ref()
    }

    pub fn with_annotation_categories(self, annotation_categories: AnnotationCategories) -> Self {
        let mut self_mut = self;
        self_mut.annotation_categories = annotation_categories;
        self_mut
    }

    pub fn annotation_categories(&self) -> &AnnotationCategories {
        &self.annotation_categories
    }
}

// ------------------------------------------------------------------------------------------------

impl ContentSection {
    pub fn new<S, I>(heading: S, items: Vec<I>) -> Self
    where
        S: Into<String>,
        I: Into<ContentItem>,
    {
        Self {
            heading: heading.into(),
            items: items.into_iter().map(|i| i.into()).collect(),
        }
    }

    pub fn item_count(&self) -> usize {
        self.items
            .iter()
            .map(|item| match item {
                ContentItem::SourceFile { .. } => 1,
                ContentItem::SourceModule { .. } => 1,
                ContentItem::Include { .. } => 1,
                ContentItem::Section { sub_section } => sub_section.item_count(),
            })
            .sum()
    }

    pub fn with_heading<S>(self, heading: S) -> Self
    where
        S: Into<String>,
    {
        let mut self_mut = self;
        self_mut.heading = heading.into();
        self_mut
    }

    pub fn heading(&self) -> &String {
        &self.heading
    }

    pub fn with_items<I>(self, items: Vec<I>) -> Self
    where
        I: Into<ContentItem>,
    {
        let mut self_mut = self;
        self_mut.items = items.into_iter().map(|i| i.into()).collect();
        self_mut
    }

    pub fn with_item<I>(self, item: I) -> Self
    where
        I: Into<ContentItem>,
    {
        let mut self_mut = self;
        self_mut.items.push(item.into());
        self_mut
    }

    pub fn with_source_file(self, module_path: PathBuf) -> Self {
        Self::with_item(self, ContentItem::SourceFile { module_path })
    }

    pub fn with_source_module(self, module: Identifier) -> Self {
        Self::with_item(self, ContentItem::SourceModule { module })
    }

    pub fn with_include(self, include_file_path: PathBuf) -> Self {
        Self::with_item(self, ContentItem::Include { include_file_path })
    }

    pub fn with_sub_section(self, sub_section: ContentSection) -> Self {
        Self::with_item(self, ContentItem::Section { sub_section })
    }

    pub fn items(&self) -> &Vec<ContentItem> {
        &self.items
    }
}

// ------------------------------------------------------------------------------------------------

impl From<ContentSection> for ContentItem {
    fn from(sub_section: ContentSection) -> Self {
        Self::Section { sub_section }
    }
}

impl From<Identifier> for ContentItem {
    fn from(module: Identifier) -> Self {
        Self::SourceModule { module }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn identifier_to_string<S>(name: &Identifier, se: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    se.serialize_str(name.as_ref())
}

struct IdentifierVisitor;

impl Visitor<'_> for IdentifierVisitor {
    type Value = Identifier;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("a string representing an SDML Identifier")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Identifier::from_str(v).map_err(|e| E::custom(e))
    }
}

fn identifier_from_string<'de, D>(de: D) -> Result<Identifier, D::Error>
where
    D: Deserializer<'de>,
{
    let visitor = IdentifierVisitor;
    de.deserialize_str(visitor)
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod markdown;

pub mod org_mode;

pub mod common;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_json_writer() {
        let config = BookConfig::default()
            .with_title("My Project")
            .with_toc(false)
            .with_introduction(PathBuf::from("./introduction.org"))
            .with_content(
                ContentSection::default()
                    .with_heading("Content Heading")
                    .with_include(PathBuf::from("./accounts.org"))
                    .with_source_module(Identifier::new_unchecked("account"))
                    .with_source_module(Identifier::new_unchecked("customer"))
                    .with_source_file(PathBuf::from("./account_enums.sdml")),
            );
        let json = serde_json::to_string_pretty(&config).unwrap();
        println!("{json}");
    }

    #[test]
    fn test_config_json_reader() {
        const JSON: &str = r##"{
  "title": "My Project",
  "introduction": "./introduction.org",
  "content": {
    "heading": "Content Heading",
    "items": [
      { "include_file_path": "./accounts.org" },
      {  "module": "account" },
      { "module": "customer" },
      { "module_path": "./account_enums.sdml" }
    ]
  },
  "format": "OrgMode",
  "output_file": "index.org",
  "include_toc": false,
  "multi_part": false,
  "copy_includes": false
}"##;
        let config: BookConfig = serde_json::from_str(JSON).unwrap();
        println!("{config:?}");
    }
}
