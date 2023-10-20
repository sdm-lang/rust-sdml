/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use sdml_core::generate::{GenerateToWriter, NoFormatOptions};
use sdml_core::load::ModuleLoader;
use sdml_core::model::modules::Module;
use sdml_core::model::HasName;
use sdml_core::error::Error;
use std::io::Write;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct OrgFileGenerator {}

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

impl GenerateToWriter<NoFormatOptions> for OrgFileGenerator {
    fn write_in_format(
        &mut self,
        module: &Module,
        loader: Option<&mut dyn ModuleLoader>,
        writer: &mut dyn Write,
        _: NoFormatOptions,
    ) -> Result<(), Error> {
        write_module(module, loader, writer)
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn write_module(me: &Module, loader: Option<&mut dyn ModuleLoader>, writer: &mut dyn Write) -> Result<(), Error>
{
    let name = me.name();
    writer.write_all(format!(
        r#"#+TITLE: Module {name}
#+LANGUAGE: en
#+STARTUP: overview hidestars inlineimages entitiespretty
#+SETUPFILE: https://fniessen.github.io/org-html-themes/org/theme-readtheorg.setup
#+HTML_HEAD: <style>img {{ max-width: 800px; height: auto; }}</style>
#+HTML_HEAD: <style>div.figure {{ text-align: center; }}</style>
#+OPTIONS: toc:3

"#
    ).as_bytes())?;

    // imports
    // definitions

    if let Some(loader) = loader {
        let source: Box<dyn AsRef<str>> = loader.get_source(name).unwrap();
        writer.write_all(&format!(
            r#"* Appendix: Module Source

#+NAME: lst:module-source
#+CAPTION: Module Source
#+BEGIN_SRC sdml :noeval
{}
#+END_SRC
"#,
            source.as_ref().as_ref()
        ).as_bytes())?;
    }

    Ok(())
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
