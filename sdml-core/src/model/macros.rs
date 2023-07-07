/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

// ------------------------------------------------------------------------------------------------
// Private Macros ❱ Basic Get/Set
// ------------------------------------------------------------------------------------------------

macro_rules! get {
    ($vis: vis $name: ident => $type: ty) => {
        get!($vis $name ($name) => $type);
    };
    ($vis: vis $name: ident ($fn_name: ident) => $type: ty) => {
        $vis fn $fn_name(&self) -> &$type {
            &self.$name
        }
    };
    //($vis: vis $name: ident => copy $type: ty) => {
    //    get!($vis $name ($name) => copy $type);
    //};
    ($vis: vis $name: ident ($fn_name: ident) => copy $type: ty) => {
        $vis fn $fn_name(&self) -> $type {
            self.$name
        }
    };
    //($vis: vis $name: ident => option $type: ty) => {
    //    get!($vis $name ($name) => option $type);
    //};
    ($vis: vis $name: ident ($fn_name: ident) => option $type: ty) => {
        paste::paste! {
            $vis fn [< has_ $fn_name >](&self) -> bool {
                self.$name.is_some()
            }

            $vis fn $fn_name(&self) -> Option<&$type> {
                self.$name.as_ref()
            }
        }
    };
}

macro_rules! mutate {
    //($vis: vis $name: ident => $type: ty) => {
    //    mutate!($vis $name ($name) => $type);
    //};
    ($vis: vis $name: ident ($fn_name: ident) => $type: ty) => {
        paste::paste! {
            $vis fn [< set_ $fn_name >](&mut self, value: $type) {
                self.$name = value;
            }
        }
    };
    ($vis: vis $name: ident => boxed $type: ty) => {
        paste::paste! {
            $vis fn [< set_ $name >](&mut self, value: $type) {
                self.$name = Box::new(value);
            }
        }
    };
    //($vis: vis $name: ident => option $type: ty) => {
    //    mutate!($vis $name ($name) => option $type);
    //};
    ($vis: vis $name: ident ($fn_name: ident) => option $type: ty) => {
        paste::paste! {
            $vis fn [< set_ $fn_name >](&mut self, value: $type) {
                self.$name = Some(value);
            }

            $vis fn [< unset_ $fn_name >](&mut self) {
                self.$name = None;
            }
        }
    };
}

macro_rules! get_and_mutate {
    ($vis: vis $name: ident => $type: ty) => {
        get_and_mutate!($vis $name ($name) => $type);
    };
    ($vis: vis $name: ident ($fn_name: ident) => $type: ty) => {
        get!($vis $name ($fn_name) => $type);
        mutate!($vis $name ($fn_name) => $type);
    };
    ($vis: vis $name: ident => copy $type: ty) => {
        get_and_mutate!($vis $name ($name) => copy $type);
    };
    ($vis: vis $name: ident($fn_name: ident) => copy $type: ty) => {
        get!($vis $name ($fn_name) => copy $type);
        mutate!($vis $name ($fn_name) => $type);
    };
    ($vis: vis $name: ident => option $type: ty) => {
        get_and_mutate!($vis $name ($name) => option $type);
    };
    ($vis: vis $name: ident ($fn_name: ident) => option $type: ty) => {
        get!($vis $name ($fn_name) => option $type);
        mutate!($vis $name ($fn_name) => option $type);
    };
    ($vis: vis $name: ident => boxed $type: ty) => {
        get!($vis $name => $type);
        mutate!($vis $name => boxed $type);
    };
}

macro_rules! with {
    //($vis: vis $name: ident => $type: ty) => {
    //    with!($vis $name ($name) => $type);
    //};
    //($vis: vis $name: ident ($fn_name: ident) => $type: ty) => {
    //    paste::paste! {
    //        $vis fn [< with_ $fn_name >](self, value: $type) -> Self {
    //            let mut self_mut = self;
    //            self_mut.$name = value;
    //            self_mut
    //        }
    //    }
    //};
    ($vis: vis $name: ident => option $type: ty) => {
        with!($vis $name ($name) => option $type);
    };
    ($vis: vis $name: ident ($fn_name: ident) => option $type: ty) => {
        paste::paste! {
            $vis fn [< with_ $fn_name >](self, value: $type) -> Self {
                let mut self_mut = self;
                self_mut.$name = Some(value);
                self_mut
            }
        }
    };
}

macro_rules! get_collection_of {
    ($vis: vis $name: ident => $itype: ty) => {
        paste::paste! {
            pub fn [< has_ $name >](&self) -> bool {
                !self.$name.is_empty()
            }

            pub fn $name(&self) -> impl Iterator<Item = &$itype> {
                self.$name.iter()
            }
        }
    };
}

macro_rules! mutate_collection_of {
    //($vis: vis $name: ident => $ctype: ty, $itype: ty) => {
    //    add_to_collection_of($vis $name, push = $ctype, $itype);
    //};
    ($vis: vis $name: ident, $add_fn: ident => $ctype: ty, $itype: ty) => {
        paste::paste! {
            pub fn [< set_ $name >](&mut self, values: $ctype<$itype>) {
                self.$name = values;
            }

            pub fn [< add_to_ $name >](&mut self, value: $itype) {
                self.$name.$add_fn(value);
            }

            pub fn [< extend_ $name >]<I>(&mut self, extension: I)
            where
                I: IntoIterator<Item = $itype>,
            {
                self.$name.extend(extension);
            }
        }
    };
}

macro_rules! get_and_mutate_collection_of {
    ($vis: vis $name: ident => $ctype: ty, $itype: ty) => {
        get_and_mutate_collection_of!($vis $name, push => $ctype, $itype);
    };
    ($vis: vis $name: ident, $add_fn: ident => $ctype: ty, $itype: ty) => {
        paste::paste! {
            get_collection_of!($vis $name => $itype);
            mutate_collection_of!($vis $name, $add_fn => $ctype, $itype);
        }
    };
}

macro_rules! delegate {
    ($fnname: ident, $fntype: ty, $fieldname: ident $(, $paramname: ident => $paramtype: ty)* ) => {
        pub fn $fnname(&self $(, $paramname: $paramtype)*) -> $fntype {
            self.$fieldname.$fnname($($paramname: $paramtype),*)
        }
    };
}

macro_rules! is_variant {
    //($vis: vis $fn_name: ident => empty $varname: ident) => {
    //    paste::paste! {
    //        $vis fn [< is_ $fn_name >](&self) -> bool {
    //            matches!(self, Self::$varname)
    //        }
    //    }
    //};
    ($vis: vis $fn_name: ident => $varname: ident) => {
        paste::paste! {
            $vis fn [< is_ $fn_name >](&self) -> bool {
                matches!(self, Self::$varname(_))
            }
        }
    };
}

macro_rules! as_variant {
    ($vis: vis $fn_name: ident => $varname: ident, $vartype: ty) => {
        paste::paste! {
            $vis fn [< as_ $fn_name >](&self) -> Option<&$vartype> {
                if let Self::$varname(v) = self {
                    Some(v)
                } else {
                    None
                }
            }
        }
    };
}

macro_rules! is_as_variant {
    ($vis: vis $fn_name: ident => $varname: ident, $vartype: ty) => {
        is_variant!($vis $fn_name => $varname);
        as_variant!($vis $fn_name => $varname, $vartype);
    };
}

macro_rules! delegate_is_variant {
    //($vis: vis $fn_name: ident, $inner: expr => empty $enumtype: ty, $varname: ident) => {
    //    paste::paste! {
    //        $vis fn [< is_ $fn_name >](&self) -> bool {
    //            matches!(self.$inner, $enumtype::$varname)
    //        }
    //    }
    //};
    ($vis: vis $fn_name: ident, $inner: expr => $enumtype: ty, $varname: ident) => {
        paste::paste! {
            $vis fn [< is_ $fn_name >](&self) -> bool {
                self.$inner.[< is_ $fn_name >]()
            }
        }
    };
}

macro_rules! delegate_as_variant {
    ($vis: vis $fn_name: ident, $inner: expr => $enumtype: ty, $varname: ident, $vartype: ty) => {
        paste::paste! {
            $vis fn [< as_ $fn_name >](&self) -> Option<&$vartype> {
                self.$inner.[< as_ $fn_name >]()
            }
        }
    };
}

macro_rules! delegate_is_as_variant {
    ($vis: vis $fn_name: ident, $inner: expr => $enumtype: ty, $varname: ident, $vartype: ty) => {
        delegate_is_variant!($vis $fn_name, $inner => $enumtype, $varname);
        delegate_as_variant!($vis $fn_name, $inner => $enumtype, $varname, $vartype);
    };
}

macro_rules! impl_from_for_variant {
    //($tyname: ty, $varname: ident) => {
    //    impl From<$vartype> for $varname {
    //        fn from(v: $vartype) -> Self {
    //            Self::$varname(v)
    //        }
    //    }
    //};
    ($tyname: ty, $varname: ident, $vartype: ty) => {
        impl From<$vartype> for $tyname {
            fn from(v: $vartype) -> Self {
                Self::$varname(v)
            }
        }
    };
    ($tyname: ty, $varname: ident, into $vartype: ty) => {
        impl<T> From<T> for $tyname
        where
            T: Into<$vartype>,
        {
            fn from(v: T) -> Self {
                Self::$varname(v.into())
            }
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Private Macros ❱ impl Display
// ------------------------------------------------------------------------------------------------

macro_rules! simple_display_impl {
    ($tyname: ty, $field: ident) => {
        impl std::fmt::Display for $tyname {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.$field)
            }
        }
    };
}

macro_rules! enum_display_impl {
    ($tyname: ty => $($varname: ident),+) => {
        impl std::fmt::Display for $tyname {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", match self {
                    $(
                        Self::$varname(v) => v.to_string(),
                    )+
                })
            }
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Private Macros ❱ impl Into/AsRef String
// ------------------------------------------------------------------------------------------------

macro_rules! into_string_impl {
    ($tyname: ty, $field: ident) => {
        impl From<$tyname> for String {
            fn from(v: $tyname) -> Self {
                v.value
            }
        }
    };
}

macro_rules! as_str_impl {
    ($tyname: ty, $field: ident) => {
        impl AsRef<str> for $tyname {
            fn as_ref(&self) -> &str {
                self.value.as_str()
            }
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Private Macros ❱ Complete Check
// ------------------------------------------------------------------------------------------------

//macro_rules! is_complete_fn {
//    () => {
//        pub fn is_complete(&self) -> bool {
//            self.body.map(|b|b.is_complete()).unwrap_or_default()
//        }
//    };
//    ($delegate: ident) => {
//        pub fn is_complete(&self) -> bool {
//            self.$delegate().is_complete()
//        }
//    };
//}

// ------------------------------------------------------------------------------------------------
// Private Macros ❱ has Annotations
// ------------------------------------------------------------------------------------------------

macro_rules! has_owned_annotations {
    () => {
        pub fn add_annotation<A>(&mut self, add: A)
        where
            A: Into<$crate::model::Annotation>,
        {
            self.add_to_annotations(add.into());
        }

        get_and_mutate_collection_of!(pub annotations => Vec, $crate::model::Annotation);

        pub fn annotation_properties(&self) -> impl Iterator<Item = &$crate::model::AnnotationProperty> {
            self.annotations().filter_map(|a| a.as_annotation_property())
        }

        pub fn annotation_constraints(&self) -> impl Iterator<Item = &$crate::model::Constraint> {
             self.annotations().filter_map(|a| a.as_constraint())
        }
    };
}

macro_rules! referenced_own_annotations {
    () => {
        pub fn referenced_annotations(&self) -> HashSet<&$crate::model::IdentifierReference> {
            self.annotation_properties().map(|a| a.name()).collect()
        }
    };
}
macro_rules! referenced_optional_body_annotations {
    () => {
        pub fn referenced_annotations(&self) -> HashSet<&$crate::model::IdentifierReference> {
            self.body
                .as_ref()
                .map(|b| b.referenced_annotations())
                .unwrap_or_default()
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Private Macros ❱ has Members
// ------------------------------------------------------------------------------------------------

macro_rules! referenced_optional_body_types {
    () => {
        pub fn referenced_types(&self) -> HashSet<&$crate::model::IdentifierReference> {
            self.body
                .as_ref()
                .map(|b| b.referenced_types())
                .unwrap_or_default()
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Private Macros ❱ isa Type Definition
// ------------------------------------------------------------------------------------------------

macro_rules! type_definition_impl {
    ($bodytype: ty $(, $flname: ident, $fltype: ty )*) => {
        pub fn new(name: $crate::model::Identifier $(, $flname: $fltype )*) -> Self {
            Self {
                span: None,
                comments: Default::default(),
                name,
                $(
                    $flname,
                ),*
                    body: None,
            }
        }

        with!(pub span (ts_span) => option $crate::model::Span);
        get_and_mutate!(pub span (ts_span) => option $crate::model::Span);

        get_and_mutate_collection_of!(pub comments => Vec, $crate::model::Comment);

        get_and_mutate!(pub body => option $bodytype);

        get_and_mutate!(pub name => $crate::model::Identifier);

        $(
            get!(pub $flname => $fltype);
        )*
    };
}

// ------------------------------------------------------------------------------------------------
// Private Macros ❱ is_complete
// ------------------------------------------------------------------------------------------------

macro_rules! is_complete_fn {
    ($value: literal) => {
        pub fn is_complete(&self) -> bool {
            $value
        }
    };
    ($field: ident) => {
        pub fn is_complete(&self) -> bool {
            self.$field.is_complete()
        }
    };
}

macro_rules! is_body_complete_fn {
    () => {
        pub fn is_complete(&self) -> bool {
            self.body
                .as_ref()
                .map(|b| b.is_complete())
                .unwrap_or_default()
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------
