/*!
Provides the traits used to define *generators*, types that convert one or more modules into
other artifacts.

See the [source] module for an example implementation.

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

///
/// This trait denotes a generator that writes to a file path.
///
/// The type parameter `F` is used to describe any format information required by the generator.
///
pub trait GenerateToFile<F: Default + Debug>: Debug {

    ///
    /// Generate from the given module into the provided file path. This method uses the
    /// default value of the format type `F`.
    ///
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

    ///
    /// Generate from the given module, in the requested format, into the provided file path.
    ///
    fn write_to_file_in_format(
        &mut self,
        module: &Module,
        loader: Option<&mut dyn ModuleLoader>,
        path: &Path,
        format: F,
    ) -> Result<(), Error>;
}

///
/// This trait denotes a generator that writes to an implementation of [Write].
///
/// The type parameter `F` is used to describe any format information required by the generator.
///
pub trait GenerateToWriter<F: Default + Debug>: Debug {
    ///
    /// Generate from the given module into the provided writer. This method uses the
    /// default value of the format type `F`.
    ///
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

    ///
    /// Generate from the given module, in the requested format, into the provided writer.
    ///
    fn write_in_format(
        &mut self,
        module: &Module,
        loader: Option<&mut dyn ModuleLoader>,
        writer: &mut dyn Write,
        format: F,
    ) -> Result<(), Error>;

    ///
    /// Generate from the given module into a string. This method uses the
    /// default value of the format type `F`.
    ///
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

    ///
    /// Generate from the given module, in the requested format, into a string.
    ///
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

    ///
    /// Generate from the given module into the provided file path. This method uses the
    /// default value of the format type `F`.
    ///
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

    ///
    /// Generate from the given module, in the requested format, into the provided file path.
    ///
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

///
/// A type that may be used when no format options are required by a generator implementation.
///
#[derive(Clone, Copy, Debug, Default)]
pub struct NoFormatOptions {}

///
/// A concrete enum that allows for either a file or writer generator to be passed.
///
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
