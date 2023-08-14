/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

// ------------------------------------------------------------------------------------------------
// Public Macros ❱ Delegate
// ------------------------------------------------------------------------------------------------

macro_rules! delegate {
    ($vis: vis $fnname: ident, $fntype: ty, $fieldname: ident $(, $paramname: ident => $paramtype: ty)* ) => {
        #[inline(always)]
        $vis fn $fnname(&self $(, $paramname: $paramtype)*) -> $fntype {
            self.$fieldname.$fnname($($paramname: $paramtype),*)
        }
    };
    ($vis: vis $fnname: ident, $fntype: ty, fn $fieldname: ident $(, $paramname: ident => $paramtype: ty)* ) => {
        #[inline(always)]
        $vis fn $fnname(&self $(, $paramname: $paramtype)*) -> $fntype {
            self.$fieldname().$fnname($($paramname: $paramtype),*)
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Public Macros ❱ Enums
// ------------------------------------------------------------------------------------------------

macro_rules! impl_from_for_variant {
    ($tyname: ty, $varname: ident, $vartype: ty) => {
        impl From<$vartype> for $tyname {
            fn from(v: $vartype) -> Self {
                Self::$varname(v)
            }
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Private Macros ❱ impl Display
// ------------------------------------------------------------------------------------------------

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
