/*!
One-line description.

TBD

# Example

TBD

 */

use crate::store::ModuleStore;
use sdml_errors::Error;
use std::{
    fs::OpenOptions,
    io::{Cursor, Write},
    path::Path,
};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The trait implemented by types which write instances of `T`.
///
pub trait RepresentationWriter {
    type Object;
    type Cache: ModuleStore;
    type Options: Default;

    ///
    /// Write an instance of `T` to the provided implementation of `Write`.
    ///
    fn write<W>(
        &self,
        w: &mut W,
        object: &Self::Object,
        store: Option<&Self::Cache>,
    ) -> Result<(), Error>
    where
        W: Write,
    {
        self.write_with(w, object, store, &Default::default())
    }

    fn write_with<W>(
        &self,
        w: &mut W,
        object: &Self::Object,
        store: Option<&Self::Cache>,
        options: &Self::Options,
    ) -> Result<(), Error>
    where
        W: Write;

    ///
    /// Write an instance of `T` to, and return, a string.
    ///
    fn write_to_string(
        &self,
        object: &Self::Object,
        store: Option<&Self::Cache>,
    ) -> Result<String, Error> {
        self.write_to_string_with(object, store, &Default::default())
    }

    fn write_to_string_with(
        &self,
        object: &Self::Object,
        store: Option<&Self::Cache>,
        options: &Self::Options,
    ) -> Result<String, Error> {
        let mut buffer = Cursor::new(Vec::new());
        self.write_with(&mut buffer, object, store, options)?;
        Ok(String::from_utf8(buffer.into_inner()).unwrap())
    }

    ///
    /// Write an instance of `T` into the file identified by `path`.
    ///
    /// This method will return an IO error if the path is invalid, or the file is not writeable.
    /// If the file exists it will be replaced.
    ///
    fn write_to_file<P>(
        &self,
        object: &Self::Object,
        store: Option<&Self::Cache>,
        path: P,
    ) -> Result<(), Error>
    where
        P: AsRef<Path>,
    {
        self.write_to_file_with(object, store, path, &Default::default())
    }

    fn write_to_file_with<P>(
        &self,
        object: &Self::Object,
        store: Option<&Self::Cache>,
        path: P,
        options: &Self::Options,
    ) -> Result<(), Error>
    where
        P: AsRef<Path>,
    {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path.as_ref())?;
        self.write_with(&mut file, object, store, options)
    }
}
