use clap::Args;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// Show tool and library versions.
#[derive(Args, Debug)]
pub(crate) struct Command;

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

const CLI_VERSION: &str = env!("CARGO_PKG_VERSION");

impl super::Command for Command {
    fn execute(&self) -> Result<(), sdml_error::Error> {
        println!("SDML CLI:        {}", CLI_VERSION);
        println!("SDML grammar:    {}", tree_sitter_sdml::GRAMMAR_VERSION);
        println!(
            "Tree-Sitter ABI: {}",
            tree_sitter_sdml::language().version()
        );
        Ok(())
    }
}
