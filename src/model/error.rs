/*!
One-line description.

More detailed description, with

# Example

YYYYY

 */

use crate::error::Error as CrateError;
use codespan_reporting::diagnostic::{Diagnostic, Severity};
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream, WriteColor};
use codespan_reporting::term::Config;
use std::io::Write;
use std::ops::{Add, AddAssign};
use std::{error::Error, fmt::Display};

use super::Identifier;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub type FileId = usize;

#[derive(Clone, Debug)]
pub struct SimpleDiagnostic {
    severity: Severity,
    code: &'static str,
    message: &'static str,
}

#[derive(Clone, Debug, Default)]
pub struct ErrorCounters {
    bugs: u32,
    errors: u32,
    warnings: u32,
    notes: u32,
    help: u32,
}

// ------------------------------------------------------------------------------------------------
// Public Error Values
// ------------------------------------------------------------------------------------------------

pub const MODULE_NOT_FOUND: SimpleDiagnostic = SimpleDiagnostic {
    severity: Severity::Bug,
    code: "B001",
    message: "module not found",
};

pub const TREE_SITTER_ERROR: SimpleDiagnostic = SimpleDiagnostic {
    severity: Severity::Error,
    code: "E010",
    message: "tree-sitter parse error",
};

pub const UNEXPECTED_NODE_KIND: SimpleDiagnostic = SimpleDiagnostic {
    severity: Severity::Error,
    code: "E011",
    message: "unexpected tree-sitter node",
};

pub const MODULE_ALREADY_IMPORTED: SimpleDiagnostic = SimpleDiagnostic {
    severity: Severity::Warning,
    code: "W020",
    message: "duplicate import of module",
};

pub const MEMBER_ALREADY_IMPORTED: SimpleDiagnostic = SimpleDiagnostic {
    severity: Severity::Warning,
    code: "W021",
    message: "duplicate import of member",
};

pub const TYPE_DEFINITION_NAME_USED: SimpleDiagnostic = SimpleDiagnostic {
    severity: Severity::Error,
    code: "E022",
    message: "a type definition with this name already exists",
};

pub const MEMBER_NAME_USED: SimpleDiagnostic = SimpleDiagnostic {
    severity: Severity::Error,
    code: "E023",
    message: "a member with this name already exists",
};

pub const VALUE_VARIANT_NAME_USED: SimpleDiagnostic = SimpleDiagnostic {
    severity: Severity::Error,
    code: "E024",
    message: "a value variant with this name already exists",
};

pub const TYPE_VARIANT_NAME_USED: SimpleDiagnostic = SimpleDiagnostic {
    severity: Severity::Error,
    code: "E025",
    message: "a type variant with this type or name already exists",
};

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

#[inline(always)]
fn severity_str(severity: Severity) -> &'static str {
    match severity {
        Severity::Bug => "bug",
        Severity::Error => "error",
        Severity::Warning => "warning",
        Severity::Note => "note",
        Severity::Help => "help",
    }
}

impl ErrorCounters {
    #[inline(always)]
    pub fn bugs(&self) -> u32 {
        self.bugs
    }

    #[inline(always)]
    pub fn report(&mut self, severity: Severity) {
        match severity {
            Severity::Bug => self.bugs += 1,
            Severity::Error => self.errors += 1,
            Severity::Warning => self.warnings += 1,
            Severity::Note => self.notes += 1,
            Severity::Help => self.help += 1,
        }
    }

    #[inline(always)]
    pub fn errors(&self) -> u32 {
        self.errors
    }

    #[inline(always)]
    pub fn warnings(&self) -> u32 {
        self.warnings
    }

    #[inline(always)]
    pub fn notes(&self) -> u32 {
        self.notes
    }

    #[inline(always)]
    pub fn help(&self) -> u32 {
        self.help
    }

    #[inline(always)]
    pub fn total(&self) -> u64 {
        (self.bugs + self.errors + self.warnings + self.notes + self.help) as u64
    }

    pub fn display(&self, name: &Identifier) -> Result<(), CrateError> {
        if self.total() > 0 {
            let config = Config::default();
            let stream = StandardStream::stderr(ColorChoice::Always);
            let mut writer = stream.lock();

            let (severity, count) = if self.bugs > 0 {
                (Severity::Bug, self.bugs)
            } else if self.errors > 0 {
                (Severity::Error, self.errors)
            } else if self.warnings > 0 {
                (Severity::Warning, self.warnings)
            } else if self.notes > 0 {
                (Severity::Note, self.notes)
            } else if self.help > 0 {
                (Severity::Help, self.help)
            } else {
                unreachable!();
            };

            writer.set_color(config.styles.header(severity))?;
            writer.write_all(severity_str(severity).as_bytes())?;
            writer.reset()?;
            writer.write_all(format!(": module `{name}` generated {count} bugs").as_bytes())?;
            // TODO: include other counts in short form
            writer.write_all(b".\n")?;
        }
        Ok(())
    }
}

impl Add for ErrorCounters {
    type Output = ErrorCounters;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            bugs: self.bugs + rhs.bugs,
            errors: self.errors + rhs.errors,
            warnings: self.warnings + rhs.warnings,
            notes: self.notes + rhs.notes,
            help: self.help + rhs.help,
        }
    }
}

impl AddAssign for ErrorCounters {
    fn add_assign(&mut self, rhs: Self) {
        self.bugs += rhs.bugs;
        self.errors += rhs.errors;
        self.warnings += rhs.warnings;
        self.notes += rhs.notes;
        self.help += rhs.help;
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for SimpleDiagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for SimpleDiagnostic {}

impl SimpleDiagnostic {
    pub fn into_diagnostic(&self) -> Diagnostic<FileId> {
        let new = match self.severity {
            Severity::Bug => Diagnostic::bug(),
            Severity::Error => Diagnostic::error(),
            Severity::Warning => Diagnostic::warning(),
            Severity::Note => Diagnostic::note(),
            Severity::Help => Diagnostic::help(),
        };
        new.with_code(self.code).with_message(self.message)
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
