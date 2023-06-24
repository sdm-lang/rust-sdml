/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use ansi_term::{Style, Color};
use lineindex::IndexedString;
use tree_sitter::Node;
use crate::model::Span;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub(crate) struct Error {
    severity: Severity,
    message: String,
    reference: Option<String>,
    file_name: Option<String>,
    location: Option<Span>,
    related: Vec<(bool, Error)>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub(crate) enum ErrorId {
    ModuleNotFound,
    TreeSitterError,
    UnexpectedNodeKind,
    ModuleAlreadyImported,
    MemberAlreadyImported,
    TypeDefinitionNameUsed,
    MemberNameUsed,
    VariantNameUsed,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

macro_rules! fmt_normal {
    ($fmt: literal $(, $value:expr)*) => {
        Style::default()
            .paint(
                format!($fmt $(, $value)*)
            )
    };
}

macro_rules! fmt_error {
    ($color:expr, $fmt: literal $(, $value:expr)*) => {
        if $color {
            Style::new()
                .fg(Color::Red)
                .bold()
                .paint(
                    format!($fmt $(, $value)*)
                )
        } else {
            fmt_normal!($fmt $(, $value)*)
        }
    };
}

macro_rules! fmt_severity {
    ($color:expr, $severity: expr, $fmt: literal $(, $value:expr)*) => {
        if $color {
            match $severity {
                Severity::Error => fmt_error!($color, $fmt $(, $value)*),
                Severity::Warning =>fmt_warning!($color, $fmt $(, $value)*),
                Severity::Note => fmt_note!($color, $fmt $(, $value)*),
                Severity::Help => fmt_advice!($color, $fmt $(, $value)*),
            }
        } else {
            fmt_normal!($fmt $(, $value)*)
        }
    };
}

macro_rules! fmt_severity_string {
    ($color:expr, $severity: expr) => {
        fmt_severity!($color, $severity, "{}", $severity)
    };
}

macro_rules! fmt_warning {
    ($color:expr, $fmt: literal $(, $value:expr)*) => {
        if $color {
            Style::new()
                .fg(Color::Yellow)
                .bold()
                .paint(
                    format!($fmt $(, $value)*)
                )
        } else {
            fmt_normal!($fmt $(, $value)*)
        }
    };
}

macro_rules! fmt_note {
    ($color:expr, $fmt: literal $(, $value:expr)*) => {
        if $color {
            Style::new()
                .fg(Color::Cyan)
                .bold()
                .paint(
                    format!($fmt $(, $value)*)
                )
        } else {
            fmt_normal!($fmt $(, $value)*)
        }
    };
}

macro_rules! fmt_advice {
    ($color:expr, $fmt: literal $(, $value:expr)*) => {
        if $color {
            Style::new()
                .fg(Color::Green)
                .bold()
                .paint(
                    format!($fmt $(, $value)*)
                )
        } else {
            fmt_normal!($fmt $(, $value)*)
        }
    };
}

macro_rules! fmt_message {
    ($color:expr, $fmt: literal $(, $value:expr)*) => {
        if $color {
            Style::new()
                .bold()
                .paint(
                    format!($fmt $(, $value)*)
                )
        } else {
            fmt_normal!($fmt $(, $value)*)
        }
    };
}

macro_rules! fmt_structure {
    ($color:expr, $fmt: literal $(, $value:expr)*) => {
        if $color {
            Style::new()
                .fg(Color::Blue)
                .paint(
                    format!($fmt $(, $value)*)
                )
        } else {
            fmt_normal!($fmt $(, $value)*)
        }
    };
}

macro_rules! fmt_structure_string {
    ($color:expr, $structure: expr) => {
        fmt_structure!($color, "{}", $structure)
    };
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Severity {
    Error,
    Warning,
    Note,
    Help,
}

const FILE_NAME_STDIN: &str = "<stdin>";

const DRAW_VBAR: &str = "│";
const DRAW_DBAR: &str = "┆";
const DRAW_RTEE: &str = "├";
const DRAW_DTEE: &str = "─";
const DRAW_HBAR: &str = "┬";
const DRAW_RARR: &str = "▶";

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Error {
    pub(crate) fn error(id: ErrorId) -> Self {
        Self::new(Severity::Error, id)
    }

    pub(crate) fn warning(id: ErrorId) -> Self {
        Self::new(Severity::Warning, id)
    }

    pub(crate) fn note(id: ErrorId) -> Self {
        Self::new(Severity::Note, id)
    }

    pub(crate) fn help(id: ErrorId) -> Self {
        Self::new(Severity::Help, id)
    }

    pub(crate) fn error_message<S>(message: S) -> Self
    where
        S: Into<String>,
    {
        Self::new_message(Severity::Error, message)
    }

    pub(crate) fn warning_message<S>(message: S) -> Self
    where
        S: Into<String>,
    {
        Self::new_message(Severity::Warning, message)
    }

    pub(crate) fn note_message<S>(message: S) -> Self
    where
        S: Into<String>
    {
        Self::new_message(Severity::Note, message)
    }

    pub(crate) fn help_message<S>(message: S) -> Self
    where
    S: Into<String>,
    {
        Self::new_message(Severity::Help, message)
    }

    fn new(severity: Severity, id: ErrorId) -> Self {
        Self {
            severity,
            message: id.message().to_string(),
            reference: if severity == Severity::Error { Some(format!("[E{:04}]", id as u32)) } else { None },
            file_name: Default::default(),
            location: Default::default(),
            related: Default::default(),
        }
    }

    fn new_message<S>(severity: Severity, message: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            severity,
            message: message.into(),
            reference: Default::default(),
            file_name: Default::default(),
            location: Default::default(),
            related: Default::default(),
        }
    }

    pub(crate) fn in_file<S>(self, file_name: Option<S>) -> Self
    where
        S: Into<String>,
    {
        let mut self_mut = self;
        self_mut.file_name = file_name.map(|f|f.into());
        self_mut
    }

    pub(crate) fn at_location<L>(self, location: L) -> Self
    where
        L: Into<Span>,
    {
        let mut self_mut = self;
        self_mut.location = Some(location.into());
        self_mut
    }

    pub(crate) fn at_node_location(self, node: &Node<'_>) -> Self {
        let span: Span = node.into();
        self.at_location(span)
    }

    pub(crate) fn add_advice(self, advice: Error) -> Self {
        let mut self_mut = self;
        self_mut.related.push((true, advice));
        self_mut
    }

    pub(crate) fn add_related(self, advice: Error) -> Self {
        let mut self_mut = self;
        self_mut.related.push((false, advice));
        self_mut
    }

    pub(crate) fn report(self, use_color: bool, source: &IndexedString<'_>) {
        self.report_any(use_color, false, source)
    }

    fn report_any(self, use_color: bool, is_advice: bool, source: &IndexedString<'_>) {
        // HeadLine     = Severity, ( "[", SeverityChar, NumericId, "]" )?, ":", Message
        // Severity     = "error" | "warning" | "note" | "help"
        // SeverityChar = "E" | "W" | "N" | "H"
        eprintln!(
            "{}{}{}",
            if is_advice { "    " } else { "" },
            fmt_severity!(
                use_color,
                self.severity,
                "{}{}",
                self.severity,
                if let Some(reference) = self.reference {
                    reference
                } else {
                    String::new()
                }
            ),
            fmt_message!(
                use_color,
                ": {}",
                self.message
            )
        );

        // LocationLine  = "-->", FileReference, ":", Location
        // FileReference = FileName | "<stdin>"
        // Location      = Line, ":", Column
        eprintln!(
            "{} {}",
            fmt_structure_string!(use_color, if is_advice { "    ├─▶" } else { "└───┬─▶" }),
            fmt_normal!(
                "{}:{}",
                self.file_name.unwrap_or_else(|| FILE_NAME_STDIN.to_string()),
                if let Some(location) = &self.location {
                    format!("{}:{}", location.start().line() + 1, location.start().column() + 1)
                } else {
                    String::new()
                }
            )
        );

        // Vertical spacing
        eprintln!("    {}", fmt_structure_string!(use_color, DRAW_VBAR));

        // SourceLine      = LineNumber, "|", String
        // SourceHighlight = Padding, "^"...
        if let Some(location) = &self.location {
            let line_number = location.start().line();
            eprintln!(
                "{}{}",
                fmt_structure!(use_color, "{:3} │ ", line_number + 1),
                source.line_str(line_number).unwrap().trim_end()
            );
            let left_pad = location.start().column();
            let carets = location.end().column() - location.start().column();
            eprintln!(
                "    {} {:left_pad$}{}",
                fmt_structure_string!(use_color, DRAW_VBAR),
                "",
                fmt_severity!(use_color, self.severity, "{:^>carets$}", "")
            );
        }

        // All related advice
        for (is_advice, error) in self.related {
            eprintln!("    {}", fmt_structure_string!(use_color, if is_advice { DRAW_DBAR } else { DRAW_VBAR }));
            error.report_any(use_color, is_advice, source)
        }

        // Vertical spacing
        eprintln!();
    }
}

// ------------------------------------------------------------------------------------------------

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Error => "error",
                Self::Warning => "warning",
                Self::Note => "note",
                Self::Help => "help",
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl ErrorId {
    pub(crate) fn message(&self) -> &'static str {
        match self {
            Self::ModuleNotFound => "module not found",
            Self::TreeSitterError => "tree-sitter parse error",
            Self::UnexpectedNodeKind => "unexpected tree-sitter node",
            Self::ModuleAlreadyImported => "module already imported",
            Self::MemberAlreadyImported => "member already imported",
            Self::TypeDefinitionNameUsed => "type definition name already defined",
            Self::MemberNameUsed => "member name already defined",
            Self::VariantNameUsed => "variable name already defined",
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
