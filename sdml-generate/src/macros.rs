// ------------------------------------------------------------------------------------------------
// Macros ❱ trace entry
// ------------------------------------------------------------------------------------------------

macro_rules! trace_entry {
    //($fn_name: literal) => {
    //    let tracing_span = ::tracing::trace_span!($fn_name);
    //    let _enter_span = tracing_span.enter();
    //    ::tracing::trace!("{}()", $fn_name);
    //};
    //($type_name: literal, $fn_name: literal) => {
    //    const FULL_NAME: &str = concat!($type_name, "::", $fn_name);
    //    let tracing_span = ::tracing::trace_span!(FULL_NAME);
    //    let _enter_span = tracing_span.enter();
    //    ::tracing::trace!("{FULL_NAME}()");
    //};
    ($fn_name: literal => $format: literal, $( $value: expr ),+ ) => {
        let tracing_span = ::tracing::trace_span!($fn_name);
        let _enter_span = tracing_span.enter();
        let arguments = format!($format, $( $value ),+);
        ::tracing::trace!("{}({arguments})", $fn_name);
    };
    ($type_name: literal, $fn_name: literal => $format: literal, $( $value: expr ),+ ) => {
        const FULL_NAME: &str = concat!($type_name, "::", $fn_name);
        let tracing_span = ::tracing::trace_span!(FULL_NAME);
        let _enter_span = tracing_span.enter();
        let arguments = format!($format, $( $value ),+);
        ::tracing::trace!("{FULL_NAME}({arguments})");
    };
}

// ------------------------------------------------------------------------------------------------
// Macros ❱ Writers ❱ To String
// ------------------------------------------------------------------------------------------------

macro_rules! write_to_string {
    ($outer:ident, $inner:ident) => {
        pub fn $outer(
            module: &::sdml_core::model::modules::Module,
        ) -> Result<String, ::sdml_core::error::Error> {
            let mut buffer = ::std::io::Cursor::new(Vec::new());
            $inner(module, &mut buffer)?;
            Ok(String::from_utf8(buffer.into_inner())?)
        }
    }; //($outer:ident, $inner:ident, $formtype:ty) => {
       //    pub fn $outer(
       //        module: &::sdml_core::model::Module,
       //        format: $formtype,
       //    ) -> Result<String, ::sdml_core::error::Error> {
       //        let mut buffer = ::std::io::Cursor::new(Vec::new());
       //        $inner(module, &mut buffer, format)?;
       //        Ok(String::from_utf8(buffer.into_inner())?)
       //    }
       //};
}

// ------------------------------------------------------------------------------------------------
// Macros ❱ Writers ❱ To File
// ------------------------------------------------------------------------------------------------

macro_rules! write_to_file {
    ($outer:ident, $inner:ident) => {
        pub fn $outer<P>(
            module: &::sdml_core::model::modules::Module,
            path: P,
        ) -> Result<(), ::sdml_core::error::Error>
        where
            P: AsRef<::std::path::Path>,
        {
            let mut file = ::std::fs::File::create(path.as_ref())?;
            $inner(module, &mut file)?;
            Ok(())
        }
    }; //($outer:ident, $inner:ident, $formtype:ty) => {
       //    pub fn $outer<P>(
       //        module: &::sdml_core::model::Module,
       //        path: P,
       //        format: $formtype,
       //    ) -> Result<(), ::sdml_core::error::Error>
       //    where
       //        P: AsRef<::std::path::Path>,
       //    {
       //        let mut file = ::std::fs::File::create(path.as_ref())?;
       //        $inner(module, &mut file, format)?;
       //        Ok(())
       //    }
       //};
}

// ------------------------------------------------------------------------------------------------
// Macros ❱ Writers ❱ To File
// ------------------------------------------------------------------------------------------------

macro_rules! print_to_stdout {
    ($outer:ident, $inner:ident) => {
        pub fn $outer(
            module: &::sdml_core::model::modules::Module,
        ) -> Result<(), ::sdml_core::error::Error> {
            $inner(module, &mut ::std::io::stdout())?;
            Ok(())
        }
    }; //($outer:ident, $inner:ident, $formtype:ty) => {
       //    pub fn $outer(
       //        module: &::sdml_core::model::Module,
       //        format: $formtype,
       //    ) -> Result<(), ::sdml_core::error::Error> {
       //        $inner(module, &mut ::std::io::stdout(), format)?;
       //        Ok(())
       //    }
       //};
}
