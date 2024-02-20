/*!
A common helper type to determine color output.
 */

use codespan_reporting::term::termcolor::ColorChoice;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum UseColor {
    Always,
    Auto,
    Never,
}

///
/// A common environment variable for command-line tools, if set to any value color should not
/// be used.
///
pub const COLOR_NO_COLOR_ENV: &str = "NO_COLOR";

///
/// A Rusty environment variable for command-line tools, if set to `0` color
/// should not be used, if set to `1` color should be used, otherwise it's up to
/// the tool to decide.
///
/// Note that the variable `NO_COLOR` overrides `CLI_COLOR`.
///
pub const COLOR_CLI_COLOR_ENV: &str = "CLI_COLOR";

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

const COLOR_CLI_COLOR_ON: &str = "1";

const COLOR_CLI_COLOR_OFF: &str = "0";

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for UseColor {
    fn default() -> Self {
        Self::Auto
    }
}

impl From<UseColor> for ColorChoice {
    fn from(value: UseColor) -> Self {
        match value {
            UseColor::Always => Self::Always,
            UseColor::Auto => Self::Auto,
            UseColor::Never => Self::Never,
        }
    }
}

impl UseColor {
    pub fn from_env() -> Self {
        match (Self::from_no_color_env(), Self::from_cli_color_env()) {
            (Some(no_color), _) => no_color,
            (None, Some(cli_color)) => cli_color,
            (None, None) => Self::default(),
        }
    }

    #[inline(always)]
    pub fn use_color(&self) -> bool {
        matches!(self, Self::Always | Self::Auto)
    }

    #[inline(always)]
    pub fn from_no_color_env() -> Option<Self> {
        std::env::var(COLOR_NO_COLOR_ENV).map(|_| Self::Never).ok()
    }

    #[inline(always)]
    pub fn from_cli_color_env() -> Option<Self> {
        std::env::var(COLOR_NO_COLOR_ENV)
            .map(|v| {
                if v == COLOR_CLI_COLOR_OFF {
                    Self::Never
                } else if v == COLOR_CLI_COLOR_ON {
                    Self::Always
                } else {
                    Self::Auto
                }
            })
            .ok()
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
