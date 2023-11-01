use std::{
    io::{self, ErrorKind, Write},
    path::Path,
    process::{Command, Output},
};
use tempfile::NamedTempFile;
use tracing::error;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub(crate) struct CommandArg {
    option: Option<String>,
    value: String,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub(crate) fn exec_with_temp_input<S1, S2>(
    program: S1,
    args: Vec<CommandArg>,
    temp_file_contents: S2,
) -> io::Result<String>
where
    S1: Into<String> + std::fmt::Debug,
    S2: Into<String>,
{
    trace_entry!("exec_with_input" => "{:?}, {:?}, ...", program, args);
    write_to_temp_file(temp_file_contents.into()).and_then(|f| {
        let mut args_mut = args;
        args_mut.push(CommandArg::from_path(f.path()));
        exec(program, args_mut)
    })
}

pub(crate) fn exec<S>(program: S, args: Vec<CommandArg>) -> io::Result<String>
where
    S: Into<String> + std::fmt::Debug,
{
    trace_entry!("exec" => "{:?}, {:?}", program, args);
    let args = args.into_iter().flat_map(|a| a.into_args()).collect();
    exec_inner(program, args).and_then(|o| {
        if o.status.code().map(|c| c != 0).unwrap_or(true) {
            error!("command execution failed; error: {:?}", o.status);
            let mes = String::from_utf8_lossy(&o.stderr).to_string();
            Err(std::io::Error::new(ErrorKind::Other, mes))
        } else {
            Ok(String::from_utf8_lossy(&o.stdout).to_string())
        }
    })
}

#[inline(always)]
fn exec_inner<S>(executable: S, args: Vec<String>) -> io::Result<Output>
where
    S: Into<String>,
{
    Command::new(executable.into()).args(args).output()
}

#[inline(always)]
fn write_to_temp_file(content: String) -> io::Result<NamedTempFile> {
    let mut file = NamedTempFile::new()?;
    file.write_all(content.as_bytes()).map(|_| file)
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl CommandArg {
    pub(crate) fn new<S>(value: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            option: None,
            value: value.into(),
        }
    }

    pub(crate) fn new_option<S1, S2>(option: S1, value: S2) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Self {
            option: Some(option.into()),
            value: value.into(),
        }
    }

    pub(crate) fn from_path<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        Self::new(path.as_ref().to_string_lossy().to_string())
    }

    pub(crate) fn into_args(self) -> Vec<String> {
        if let Some(option) = self.option {
            vec![option, self.value]
        } else {
            vec![self.value]
        }
    }

    #[allow(dead_code)]
    pub(crate) fn into_single(self) -> Self {
        if let Some(option) = self.option {
            Self::new(format!("{} {}", option, self.value))
        } else {
            self
        }
    }
}
