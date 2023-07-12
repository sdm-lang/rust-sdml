// ------------------------------------------------------------------------------------------------
// Macros ❱ Tree Wrapper
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Macros ❱ Node Wrapper
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Macros ❱ Node Wrapper ❱ Children
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Macros ❱ Node Wrapper ❱ Fields
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Macros ❱ Choice Wrapper
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Macros ❱ Writers ❱ To String
// ------------------------------------------------------------------------------------------------

macro_rules! write_to_string {
    ($outer:ident, $inner:ident) => {
        pub fn $outer(
            module: &::sdml_core::model::Module,
        ) -> Result<String, ::sdml_core::error::Error> {
            let mut buffer = ::std::io::Cursor::new(Vec::new());
            $inner(module, &mut buffer)?;
            Ok(String::from_utf8(buffer.into_inner())?)
        }
    };
    //($outer:ident, $inner:ident, $formtype:ty) => {
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
            module: &::sdml_core::model::Module,
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
            module: &::sdml_core::model::Module,
        ) -> Result<(), ::sdml_core::error::Error> {
            $inner(module, &mut ::std::io::stdout())?;
            Ok(())
        }
    };
    //($outer:ident, $inner:ident, $formtype:ty) => {
    //    pub fn $outer(
    //        module: &::sdml_core::model::Module,
    //        format: $formtype,
    //    ) -> Result<(), ::sdml_core::error::Error> {
    //        $inner(module, &mut ::std::io::stdout(), format)?;
    //        Ok(())
    //    }
    //};
}
