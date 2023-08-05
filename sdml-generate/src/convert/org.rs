/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use sdml_core::generate::{GenerateToWriter, NoFormatOptions};
use sdml_core::load::{ModuleLoader, ModuleLoaderRef};
use sdml_core::model::walk::ModuleWalker;
use sdml_core::model::Module;
use sdml_core::{error::Error, model::walk::walk_module};
use std::io::Write;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct OrgFileGenerator<T>
where
    T: ModuleLoader + Clone,
{
    buffer: String,
    loader: Option<ModuleLoaderRef<T>>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl<T> GenerateToWriter<NoFormatOptions> for OrgFileGenerator<T>
where
    T: ModuleLoader + Clone,
{
    fn write_in_format(
        &mut self,
        module: &Module,
        writer: &mut dyn Write,
        _: NoFormatOptions,
    ) -> Result<(), Error> {
        walk_module(module, self)?;
        writer.write_all(self.buffer.as_bytes())?;
        Ok(())
    }
}

impl<T> OrgFileGenerator<T>
where
    T: ModuleLoader + Clone,
{
    pub fn with_loader(self, loader: ModuleLoaderRef<T>) -> Self {
        let mut self_mut = self;
        self_mut.loader = Some(loader);
        self_mut
    }
}

impl<T> ModuleWalker for OrgFileGenerator<T>
where
    T: ModuleLoader + Clone,
{
    fn start_module(
        &mut self,
        name: &sdml_core::model::Identifier,
        _span: Option<&sdml_core::model::Span>,
    ) -> Result<(), Error> {
        self.buffer.push_str(&format!(
            r#"#+TITLE: Module {name}
#+LANGUAGE: en
#+STARTUP: overview hidestars inlineimages entitiespretty
#+SETUPFILE: https://fniessen.github.io/org-html-themes/org/theme-readtheorg.setup
#+HTML_HEAD: <style>img {{ max-width: 800px; height: auto; }}</style>
#+HTML_HEAD: <style>div.figure {{ text-align: center; }}</style>
#+OPTIONS: toc:3

"#
        ));
        Ok(())
    }

    fn import(
        &mut self,
        _imported: &[sdml_core::model::Import],
        _span: Option<&sdml_core::model::Span>,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn annotation_property(
        &mut self,
        _name: &sdml_core::model::IdentifierReference,
        _value: &sdml_core::model::Value,
        _span: Option<&sdml_core::model::Span>,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn informal_constraint(
        &mut self,
        _name: Option<&sdml_core::model::Identifier>,
        _value: &str,
        _language: Option<&sdml_core::model::ControlledLanguageTag>,
        _span: Option<&sdml_core::model::Span>,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn start_datatype(
        &mut self,
        _name: &sdml_core::model::Identifier,
        _base_type: &sdml_core::model::IdentifierReference,
        _has_body: bool,
        _span: Option<&sdml_core::model::Span>,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn end_datatype(
        &mut self,
        _name: &sdml_core::model::Identifier,
        _had_body: bool,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn start_entity(
        &mut self,
        _name: &sdml_core::model::Identifier,
        _has_body: bool,
        _span: Option<&sdml_core::model::Span>,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn start_identity_member(
        &mut self,
        _name: &sdml_core::model::Identifier,
        _inner: &sdml_core::model::IdentityMemberInner,
        _span: Option<&sdml_core::model::Span>,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn start_by_value_member(
        &mut self,
        _name: &sdml_core::model::Identifier,
        _inner: &sdml_core::model::ByValueMemberInner,
        _span: Option<&sdml_core::model::Span>,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn start_by_reference_member(
        &mut self,
        _name: &sdml_core::model::Identifier,
        _inner: &sdml_core::model::ByReferenceMemberInner,
        _span: Option<&sdml_core::model::Span>,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn end_member(&mut self, _name: &sdml_core::model::Identifier) -> Result<(), Error> {
        Ok(())
    }

    fn end_entity(
        &mut self,
        _name: &sdml_core::model::Identifier,
        _had_body: bool,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn start_enum(
        &mut self,
        _name: &sdml_core::model::Identifier,
        _has_body: bool,
        _span: Option<&sdml_core::model::Span>,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn start_value_variant(
        &mut self,
        _identifier: &sdml_core::model::Identifier,
        _value: u32,
        _has_body: bool,
        _span: Option<&sdml_core::model::Span>,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn end_value_variant(
        &mut self,
        _name: &sdml_core::model::Identifier,
        _had_body: bool,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn end_enum(
        &mut self,
        _name: &sdml_core::model::Identifier,
        _had_body: bool,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn start_event(
        &mut self,
        _name: &sdml_core::model::Identifier,
        _source: &sdml_core::model::IdentifierReference,
        _has_body: bool,
        _span: Option<&sdml_core::model::Span>,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn start_group(&mut self, _span: Option<&sdml_core::model::Span>) -> Result<(), Error> {
        Ok(())
    }

    fn end_group(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn end_event(
        &mut self,
        _name: &sdml_core::model::Identifier,
        _had_body: bool,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn start_structure(
        &mut self,
        _name: &sdml_core::model::Identifier,
        _has_body: bool,
        _span: Option<&sdml_core::model::Span>,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn end_structure(
        &mut self,
        _name: &sdml_core::model::Identifier,
        _had_body: bool,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn start_union(
        &mut self,
        _name: &sdml_core::model::Identifier,
        _has_body: bool,
        _span: Option<&sdml_core::model::Span>,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn start_type_variant(
        &mut self,
        _identifier: &sdml_core::model::IdentifierReference,
        _rename: Option<&sdml_core::model::Identifier>,
        _has_body: bool,
        _span: Option<&sdml_core::model::Span>,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn end_type_variant(
        &mut self,
        _name: &sdml_core::model::IdentifierReference,
        _had_body: bool,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn end_union(
        &mut self,
        _name: &sdml_core::model::Identifier,
        _had_body: bool,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn start_property(
        &mut self,
        _name: &sdml_core::model::Identifier,
        _has_body: bool,
        _span: Option<&sdml_core::model::Span>,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn start_property_role(
        &mut self,
        _name: &sdml_core::model::Identifier,
        _inverse_name: Option<&Option<sdml_core::model::Identifier>>,
        _target_cardinality: Option<&sdml_core::model::Cardinality>,
        _target_type: &sdml_core::model::TypeReference,
        _has_body: bool,
        _span: Option<&sdml_core::model::Span>,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn end_property_role(
        &mut self,
        _name: &sdml_core::model::Identifier,
        _had_body: bool,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn end_property(
        &mut self,
        _name: &sdml_core::model::Identifier,
        _had_body: bool,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn end_module(&mut self, name: &sdml_core::model::Identifier) -> Result<(), Error> {
        if let Some(loader) = &self.loader {
            let loader = loader.borrow();
            let source = loader.get_source(name).unwrap();
            self.buffer.push_str(&format!(
                r#"* Appendix: Module Source

#+NAME: lst:module-source
#+CAPTION: Module Source
#+BEGIN_SRC sdml :noeval
{source}
#+END_SRC
"#
            ));
        }
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
