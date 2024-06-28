/*!
One-line description.

More detailed description, with

# Example

End of file during parsingSymbol’s value as variable is void: rustEnd of file during parsing

 */

use crate::color::set_colorize;
use crate::convert::doc::org_mode::{write_heading, DocumentationGenerator as OrgModeGenerator};
use crate::convert::doc::writer::org::IncludeArguments;
use crate::convert::doc::writer::{make_label, BlockFormat, PageFormat};
use crate::convert::doc::DocumentationWriter;
use crate::convert::doc::Heading;
use crate::errors::generator_error;
use crate::GenerateToWriter;
use console::Term;
use indicatif::{ProgressBar, ProgressStyle};
use sdml_core::cache::{ModuleCache, ModuleStore};
use sdml_core::error::Error;
use sdml_core::load::ModuleLoader;
use sdml_core::model::identifiers::Identifier;
use sdml_core::model::modules::Module;
use sdml_core::model::HasName;
use sdml_error::diagnostics::UseColor;
use std::cell::RefCell;
use std::fs::{read_to_string, File};
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::str::FromStr;
use std::{fmt::Debug, fs::OpenOptions, path::Path};
use tracing::{error, trace, warn};

use super::AnnotationCategories;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

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
