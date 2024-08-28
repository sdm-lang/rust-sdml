/*!
This module provides the generators for *concept*, *entity-relationship*, and *UML class* diagrams. It also provides a
common [`OutputFormat`] type that describes the image format.

- `concepts` -- A simple diagram showing only the entities and their relationships.
- `erd` -- An Entity-Relationship diagram.
- `uml` -- A detailed UML Class diagram.
*/

use crate::exec::CommandArg;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The format for image output.
///
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum OutputFormat {
    Source,
    ImageJpeg,
    ImagePng,
    #[default]
    ImageSvg,
}

/// Name of the command-line tool for GraphViz generation.
pub const DOT_PROGRAM: &str = "dot";

/// Name of the command-line tool for PlantUML generation.
pub const UML_PROGRAM: &str = "plantuml";

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl From<OutputFormat> for CommandArg {
    fn from(value: OutputFormat) -> Self {
        CommandArg::new_option(
            "-T",
            match value {
                OutputFormat::ImageJpeg => "jpg",
                OutputFormat::ImagePng => "png",
                OutputFormat::ImageSvg => "svg",
                _ => unreachable!(),
            },
        )
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod concepts;

pub mod erd;

pub mod uml;

pub mod filter;
