/*!
Provides the traits used to define *generators*, types that convert one or more modules into
other artifacts.
*/

use crate::{error::Error, model::Module};
use std::{fmt::Debug, fs::File, io::Cursor, io::Write, path::Path};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub trait GenerateToFile<F: Default>: Debug {
    fn write_to_file(&mut self, module: &Module, path: &Path) -> Result<(), Error> {
        self.write_to_file_in_format(module, path, F::default())
    }

    fn write_to_file_in_format(
        &mut self,
        module: &Module,
        path: &Path,
        format: F,
    ) -> Result<(), Error>;
}

pub trait GenerateToWriter<F: Default>: Debug {
    fn write(&mut self, module: &Module, writer: &mut dyn Write) -> Result<(), Error> {
        self.write_in_format(module, writer, F::default())
    }

    fn write_in_format(
        &mut self,
        module: &Module,
        writer: &mut dyn Write,
        format: F,
    ) -> Result<(), Error>;

    fn write_to_string(&mut self, module: &Module) -> Result<String, Error> {
        let mut buffer = Cursor::new(Vec::new());
        self.write(module, &mut buffer)?;
        Ok(String::from_utf8(buffer.into_inner())?)
    }

    fn write_to_file(&mut self, module: &Module, path: &Path) -> Result<(), Error> {
        self.write_to_file_in_format(module, path, F::default())
    }

    fn write_to_file_in_format(
        &mut self,
        module: &Module,
        path: &Path,
        format: F,
    ) -> Result<(), Error> {
        let mut file = File::create(path)?;
        self.write_in_format(module, &mut file, format)?;
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct NoFormatOptions {}

#[derive(Debug)]
pub enum Generator<F: Default> {
    File(Box<dyn GenerateToFile<F>>),
    Write(Box<dyn GenerateToWriter<F>>),
}

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

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
