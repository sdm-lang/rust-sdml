/*!

 */

use std::env;
use std::sync::OnceLock;
use std::sync::RwLock;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum UseColor {
    Always,
    Auto,
    Never,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

static COLORIZE: OnceLock<RwLock<UseColor>> = OnceLock::new();

pub fn colorize() -> UseColor {
    *COLORIZE.get_or_init(init_colorize).read().unwrap()
}

pub fn set_colorize(colorize: UseColor) {
    *COLORIZE.get_or_init(init_colorize).write().unwrap() = colorize;
}

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

fn init_colorize() -> RwLock<UseColor> {
    let mut colorize = UseColor::Always;

    if env::var("NO_COLOR").is_ok() {
        colorize = UseColor::Never;
    } else if let Ok(value) = env::var("CLICOLOR") {
        if value == "0" {
            colorize = UseColor::Never;
        } else if value == "1" {
            colorize = UseColor::Always
        }
    };

    RwLock::new(colorize)
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for UseColor {
    fn default() -> Self {
        Self::Auto
    }
}

impl UseColor {
    #[inline(always)]
    pub fn colorize(&self) -> bool {
        *self != Self::Never
    }

    #[inline(always)]
    pub fn always(&self) -> bool {
        *self == Self::Always
    }

    #[inline(always)]
    pub fn auto(&self) -> bool {
        *self == Self::Auto
    }

    #[inline(always)]
    pub fn never(&self) -> bool {
        *self == Self::Never
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
