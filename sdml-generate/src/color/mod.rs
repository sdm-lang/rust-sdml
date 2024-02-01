/*!

*/

use nu_ansi_term::{Color, Style};
use std::env;
use std::fmt::Display;
use std::sync::OnceLock;
use std::sync::RwLock;

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

macro_rules! method {
    ($name:ident, $el:ident) => {
        #[inline(always)]
        fn $name<S>(&self, value: S) -> String
        where
            S: AsRef<str>,
        {
            self.colorize(LanguageElement::$el, value)
        }
    };
}

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
pub enum LanguageElement {
    Comment,
    Constant,
    ConstantDefinition,
    Embedded,
    Error,
    Function,
    FunctionCall,
    FunctionDefinition,
    Keyword,
    LiteralBoolean,
    LiteralNumber,
    LiteralString,
    LiteralStringSpecial,
    Operator,
    Method,
    MethodDefinition,
    Module,
    ModuleDefinition,
    Property,
    PropertyDefinition,
    Punctuation,
    PunctuationBracket,
    PunctuationDelimiter,
    PunctuationSeparator,
    Type,
    TypeBuiltin,
    TypeDefinition,
    Variable,
    VariableBuiltin,
    VariableField,
    VariableParameter,
}

pub trait Colorizer {
    fn colorize<S>(&self, el: LanguageElement, value: S) -> String
    where
        S: AsRef<str>;

    method!(comment, Comment);
    method!(constant, Constant);
    method!(constant_definition, ConstantDefinition);
    method!(embedded, Embedded);
    method!(error, Error);
    method!(function, Function);
    method!(function_call, FunctionCall);
    method!(function_definition, FunctionDefinition);
    method!(keyword, Keyword);
    method!(boolean, LiteralBoolean);
    method!(number, LiteralNumber);
    method!(string, LiteralString);
    method!(string_special, LiteralStringSpecial);
    method!(url, LiteralStringSpecial);
    method!(operator, Operator);
    method!(method, Method);
    method!(method_definition, MethodDefinition);
    method!(module, Module);
    method!(module_definition, ModuleDefinition);
    method!(property, Property);
    method!(property_definitions, PropertyDefinition);
    method!(punctuation, Punctuation);
    method!(bracket, PunctuationBracket);
    method!(delimiter, PunctuationDelimiter);
    method!(separator, PunctuationSeparator);
    method!(type_ref, Type);
    method!(type_builtin, TypeBuiltin);
    method!(type_definition, TypeDefinition);
    method!(variable, Variable);
    method!(variable_field, VariableField);
    method!(variable_parameter, VariableParameter);
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ConsoleColor {}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct HtmlColor {}

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

impl Display for LanguageElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LanguageElement::Comment => "comment",
                LanguageElement::Constant => "constant",
                LanguageElement::ConstantDefinition => "constant.definition",
                LanguageElement::Embedded => "embedded",
                LanguageElement::Error => "error",
                LanguageElement::Function => "function",
                LanguageElement::FunctionCall => "function.call",
                LanguageElement::FunctionDefinition => "function.definition",
                LanguageElement::Keyword => "keyword",
                LanguageElement::LiteralBoolean => "boolean",
                LanguageElement::LiteralNumber => "number",
                LanguageElement::LiteralString => "string",
                LanguageElement::LiteralStringSpecial => "string.special",
                LanguageElement::Operator => "operator",
                LanguageElement::Method => "method",
                LanguageElement::MethodDefinition => "method.definition",
                LanguageElement::Module => "module",
                LanguageElement::ModuleDefinition => "module.definition",
                LanguageElement::Property => "property",
                LanguageElement::PropertyDefinition => "property.definition",
                LanguageElement::Punctuation => "punctuation",
                LanguageElement::PunctuationBracket => "punctuation.bracket",
                LanguageElement::PunctuationDelimiter => "punctuation.delimiter",
                LanguageElement::PunctuationSeparator => "punctuation.separator",
                LanguageElement::Type => "type",
                LanguageElement::TypeBuiltin => "type.builtin",
                LanguageElement::TypeDefinition => "type.definition",
                LanguageElement::Variable => "variable",
                LanguageElement::VariableBuiltin => "variable.builtin",
                LanguageElement::VariableField => "variable.field",
                LanguageElement::VariableParameter => "variable.parameter",
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl Colorizer for ConsoleColor {
    fn colorize<S>(&self, el: LanguageElement, value: S) -> String
    where
        S: AsRef<str>,
    {
        if colorize().colorize() {
            self.style(el).paint(value.as_ref()).to_string()
        } else {
            value.as_ref().to_string()
        }
    }
}

impl ConsoleColor {
    pub const fn new() -> Self {
        Self {}
    }

    fn style(&self, el: LanguageElement) -> Style {
        match el {
            LanguageElement::Comment => Style::new().fg(Color::Fixed(248)).italic(),
            // Constants
            LanguageElement::Constant => Style::new().fg(Color::Fixed(58)),
            LanguageElement::ConstantDefinition => Style::new().fg(Color::Fixed(58)).bold(),
            // Other languages embedded in this one
            LanguageElement::Embedded => Style::new().fg(Color::Fixed(248)).italic(),
            // Those pesky errors
            LanguageElement::Error => Style::new().fg(Color::Fixed(9)).underline(),
            // Functions (vs methods)
            LanguageElement::Function => Style::new().fg(Color::Fixed(26)),
            LanguageElement::FunctionCall => Style::new().fg(Color::Fixed(26)),
            LanguageElement::FunctionDefinition => Style::new().fg(Color::Fixed(26)).bold(),
            // Language Keywords
            LanguageElement::Keyword => Style::new().fg(Color::Fixed(100)),
            // Literal values
            LanguageElement::LiteralBoolean => Style::new().fg(Color::Fixed(58)),
            LanguageElement::LiteralNumber => Style::new().fg(Color::Fixed(58)),
            LanguageElement::LiteralString => Style::new().fg(Color::Fixed(70)),
            LanguageElement::LiteralStringSpecial => Style::new().fg(Color::Fixed(92)),
            // Operators
            LanguageElement::Operator => Style::new().fg(Color::Fixed(239)).bold(),
            // Methods (vs functions)
            LanguageElement::Method => Style::new().fg(Color::Fixed(26)),
            LanguageElement::MethodDefinition => Style::new().fg(Color::Fixed(26)).bold(),
            // Modules
            LanguageElement::Module => Style::new().fg(Color::Fixed(19)),
            LanguageElement::ModuleDefinition => Style::new().fg(Color::Fixed(19)).bold(),
            // Properties/Attributes
            LanguageElement::Property => Style::new().fg(Color::Fixed(160)),
            LanguageElement::PropertyDefinition => Style::new().fg(Color::Fixed(160)).bold(),
            // Punctuation
            LanguageElement::Punctuation => Style::new().fg(Color::Fixed(239)),
            LanguageElement::PunctuationBracket => Style::new().fg(Color::Fixed(239)),
            LanguageElement::PunctuationDelimiter => Style::new().fg(Color::Fixed(239)),
            LanguageElement::PunctuationSeparator => Style::new().fg(Color::Fixed(239)),
            // Types
            LanguageElement::Type => Style::new().fg(Color::Fixed(27)),
            LanguageElement::TypeBuiltin => Style::new().fg(Color::Fixed(21)),
            LanguageElement::TypeDefinition => Style::new().fg(Color::Fixed(27)).bold(),
            // Variables
            LanguageElement::Variable => Style::new().fg(Color::Fixed(67)),
            LanguageElement::VariableBuiltin => Style::new().fg(Color::Fixed(67)),
            LanguageElement::VariableField => Style::new().fg(Color::Fixed(67)),
            LanguageElement::VariableParameter => Style::new().fg(Color::Fixed(67)),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Colorizer for HtmlColor {
    fn colorize<S>(&self, el: LanguageElement, value: S) -> String
    where
        S: AsRef<str>,
    {
        if colorize().colorize() {
            format!(
                "<span class=\"{}\">{}</span>",
                el.to_string().replace('.', "-"),
                value.as_ref()
            )
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

pub mod sdml;
