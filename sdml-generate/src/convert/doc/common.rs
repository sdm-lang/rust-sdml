/*!
Common traits for structured document writers.

 */

#![allow(dead_code)]
#![allow(unused_macro_rules)]

use std::collections::HashMap;

///
/// Page-level generation.
///
pub trait PageFormat<TInclude>
where
    TInclude: ArgumentType,
{
    fn title<S>(&self, s: S) -> String
    where
        S: AsRef<str>;
    fn language<S>(&self, s: S) -> String
    where
        S: AsRef<str>;
    fn style_uri<S>(&self, s: S) -> String
    where
        S: AsRef<str>;
    fn style_inline<S>(&self, s: S) -> String
    where
        S: AsRef<str>;
    fn options<S>(&self, s: S) -> String
    where
        S: AsRef<str>;
    fn include_file<S>(&self, s: S) -> String
    where
        S: AsRef<str>;
    fn include_file_with_args<S>(&self, s: S, arguments: TInclude) -> String
    where
        S: AsRef<str>;
}

pub trait BlockFormat<TSource>
where
    TSource: ArgumentType,
{
    fn heading<S>(&self, text: S, depth: usize) -> String
    where
        S: AsRef<str>;
    fn pseudo_heading<S>(&self, text: S) -> String
    where
        S: AsRef<str>;
    fn heading_with_id<S1, S2>(&self, text: S1, depth: usize, id: S2) -> String
    where
        S1: AsRef<str>,
        S2: AsRef<str>;
    fn pseudo_heading_with_id<S1, S2>(&self, text: S1, id: S2) -> String
    where
        S1: AsRef<str>,
        S2: AsRef<str>;
    fn paragraph<S>(&self, text: S) -> String
    where
        S: AsRef<str>;
    fn quote<S>(&self, text: S) -> String
    where
        S: AsRef<str>;
    fn verbatim<S>(&self, content: S) -> String
    where
        S: AsRef<str>;
    fn indented_verbatim<S>(&self, content: S) -> String
    where
        S: AsRef<str>;
    fn example<S>(&self, content: S) -> String
    where
        S: AsRef<str>;
    fn export<S>(&self, content: S) -> String
    where
        S: AsRef<str>;
    fn center<S>(&self, content: S) -> String
    where
        S: AsRef<str>;
    fn comment<S>(&self, text: S) -> String
    where
        S: AsRef<str>;
    fn line_comment<S>(&self, text: S) -> String
    where
        S: AsRef<str>;
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
        S4: AsRef<str>;
    fn caption<S>(&self, text: S) -> String
    where
        S: AsRef<str>;
    fn results_of<S>(&self, id: S) -> String
    where
        S: AsRef<str>;
    fn source<S1, S2>(&self, language: S1, src: S2, arguments: TSource) -> String
    where
        S1: AsRef<str>,
        S2: AsRef<str>;
    fn source_with<S1, S2, S3, S4>(
        &self,
        language: S1,
        src: S2,
        arguments: TSource,
        id: S3,
        caption: S4,
    ) -> String
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
        S3: AsRef<str>,
        S4: AsRef<str>;
    fn inline_source<S1, S2>(&self, language: S1, src: S2, arguments: TSource) -> String
    where
        S1: AsRef<str>,
        S2: AsRef<str>;
    fn ordered_list(&self, items: &[String], indentation: usize) -> String;
    fn unordered_list(&self, items: &[String], indentation: usize) -> String;
    fn definition_list(&self, items: &HashMap<String, String>) -> String;

    fn table<S>(&self, data: &[Vec<S>], header_row: bool) -> String
    where
        S: AsRef<str>,
    {
        if data.is_empty() {
            String::default()
        } else if header_row {
            let header = &data[0];
            let data = &data[1..];
            format!(
                "{}{}\n",
                self.table_header_row(header, true),
                data.iter()
                    .map(|row| self.table_row(row))
                    .collect::<String>()
            )
        } else {
            data.iter()
                .map(|row| self.table_row(row))
                .collect::<String>()
        }
    }
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
        S3: AsRef<str>;
    fn table_header_row<S>(&self, columns: &[S], add_hline: bool) -> String
    where
        S: AsRef<str>;
    fn table_hline(&self, widths: &[usize]) -> String;
    fn table_row<S>(&self, values: &[S]) -> String
    where
        S: AsRef<str>;
}

pub trait TextFormat {
    fn bold<S>(&self, text: S) -> String
    where
        S: AsRef<str>;
    fn italic<S>(&self, text: S) -> String
    where
        S: AsRef<str>;
    fn underline<S>(&self, text: S) -> String
    where
        S: AsRef<str>;
    fn mono<S>(&self, text: S) -> String
    where
        S: AsRef<str>;
    fn code<S>(&self, text: S) -> String
    where
        S: AsRef<str>;
    fn strikethrough<S>(&self, text: S) -> String
    where
        S: AsRef<str>;
    fn superscript<S>(&self, text: S) -> String
    where
        S: AsRef<str>;
    fn subscript<S>(&self, text: S) -> String
    where
        S: AsRef<str>;
}

pub trait LinkFormat {
    fn noweb_target<S>(&self, s: S) -> String
    where
        S: AsRef<str>;
    fn name_target<S>(&self, s: S) -> String
    where
        S: AsRef<str>;
    fn make_id_link<S>(&self, id: S) -> String
    where
        S: AsRef<str>;
    fn make_heading_link<S>(&self, heading: S) -> String
    where
        S: AsRef<str>;
    fn link<S>(&self, target: S) -> String
    where
        S: AsRef<str>;
    fn link_with_description<S1, S2>(&self, target: S1, description: S2) -> String
    where
        S1: AsRef<str>,
        S2: AsRef<str>;
}

pub trait Formatter<TInclude, TSource>:
    PageFormat<TInclude> + BlockFormat<TSource> + TextFormat + LinkFormat
where
    TInclude: ArgumentType,
    TSource: ArgumentType,
{
}

pub trait ArgumentType: Default + Into<String> {
    fn is_default(&self) -> bool;
}

pub fn make_label<S1, S2>(prefix: S1, parts: &[S2]) -> String
where
    S1: AsRef<str>,
    S2: AsRef<str>,
{
    format!(
        "{}:{}",
        prefix.as_ref(),
        parts
            .iter()
            .map(|v| v.as_ref())
            .collect::<Vec<&str>>()
            .join("-")
    )
}

pub fn make_section_label<S>(parts: &[S]) -> String
where
    S: AsRef<str>,
{
    make_label("sec", parts)
}

pub fn make_listing_label<S>(parts: &[S]) -> String
where
    S: AsRef<str>,
{
    make_label("lst", parts)
}

pub fn make_figure_label<S>(parts: &[S]) -> String
where
    S: AsRef<str>,
{
    make_label("fig", parts)
}

pub fn make_table_label<S>(parts: &[S]) -> String
where
    S: AsRef<str>,
{
    make_label("tbl", parts)
}

pub fn make_appendix_label<S>(parts: &[S]) -> String
where
    S: AsRef<str>,
{
    make_label("app", parts)
}
