/*!
Provides the traits used to define *generators*, types that convert one or more modules into
other artifacts.
*/

use crate::{error::Error, load::ModuleLoader, model::modules::Module, model::HasName};
use std::{fmt::Debug, fs::File, io::Cursor, io::Write, path::Path};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

macro_rules! trace_entry {
    ($type_name: literal, $fn_name: literal => $format: literal, $( $value: expr ),+ ) => {
        const FULL_NAME: &str = concat!($type_name, "::", $fn_name);
        let tracing_span = ::tracing::trace_span!(FULL_NAME);
        let _enter_span = tracing_span.enter();
        let arguments = format!($format, $( $value ),+);
        ::tracing::trace!("{FULL_NAME}({arguments})");
    };
}

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub trait GenerateToFile<F: Default + Debug>: Debug {
    fn write_to_file(
        &mut self,
        module: &Module,
        loader: Option<&mut dyn ModuleLoader>,
        path: &Path,
    ) -> Result<(), Error> {
        trace_entry!(
            "GenerateToFile",
            "write_to_file" =>
                "module: {}, loader: {}, path: {:?}",
            module.name(),
            loader.is_some(),
            path
        );
        self.write_to_file_in_format(module, loader, path, F::default())
    }

    fn write_to_file_in_format(
        &mut self,
        module: &Module,
        loader: Option<&mut dyn ModuleLoader>,
        path: &Path,
        format: F,
    ) -> Result<(), Error>;
}

pub trait GenerateToWriter<F: Default + Debug>: Debug {
    fn write(
        &mut self,
        module: &Module,
        loader: Option<&mut dyn ModuleLoader>,
        writer: &mut dyn Write,
    ) -> Result<(), Error> {
        trace_entry!(
            "GenerateToWriter",
            "write" =>
                "module: {}, loader: {}, ...",
            module.name(),
            loader.is_some()
        );
        self.write_in_format(module, loader, writer, F::default())
    }

    fn write_in_format(
        &mut self,
        module: &Module,
        loader: Option<&mut dyn ModuleLoader>,
        writer: &mut dyn Write,
        format: F,
    ) -> Result<(), Error>;

    fn write_to_string(
        &mut self,
        module: &Module,
        loader: Option<&mut dyn ModuleLoader>,
    ) -> Result<String, Error> {
        trace_entry!(
            "GenerateToWriter",
            "write_to_string" =>
                "module: {}, loader: {}",
            module.name(),
            loader.is_some()
        );
        self.write_to_string_in_format(module, loader, F::default())
    }

    fn write_to_string_in_format(
        &mut self,
        module: &Module,
        loader: Option<&mut dyn ModuleLoader>,
        format: F,
    ) -> Result<String, Error> {
        trace_entry!(
            "GenerateToWriter",
            "write_to_string_in_format" =>
                "module: {}, loader: {}, format: {:?}",
            module.name(),
            loader.is_some(),
            format
        );
        let mut buffer = Cursor::new(Vec::new());
        self.write(module, loader, &mut buffer)?;
        Ok(String::from_utf8(buffer.into_inner())?)
    }

    fn write_to_file(
        &mut self,
        module: &Module,
        loader: Option<&mut dyn ModuleLoader>,
        path: &Path,
    ) -> Result<(), Error> {
        trace_entry!(
            "GenerateToWriter",
            "write_to_file" =>
                "module: {}, loader: {}, path: {:?}",
            module.name(),
            loader.is_some(),
            path
        );
        self.write_to_file_in_format(module, loader, path, F::default())
    }

    fn write_to_file_in_format(
        &mut self,
        module: &Module,
        loader: Option<&mut dyn ModuleLoader>,
        path: &Path,
        format: F,
    ) -> Result<(), Error> {
        trace_entry!(
            "GenerateToWriter",
            "write_to_file_in_format" =>
                "module: {}, loader: {}, path: {:?}, format: {:?}",
            module.name(),
            loader.is_some(),
            path,
            format
        );
        let mut file = File::create(path)?;
        self.write_in_format(module, loader, &mut file, format)?;
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

pub mod source;
