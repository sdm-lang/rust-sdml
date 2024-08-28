/*!
Generate a bare-bones CTag file from the provided module.

*/

use sdml_core::error::Error;
use sdml_core::model::definitions::HasMembers;
use sdml_core::model::definitions::{Definition, HasVariants};
use sdml_core::model::identifiers::Identifier;
use sdml_core::model::modules::Module;
use sdml_core::model::{HasBody, HasName, HasOptionalBody, HasSourceSpan};
use std::io::Write;
use std::path::PathBuf;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn write_ctags<W: Write>(
    module: &Module,
    file_name: Option<&PathBuf>,
    w: &mut W,
) -> Result<(), Error> {
    let file_name: String = file_name
        .map(|file| file.to_string_lossy().into_owned())
        .unwrap_or_else(|| module.name().to_string());
    let mut tags: Vec<(String, String)> = Default::default();
    tags.push(ctag_line(module, &file_name));

    for defn in module.body().definitions() {
        tags.push(ctag_line(defn, &file_name));
        match defn {
            Definition::Entity(v) => {
                if let Some(body) = v.body() {
                    tags.push(ctag_line_from(body.identity().name(), &file_name));
                    for member in body.members() {
                        tags.push(ctag_line_from(member.name(), &file_name));
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
                    for member in body.members() {
                        tags.push(ctag_line_from(member.name(), &file_name));
                    }
                }
            }
            Definition::Structure(v) => {
                if let Some(body) = v.body() {
                    for member in body.members() {
                        tags.push(ctag_line_from(member.name(), &file_name));
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
            _ => {
                // no additional tags
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
fn ctag_line(named: &impl HasName, file_name: &str) -> (String, String) {
    ctag_line_from(named.name(), file_name)
}

#[inline(always)]
fn ctag_line_from(name: &Identifier, file_name: &str) -> (String, String) {
    (
        name.to_string(),
        format!(
            "{}\t{}go",
            file_name,
            name.source_span()
                .map(|span| span.start() + 1)
                .unwrap_or_default()
        ),
    )
}
