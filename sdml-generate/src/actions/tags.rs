/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use sdml_core::error::Error;
use sdml_core::model::definitions::{Definition, HasVariants};
use sdml_core::model::identifiers::Identifier;
use sdml_core::model::modules::Module;
use sdml_core::model::{HasBody, HasName, HasOptionalBody, HasSourceSpan};
use std::io::Write;
use std::path::{Path, PathBuf};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn write_ctags<W: Write>(module: &Module, file_name: PathBuf, w: &mut W) -> Result<(), Error> {
    let mut tags: Vec<(String, String)> = Default::default();
    tags.push(ctag_line(module, &file_name));

    for defn in module.body().definitions() {
        tags.push(ctag_line(defn, &file_name));
        match defn {
            Definition::Datatype(_) => {}
            Definition::Entity(v) => {
                if let Some(body) = v.body() {
                    tags.push(ctag_line(body.identity(), &file_name));
                    for member in body.flat_members() {
                        tags.push(ctag_line(member, &file_name));
                    }
                }
            }
            Definition::Enum(v) => {
                if let Some(body) = v.body() {
                    for variant in body.variants() {
                        tags.push(ctag_line(variant, &file_name));
                    }
                }
            }
            Definition::Event(v) => {
                if let Some(body) = v.body() {
                    for member in body.flat_members() {
                        tags.push(ctag_line(member, &file_name));
                    }
                }
            }
            Definition::Property(v) => {
                if let Some(body) = v.body() {
                    for role in body.roles() {
                        tags.push(ctag_line(role, &file_name));
                    }
                }
            }
            Definition::Structure(v) => {
                if let Some(body) = v.body() {
                    for member in body.flat_members() {
                        tags.push(ctag_line(member, &file_name));
                    }
                }
            }
            Definition::TypeClass(v) => {
                if let Some(body) = v.body() {
                    for method in body.methods() {
                        tags.push(ctag_line(method, &file_name));
                    }
                }
            }
            Definition::Union(v) => {
                if let Some(body) = v.body() {
                    for variant in body.variants() {
                        if let Some(rename) = variant.rename() {
                            tags.push(ctag_line_from(rename, &file_name));
                        }
                    }
                }
            }
        }
    }

    tags.sort_by_cached_key(|v| v.0.clone());

    for line in tags {
        w.write_all(format!("{}\t{}\n", line.0, line.1).as_bytes())?;
    }

    Ok(())
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

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

#[inline(always)]
fn ctag_line(named: &impl HasName, file_name: &Path) -> (String, String) {
    ctag_line_from(named.name(), file_name)
}

#[inline(always)]
fn ctag_line_from(name: &Identifier, file_name: &Path) -> (String, String) {
    let file_name = file_name.to_str().unwrap();
    (
        name.to_string(),
        format!(
            "{}\t{}go",
            file_name,
            name.source_span().unwrap().start() + 1
        ),
    )
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
