use std::process::ExitCode;

use clap::{Args, ValueEnum};
use sdml_core::model::modules::Module;
use sdml_core::{load::ModuleLoader, store::ModuleStore};
use sdml_errors::Error;
use sdml_generate::convert::source::{SourceGenerator, SourceGeneratorOptions};
use sdml_generate::Generator;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// View formatted module source.
///
/// This command will generate source code from a module file, which at first seems redundant.
/// However, this view provides levels of detail that allow for an overview of module definitions.
/// The `--level` argument can be used to elide content and get an overview of a module.
///
/// - Definitions (default) :: Show only the definitions in the module, any definition body will
///   be elided, for an overview of the module contents. Elided definitions are followed by
///   `";; ..."`.
///
/// ```text
/// ❯ sdml view --level definitions -i examples/example.sdm
/// module example <https://example.com/api> is
///
///   import [ dc xsd ]
///
///   datatype Uuid <- sdml:string ;; ...
///
///   entity Example ;; ...
///
/// end
/// ```
///
/// - Members :: Show definitions in the module and show the members of product types and variants
///   of sum types but not their bodies if present.
///
/// ```text
/// ❯ sdml view --level members -i examples/example.sdm
/// module example <https://example.com/api> is
///
///   import [ dc xsd ]
///
///   datatype Uuid <- sdml:string ;; ...
///
///   entity Example is
///     version -> Uuid
///     name -> sdml:string ;; ...
///   end
///
/// end
/// ```
///
/// - Full :: Show all contents of the module.
///
/// ```text
/// ❯ sdml view --level full -i examples/example.sdm
/// module example <https://example.com/api> is
///
///   import [ dc xsd ]
///
///   datatype Uuid <- sdml:string is
///     @xsd:pattern = "[0-9a-f]{8}-([0-9a-f]{4}-){3}[0-9a-f]{12}"
///   end
///
///   entity Example is
///     version -> Uuid
///     name -> sdml:string is
///       @dc:description = "the name of this thing"@en
///     end
///   end
///
/// end
/// ```
///
/// Indentation level can also be specified, although the default remains 2 spaces.
///
#[derive(Args, Debug)]
pub(crate) struct Command {
    #[arg(short = 'l', long)]
    #[arg(value_enum)]
    #[arg(default_value_t = SourceGenerationLevel::Definitions)]
    level: SourceGenerationLevel,

    /// Set the number of spaces for indentation
    #[arg(short = 's', long)]
    #[arg(default_value = "2")]
    indent_spaces: usize,

    #[command(flatten)]
    files: super::FileArgs,
}

/// Determine the amount of source to generate
#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum SourceGenerationLevel {
    /// Top-level definitions, incomplete
    Definitions,
    /// Top-level definitions and members, incomplete
    Members,
    /// Full source
    Full,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl super::Command for Command {
    fn execute(&self) -> Result<ExitCode, Error> {
        call_with_module!(self, |module: &Module, cache, _| {
            let options = SourceGeneratorOptions::default().with_level(self.level.into());
            let mut generator = SourceGenerator::default();
            let mut output = self.files.output.clone();
            let mut writer = output.lock();

            generator.generate_with_options(module, cache, options, None, &mut writer)?;

            Ok(ExitCode::SUCCESS)
        });
    }
}

// ------------------------------------------------------------------------------------------------

impl From<SourceGenerationLevel> for sdml_generate::convert::source::SourceGenerationLevel {
    fn from(v: SourceGenerationLevel) -> Self {
        match v {
            SourceGenerationLevel::Full => Self::Full,
            SourceGenerationLevel::Definitions => Self::Definitions,
            SourceGenerationLevel::Members => Self::Members,
        }
    }
}
