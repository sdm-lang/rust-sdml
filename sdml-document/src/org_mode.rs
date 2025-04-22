/*!
This module provides a generator for Emacs org-mode documentation from a module.
*/

use super::{ContentItem, ContentSection};
use crate::actions::deps::{DependencyViewGenerator, DependencyViewOptions};
use crate::color::set_colorize;
use crate::convert::doc::common::{
    make_figure_label, make_label, make_listing_label, make_section_label, make_table_label,
    ArgumentType, BlockFormat, Formatter, LinkFormat, PageFormat, TextFormat,
};
use crate::convert::doc::{AnnotationCategories, BookConfig, DocumentationWriter, Heading};
use crate::draw::OutputFormat;
use crate::Generator;
use console::Term;
use indicatif::{ProgressBar, ProgressStyle};
use rdftk_io::{turtle::TurtleWriter, ObjectWriter};
use sdml_core::error::Error;
use sdml_core::load::ModuleLoader;
use sdml_core::model::annotations::HasAnnotations;
use sdml_core::model::check::MaybeIncomplete;
use sdml_core::model::constraints::ConstraintBody;
use sdml_core::model::definitions::{
    DatatypeDef, Definition, DimensionDef, EntityDef, EnumDef, EventDef, PropertyDef, StructureDef,
    TypeClassDef, UnionDef,
};
use sdml_core::model::identifiers::{Identifier, IdentifierReference};
use sdml_core::model::members::{Member, MemberKind, PseudoSequenceType, DEFAULT_CARDINALITY};
use sdml_core::model::modules::Module;
use sdml_core::model::values::{LanguageString, SimpleValue, Value};
use sdml_core::model::{HasBody, HasName, HasNameReference, HasOptionalBody};
use sdml_core::store::ModuleStore;
use sdml_errors::diagnostics::UseColor;
use sdml_errors::Source;
use sdml_rdf::write::module_to_graph;
use std::collections::HashMap;
use std::fmt::Display;
use std::fs::{read_to_string, File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use tracing::trace;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Generator for Emacs org-mode documentation.
///
#[derive(Debug, Default)]
pub struct DocumentationGenerator {
    source: Option<Source>,
    annotation_categories: AnnotationCategories,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct OrgModeFormatter;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SourceBlockArguments {
    pub add_line_numbers: bool,
    pub continue_line_numbers: bool,
    pub line_number_start: Option<usize>,
    pub remove_source_labels: bool,
    pub label_syntax: Option<String>,
    pub preserve_global_indentation: bool,
    pub eval: SourceEval,
    pub exports: SourceExports,
    pub results: SourceResults,
    pub session: Option<String>,
    pub cmd_line: Option<String>,
    pub cache: bool,
    pub noweb: bool,
    pub hlines: bool,
    pub tangle: bool,
    pub rest: Vec<String>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum SourceEval {
    #[default]
    Yes,
    No,
    Query,
    NoExport,
    QueryExport,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum SourceExports {
    None,
    #[default]
    Code,
    Results,
    Both,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SourceResults {
    pub collection: Option<SourceResultsCollection>,
    pub as_type: Option<SourceResultsType>,
    pub format: Option<SourceResultsFormat>,
    pub handling: Option<SourceResultsHandling>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum SourceResultsCollection {
    #[default]
    Value,
    Output,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SourceResultsFormat {
    Raw,
    Code,
    Drawer,
    Html,
    LaTeX,
    Link,
    Graphics,
    Org,
    PrettyPrint,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SourceResultsFile {
    pub file_name: Option<PathBuf>,
    pub output_dir: Option<PathBuf>,
    pub file_extension: Option<String>,
    pub file_mode: Option<u32>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SourceResultsType {
    Table,
    Vector,
    List,
    Scalar,
    Verbatim,
    File(SourceResultsFile),
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum SourceResultsHandling {
    #[default]
    Replace,
    Silent,
    Discard,
    Append,
    Prepend,
    None,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct IncludeArguments {
    pub block_name: Option<String>,
    pub language: Option<String>,
    pub min_level: Option<usize>,
    pub from_line: Option<usize>,
    pub to_line: Option<usize>,
    pub only_contents: bool,
    pub rest: Vec<String>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn write_heading(heading: Heading, writer: &mut dyn Write) -> Result<(), Error> {
    let heading_string = if let Some(label) = heading.label() {
        FORMATTER.heading_with_id(heading.title(), heading.level().into(), label)
    } else {
        FORMATTER.heading(heading.title(), heading.level().into())
    };
    writer.write_all(heading_string.as_bytes())?;

    Ok(())
}

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

const FORMATTER: OrgModeFormatter = OrgModeFormatter::new();

type Appendix = String;

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Generator for DocumentationGenerator {
    type Options = AnnotationCategories;

    fn generate_with_options<W>(
        &mut self,
        module: &Module,
        cache: &impl ModuleStore,
        options: Self::Options,
        _: Option<PathBuf>,
        writer: &mut W,
    ) -> Result<(), Error>
    where
        W: Write + Sized,
    {
        self.annotation_categories = options;
        set_colorize(UseColor::Never);

        self.write_preamble(
            &format!("Module {}", FORMATTER.mono(module.name())),
            "en",
            true,
            writer,
        )?;

        self.write_module_description(Heading::new_section("Description"), module, cache, writer)?;

        self.write_module_uml_overview(module, cache, writer)?;

        write_heading(Heading::new_section("Dependencies"), writer)?;
        self.write_module_dependency_table(module, cache, writer)?;

        write_heading(Heading::new_section("Definitions"), writer)?;
        self.write_module_definitions(Heading::LEVEL_SECTION, module, cache, writer)?;

        self.write_module_dependency_graph(
            Heading::new_section("Appendix: Dependency Graph"),
            module,
            cache,
            writer,
        )?;

        self.write_module_sdml_listing(
            Heading::new_section("Appendix: SDML Source"),
            module,
            writer,
        )?;

        self.write_module_rdf_listing(
            Heading::new_section("Appendix: RDF Source"),
            module,
            cache,
            writer,
        )?;

        Ok(())
    }
}

impl DocumentationWriter<IncludeArguments, SourceBlockArguments, OrgModeFormatter>
    for DocumentationGenerator
{
    fn formatter() -> &'static OrgModeFormatter {
        &FORMATTER
    }

    fn write_book<T>(
        &mut self,
        loader: &mut T,
        cache: &mut impl ModuleStore,
        config: BookConfig,
    ) -> Result<(), Error>
    where
        T: ModuleLoader,
    {
        trace!(config = ?&config, "DocumentationGenerator::write()");
        let console = Term::stdout();
        let config_item_count = config.item_count();
        let index_progress = ProgressBar::new((config_item_count + 1) as u64).with_style(
            ProgressStyle::default_bar()
                .template("{bar} {pos:>3}/{len:3}")
                .unwrap(),
        );
        set_colorize(UseColor::Never);

        self.annotation_categories = config.annotation_categories.clone();

        let output_directory = config.output_file.parent().unwrap_or(Path::new("./"));
        if console.is_term() {
            console.write_line(&format!(
                "Creating documentation in directory {output_directory:?}"
            ))?;
        }

        trace!(index_file = ?config.output_file, "Creating the index file");
        let mut index_file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&config.output_file)?;

        trace!("Writing preamble");
        let default_language = "en".to_string();
        self.write_preamble(
            &config.title,
            config
                .language
                .as_ref()
                .unwrap_or(&default_language)
                .as_str(),
            config.options.include_toc,
            &mut index_file,
        )?;
        index_progress.inc(1);

        if let Some(file_name) = &config.introduction {
            trace!("Including introduction");
            include_file(file_name, config.options.multi_part, &mut index_file)?;
            index_progress.inc(1);
        }

        trace!("Write top-level section");
        let appendices = self.write_section(
            loader,
            cache,
            &config,
            1,
            &config.content,
            true,
            &mut index_file,
            &index_progress,
        )?;

        write_heading(Heading::new_section("Appendices"), &mut index_file)?;

        let mut appendices_file = if config.options.multi_part {
            let directory = config.output_file.parent().unwrap_or(Path::new("./"));
            let appendices_path = directory.join("_appendices.org");
            index_file.write_all(
                FORMATTER
                    .include_file_with_args(
                        appendices_path.to_string_lossy(),
                        IncludeArguments::default(),
                    )
                    .as_bytes(),
            )?;
            OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(appendices_path)?
        } else {
            index_file
        };

        trace!("Writing appendices");
        for appendix in appendices {
            appendices_file.write_all(appendix.as_bytes())?;
            appendices_file.write_all(b"\n")?;
        }
        index_progress.inc(1);

        index_progress.finish_and_clear();

        Ok(())
    }

    fn write_preamble<W>(
        &mut self,
        title: &str,
        language: &str,
        include_toc: bool,
        writer: &mut W,
    ) -> Result<(), Error>
    where
        W: Write + Sized,
    {
        let source_block_arguments = SourceBlockArguments::default();
        let headers = [
            FORMATTER.title(title),
            FORMATTER.language(language),
            FORMATTER
                .style_uri("https://fniessen.github.io/org-html-themes/org/theme-readtheorg.setup"),
            FORMATTER.style_inline("<style>table {{ min-width: 50%; }}</style>"),
            FORMATTER.style_inline("<style>img {{ max-width: 800px; height: auto; }}</style>"),
            FORMATTER.style_inline("<style>div.figure {{ text-align: center; }}</style>"),
            FORMATTER.options(format!(
                "h:5 toc:{} ^:{{}}",
                if include_toc { "3" } else { "'nil" }
            )),
            String::from("\n"),
            FORMATTER.source(
                "emacs-lisp",
                "(require 'ob-dot)\n(require 'ob-sdml)",
                SourceBlockArguments {
                    exports: SourceExports::None,
                    ..source_block_arguments
                },
            ),
            String::from("\n"),
        ]
        .join("");
        writer.write_all(headers.as_bytes())?;
        Ok(())
    }

    fn write_module_description<W>(
        &mut self,
        heading: super::Heading,
        module: &Module,
        _: &impl ModuleStore,
        writer: &mut W,
    ) -> Result<(), Error>
    where
        W: Write + Sized,
    {
        write_heading(heading, writer)?;

        let module_body = module.body();

        write_annotations(module_body, &self.annotation_categories, writer)?;

        Ok(())
    }

    fn write_module_definitions<W>(
        &mut self,
        heading_level: u8,
        module: &Module,
        cache: &impl ModuleStore,
        writer: &mut W,
    ) -> Result<(), Error>
    where
        W: Write + Sized,
    {
        write_definitions(
            heading_level,
            module,
            cache,
            &self.annotation_categories,
            writer,
        )?;

        Ok(())
    }

    fn write_module_uml_overview<W>(
        &mut self,
        module: &Module,
        _: &impl ModuleStore,
        writer: &mut W,
    ) -> Result<(), Error>
    where
        W: Write + Sized,
    {
        let name = module.name();
        let link_label = format!("module-{name}-uml-class");
        let listing_name = format!("lst:{link_label}");
        let figure_name = format!("fig:{link_label}");

        if let Some(source) = &self.source {
            writer.write_all(FORMATTER.name_target(&listing_name).as_bytes())?;
            writer.write_all(
                FORMATTER
                    .source(
                        "sdml",
                        source,
                        SourceBlockArguments {
                            cmd_line: Some(
                                "draw --diagram uml-class --output-format svg".to_string(),
                            ),
                            results: SourceResults {
                                as_type: Some(SourceResultsType::File(SourceResultsFile {
                                    file_name: Some(PathBuf::from(format!("./{link_label}.svg"))),
                                    ..SourceResultsFile::default()
                                })),
                                ..SourceResults::default()
                            },
                            exports: SourceExports::Results,
                            noweb: true,
                            ..SourceBlockArguments::default()
                        },
                    )
                    .as_bytes(),
            )?;
            writer.write_all(b"\n")?;
            writer.write_all(
                FORMATTER
                    .figure_with(
                        figure_name,
                        format!("Module {} UML Class Diagram", FORMATTER.mono(name)),
                        format!("file:./{link_label}.svg"),
                        Some(listing_name),
                    )
                    .as_bytes(),
            )?;
        }
        Ok(())
    }

    fn write_module_sdml_listing<W>(
        &mut self,
        heading: super::Heading,
        module: &Module,
        writer: &mut W,
    ) -> Result<(), Error>
    where
        W: Write + Sized,
    {
        if self.source.is_some() {
            write_heading(heading, writer)?;

            let name = module.name();
            let link_label = format!("module-{name}-src-sdml");
            let link_back_label = format!("module-{name}-uml-class");
            writer.write_all(b"\n")?;
            writer.write_all(
                FORMATTER
                    .source_with(
                        "sdml",
                        FORMATTER.noweb_target(format!("lst:{link_back_label}")),
                        SourceBlockArguments {
                            exports: SourceExports::Code,
                            noweb: true,
                            ..SourceBlockArguments::default()
                        },
                        format!("lst:{link_label}"),
                        format!("Module {} SDML Source", FORMATTER.mono(name)),
                    )
                    .as_bytes(),
            )?;
        }

        Ok(())
    }

    fn write_module_rdf_listing<W>(
        &mut self,
        heading: super::Heading,
        module: &Module,
        cache: &impl ModuleStore,
        writer: &mut W,
    ) -> Result<(), Error>
    where
        W: Write + Sized,
    {
        write_heading(heading, writer)?;

        let graph = module_to_graph(module, cache)?;
        let rdf_src = TurtleWriter::default()
            .write_to_string(&graph)
            .map_err(|e| Error::GeneratorError {
                name: "rdf".to_string(),
                message: e.to_string(),
            })?;

        let name = module.name();
        let link_label = format!("module-{name}-src-rdf");
        writer.write_all(b"\n")?;
        writer.write_all(
            FORMATTER
                .source_with(
                    "ttl",
                    rdf_src,
                    SourceBlockArguments::default(),
                    format!("lst:{link_label}"),
                    format!("Module {} RDF Source", FORMATTER.mono(name)),
                )
                .as_bytes(),
        )?;

        Ok(())
    }

    fn write_module_dependency_table<W>(
        &mut self,
        module: &Module,
        cache: &impl ModuleStore,
        writer: &mut W,
    ) -> Result<(), Error>
    where
        W: Write + Sized,
    {
        let module_body = module.body();
        let mut imported_modules: Vec<&Identifier> =
            module_body.imported_modules().into_iter().collect();

        if !imported_modules.is_empty() {
            imported_modules.sort();
            let mut data: Vec<Vec<String>> = vec![vec!["Name".to_string(), "Base IRI".to_string()]];
            for import in imported_modules {
                let base = if let Some(module) = cache.get(import) {
                    if let Some(base_uri) = module.base_uri() {
                        base_uri.value().to_string()
                    } else {
                        FORMATTER.italic("relative")
                    }
                } else {
                    FORMATTER.italic("not loaded")
                };
                data.push(vec![FORMATTER.mono(import), base]);
            }

            let name = module.name();
            writer.write_all(
                FORMATTER
                    .table_with(
                        &data,
                        true,
                        make_table_label(&["module", name.as_ref(), "imports"]),
                        format!("Module {} Imports", FORMATTER.mono(name)),
                    )
                    .as_bytes(),
            )?;

            writer.write_all(b"\n")?;
        }

        Ok(())
    }

    fn write_module_dependency_graph<W>(
        &mut self,
        heading: super::Heading,
        module: &Module,
        cache: &impl ModuleStore,
        writer: &mut W,
    ) -> Result<(), Error>
    where
        W: Write + Sized,
    {
        write_heading(heading, writer)?;
        writer.write_all(b"\n")?;

        let options = DependencyViewOptions::default().as_dot_graph(OutputFormat::Source);
        let mut generator = DependencyViewGenerator::default();
        let dot_graph = generator.generate_to_string(module, cache, options, Default::default())?;

        let name = module.name();
        let link_label = format!("module-{name}-dep-graph");
        writer.write_all(
            FORMATTER
                .name_target(format!("lst:{link_label}"))
                .as_bytes(),
        )?;
        writer.write_all(
            FORMATTER
                .source(
                    "dot",
                    dot_graph,
                    SourceBlockArguments {
                        exports: SourceExports::Results,
                        results: SourceResults {
                            as_type: Some(SourceResultsType::File(SourceResultsFile {
                                file_name: Some(PathBuf::from(format!("./{link_label}.svg"))),
                                ..SourceResultsFile::default()
                            })),
                            ..SourceResults::default()
                        },
                        ..SourceBlockArguments::default()
                    },
                )
                .as_bytes(),
        )?;
        writer.write_all(b"\n")?;
        writer.write_all(
            FORMATTER
                .figure_with(
                    make_figure_label(&[&link_label]),
                    format!("Module {} Dependency Graph", FORMATTER.mono(name)),
                    format!("./{link_label}.svg"),
                    Some(make_listing_label(&[link_label])),
                )
                .as_bytes(),
        )?;

        Ok(())
    }
}

impl DocumentationGenerator {
    pub fn new(source: Option<Source>, annotation_categories: AnnotationCategories) -> Self {
        Self {
            source,
            annotation_categories,
        }
    }

    pub fn without_source() -> Self {
        Self::default()
    }

    pub fn with_source(source: Source) -> Self {
        Self::new(Some(source), Default::default())
    }

    #[allow(clippy::too_many_arguments)]
    fn write_section(
        &mut self,
        loader: &mut impl ModuleLoader,
        cache: &mut impl ModuleStore,
        config: &BookConfig,
        level: u8,
        section: &ContentSection,
        is_top: bool,
        index_file: &mut File,
        index_progress: &ProgressBar,
    ) -> Result<Vec<Appendix>, Error> {
        trace!(
            level = level,
            heading = &section.heading,
            count = section.items.len(),
            "DocumentationGenerator::write_section"
        );
        let mut appendices = Vec::default();
        let heading = Heading::new(level, &section.heading);
        write_heading(heading, index_file)?;
        for item in &section.items {
            let additional = self.write_item(
                loader,
                cache,
                config,
                level,
                item,
                is_top,
                index_file,
                index_progress,
            )?;
            appendices.extend(additional);
        }
        Ok(appendices)
    }

    #[allow(clippy::too_many_arguments)]
    fn write_item(
        &mut self,
        loader: &mut impl ModuleLoader,
        cache: &mut impl ModuleStore,
        config: &BookConfig,
        level: u8,
        item: &ContentItem,
        is_top: bool,
        index_file: &mut File,
        index_progress: &ProgressBar,
    ) -> Result<Vec<Appendix>, Error> {
        trace!(level = level,
               item = ?item,
               "DocumentationGenerator::write_item");
        let mut result = Vec::default();
        match item {
            ContentItem::SourceFile { module_path } => {
                let _file_name = if let Ok(canonical) = module_path.canonicalize() {
                    canonical
                } else {
                    module_path.clone()
                };
                //let module_name = loader.load_from_file(file_name.to_path_buf(), cache, true)?;
                //self.write_module(loader, cache, level + 1, module_name, index_file)?;
                index_progress.inc(1);
                todo!();
            }
            ContentItem::SourceModule { module } => {
                let module_name = loader.load(module, loader.get_file_id(module), cache, true)?;
                result.push(self.write_module(
                    loader,
                    cache,
                    config,
                    level + 1,
                    &module_name,
                    index_file,
                )?);
                index_progress.inc(1);
            }
            ContentItem::Include { include_file_path } => {
                include_file(include_file_path, config.options.multi_part, index_file)?;
                index_progress.inc(1);
            }
            ContentItem::Section { sub_section } => result.extend(self.write_section(
                loader,
                cache,
                config,
                if is_top { level } else { level + 1 },
                sub_section,
                false,
                index_file,
                index_progress,
            )?),
        }

        Ok(result)
    }

    fn write_module(
        &mut self,
        loader: &mut impl ModuleLoader,
        cache: &mut impl ModuleStore,
        config: &BookConfig,
        level: u8,
        module_name: &Identifier,
        index_file: &mut File,
    ) -> Result<Appendix, Error> {
        trace!(
            level = level,
            module = ?module_name,
            "DocumentationGenerator::write_module"
        );
        if config.options.multi_part {
            let directory = config.output_file.parent().unwrap_or(Path::new("./"));
            let module_path = directory.join(format!("{module_name}.org"));
            index_file.write_all(
                FORMATTER
                    .include_file(module_path.to_string_lossy())
                    .as_bytes(),
            )?;
            let mut module_file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(module_path)?;
            self.write_module_file(loader, cache, level, module_name, &mut module_file)
        } else {
            self.write_module_file(loader, cache, level, module_name, index_file)
        }
    }

    fn write_module_file(
        &mut self,
        loader: &mut impl ModuleLoader,
        cache: &impl ModuleStore,
        level: u8,
        module_name: &Identifier,
        file: &mut File,
    ) -> Result<Appendix, Error> {
        let module = cache.get(module_name).unwrap();
        self.source = loader.get_source_by_name(module_name);
        self.write_module_actual(cache, level, module, file)
    }

    fn write_module_actual(
        &mut self,
        cache: &impl ModuleStore,
        level: u8,
        module: &Module,
        index_file: &mut File,
    ) -> Result<Appendix, Error> {
        let module_name = module.name();
        trace!(level = level, "DocumentationGenerator::write_module_actual");
        let heading = Heading::new(level, format!("Module ={module_name}="))
            .with_label(make_label("sec", &[module_name.as_ref()]));
        self.write_module_description(heading, module, cache, index_file)?;

        self.write_module_uml_overview(module, cache, index_file)?;

        index_file.write_all(Self::formatter().pseudo_heading("Dependencies").as_bytes())?;
        self.write_module_dependency_table(module, cache, index_file)?;

        index_file.write_all(Self::formatter().pseudo_heading("Definitions").as_bytes())?;
        self.write_module_definitions(level, module, cache, index_file)?;

        // ---------- Write appendices to string ----------

        let mut buf = BufWriter::new(Vec::new());

        let heading = format!("Module ={module_name}=");
        write_heading(Heading::new_subsection(heading), &mut buf)?;

        self.write_module_dependency_graph(
            Heading::new_subsubsection("Dependency Graph"),
            module,
            cache,
            &mut buf,
        )?;

        self.write_module_sdml_listing(
            Heading::new_subsubsection("SDML Source"),
            module,
            &mut buf,
        )?;

        self.write_module_rdf_listing(
            Heading::new_subsubsection("RDF Representation"),
            module,
            cache,
            &mut buf,
        )?;

        let bytes = buf.into_inner().unwrap();

        Ok(String::from_utf8(bytes)?)
    }
}

// ------------------------------------------------------------------------------------------------

impl PageFormat<IncludeArguments> for OrgModeFormatter {
    #[inline(always)]
    fn title<S>(&self, s: S) -> String
    where
        S: AsRef<str>,
    {
        format!("#+TITLE: {}", s.as_ref())
    }

    #[inline(always)]
    fn language<S>(&self, s: S) -> String
    where
        S: AsRef<str>,
    {
        format!("#+LANGUAGE: {}", s.as_ref())
    }

    #[inline(always)]
    fn style_uri<S>(&self, s: S) -> String
    where
        S: AsRef<str>,
    {
        format!("#+SETUPFILE: {}", s.as_ref())
    }

    #[inline(always)]
    fn style_inline<S>(&self, s: S) -> String
    where
        S: AsRef<str>,
    {
        format!("#+HTML_HEAD: {}", s.as_ref())
    }

    #[inline(always)]
    fn options<S>(&self, s: S) -> String
    where
        S: AsRef<str>,
    {
        format!("#+OPTIONS: {}", s.as_ref())
    }

    #[inline(always)]
    fn include_file<S>(&self, s: S) -> String
    where
        S: AsRef<str>,
    {
        format!("#+INCLUDE: {}", s.as_ref())
    }

    #[inline(always)]
    fn include_file_with_args<S>(&self, s: S, arguments: IncludeArguments) -> String
    where
        S: AsRef<str>,
    {
        let arguments: String = arguments.into();
        let arg_spacer = if arguments.is_empty() { "" } else { " " };
        format!("#+INCLUDE: {}{}{}", s.as_ref(), arg_spacer, arguments)
    }
}

impl BlockFormat<SourceBlockArguments> for OrgModeFormatter {
    #[inline(always)]
    fn heading<S>(&self, text: S, depth: usize) -> String
    where
        S: AsRef<str>,
    {
        if depth == 0 {
            self.pseudo_heading(text)
        } else {
            format!(
                "{} {}\n\n",
                &(0..depth).map(|_| "*").collect::<String>(),
                text.as_ref()
            )
        }
    }

    #[inline(always)]
    fn heading_with_id<S1, S2>(&self, text: S1, depth: usize, id: S2) -> String
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        if depth == 0 {
            self.pseudo_heading_with_id(text, id)
        } else {
            format!(
                "{} {}\n{}\n\n",
                &(0..depth).map(|_| "*").collect::<String>(),
                text.as_ref(),
                self.property_drawer(
                    [("CUSTOM_ID".to_string(), id.as_ref().to_string())]
                        .into_iter()
                        .collect()
                )
            )
        }
    }

    #[inline(always)]
    fn pseudo_heading<S>(&self, s: S) -> String
    where
        S: AsRef<str>,
    {
        format!("*{}*\n\n", s.as_ref())
    }

    #[inline(always)]
    fn pseudo_heading_with_id<S1, S2>(&self, text: S1, id: S2) -> String
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        format!("<<{}>>{}", id.as_ref(), self.pseudo_heading(text))
    }

    #[inline(always)]
    fn paragraph<S>(&self, s: S) -> String
    where
        S: AsRef<str>,
    {
        format!("{}\n\n", s.as_ref())
    }

    #[inline(always)]
    fn quote<S>(&self, s: S) -> String
    where
        S: AsRef<str>,
    {
        self.make_block("QUOTE", s)
    }

    #[inline(always)]
    fn verbatim<S>(&self, s: S) -> String
    where
        S: AsRef<str>,
    {
        self.make_block("VERBATIM", s)
    }

    #[inline(always)]
    fn indented_verbatim<S>(&self, s: S) -> String
    where
        S: AsRef<str>,
    {
        format!("  : {}\n\n", s.as_ref().replace('\n', "\n  : "))
    }

    #[inline(always)]
    fn example<S>(&self, s: S) -> String
    where
        S: AsRef<str>,
    {
        self.make_block("EXAMPLE", s)
    }

    #[inline(always)]
    fn export<S>(&self, s: S) -> String
    where
        S: AsRef<str>,
    {
        self.make_block("EXPORT", s)
    }

    #[inline(always)]
    fn center<S>(&self, s: S) -> String
    where
        S: AsRef<str>,
    {
        self.make_block("CENTER", s)
    }

    #[inline(always)]
    fn comment<S>(&self, s: S) -> String
    where
        S: AsRef<str>,
    {
        self.make_block("COMMENT", s)
    }

    #[inline(always)]
    fn line_comment<S>(&self, s: S) -> String
    where
        S: AsRef<str>,
    {
        format!("# {}\n\n", s.as_ref().replace('\n', "\n# "))
    }

    #[inline(always)]
    fn source<S1, S2>(&self, language: S1, src: S2, arguments: SourceBlockArguments) -> String
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        // TODO: get language into the block arguments!
        let arguments: String = vec![format!("{} ", language.as_ref()), arguments.into()]
            .into_iter()
            .collect::<String>();
        self.make_block_with_args("SRC", src, arguments)
    }

    #[inline(always)]
    fn source_with<S1, S2, S3, S4>(
        &self,
        language: S1,
        src: S2,
        arguments: SourceBlockArguments,
        id: S3,
        caption: S4,
    ) -> String
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
        S3: AsRef<str>,
        S4: AsRef<str>,
    {
        format!(
            "{}{}{}",
            self.name_target(id),
            self.caption(caption),
            self.source(language, src, arguments)
        )
    }

    #[inline(always)]
    fn inline_source<S1, S2>(
        &self,
        language: S1,
        src: S2,
        arguments: SourceBlockArguments,
    ) -> String
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        let arguments: String = arguments.into();
        format!(
            "src_{}[{}]{{{}}}",
            language.as_ref(),
            arguments,
            src.as_ref()
        )
    }

    #[inline(always)]
    fn figure_with<S1, S2, S3, S4>(
        &self,
        id: S1,
        caption: S2,
        file_name: S3,
        result_of: Option<S4>,
    ) -> String
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
        S3: AsRef<str>,
        S4: AsRef<str>,
    {
        format!(
            "{}{}{}[[{}]]\n\n",
            self.name_target(id),
            self.caption(caption),
            if let Some(result_of) = result_of {
                self.results_of(result_of)
            } else {
                String::default()
            },
            file_name.as_ref()
        )
    }

    #[inline(always)]
    fn caption<S>(&self, text: S) -> String
    where
        S: AsRef<str>,
    {
        format!("#+CAPTION: {}", text.as_ref())
    }

    #[inline(always)]
    fn results_of<S>(&self, id: S) -> String
    where
        S: AsRef<str>,
    {
        format!("#+RESULTS: {}", id.as_ref())
    }

    #[inline(always)]
    fn ordered_list(&self, items: &[String], indentation: usize) -> String {
        let indentation = &(0..(indentation * 2)).map(|_| " ").collect::<String>();

        format!(
            "{}\n\n",
            items
                .iter()
                .map(|s| format!("{}- {}", indentation, s))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }

    #[inline(always)]
    fn unordered_list(&self, items: &[String], indentation: usize) -> String {
        let indentation = &(0..(indentation * 3)).map(|_| " ").collect::<String>();

        format!(
            "{}\n\n",
            items
                .iter()
                .map(|s| format!("{}1. {}", indentation, s))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }

    #[inline(always)]
    fn definition_list(&self, items: &HashMap<String, String>) -> String {
        self.unordered_list(
            &items
                .iter()
                .map(|(key, value)| format!("{key} :: {value}"))
                .collect::<Vec<String>>(),
            Default::default(),
        )
    }

    #[inline(always)]
    fn table_with<S1, S2, S3>(
        &self,
        data: &[Vec<S1>],
        header_row: bool,
        id: S2,
        caption: S3,
    ) -> String
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
        S3: AsRef<str>,
    {
        format!(
            "{}{}{}",
            self.name_target(id),
            self.caption(caption),
            self.table(data, header_row)
        )
    }

    #[inline(always)]
    fn table_header_row<S>(&self, columns: &[S], add_hline: bool) -> String
    where
        S: AsRef<str>,
    {
        let columns = columns
            .iter()
            .map(|s| self.bold(s))
            .collect::<Vec<String>>();
        format!(
            "{}{}",
            self.table_row(&columns),
            if add_hline {
                let widths = columns
                    .iter()
                    .map(|s| {
                        let s: &str = s.as_str();
                        s.len()
                    })
                    .collect::<Vec<usize>>();
                self.table_hline(&widths)
            } else {
                String::default()
            }
        )
    }

    #[inline(always)]
    fn table_hline(&self, widths: &[usize]) -> String {
        format!(
            "|{}|\n",
            widths
                .iter()
                .map(|w| (0..*w).map(|_| "-").collect::<String>())
                .collect::<Vec<String>>()
                .join("+")
        )
    }

    #[inline(always)]
    fn table_row<S>(&self, values: &[S]) -> String
    where
        S: AsRef<str>,
    {
        format!(
            "| {} |\n",
            values
                .iter()
                .map(|v| v.as_ref())
                .collect::<Vec<&str>>()
                .join(" | ")
        )
    }
}

impl TextFormat for OrgModeFormatter {
    #[inline(always)]
    fn bold<S>(&self, s: S) -> String
    where
        S: AsRef<str>,
    {
        format!("*{}*", s.as_ref())
    }

    #[inline(always)]
    fn italic<S>(&self, s: S) -> String
    where
        S: AsRef<str>,
    {
        format!("/{}/", s.as_ref())
    }

    #[inline(always)]
    fn underline<S>(&self, s: S) -> String
    where
        S: AsRef<str>,
    {
        format!("_{}_", s.as_ref())
    }

    #[inline(always)]
    fn mono<S>(&self, s: S) -> String
    where
        S: AsRef<str>,
    {
        format!("={}=", s.as_ref())
    }

    #[inline(always)]
    fn code<S>(&self, s: S) -> String
    where
        S: AsRef<str>,
    {
        format!("~{}~", s.as_ref())
    }

    #[inline(always)]
    fn strikethrough<S>(&self, s: S) -> String
    where
        S: AsRef<str>,
    {
        format!("+{}+", s.as_ref())
    }

    #[inline(always)]
    fn superscript<S>(&self, s: S) -> String
    where
        S: AsRef<str>,
    {
        format!("^{{{}}}", s.as_ref())
    }

    #[inline(always)]
    fn subscript<S>(&self, s: S) -> String
    where
        S: AsRef<str>,
    {
        format!("_{{{}}}", s.as_ref())
    }
}

impl LinkFormat for OrgModeFormatter {
    #[inline(always)]
    fn noweb_target<S>(&self, s: S) -> String
    where
        S: AsRef<str>,
    {
        format!("<<{}>>", s.as_ref())
    }

    #[inline(always)]
    fn name_target<S>(&self, s: S) -> String
    where
        S: AsRef<str>,
    {
        format!("#+NAME: {}\n", s.as_ref())
    }

    #[inline(always)]
    fn make_id_link<S>(&self, id: S) -> String
    where
        S: AsRef<str>,
    {
        format!("#{}", id.as_ref())
    }

    #[inline(always)]
    fn make_heading_link<S>(&self, heading: S) -> String
    where
        S: AsRef<str>,
    {
        format!("*{}", heading.as_ref())
    }

    #[inline(always)]
    fn link<S>(&self, target: S) -> String
    where
        S: AsRef<str>,
    {
        format!("[[{}]]", target.as_ref())
    }

    #[inline(always)]
    fn link_with_description<S1, S2>(&self, target: S1, description: S2) -> String
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        format!("[[{}][{}]]", target.as_ref(), description.as_ref())
    }
}

impl Formatter<IncludeArguments, SourceBlockArguments> for OrgModeFormatter {}

impl OrgModeFormatter {
    #[inline(always)]
    pub const fn new() -> Self {
        OrgModeFormatter {}
    }

    #[inline(always)]
    fn make_block<S1, S2>(&self, kind: S1, content: S2) -> String
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        self.make_block_with_args(kind, content, "")
    }

    #[inline(always)]
    fn make_block_with_args<S1, S2, S3>(&self, kind: S1, content: S2, arguments: S3) -> String
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
        S3: AsRef<str>,
    {
        let kind = kind.as_ref();
        let arguments = arguments.as_ref();
        let arg_spacer = if arguments.is_empty() { "" } else { " " };
        let content = content.as_ref();

        format!(
            r##"#+BEGIN_{kind}{arg_spacer}{arguments}
{content}
#+END_{kind}
"##,
        )
    }

    // ----------------------------------------------------------------------------------------
    // Drawers
    // ----------------------------------------------------------------------------------------

    #[inline(always)]
    pub fn drawer<S>(&self, name: S) -> String
    where
        S: AsRef<str>,
    {
        self.drawer_with_content(name, "")
    }

    #[inline(always)]
    pub fn drawer_with_content<S1, S2>(&self, name: S1, content: S2) -> String
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        let name = self.property_name(name.as_ref().to_uppercase());
        let end = self.property_name("END");
        let content = content.as_ref();
        if content.is_empty() {
            "{name}\n{end}\n".to_string()
        } else {
            format!(
                r##"{name}
{content}{end}
"##
            )
        }
    }

    // ----------------------------------------------------------------------------------------
    // Drawers  Properties
    // ----------------------------------------------------------------------------------------

    #[inline(always)]
    pub fn property_drawer(&self, properties: HashMap<String, String>) -> String {
        self.drawer_with_content(
            "PROPERTIES",
            properties
                .iter()
                .map(|(key, value)| self.property(key, value))
                .collect::<String>(),
        )
    }

    #[inline(always)]
    fn property_name<S>(&self, name: S) -> String
    where
        S: AsRef<str>,
    {
        format!(":{}:", name.as_ref())
    }

    #[inline(always)]
    pub fn property<S1, S2>(&self, name: S1, value: S2) -> String
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        format!("{} {}\n", self.property_name(name), value.as_ref())
    }

    // ----------------------------------------------------------------------------------------
    // Drawers  Log Book
    // ----------------------------------------------------------------------------------------

    #[inline(always)]
    pub fn logbook_drawer<S1, S2, S3>(&self, log_entries: &[(S1, S2, Option<S3>)]) -> String
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
        S3: AsRef<str>,
    {
        self.drawer_with_content(
            "LOGBOOK",
            log_entries
                .iter()
                .map(|(headline, date, note)| {
                    self.log_entry(
                        headline,
                        date,
                        note.as_ref().map(|v| v.as_ref().to_string()),
                    )
                })
                .collect::<String>(),
        )
    }

    #[inline(always)]
    pub fn log_entry<S1, S2, S3>(&self, headline: S1, date: S2, note: Option<S3>) -> String
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
        S3: AsRef<str>,
    {
        format!(
            r##"- {} <{}>{}\n"##,
            headline.as_ref(),
            date.as_ref(),
            if let Some(note) = note {
                format!(" ||\n  {}", note.as_ref())
            } else {
                String::default()
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl From<SourceBlockArguments> for String {
    fn from(value: SourceBlockArguments) -> Self {
        let mut results = Vec::default();

        match (
            value.add_line_numbers,
            value.continue_line_numbers,
            value.line_number_start,
        ) {
            (_, true, Some(n)) => results.push(format!("+n {n}")),
            (true, false, Some(n)) => results.push(format!("-n {n}")),
            (_, true, None) => results.push("+n".to_string()),
            (true, false, None) => results.push("-n".to_string()),
            _ => {}
        }
        if value.remove_source_labels {
            results.push("-r".into());
        }
        if let Some(label_syntax) = &value.label_syntax {
            results.push(format!("-l \"{label_syntax}\""));
        }
        if value.preserve_global_indentation {
            results.push("-i".into());
        }
        results.push(value.eval.to_string());
        results.push(value.exports.to_string());
        results.push(value.results.to_string());
        if let Some(session) = &value.session {
            results.push(session.to_string());
        }
        if let Some(cmd_line) = &value.cmd_line {
            results.push(format!(":cmdline {cmd_line}"));
        }
        if value.cache {
            results.push(":cache yes".into());
        }
        if value.noweb {
            results.push(":noweb yes".into());
        }
        if value.hlines {
            results.push(":hlines yes".into());
        }
        if value.tangle {
            results.push(":tangle yes".into());
        }
        results.join(" ")
    }
}

impl ArgumentType for SourceBlockArguments {
    fn is_default(&self) -> bool {
        !self.add_line_numbers
            && !self.continue_line_numbers
            && self.line_number_start.is_none()
            && !self.remove_source_labels
            && self.label_syntax.is_none()
            && !self.preserve_global_indentation
            && self.eval == SourceEval::default()
            && self.exports == SourceExports::default()
            && self.results.is_default()
            && self.session.is_none()
            && self.cmd_line.is_none()
            && !self.cache
            && !self.noweb
            && !self.hlines
            && !self.tangle
            && self.rest.is_empty()
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for SourceEval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            ":eval {}",
            match self {
                Self::Yes => "yes",
                Self::No => "no",
                Self::Query => "query",
                Self::NoExport => "no-export",
                Self::QueryExport => "query-export",
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for SourceExports {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            ":exports {}",
            match self {
                Self::None => "none",
                Self::Code => "code",
                Self::Results => "results",
                Self::Both => "both",
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl SourceResults {
    fn is_default(&self) -> bool {
        self.collection.is_none()
            && self.as_type.is_none()
            && self.format.is_none()
            && self.handling.is_none()
    }
}

impl Display for SourceResults {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.collection.is_some()
            || self.as_type.is_some()
            || self.format.is_some()
            || self.handling.is_some()
        {
            write!(
                f,
                ":results{}{}{}{}",
                if let Some(collection) = &self.collection {
                    format!(" {collection}")
                } else {
                    String::default()
                },
                if let Some(as_type) = &self.as_type {
                    format!(" {as_type}")
                } else {
                    String::default()
                },
                if let Some(format) = &self.format {
                    format!(" {format}")
                } else {
                    String::default()
                },
                if let Some(handling) = &self.handling {
                    format!(" {handling}")
                } else {
                    String::default()
                }
            )
        } else {
            Ok(())
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for SourceResultsCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Value => "value",
                Self::Output => "output",
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for SourceResultsFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Raw => "raw",
                Self::Code => "code",
                Self::Drawer => "drawer",
                Self::Html => "html",
                Self::LaTeX => "latex",
                Self::Link => "link",
                Self::Graphics => "graphics",
                Self::Org => "org",
                Self::PrettyPrint => "pp",
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl ArgumentType for SourceResultsFile {
    fn is_default(&self) -> bool {
        self.file_name.is_none()
            && self.output_dir.is_none()
            && self.file_extension.is_none()
            && self.file_mode.is_none()
    }
}

impl From<SourceResultsFile> for String {
    fn from(value: SourceResultsFile) -> Self {
        Self::from(&value)
    }
}

impl From<&SourceResultsFile> for String {
    fn from(value: &SourceResultsFile) -> Self {
        vec![
            if let Some(file_name) = &value.file_name {
                format!(" :file {}", file_name.to_string_lossy().into_owned())
            } else {
                String::default()
            },
            if let Some(output_dir) = &value.output_dir {
                format!(" :output-dir {}", output_dir.to_string_lossy().into_owned())
            } else {
                String::default()
            },
            if let Some(file_extension) = &value.file_extension {
                format!(" :file-ext {file_extension}")
            } else {
                String::default()
            },
            if let Some(file_mode) = value.file_mode {
                format!(" :file-mode (identity #o{:o})", file_mode)
            } else {
                String::default()
            },
        ]
        .into_iter()
        .collect::<String>()
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for SourceResultsType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Table => "table".to_string(),
                Self::Vector => "vector".to_string(),
                Self::List => "list".to_string(),
                Self::Scalar => "scalar".to_string(),
                Self::Verbatim => "verbatim".to_string(),
                Self::File(file) => String::from(file),
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for SourceResultsHandling {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Replace => "replace",
                Self::Silent => "silent",
                Self::Discard => "discard",
                Self::Append => "append",
                Self::Prepend => "prepend",
                Self::None => "none",
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl From<IncludeArguments> for String {
    fn from(value: IncludeArguments) -> Self {
        let mut results = Vec::default();

        if let Some(block_name) = &value.block_name {
            results.push(block_name.to_string());
        }
        if let Some(language) = &value.language {
            results.push(language.to_string());
        }
        if let Some(min_level) = &value.min_level {
            if *min_level > 0 {
                results.push(format!(":minlevel {min_level}"));
            }
        }
        match (value.from_line, value.to_line) {
            (Some(from), Some(to)) => results.push(format!(":lines {from}-{to}")),
            (Some(from), None) => results.push(format!(":lines {from}-")),
            (None, Some(to)) => results.push(format!(":lines -{to}")),
            _ => {}
        }
        if value.only_contents {
            results.push(":only-contents t".to_string());
        }

        let rest: Vec<String> = value.rest.clone();
        results.extend(rest);

        results.join(" ")
    }
}

impl ArgumentType for IncludeArguments {
    fn is_default(&self) -> bool {
        self.block_name.is_none()
            && self.language.is_none()
            && self.min_level.is_none()
            && self.from_line.is_none()
            && self.to_line.is_none()
            && !self.only_contents
            && self.rest.is_empty()
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn include_file<P>(file_name: P, as_link: bool, index_file: &mut File) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    let file_name = file_name.as_ref();
    let file_name = if let Ok(canonical) = file_name.canonicalize() {
        canonical
    } else {
        file_name.to_path_buf()
    };
    if as_link {
        // TODO: make relative to index file or copy?
        index_file.write_all(
            FORMATTER
                .include_file(file_name.to_string_lossy())
                .as_bytes(),
        )?;
    } else {
        let included = read_to_string(&file_name)?;
        index_file.write_all(
            FORMATTER
                .line_comment(format!(">>> INCLUDED {file_name:?}"))
                .as_bytes(),
        )?;
        index_file.write_all(included.as_bytes())?;
        index_file.write_all(
            FORMATTER
                .line_comment(format!("<<< INCLUDED {file_name:?}"))
                .as_bytes(),
        )?;
    }

    Ok(())
}

fn write_annotations(
    annotated: &impl HasAnnotations,
    categories: &AnnotationCategories,
    writer: &mut dyn Write,
) -> Result<(), Error> {
    if annotated.has_annotation_properties() {
        let mut property_map: HashMap<&IdentifierReference, Vec<&Value>> = HashMap::default();

        for property in annotated.annotation_properties() {
            if let Some(values) = property_map.get_mut(property.name_reference()) {
                values.push(property.value());
            } else {
                property_map.insert(property.name_reference(), vec![property.value()]);
            }
        }

        let mut values: Vec<&LanguageString> = Vec::default();
        // TODO: prefLabel
        for property in categories.label_properties() {
            if let Some(vs) = property_map.get(property) {
                values.extend(vs.iter().filter_map(|v| {
                    if let Value::Simple(SimpleValue::String(s)) = v {
                        Some(s)
                    } else {
                        None
                    }
                }));
            }
        }
        if !values.is_empty() {
            let values = values
                .iter()
                .map(|v| {
                    format!(
                        "{}{}",
                        v.value(),
                        if let Some(lang) = v.language() {
                            FORMATTER.superscript(FORMATTER.italic(lang))
                        } else {
                            String::default()
                        }
                    )
                })
                .collect::<Vec<String>>();
            writer.write_all(
                FORMATTER
                    .paragraph(format!(
                        "{}: {}",
                        FORMATTER.bold("Labels"),
                        values.join(", ")
                    ))
                    .as_bytes(),
            )?;
        }

        write_string_property_block(
            "Definition",
            categories.definition_properties(),
            &property_map,
            writer,
        )?;

        write_string_property_block(
            "Description",
            categories.description_properties(),
            &property_map,
            writer,
        )?;

        write_string_property_block(
            "Comments",
            categories.comment_properties(),
            &property_map,
            writer,
        )?;
    }

    if annotated.has_constraints() {
        writer.write_all(
            FORMATTER
                .pseudo_heading("Annotation Constraints")
                .as_bytes(),
        )?;

        for constraint in annotated.annotation_constraints() {
            writer.write_all(format!("{}: ", FORMATTER.bold(constraint.name())).as_bytes())?;
            let body = match constraint.body() {
                ConstraintBody::Informal(v) => format!(
                    "{}{}",
                    v.value(),
                    if let Some(language) = v.language() {
                        format!(" [{}]", FORMATTER.italic(language))
                    } else {
                        String::default()
                    }
                ),
                ConstraintBody::Formal(_) => format!("... [{}]", FORMATTER.italic("sdml-cl")),
            };
            writer.write_all(body.as_bytes())?;
        }
    }

    Ok(())
}

fn write_string_property_block(
    heading: &str,
    properties: &[IdentifierReference],
    property_map: &HashMap<&IdentifierReference, Vec<&Value>>,
    writer: &mut dyn Write,
) -> Result<(), Error> {
    let mut values: Vec<&LanguageString> = Vec::default();
    for property in properties {
        if let Some(vs) = property_map.get(property) {
            values.extend(vs.iter().filter_map(|v| {
                if let Value::Simple(SimpleValue::String(s)) = v {
                    Some(s)
                } else {
                    None
                }
            }));
        }
    }
    if !values.is_empty() {
        writer.write_all(FORMATTER.pseudo_heading(heading).as_bytes())?;
        let values = values
            .iter()
            .map(|v| {
                format!(
                    "{}{}",
                    v.value(),
                    if let Some(lang) = v.language() {
                        FORMATTER.superscript(FORMATTER.italic(lang))
                    } else {
                        String::default()
                    }
                )
            })
            .collect::<Vec<String>>();

        writer.write_all(
            if values.len() == 1 {
                FORMATTER.paragraph(values.first().unwrap())
            } else {
                FORMATTER.unordered_list(&values, 0)
            }
            .as_bytes(),
        )?;
    }
    Ok(())
}

#[allow(single_use_lifetimes)]
fn write_definitions(
    parent_level: u8,
    module: &Module,
    cache: &impl ModuleStore,
    categories: &AnnotationCategories,
    writer: &mut dyn Write,
) -> Result<(), Error> {
    for definition in module.body().definitions() {
        match definition {
            Definition::Datatype(v) => {
                write_datatype(parent_level, module, v, cache, categories, writer)?
            }
            Definition::Dimension(v) => {
                write_dimension(parent_level, module, v, cache, categories, writer)?
            }
            Definition::Entity(v) => {
                write_entity(parent_level, module, v, cache, categories, writer)?
            }
            Definition::Enum(v) => write_enum(parent_level, module, v, cache, categories, writer)?,
            Definition::Event(v) => {
                write_event(parent_level, module, v, cache, categories, writer)?
            }
            Definition::Property(v) => {
                write_property(parent_level, module, v, cache, categories, writer)?
            }
            Definition::Rdf(_) => todo!(),
            Definition::Structure(v) => {
                write_structure(parent_level, module, v, cache, categories, writer)?
            }
            Definition::TypeClass(v) => {
                write_typeclass(parent_level, module, v, cache, categories, writer)?
            }
            Definition::Union(v) => {
                write_union(parent_level, module, v, cache, categories, writer)?
            }
        }
    }
    Ok(())
}

fn write_datatype(
    parent_level: u8,
    module: &Module,
    datatype: &DatatypeDef,
    _: &impl ModuleStore,
    categories: &AnnotationCategories,
    writer: &mut dyn Write,
) -> Result<(), Error> {
    let name = datatype.name();
    write_heading(
        Heading::new(
            parent_level + 1,
            format!("Datatype {}", FORMATTER.mono(name)),
        )
        .with_label(make_section_label(&[
            "definition",
            module.name().as_ref(),
            name.as_ref(),
        ])),
        writer,
    )?;

    writer.write_all(
        // TODO: make local names links
        FORMATTER.paragraph(
            format!(
                "The datatype {} is based on the existing datatype {}.{}",
                FORMATTER.bold(FORMATTER.mono(name)),
                FORMATTER.mono(datatype.base_type().to_string()),
                if datatype.is_opaque() {
                    format!(
                        " It is marked as an {} datatype, meaning constraints may only use strict equality tests between values.",
                        FORMATTER.italic("opaque"),
                    )
                } else {
                    String::default()
                }
            )).as_bytes())?;

    // TODO: special annotation properties.

    if let Some(body) = datatype.body() {
        write_annotations(body, categories, writer)?;
    }

    writer.write_all(b"\n")?;

    Ok(())
}

fn write_dimension(
    parent_level: u8,
    module: &Module,
    dimension: &DimensionDef,
    cache: &impl ModuleStore,
    categories: &AnnotationCategories,
    writer: &mut dyn Write,
) -> Result<(), Error> {
    let name = dimension.name();
    write_heading(
        Heading::new(
            parent_level + 1,
            format!("Dimension {}", FORMATTER.mono(name)),
        )
        .with_label(make_section_label(&[
            "definition",
            module.name().as_ref(),
            name.as_ref(),
        ])),
        writer,
    )?;

    if !dimension.is_incomplete(module, cache) {
        if let Some(body) = dimension.body() {
            // TODO: identity

            if body.has_annotations() {
                write_annotations(body, categories, writer)?;
            }
            for member in body.members() {
                write_member(
                    parent_level + 1,
                    module,
                    name,
                    member,
                    cache,
                    categories,
                    writer,
                )?;
            }
        }
    } else {
        writer.write_all(type_is_incomplete().as_bytes())?;
    }

    Ok(())
}

fn write_entity(
    parent_level: u8,
    module: &Module,
    entity: &EntityDef,
    cache: &impl ModuleStore,
    categories: &AnnotationCategories,
    writer: &mut dyn Write,
) -> Result<(), Error> {
    let name = entity.name();
    write_heading(
        Heading::new(parent_level + 1, format!("Entity {}", FORMATTER.mono(name))).with_label(
            make_section_label(&["definition", module.name().as_ref(), name.as_ref()]),
        ),
        writer,
    )?;

    if !entity.is_incomplete(module, cache) {
        if let Some(body) = entity.body() {
            // TODO: identity
            if body.has_annotations() {
                write_annotations(body, categories, writer)?;
            }
            for member in body.members() {
                write_member(
                    parent_level + 1,
                    module,
                    name,
                    member,
                    cache,
                    categories,
                    writer,
                )?;
            }
        }
    } else {
        writer.write_all(type_is_incomplete().as_bytes())?;
    }

    Ok(())
}

fn write_enum(
    parent_level: u8,
    module: &Module,
    an_enum: &EnumDef,
    cache: &impl ModuleStore,
    categories: &AnnotationCategories,
    writer: &mut dyn Write,
) -> Result<(), Error> {
    let name = an_enum.name();
    write_heading(
        Heading::new(
            parent_level + 1,
            format!("Enumeration {}", FORMATTER.mono(name)),
        )
        .with_label(make_section_label(&[
            "definition",
            module.name().as_ref(),
            name.as_ref(),
        ])),
        writer,
    )?;

    if !an_enum.is_incomplete(module, cache) {
        let body = an_enum.body().unwrap();
        write_annotations(body, categories, writer)?;

        for variant in body.variants() {
            let name = variant.name();
            write_heading(
                Heading::new(
                    parent_level + 2,
                    format!("Variant {}", FORMATTER.mono(name)),
                )
                .with_label(make_section_label(&[
                    "variant",
                    module.name().as_ref(),
                    an_enum.name().as_ref(),
                    name.as_ref(),
                ])),
                writer,
            )?;
            if let Some(body) = variant.body() {
                write_annotations(body, categories, writer)?;
            }
        }
    } else {
        writer.write_all(type_is_incomplete().as_bytes())?;
    }

    Ok(())
}

fn write_event(
    parent_level: u8,
    module: &Module,
    event: &EventDef,
    cache: &impl ModuleStore,
    categories: &AnnotationCategories,
    writer: &mut dyn Write,
) -> Result<(), Error> {
    let name = event.name();
    write_heading(
        Heading::new(parent_level + 1, format!("Event {}", FORMATTER.mono(name))).with_label(
            make_section_label(&["definition", module.name().as_ref(), name.as_ref()]),
        ),
        writer,
    )?;

    if !event.is_incomplete(module, cache) {
        // generated by source...

        if let Some(body) = event.body() {
            if body.has_annotations() {
                write_annotations(body, categories, writer)?;
            }
            for member in body.members() {
                write_member(
                    parent_level + 1,
                    module,
                    name,
                    member,
                    cache,
                    categories,
                    writer,
                )?;
            }
        } else {
            writer.write_all(type_is_incomplete().as_bytes())?;
        }
    } else {
        writer.write_all(type_is_incomplete().as_bytes())?;
    }

    Ok(())
}

fn write_property(
    parent_level: u8,
    module: &Module,
    property: &PropertyDef,
    _: &impl ModuleStore,
    _: &AnnotationCategories,
    writer: &mut dyn Write,
) -> Result<(), Error> {
    let name = property.name();
    write_heading(
        Heading::new(
            parent_level + 1,
            format!("Property {}", FORMATTER.mono(name)),
        )
        .with_label(make_section_label(&[
            "definition",
            module.name().as_ref(),
            name.as_ref(),
        ])),
        writer,
    )?;

    // TODO: property

    Ok(())
}

fn write_structure(
    parent_level: u8,
    module: &Module,
    structure: &StructureDef,
    cache: &impl ModuleStore,
    categories: &AnnotationCategories,
    writer: &mut dyn Write,
) -> Result<(), Error> {
    let name = structure.name();
    write_heading(
        Heading::new(
            parent_level + 1,
            format!("Structure {}", FORMATTER.mono(name)),
        )
        .with_label(make_section_label(&[
            "definition",
            module.name().as_ref(),
            name.as_ref(),
        ])),
        writer,
    )?;

    if !structure.is_incomplete(module, cache) {
        if let Some(body) = structure.body() {
            if body.has_annotations() {
                write_annotations(body, categories, writer)?;
            }
            for member in body.members() {
                write_member(
                    parent_level + 1,
                    module,
                    name,
                    member,
                    cache,
                    categories,
                    writer,
                )?;
            }
        }
    } else {
        writer.write_all(type_is_incomplete().as_bytes())?;
    }

    Ok(())
}

fn write_member(
    parent_level: u8,
    module: &Module,
    parent_def: &Identifier,
    member: &Member,
    _: &impl ModuleStore,
    categories: &AnnotationCategories,
    writer: &mut dyn Write,
) -> Result<(), Error> {
    let name = member.name();
    match member.kind() {
        MemberKind::Reference(_property) => {
            write_heading(
                Heading::new(
                    parent_level + 1,
                    format!("Member {} (property)", FORMATTER.mono(name)),
                )
                .with_label(make_section_label(&[
                    "member",
                    module.name().as_ref(),
                    parent_def.as_ref(),
                    name.as_ref(),
                ])),
                writer,
            )?;
        }
        MemberKind::Definition(member) => {
            write_heading(
                Heading::new(parent_level + 1, format!("Member {}", FORMATTER.mono(name)))
                    .with_label(make_section_label(&[
                        "member",
                        module.name().as_ref(),
                        parent_def.as_ref(),
                        name.as_ref(),
                    ])),
                writer,
            )?;
            let cardinality = member.target_cardinality();
            let cardinality_string = if cardinality == &DEFAULT_CARDINALITY {
                format!("a {} value of type ", FORMATTER.bold("required"))
            } else {
                let seq_type_string = match cardinality.sequence_type() {
                    PseudoSequenceType::Maybe => "an optional",
                    PseudoSequenceType::Bag => "an unordered bag",
                    PseudoSequenceType::List => "an ordered list",
                    PseudoSequenceType::Set => "a set",
                    PseudoSequenceType::UnorderedSet => "an unordered set",
                };
                let range = cardinality.range();
                let range_string = match (range.min_occurs(), range.max_occurs()) {
                    (0, None) => ", with zero or more values,".to_string(),
                    (1, None) => ", with one or more values,".to_string(),
                    (min, None) => format!(", with {min} or more values,"),
                    (min, Some(max)) => {
                        if min == max {
                            format!(", with {min} values,")
                        } else {
                            format!(", with between {min} and {max} values,")
                        }
                    }
                };
                format!(
                    "{}{range_string} of type ",
                    FORMATTER.italic(seq_type_string)
                )
            };
            writer.write_all(
                FORMATTER
                    .paragraph(format!(
                        "Member {} is {}{}.\n\n",
                        FORMATTER.mono(name),
                        cardinality_string,
                        FORMATTER.mono(member.target_type().to_string())
                    ))
                    .as_bytes(),
            )?;
            if let Some(body) = member.body() {
                write_annotations(body, categories, writer)?;
            }
        }
    }

    // properties!

    Ok(())
}

fn write_typeclass(
    parent_level: u8,
    module: &Module,
    typeclass: &TypeClassDef,
    cache: &impl ModuleStore,
    categories: &AnnotationCategories,
    writer: &mut dyn Write,
) -> Result<(), Error> {
    let name = typeclass.name();
    write_heading(
        Heading::new(
            parent_level + 1,
            format!("Typeclass {}", FORMATTER.mono(name)),
        )
        .with_label(make_section_label(&[
            "definition",
            module.name().as_ref(),
            name.as_ref(),
        ])),
        writer,
    )?;

    if !typeclass.is_incomplete(module, cache) {
        if let Some(body) = typeclass.body() {
            if body.has_annotations() {
                write_annotations(body, categories, writer)?;
            }
        }
    } else {
        writer.write_all(type_is_incomplete().as_bytes())?;
    }

    Ok(())
}

fn write_union(
    parent_level: u8,
    module: &Module,
    union: &UnionDef,
    cache: &impl ModuleStore,
    categories: &AnnotationCategories,
    writer: &mut dyn Write,
) -> Result<(), Error> {
    let name = union.name();
    write_heading(
        Heading::new(parent_level + 1, format!("Union {}", FORMATTER.mono(name))).with_label(
            make_section_label(&["definition", module.name().as_ref(), name.as_ref()]),
        ),
        writer,
    )?;

    if !union.is_incomplete(module, cache) {
        if let Some(body) = union.body() {
            write_annotations(body, categories, writer)?;
            for variant in body.variants() {
                let name = variant.name();
                write_heading(
                    Heading::new(
                        parent_level + 2,
                        format!("Variant {}", FORMATTER.mono(name)),
                    )
                    .with_label(make_section_label(&[
                        "variant".to_string(),
                        module.name().to_string(),
                        union.name().to_string(),
                        if let Some(label) = variant.rename() {
                            label.to_string()
                        } else {
                            variant.name_reference().to_string()
                        },
                    ])),
                    writer,
                )?;
                writer.write_all(
                    FORMATTER
                        .indented_verbatim(format!(
                            "{}{}",
                            // TODO: make local names links
                            variant.name_reference(),
                            if let Some(label) = variant.rename() {
                                format!(" as {label}")
                            } else {
                                String::default()
                            },
                        ))
                        .as_bytes(),
                )?;
                if let Some(body) = variant.body() {
                    write_annotations(body, categories, writer)?;
                }
            }
        }
    } else {
        writer.write_all(type_is_incomplete().as_bytes())?;
    }

    Ok(())
}

#[inline(always)]
fn type_is_incomplete() -> String {
    FORMATTER.paragraph(format!(
        "This type is currently {}.",
        FORMATTER.italic("incomplete")
    ))
}
