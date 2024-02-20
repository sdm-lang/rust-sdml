/*!
This module contains the trait [`Reporter`] and common implementations.
 */

use crate::diagnostics::color::UseColor;
use crate::diagnostics::{diagnostic_level_enabled, Diagnostic, ErrorCode, SeverityFilter};
use crate::errors::Error;
use crate::SourceFiles;
use codespan_reporting::{
    diagnostic::Severity,
    term::{
        emit,
        termcolor::{ColorChoice, StandardStream, WriteColor},
        Chars, Config,
    },
};
use std::cell::RefCell;
use std::io::Write;
use std::ops::{Add, AddAssign};
use tracing::{error, info, warn};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This trait describes a facility to report diagnostics.
///
pub trait Reporter: Default {
    ///
    /// Emit a diagnostic, providing a mapping for source code.
    ///
    fn emit(&self, diagnostic: &Diagnostic, sources: &SourceFiles) -> Result<(), Error>;

    fn emit_without_source(&self, diagnostic: &Diagnostic) -> Result<(), Error> {
        self.emit(diagnostic, &SourceFiles::new())
    }

    fn done(&self, module_name: Option<String>) -> Result<(), Error>;

    fn log(diagnostic: &Diagnostic) {
        match diagnostic.severity {
            Severity::Bug | Severity::Error => error!(
                "[{}] {}",
                diagnostic.code.as_ref().unwrap(),
                diagnostic.message
            ),
            Severity::Warning => warn!(
                "[{}] {}",
                diagnostic.code.as_ref().unwrap(),
                diagnostic.message
            ),
            Severity::Note | Severity::Help => info!(
                "[{}] {}",
                diagnostic.code.as_ref().unwrap(),
                diagnostic.message
            ),
        }
    }
}

#[derive(Debug)]
pub struct StandardStreamReporter {
    stream: StandardStream,
    config: Config,
    counters: RefCell<ErrorCounters>,
}

#[derive(Debug, Default)]
pub struct BailoutReporter;

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, Default)]
struct ErrorCounters {
    bugs: u32,
    errors: u32,
    warnings: u32,
    info: u32,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl PartialEq<Severity> for SeverityFilter {
    fn eq(&self, other: &Severity) -> bool {
        matches!(
            (self, other),
            (SeverityFilter::Bug, Severity::Bug)
                | (SeverityFilter::Error, Severity::Error)
                | (SeverityFilter::Warning, Severity::Warning)
                | (SeverityFilter::Note, Severity::Note)
                | (SeverityFilter::Help, Severity::Help)
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl From<ErrorCode> for Diagnostic {
    fn from(code: ErrorCode) -> Self {
        Self::new(code.severity())
            .with_code(code.to_string())
            .with_message(code.message().to_string())
    }
}

// ------------------------------------------------------------------------------------------------

impl Add for ErrorCounters {
    type Output = ErrorCounters;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            bugs: self.bugs + rhs.bugs,
            errors: self.errors + rhs.errors,
            warnings: self.warnings + rhs.warnings,
            info: self.info,
        }
    }
}

impl AddAssign for ErrorCounters {
    fn add_assign(&mut self, rhs: Self) {
        self.bugs += rhs.bugs;
        self.errors += rhs.errors;
        self.warnings += rhs.warnings;
        self.info += rhs.info;
    }
}

impl ErrorCounters {
    #[inline(always)]
    fn report(&mut self, severity: Severity) {
        match severity {
            Severity::Bug => self.bugs += 1,
            Severity::Error => self.errors += 1,
            Severity::Warning => self.warnings += 1,
            Severity::Note => self.info += 1,
            Severity::Help => self.info += 1,
        }
    }

    #[inline(always)]
    fn total(&self) -> u64 {
        (self.bugs + self.errors + self.warnings + self.info) as u64
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for StandardStreamReporter {
    fn default() -> Self {
        Self::stderr(UseColor::from_env().into())
    }
}

impl Reporter for StandardStreamReporter {
    fn emit(&self, diagnostic: &Diagnostic, sources: &SourceFiles) -> Result<(), Error> {
        if diagnostic_level_enabled(diagnostic.severity) {
            <StandardStreamReporter as Reporter>::log(diagnostic);
            let mut counters = self.counters.borrow_mut();
            counters.report(diagnostic.severity);
            Ok(emit(
                &mut self.stream.lock(),
                &self.config,
                sources,
                diagnostic,
            )?)
        } else {
            Ok(())
        }
    }

    fn done(&self, module_name: Option<String>) -> Result<(), Error> {
        self.done_stats(module_name)?;
        let _ = self.counters.replace(ErrorCounters::default());
        Ok(())
    }
}

impl StandardStreamReporter {
    pub fn stderr(color_choice: ColorChoice) -> Self {
        Self {
            stream: StandardStream::stderr(color_choice),
            config: Self::default_config(),
            counters: Default::default(),
        }
    }

    pub fn stdout(color_choice: ColorChoice) -> Self {
        Self {
            stream: StandardStream::stdout(color_choice),
            config: Self::default_config(),
            counters: Default::default(),
        }
    }

    fn default_config() -> Config {
        Config {
            chars: Chars::box_drawing(),
            ..Default::default()
        }
    }

    fn done_stats(&self, module_name: Option<String>) -> Result<(), Error> {
        let counters = self.counters.borrow();
        if counters.total() > 0 {
            let severity = if counters.bugs > 0 {
                Severity::Bug
            } else if counters.errors > 0 {
                Severity::Error
            } else if counters.warnings > 0 {
                Severity::Warning
            } else if counters.info > 0 {
                Severity::Note
            } else {
                unreachable!();
            };

            let mut writer = self.stream.lock();

            writer.set_color(self.config.styles.header(severity))?;
            writer.write_all(
                match severity {
                    Severity::Bug => i18n!("word_bug"),
                    Severity::Error => i18n!("word_error"),
                    Severity::Warning => i18n!("word_warning"),
                    Severity::Note => i18n!("word_note"),
                    Severity::Help => i18n!("word_help"),
                }
                .as_bytes(),
            )?;
            writer.reset()?;
            writer.write_all(b": ")?;
            writer.write_all(
                format!(
                    "{} ",
                    if let Some(name) = module_name {
                        i18n!("lbl_module_name_short", name = name)
                    } else {
                        i18n!("lbl_parser")
                    }
                )
                .as_bytes(),
            )?;
            let mut count_strings: Vec<String> = Default::default();
            if counters.bugs > 0 {
                count_strings.push(i18n!("count_of_bugs", count = counters.bugs));
            }
            if counters.errors > 0 {
                count_strings.push(i18n!("count_of_errors", count = counters.errors));
            }
            if counters.warnings > 0 {
                count_strings.push(i18n!("count_of_warnings", count = counters.warnings));
            }
            if counters.info > 0 {
                count_strings.push(i18n!("count_of_informational", count = counters.info));
            }
            writer.write_all(
                i18n!(
                    "counts_generated_summary",
                    counts = count_strings.join(", ")
                )
                .as_bytes(),
            )?;
            writer.write_all(b"\n")?;
        }
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------

impl Reporter for BailoutReporter {
    fn emit(&self, diagnostic: &Diagnostic, _: &SourceFiles) -> Result<(), Error> {
        if diagnostic_level_enabled(diagnostic.severity) {
            <BailoutReporter as Reporter>::log(diagnostic);
            Err(diagnostic.clone().into())
        } else {
            Ok(())
        }
    }

    fn done(&self, _: Option<String>) -> Result<(), Error> {
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------
