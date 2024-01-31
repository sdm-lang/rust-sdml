/*!

*/

use nu_ansi_term::{Color, Style};
use std::env;
use std::fmt::Display;
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ColorElement {
    Comment,
    Constant,
    Embedded,
    Error,
    FunctionCall,
    FunctionDefinition,
    Keyword,
    LiteralBoolean,
    LiteralNumber,
    Operator,
    MethodDefinition,
    Module,
    ModuleDefinition,
    Property,
    PunctuationBracket,
    PunctuationDelimiter,
    PunctuationSeparator,
    LiteralString,
    LiteralStringSpecial,
    Type,
    TypeBuiltin,
    TypeDefinition,
    Variable,
    VariableBuiltin,
    VariableField,
    VariableParameter,
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

impl Display for ColorElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ColorElement::Comment => "comment",
                ColorElement::Constant => "constant",
                ColorElement::Embedded => "embedded",
                ColorElement::Error => "error",
                ColorElement::FunctionCall => "function.call",
                ColorElement::FunctionDefinition => "function.definition",
                ColorElement::Keyword => "keyword",
                ColorElement::LiteralBoolean => "boolean",
                ColorElement::LiteralNumber => "number",
                ColorElement::LiteralString => "string",
                ColorElement::LiteralStringSpecial => "string.special",
                ColorElement::Operator => "operator",
                ColorElement::MethodDefinition => "method.definition",
                ColorElement::Module => "module",
                ColorElement::ModuleDefinition => "module.definition",
                ColorElement::Property => "property",
                ColorElement::PunctuationBracket => "punctuation.bracket",
                ColorElement::PunctuationDelimiter => "punctuation.delimiter",
                ColorElement::PunctuationSeparator => "punctuation.separator",
                ColorElement::Type => "type",
                ColorElement::TypeBuiltin => "type.builtin",
                ColorElement::TypeDefinition => "type.definition",
                ColorElement::Variable => "variable",
                ColorElement::VariableBuiltin => "variable.builtin",
                ColorElement::VariableField => "variable.field",
                ColorElement::VariableParameter => "variable.parameter",
            }
        )
    }
}

impl From<&ColorElement> for Style {
    fn from(value: &ColorElement) -> Self {
        match value {
            ColorElement::Comment => Style::new().fg(Color::Fixed(248)).italic(),
            ColorElement::Constant => Style::new().fg(Color::Fixed(58)),
            ColorElement::Embedded => Style::new().fg(Color::Fixed(248)).italic(),
            ColorElement::Error => Style::new().fg(Color::Fixed(9)).underline(),
            ColorElement::FunctionCall => Style::new().fg(Color::Fixed(26)),
            ColorElement::FunctionDefinition => Style::new().fg(Color::Fixed(26)).bold(),
            ColorElement::Keyword => Style::new().fg(Color::Fixed(100)),
            ColorElement::LiteralBoolean => Style::new().fg(Color::Fixed(58)),
            ColorElement::LiteralNumber => Style::new().fg(Color::Fixed(58)),
            ColorElement::LiteralString => Style::new().fg(Color::Fixed(70)),
            ColorElement::LiteralStringSpecial => Style::new().fg(Color::Fixed(92)),
            ColorElement::Operator => Style::new().fg(Color::Fixed(239)).bold(),
            ColorElement::MethodDefinition => Style::new().fg(Color::Fixed(26)).bold(),
            ColorElement::Module => Style::new().fg(Color::Fixed(19)),
            ColorElement::ModuleDefinition => Style::new().fg(Color::Fixed(19)).bold(),
            ColorElement::Property => Style::new().fg(Color::Fixed(160)),
            ColorElement::PunctuationBracket => Style::new().fg(Color::Fixed(239)),
            ColorElement::PunctuationDelimiter => Style::new().fg(Color::Fixed(239)),
            ColorElement::PunctuationSeparator => Style::new().fg(Color::Fixed(239)),
            ColorElement::Type => Style::new().fg(Color::Fixed(27)),
            ColorElement::TypeBuiltin => Style::new().fg(Color::Fixed(21)),
            ColorElement::TypeDefinition => Style::new().fg(Color::Fixed(27)).bold(),
            ColorElement::Variable => Style::new().fg(Color::Fixed(67)),
            ColorElement::VariableBuiltin => Style::new().fg(Color::Fixed(67)),
            ColorElement::VariableField => Style::new().fg(Color::Fixed(67)),
            ColorElement::VariableParameter => Style::new().fg(Color::Fixed(67)),
        }
    }
}

impl ColorElement {
    pub fn style(&self) -> Style {
        Style::from(self)
    }

    pub fn colorize<S>(&self, value: S) -> String
    where
        S: AsRef<str>,
    {
        if colorize().colorize() {
            self.style().paint(value.as_ref()).to_string()
        } else {
            value.as_ref().to_string()
        }
    }

    pub fn colorize_if<S>(&self, value: S, flag: UseColor) -> String
    where
        S: AsRef<str>,
    {
        if colorize() == flag {
            self.style().paint(value.as_ref()).to_string()
        } else {
            value.as_ref().to_string()
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod rdf;
