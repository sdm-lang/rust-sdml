/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::error::Error;
use crate::model::Module;
use std::io::Write;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

const ORG_HEADER: &str = r#"#+TITLE: Module
#+LANGUAGE: en
#+STARTUP: overview hidestars inlineimages entitiespretty
#+SETUPFILE: https://fniessen.github.io/org-html-themes/org/theme-readtheorg.setup
#+HTML_HEAD: <style>img { max-width: 800px; height: auto; }</style>
#+HTML_HEAD: <style>div.figure { text-align: center; }</style>
#+OPTIONS: toc:3

"#;

const BEGIN_SRC: &str = r#"#+NAME: lst:full-module-listing
#+CAPTION: Module Concepts
#+BEGIN_SRC sdml :cmdline draw --diagram concepts :file ./module-concepts.svg :exports both :noweb yes
"#;

const END_SRC: &str = "#+END_SRC\n";

pub fn write_as_org<W: Write>(_module: &Module, w: &mut W) -> Result<(), Error> {
    w.write_all(ORG_HEADER.as_bytes())?;

    w.write_all(BEGIN_SRC.as_bytes())?;
    // w.write(tree.source().as_str().as_bytes())?;
    w.write_all(END_SRC.as_bytes())?;

    Ok(())
}

write_to_string!(to_org_string, write_as_org);

write_to_file!(to_org_file, write_as_org);

print_to_stdout!(print_org, write_as_org);

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

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
