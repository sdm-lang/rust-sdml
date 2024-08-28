use std::process::ExitCode;

use clap::Args;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// Show tool and library versions.
///
/// This command shows more information than the simple `--version` global argument and is useful
/// for debugging.
///
/// ```text
/// ❯ sdml versions
/// SDML CLI:        0.2.7
/// SDML grammar:    0.2.16
/// Tree-Sitter ABI: 14
/// ```
///
#[derive(Args, Debug)]
pub(crate) struct Command;

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

const CLI_VERSION: &str = env!("CARGO_PKG_VERSION");

impl super::Command for Command {
    fn execute(&self) -> Result<ExitCode, sdml_errors::Error> {
        println!("SDML CLI:        {}", CLI_VERSION);
        println!("SDML grammar:    {}", tree_sitter_sdml::GRAMMAR_VERSION);
        println!(
            "Tree-Sitter ABI: {}",
            tree_sitter_sdml::language().version()
        );
        Ok(ExitCode::SUCCESS)
    }
}
