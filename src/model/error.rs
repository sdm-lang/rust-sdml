/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use ansi_term::{Style, Color};
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
    identifier: u32,
    message: String,
    file_name: String,
    location: Option<Span>,
    message_additional: Option<String>,
    more_info: Vec<MoreInformation>,
}

#[derive(Clone, Debug)]
pub(crate) struct MoreInformation {
    severity: Severity,
    message: String,
    file_name: Option<String>,
    location: Option<Span>,
    message_additional: Option<String>,
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

pub(crate) fn message_found_node(found: &str) -> String {
    format!("found `{found}`")
}

pub(crate) fn message_expecting_node(expecting: &str) -> String {
    format!("expecting: `{expecting}`")
}

pub(crate) fn message_expecting_one_of_node(expecting: &[&str]) -> String {
    format!(
        "expecting on of: {}",
        expecting
            .iter()
            .map(|s| format!("`{s}`"))
            .collect::<Vec<String>>()
            .join("|")
    )
}

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
                Severity::Advice => fmt_advice!($color, $fmt $(, $value)*),
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

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Severity {
    Error,
    Warning,
    Note,
    Advice,
}

const FILE_NAME_STDIN: &str = "<stdin>";

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Error {
    pub(crate) fn error(id: ErrorId) -> Self {
        Self {
            severity: Severity::Error,
            identifier: id as u32,
            message: id.message().to_string(),
            file_name: FILE_NAME_STDIN.to_string(),
            location: Default::default(),
            message_additional: Default::default(),
            more_info: Default::default(),
        }
    }

    pub(crate) fn warning(id: ErrorId) -> Self {
        Self {
            severity: Severity::Warning,
            identifier: id as u32,
            message: id.message().to_string(),
            file_name: FILE_NAME_STDIN.to_string(),
            location: Default::default(),
            message_additional: Default::default(),
            more_info: Default::default(),
        }
    }

    pub(crate) fn note(id: ErrorId) -> Self {
        Self {
            severity: Severity::Note,
            identifier: id as u32,
            message: id.message().to_string(),
            file_name: FILE_NAME_STDIN.to_string(),
            location: Default::default(),
            message_additional: Default::default(),
            more_info: Default::default(),
        }
    }

    pub(crate) fn advice(id: ErrorId) -> Self {
        Self {
            severity: Severity::Advice,
            identifier: id as u32,
            message: id.message().to_string(),
            file_name: FILE_NAME_STDIN.to_string(),
            location: Default::default(),
            message_additional: Default::default(),
            more_info: Default::default(),
        }
    }

    pub(crate) fn in_file<S>(self, file_name: S) -> Self
    where
        S: Into<String>,
    {
        let mut self_mut = self;
        self_mut.file_name = file_name.into();
        self_mut
    }

    pub(crate) fn in_file_or_stdin<S>(self, file_name: Option<S>) -> Self
    where
        S: Into<String>,
    {
        if let Some(file_name) = file_name {
            self.in_file(file_name)
        } else {
            self.from_stdin()
        }
    }

    pub(crate) fn from_stdin(self) -> Self {
        let mut self_mut = self;
        self_mut.file_name = FILE_NAME_STDIN.to_string();
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

    pub(crate) fn additional_message<S>(self, message: S) -> Self
    where
        S: Into<String>,
    {
        let mut self_mut = self;
        self_mut.message_additional = Some(message.into());
        self_mut
    }

    pub(crate) fn add(self, more: MoreInformation) -> Self {
        let mut self_mut = self;
        self_mut.more_info.push(more);
        self_mut
    }

    pub(crate) fn report(self, use_color: bool) {
        let error_id = if self.severity == Severity::Error {
            format!("[E{:04}]", self.identifier)
        } else {
            String::new()
        };
        eprintln!(
            "{}{}",
            fmt_severity!(
                use_color,
                self.severity,
                "{}{}",
                self.severity,
                error_id
            ),
            fmt_message!(
                use_color,
                ": {}",
                self.message
            )
        );
        eprintln!(
            "  {} {}",
            fmt_structure!(use_color, "-->"),
            fmt_normal!(
                "{}:{}",
                self.file_name,
                if let Some(location) = &self.location {
                    location.start().to_string()
                } else {
                    String::new()
                }
            )
        );
        if let Some(additional) = self.message_additional {
            eprintln!(
                "   {} {}{}",
                fmt_structure!(use_color, "="),
                fmt_severity_string!(use_color, Severity::Note),
                fmt_normal!(": {}", additional)
            );
        }
        for more in self.more_info {
            more.report(use_color, &self.file_name);
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl MoreInformation {
    pub(crate) fn note<S>(message: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            severity: Severity::Note,
            message: message.into(),
            file_name: Default::default(),
            location: Default::default(),
            message_additional: Default::default(),
        }
    }

    pub(crate) fn advice<S>(message: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            severity: Severity::Advice,
            message: message.into(),
            file_name: Default::default(),
            location: Default::default(),
            message_additional: Default::default(),
        }
    }

    pub(crate) fn in_file<S>(self, named: S) -> Self
    where
        S: Into<String>,
    {
        let mut self_mut = self;
        self_mut.file_name = Some(named.into());
        self_mut
    }

    pub(crate) fn from_stdin(self) -> Self {
        let mut self_mut = self;
        self_mut.file_name = None;
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

    pub(crate) fn additional_message<S>(self, message: S) -> Self
    where
        S: Into<String>,
    {
        let mut self_mut = self;
        self_mut.message_additional = Some(message.into());
        self_mut
    }

    fn report(self, use_color: bool, parent_file: &String) {
        eprintln!(
            "{}{}",
            fmt_severity_string!(use_color, self.severity),
            fmt_message!(use_color, ": {}", self.message)
        );
        eprintln!(
            "   {} {}",
            fmt_structure!(use_color, "-->"),
            fmt_normal!(
                "{}:{}",
                if let Some(file_name) = &self.file_name {
                    file_name
                } else {
                    parent_file
                },
                if let Some(location) = &self.location {
                    location.start().to_string()
                } else {
                    String::new()
                }
            )
        );
        if let Some(additional) = self.message_additional {
            eprintln!(
                "   {} {}{}",
                fmt_structure!(use_color, "="),
                fmt_severity_string!(use_color, Severity::Note),
                fmt_normal!(": {}", additional)
            );
        }
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
                Self::Advice => "advice",
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl ErrorId {
    pub(crate) fn message(&self) -> &'static str {
        match self {
            Self::ModuleNotFound => "Module not found",
            Self::TreeSitterError => "Tree-sitter parse error",
            Self::UnexpectedNodeKind => "Unexpected tree-sitter node",
            Self::ModuleAlreadyImported => "Module already imported",
            Self::MemberAlreadyImported => "Member already imported",
            Self::TypeDefinitionNameUsed => "Type definition name already defined",
            Self::MemberNameUsed => "Member name already defined",
            Self::VariantNameUsed => "Variable name already defined",
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
