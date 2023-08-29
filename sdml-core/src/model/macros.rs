/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

// ------------------------------------------------------------------------------------------------
// Public Macros ❱ trait HasSourceSpan
// ------------------------------------------------------------------------------------------------

macro_rules! impl_has_source_span_for {
    ($type: ty) => {
        impl_has_source_span_for!($type, span);
    };
    ($type: ty, $inner: ident) => {
        impl $crate::model::HasSourceSpan for $type {
            fn with_source_span(self, span: Span) -> Self {
                let mut self_mut = self;
                self_mut.span = Some(span);
                self_mut
            }

            fn source_span(&self) -> Option<&$crate::model::Span> {
                self.$inner.as_ref()
            }

            fn set_source_span(&mut self, span: $crate::model::Span) {
                self.$inner = Some(span);
            }

            fn unset_source_span(&mut self) {
                self.$inner = None;
            }
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Public Macros ❱ trait HasName
// ------------------------------------------------------------------------------------------------

macro_rules! impl_has_name_for {
    ($type: ty) => {
        impl_has_name_for!($type, name);
    };
    ($type: ty, $inner: ident) => {
        impl $crate::model::HasName for $type {
            fn name(&self) -> &$crate::model::identifiers::Identifier {
                &self.$inner
            }

            fn set_name(&mut self, name: $crate::model::identifiers::Identifier) {
                self.$inner = name;
            }
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Public Macros ❱ trait HasNameReference
// ------------------------------------------------------------------------------------------------

macro_rules! impl_has_name_reference_for {
    ($type: ty) => {
        impl_has_name_reference_for!($type, name_reference);
    };
    ($type: ty, $inner: ident) => {
        impl $crate::model::HasNameReference for $type {
            fn name_reference(&self) -> &$crate::model::identifiers::IdentifierReference {
                &self.$inner
            }

            fn set_name_reference(
                &mut self,
                name: $crate::model::identifiers::IdentifierReference,
            ) {
                self.$inner = name;
            }
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Public Macros ❱ trait HasType
// ------------------------------------------------------------------------------------------------

macro_rules! impl_has_type_for {
    ($type: ty) => {
        impl_has_type_for!($type, target_type);
    };
    ($type: ty, $inner: ident) => {
        impl $crate::model::members::HasType for $type {
            fn target_type(&self) -> &$crate::model::members::TypeReference {
                &self.$inner
            }

            fn set_target_type(&mut self, target_type: $crate::model::members::TypeReference) {
                self.$inner = target_type;
            }
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Public Macros ❱ trait HasCardinality
// ------------------------------------------------------------------------------------------------

macro_rules! impl_has_cardinality_for {
    ($type: ty) => {
        impl_has_cardinality_for!($type, target_cardinality);
    };
    ($type: ty, $inner: ident) => {
        impl $crate::model::members::HasCardinality for $type {
            fn target_cardinality(&self) -> &Cardinality {
                &self.$inner
            }

            fn set_target_cardinality(&mut self, target_cardinality: Cardinality) {
                self.$inner = target_cardinality;
            }
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Public Macros ❱ trait HasBody
// ------------------------------------------------------------------------------------------------

#[allow(unused_macro_rules)]
macro_rules! impl_has_body_for {
    ($type: ty) => {
        impl_has_body_for!($type, $crate::model::annotations::AnnotationOnlyBody, body);
    };
    ($type: ty, $bodytype: ty) => {
        impl_has_body_for!($type, $bodytype, body);
    };
    ($type: ty, $bodytype: ty, $inner: ident) => {
        impl $crate::model::HasBody<$bodytype> for $type {
            fn body(&self) -> &$bodytype {
                &self.$inner
            }

            fn set_body(&mut self, body: $bodytype) {
                self.$inner = body;
            }
        }
    };
    ($type: ty, boxed $bodytype: ty) => {
        impl_has_body_for!($type, boxed $bodytype, body);
    };
    ($type: ty, boxed $bodytype: ty, $inner: ident) => {
        impl $crate::model::HasBody<$bodytype> for $type {
            fn body(&self) -> &$bodytype {
                &self.$inner
            }

            fn set_body(&mut self, body: $bodytype) {
                self.$inner = Box::new(body);
            }
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Public Macros ❱ trait HasOptionalBody
// ------------------------------------------------------------------------------------------------

macro_rules! impl_has_optional_body_for {
    ($type: ty) => {
        impl_has_optional_body_for!($type, $crate::model::annotations::AnnotationOnlyBody, body);
    };
    ($type: ty, $bodytype: ty) => {
        impl_has_optional_body_for!($type, $bodytype, body);
    };
    ($type: ty, $bodytype: ty, $inner: ident) => {
        impl $crate::model::HasOptionalBody<$bodytype> for $type {
            fn body(&self) -> Option<&$bodytype> {
                self.$inner.as_ref()
            }

            fn set_body(&mut self, body: $bodytype) {
                self.$inner = Some(body);
            }

            fn unset_body(&mut self) {
                self.$inner = None;
            }
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Public Macros ❱ trait Member
// ------------------------------------------------------------------------------------------------

#[allow(unused_macros)]
macro_rules! impl_member_for {
    ($type: ty, $deftype: ty) => {
        impl_member_for!($type, $deftype, kind);
    };
    ($type: ty, $deftype: ty, $inner: ident) => {
        impl<'a> $crate::model::members::Member<'a, $deftype> for $type {
            fn kind(&'a self) -> &'a MemberKind<$deftype> {
                &self.$inner
            }

            fn set_kind(&mut self, kind: MemberKind<$deftype>) {
                self.$inner = kind;
            }
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Public Macros ❱ trait References
// ------------------------------------------------------------------------------------------------

macro_rules! impl_references_for {
    ($type: ty => variants $($varname: ident),+) => {
        impl $crate::model::References for $type {
            fn referenced_annotations<'a>(&'a self, names: &mut ::std::collections::HashSet<&'a $crate::model::identifiers::IdentifierReference>) {
                match self {
                    $(
                        Self::$varname(v) => v.referenced_annotations(names),
                    )+
                }
            }

            fn referenced_types<'a>(&'a self, names: &mut ::std::collections::HashSet<&'a $crate::model::identifiers::IdentifierReference>) {
                match self {
                    $(
                        Self::$varname(v) => v.referenced_types(names),
                    )+
                }
            }
        }
    };
   ($type: ty => delegate $inner: ident) => {
        impl $crate::model::References for $type {
            fn referenced_annotations<'a>(&'a self, names: &mut ::std::collections::HashSet<&'a $crate::model::identifiers::IdentifierReference>) {
                self.$inner.referenced_annotations(names);
            }

            fn referenced_types<'a>(&'a self, names: &mut ::std::collections::HashSet<&'a $crate::model::identifiers::IdentifierReference>) {
                self.$inner.referenced_types(names);
            }
        }
    };
   ($type: ty => delegate optional $inner: ident) => {
        impl $crate::model::References for $type {
            fn referenced_annotations<'a>(&'a self, names: &mut ::std::collections::HashSet<&'a $crate::model::identifiers::IdentifierReference>) {
                if let Some(inner) = &self.$inner {
                    inner.referenced_annotations(names);
                }
            }

            fn referenced_types<'a>(&'a self, names: &mut ::std::collections::HashSet<&'a $crate::model::identifiers::IdentifierReference>) {
                if let Some(inner) = &self.$inner {
                    inner.referenced_types(names);
                }
            }
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Public Macros ❱ trait Validate
// ------------------------------------------------------------------------------------------------

macro_rules! impl_validate_for {
    ($type: ty => variants $($varname: ident),+) => {
        impl $crate::model::check::Validate for $type {
            fn is_complete(&self, top: &$crate::model::modules::Module) -> Result<bool, $crate::error::Error> {
                match self {
                    $(
                        Self::$varname(v) => v.is_complete(top),
                    )+
                }
            }

            fn is_valid(&self, check_constraints: bool, top: &$crate::model::modules::Module) -> Result<bool, $crate::error::Error> {
                match self {
                    $(
                        Self::$varname(v) => v.is_valid(check_constraints, top),
                    )+
                }
            }
        }
    };
    ($type: ty => delegate $inner: ident) => {
        impl $crate::model::check::Validate for $type {
            fn is_complete(&self, top: &$crate::model::modules::Module) -> Result<bool, $crate::error::Error> {
                self.$inner.is_complete(top)
            }

            fn is_valid(&self, check_constraints: bool, top: &$crate::model::modules::Module) -> Result<bool, $crate::error::Error> {
                self.$inner.is_valid(check_constraints, top)
            }
        }
    };
    ($type: ty => delegate optional $inner: ident, $def_complete: expr, $def_valid: expr) => {
        impl $crate::model::check::Validate for $type {
            fn is_complete(&self, top: &$crate::model::modules::Module) -> Result<bool, $crate::error::Error> {
                match &self.$inner {
                    Some(inner) => inner.is_complete(top),
                    None => Ok($def_complete),
                }
            }

            fn is_valid(&self, check_constraints: bool, top: &$crate::model::modules::Module) -> Result<bool, $crate::error::Error> {
                match &self.$inner {
                    Some(inner) => inner.is_valid(check_constraints, top),
                    None => Ok($def_valid),
                }
            }
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Public Macros ❱ trait HasAnnotations
// ------------------------------------------------------------------------------------------------

macro_rules! impl_has_annotations_for {
    ($type: ty) => {
        impl_has_annotations_for!($type, annotations);
    };
    ($type: ty, $inner: ident) => {
        impl $crate::model::annotations::HasAnnotations for $type {
            fn has_annotations(&self) -> bool {
                !self.$inner.is_empty()
            }

            fn annotations_len(&self) -> usize {
                self.$inner.len()
            }

            fn annotations(
                &self,
            ) -> Box<dyn Iterator<Item = &$crate::model::annotations::Annotation> + '_> {
                Box::new(self.$inner.iter())
            }

            fn annotations_mut(
                &mut self,
            ) -> Box<dyn Iterator<Item = &mut $crate::model::annotations::Annotation> + '_> {
                Box::new(self.$inner.iter_mut())
            }

            fn add_to_annotations<I>(&mut self, value: I)
            where
                I: Into<$crate::model::annotations::Annotation>,
            {
                self.$inner.push(value.into())
            }

            fn extend_annotations<I>(&mut self, extension: I)
            where
                I: IntoIterator<Item = $crate::model::annotations::Annotation>,
            {
                self.$inner.extend(extension.into_iter())
            }
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Public Macros ❱ trait HasMembers
// ------------------------------------------------------------------------------------------------

macro_rules! impl_has_members_for {
    ($type: ty, $membertype: ty) => {
        impl_has_members_for!($type, $membertype, members);
    };
    ($type: ty, $membertype: ty, $inner: ident) => {
        impl $crate::model::definitions::HasMembers<$membertype> for $type {
            fn has_members(&self) -> bool {
                !self.$inner.is_empty()
            }

            fn members_len(&self) -> usize {
                self.$inner.len()
            }

            fn members(&self) -> Box<dyn Iterator<Item = &$membertype> + '_> {
                Box::new(self.$inner.iter())
            }

            fn members_mut(&mut self) -> Box<dyn Iterator<Item = &mut $membertype> + '_> {
                Box::new(self.$inner.iter_mut())
            }

            fn add_to_members(&mut self, value: $membertype) {
                self.$inner.push(value.into())
            }

            fn extend_members<I>(&mut self, extension: I)
            where
                I: IntoIterator<Item = $membertype>,
            {
                self.$inner.extend(extension.into_iter())
            }
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Public Macros ❱ trait HasVariants
// ------------------------------------------------------------------------------------------------

macro_rules! impl_has_variants_for {
    ($type: ty, $varianttype: ty) => {
        impl_has_variants_for!($type, $varianttype, variants);
    };
    ($type: ty, $varianttype: ty, $inner: ident) => {
        impl $crate::model::definitions::HasVariants<$varianttype> for $type {
            fn has_variants(&self) -> bool {
                !self.$inner.is_empty()
            }

            fn variants_len(&self) -> usize {
                self.$inner.len()
            }

            fn variants(&self) -> Box<dyn Iterator<Item = &$varianttype> + '_> {
                Box::new(self.$inner.iter())
            }

            fn variants_mut(&mut self) -> Box<dyn Iterator<Item = &mut $varianttype> + '_> {
                Box::new(self.$inner.iter_mut())
            }

            fn add_to_variants(&mut self, value: $varianttype) {
                self.$inner.push(value.into())
            }

            fn extend_variants<I>(&mut self, extension: I)
            where
                I: IntoIterator<Item = $varianttype>,
            {
                self.$inner.extend(extension.into_iter())
            }
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Public Macros ❱ trait HasGroups
// ------------------------------------------------------------------------------------------------

macro_rules! impl_has_groups_for {
    ($type: ty, $grouptype: ty, $membertype: ty) => {
        impl_has_groups_for!($type, $grouptype, $membertype, groups);
    };
    ($type: ty, $grouptype: ty, $membertype: ty, $inner: ident) => {
        impl $crate::model::definitions::HasGroups<$grouptype, $membertype> for $type {
            fn has_groups(&self) -> bool {
                !self.$inner.is_empty()
            }

            fn groups_len(&self) -> usize {
                self.$inner.len()
            }

            fn groups(&self) -> Box<dyn Iterator<Item = &$grouptype> + '_> {
                Box::new(self.$inner.iter())
            }

            fn groups_mut(&mut self) -> Box<dyn Iterator<Item = &mut $grouptype> + '_> {
                Box::new(self.$inner.iter_mut())
            }

            fn add_to_groups(&mut self, value: $grouptype) {
                self.$inner.push(value.into())
            }

            fn extend_groups<I>(&mut self, extension: I)
            where
                I: IntoIterator<Item = $grouptype>,
            {
                self.$inner.extend(extension.into_iter())
            }
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Public Macros ❱ Delegate
// ------------------------------------------------------------------------------------------------

#[allow(unused_macro_rules)]
macro_rules! delegate {
    ($vis: vis $fnname: ident, (), $fieldname: ident $(, $paramname: ident => $paramtype: ty)* ) => {
        #[inline(always)]
        $vis fn $fnname(&self $(, $paramname: $paramtype)*) {
            self.$fieldname.$fnname($($paramname,)*)
        }
    };
    ($vis: vis $fnname: ident, $fntype: ty, $fieldname: ident $(, $paramname: ident => $paramtype: ty)* ) => {
        #[inline(always)]
        $vis fn $fnname(&self $(, $paramname: $paramtype)*) -> $fntype {
            self.$fieldname.$fnname($($paramname,)*)
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
