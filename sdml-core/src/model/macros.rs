/*!
Internal

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
            fn with_source_span(self, span: $crate::model::Span) -> Self {
                let mut self_mut = self;
                self_mut.span = Some(Box::new(span));
                self_mut
            }

            fn source_span(&self) -> Option<&$crate::model::Span> {
                match &self.$inner {
                    Some(v) => Some(v.as_ref()),
                    None => None,
                }
            }

            fn set_source_span(&mut self, span: $crate::model::Span) {
                self.$inner = Some(Box::new(span));
            }

            fn unset_source_span(&mut self) {
                self.$inner = None;
            }
        }
    };
     ($type: ty => variants $($varname: ident),+) => {
        impl $crate::model::HasSourceSpan for $type {
            #[inline]
            fn with_source_span(self, span: $crate::model::Span) -> Self {
                match self {
                    $(
                        Self::$varname(v) => Self::$varname(v.with_source_span(span)),
                    )+
                }
            }

            #[inline]
            fn source_span(&self) -> Option<&$crate::model::Span> {
                match self {
                    $(
                        Self::$varname(v) => v.source_span(),
                    )+
                }
            }

            #[inline]
            fn set_source_span(&mut self, span: $crate::model::Span) {
                match self {
                    $(
                        Self::$varname(v) => v.set_source_span(span),
                    )+
                }
            }

            #[inline]
            fn unset_source_span(&mut self) {
                match self {
                    $(
                        Self::$varname(v) => v.unset_source_span(),
                    )+
                }
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
    ($type: ty => variants $($varname: ident),+) => {
        impl $crate::model::HasName for $type {
            fn name(&self) -> &$crate::model::identifiers::Identifier {
                match self {
                    $(
                        Self::$varname(v) => v.name(),
                    )+
                }
            }

            fn set_name(&mut self, name: $crate::model::identifiers::Identifier) {
                match self {
                    $(
                        Self::$varname(v) => v.set_name(name),
                    )+
                }
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
        impl $crate::model::HasBody for $type {
            type Body = $bodytype;

            fn body(&self) -> &Self::Body {
                &self.$inner
            }

            fn body_mut(&mut self) -> &mut Self::Body {
                &mut self.$inner
            }

            fn set_body(&mut self, body: Self::Body) {
                self.$inner = body;
            }
        }
    };
    ($type: ty, boxed $bodytype: ty) => {
        impl_has_body_for!($type, boxed $bodytype, body);
    };
    ($type: ty, boxed $bodytype: ty, $inner: ident) => {
        impl $crate::model::HasBody for $type {
            type Body = $bodytype;

            fn body(&self) -> &Self::Body {
                &self.$inner
            }

            fn body_mut(&mut self) -> &mut Self::Body {
                &mut self.$inner
            }

            fn set_body(&mut self, body: Self::Body) {
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
        impl $crate::model::HasOptionalBody for $type {
            type Body = $bodytype;

            fn body(&self) -> Option<&Self::Body> {
                self.$inner.as_ref()
            }

            fn body_mut(&mut self) -> Option<&mut Self::Body> {
                self.$inner.as_mut()
            }

            fn set_body(&mut self, body: Self::Body) {
                self.$inner = Some(body);
            }

            fn unset_body(&mut self) {
                self.$inner = None;
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
// Public Macros ❱ trait MaybeIncomplete
// ------------------------------------------------------------------------------------------------

macro_rules! impl_maybe_incomplete_for {
    ($type: ty) => {
        impl_maybe_incomplete_for!($type; optional body);
    };
    ($type: ty; always $default: expr) => {
        impl $crate::model::check::MaybeIncomplete for $type {
            fn is_incomplete(
                &self,
                _: &$crate::model::modules::Module,
                _: &impl $crate::store::ModuleStore) -> bool
            {
                $default
            }
        }
    };
    ($type: ty; exists $field: ident) => {
        impl $crate::model::check::MaybeIncomplete for $type {
            fn is_incomplete(
                &self,
                _: &$crate::model::modules::Module,
                _: &impl $crate::store::ModuleStore) -> bool
            {
                self.$field.is_none()
            }
        }
    };
    ($type: ty; delegate $field: ident) => {
        impl $crate::model::check::MaybeIncomplete for $type {
            fn is_incomplete(
                &self,
                top: &$crate::model::modules::Module,
                cache: &impl $crate::store::ModuleStore) -> bool
            {
                self.$field.is_incomplete(top, cache)
            }
        }
    };
    ($type: ty; optional $field: ident) => {
        impl $crate::model::check::MaybeIncomplete for $type {
            fn is_incomplete(
                &self,
                top: &$crate::model::modules::Module,
                cache: &impl $crate::store::ModuleStore) -> bool
            {
                if let Some($field) = &self.$field {
                    $field.is_incomplete(top, cache)
                } else {
                    true
                }
            }
        }
    };
    ($type: ty; over $collection: ident) => {
        impl $crate::model::check::MaybeIncomplete for $type {
            fn is_incomplete(
                &self,
                top: &$crate::model::modules::Module,
                cache: &impl $crate::store::ModuleStore) -> bool
            {
                self.$collection().any(|elem| elem.is_incomplete(top, cache))
            }
        }
    };
    ($type: ty; variants $($varname: ident),+) => {
        impl $crate::model::check::MaybeIncomplete for $type {
            fn is_incomplete(
                &self,
                top: &$crate::model::modules::Module,
                cache: &impl $crate::store::ModuleStore) -> bool
            {
                match self {
                    $(
                        Self::$varname(v) => v.is_incomplete(top, cache),
                    )+
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
            fn validate(
                &self,
                top: &$crate::model::modules::Module,
                cache: &impl $crate::store::ModuleStore,
                loader: &impl $crate::load::ModuleLoader,
                check_constraints: bool,
            ) {
                match self {
                    $(
                        Self::$varname(v) => v.validate(top, cache, loader, check_constraints),
                    )+
                }
            }
        }
    };
}

macro_rules! impl_validate_for_annotations_and_members {
    ($type: ty) => {
        impl Validate for $type {
            fn validate(
                &self,
                top: &$crate::model::modules::Module,
                cache: &impl $crate::store::ModuleStore,
                loader: &impl $crate::load::ModuleLoader,
                check_constraints: bool,
            ) {
                self.annotations()
                    .for_each(|a| a.validate(top, cache, loader, check_constraints));
                self.members()
                    .for_each(|m| m.validate(top, cache, loader, check_constraints));
            }
        }
    };
}

macro_rules! impl_validate_for_annotations_and_variants {
    ($type: ty) => {
        impl Validate for $type {
            fn validate(
                &self,
                top: &Module,
                cache: &impl $crate::store::ModuleStore,
                loader: &impl $crate::load::ModuleLoader,
                check_constraints: bool,
            ) {
                self.annotations()
                    .for_each(|a| a.validate(top, cache, loader, check_constraints));
                self.variants()
                    .for_each(|v| v.validate(top, cache, loader, check_constraints));
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

            fn annotations(&self) -> impl Iterator<Item = &$crate::model::annotations::Annotation> {
                self.$inner.iter()
            }

            fn annotations_mut(
                &mut self,
            ) -> impl Iterator<Item = &mut $crate::model::annotations::Annotation> {
                self.$inner.iter_mut()
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
// Public Macros ❱ trait AnnotationBuilder
// ------------------------------------------------------------------------------------------------

macro_rules! impl_annotation_builder {
    ($type: ty) => {
        impl_annotation_builder!($type, body);
    };
    ($type: ty, $inner: ident) => {
        impl $crate::model::annotations::AnnotationBuilder for $type {
            fn with_predicate<I, V>(self, predicate: I, value: V) -> Self
            where
                Self: Sized,
                I: Into<$crate::model::identifiers::IdentifierReference>,
                V: Into<$crate::model::values::Value>,
            {
                let mut self_mut = self;
                self_mut.$inner.add_to_annotations(
                    $crate::model::annotations::AnnotationProperty::new(
                        predicate.into(),
                        value.into(),
                    ),
                );
                self_mut
            }
        }
    };
    ($type: ty, optional $inner: ident) => {
        impl $crate::model::annotations::AnnotationBuilder for $type {
            fn with_predicate<I, V>(self, predicate: I, value: V) -> Self
            where
                Self: Sized,
                I: Into<$crate::model::identifiers::IdentifierReference>,
                V: Into<$crate::model::values::Value>,
            {
                use $crate::model::annotations::HasAnnotations;
                let mut self_mut = self;
                if let Some(ref mut inner) = self_mut.$inner {
                    inner.add_to_annotations($crate::model::annotations::AnnotationProperty::new(
                        predicate.into(),
                        value.into(),
                    ));
                }
                self_mut
            }
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Public Macros ❱ trait HasMembers
// ------------------------------------------------------------------------------------------------

macro_rules! impl_has_members_for {
    ($type: ty) => {
        impl_has_members_for!($type, members);
    };
    ($type: ty, $inner: ident) => {
        impl $crate::model::definitions::HasMembers for $type {
            fn has_members(&self) -> bool {
                !self.$inner.is_empty()
            }

            fn members_len(&self) -> usize {
                self.$inner.len()
            }

            fn members(&self) -> impl Iterator<Item = &$crate::model::members::Member> {
                self.$inner.iter()
            }

            fn members_mut(&mut self) -> impl Iterator<Item = &mut $crate::model::members::Member> {
                self.$inner.iter_mut()
            }

            fn add_to_members(&mut self, value: $crate::model::members::Member) {
                self.$inner.push(value.into())
            }

            fn extend_members<I>(&mut self, extension: I)
            where
                I: IntoIterator<Item = $crate::model::members::Member>,
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
        impl $crate::model::definitions::HasVariants for $type {
            type Variant = $varianttype;

            fn has_variants(&self) -> bool {
                !self.$inner.is_empty()
            }

            fn variants_len(&self) -> usize {
                self.$inner.len()
            }

            fn variants(&self) -> impl Iterator<Item = &Self::Variant> {
                self.$inner.iter()
            }

            fn variants_mut(&mut self) -> impl Iterator<Item = &mut Self::Variant> {
                self.$inner.iter_mut()
            }

            fn add_to_variants(&mut self, value: Self::Variant) {
                self.$inner.push(value.into())
            }

            fn extend_variants<I>(&mut self, extension: I)
            where
                I: IntoIterator<Item = Self::Variant>,
            {
                self.$inner.extend(extension.into_iter())
            }
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Public Macros ❱ Fields
// ------------------------------------------------------------------------------------------------

macro_rules! builder_fn {
    ($vis: vis $name: ident, $field: ident => $type: ty) => {
        $vis fn $name(self, $field: $type) -> Self {
            let mut self_mut = self;
            self_mut.$field = $field;
            self_mut
        }
    };
    //($vis: vis $name: ident, $field: ident => into $type: ty) => {
    //    $vis fn $name<T>(self, $field: T) -> Self
    //    where
    //        T: Into<$type>,
    //    {
    //        let mut self_mut = self;
    //        self_mut.$field = $field.into();
    //        self_mut
    //    }
    //};
    ($vis: vis $name: ident, boxed $field: ident => into $type: ty) => {
        $vis fn $name<T>(self, $field: T) -> Self
        where
            T: Into<$type>,
        {
            let mut self_mut = self;
            self_mut.$field = Box::new($field.into());
            self_mut
        }
    };
    ($vis: vis $name: ident, $field: ident => optional $type: ty) => {
        $vis fn $name(self, $field: $type) -> Self {
            let mut self_mut = self;
            self_mut.$field = Some($field);
            self_mut
        }
    };
}

macro_rules! getter {
    ($vis: vis $fieldname: ident => $fieldtype: ty) => {
        getter!($vis $fieldname => $fieldname, $fieldtype);
    };
    ($vis: vis $fieldname: ident => $fnname: ident, $fieldtype: ty) => {
        $vis const fn $fnname(&self) -> &$fieldtype {
            &self.$fieldname
        }
    };
    // --------------------------------------------------------------------------------------------
    //($vis: vis $fieldname: ident => copy $fieldtype: ty) => {
    //    getter!($vis $fieldname => copy $fieldname, $fieldtype);
    //};
    // ($vis: vis $fieldname: ident => copy $fnname: ident, $fieldtype: ty) => {
    //     $vis const fn $fnname(&self) -> $fieldtype {
    //         self.$fieldname
    //     }
    // };
    // --------------------------------------------------------------------------------------------
    //($vis: vis $fieldname: ident => optional $has_fnname: ident, $fieldtype: ty) => {
    //    getter!($vis $fieldname => optional $has_fnname: ident, $fieldname, $fieldtype);
    //};
    ($vis: vis $fieldname: ident => optional $has_fnname: ident, $fnname: ident, $fieldtype: ty) => {
        $vis const fn $has_fnname(&self) -> bool {
            self.$fieldname.is_some()
        }
        $vis const fn $fnname(&self) -> Option<&$fieldtype> {
            self.$fieldname.as_ref()
        }
    };
    // --------------------------------------------------------------------------------------------
    //($vis: vis $fieldname: ident => optional copy $has_fnname: ident, $fieldtype: ty) => {
    //    getter!($vis $fieldname => optional copy $fieldname, $fieldtype);
    //};
    ($vis: vis $fieldname: ident => optional copy $has_fnname: ident, $fnname: ident, $fieldtype: ty) => {
        $vis const fn $has_fnname(&self) -> bool {
            self.$fieldname.is_some()
        }
        $vis const fn $fnname(&self) -> Option<$fieldtype> {
            self.$fieldname
        }
    };
}

macro_rules! setter {
    ($vis: vis $fnname: ident => $fieldname: ident, $fieldtype: ty) => {
        $vis fn $fnname(&mut self, $fieldname: $fieldtype) {
            self.$fieldname = $fieldname;
        }
    };
    // --------------------------------------------------------------------------------------------
    ($vis: vis $fnname: ident => $fieldname: ident, into $fieldtype: ty) => {
        $vis fn $fnname<T>(&mut self, $fieldname: T)
        where
            T: Into<$fieldtype>,
        {
            self.$fieldname = $fieldname.into();
        }
    };
    // --------------------------------------------------------------------------------------------
    ($vis: vis $fnname: ident => optional $fieldname: ident, $fieldtype: ty) => {
        $vis fn $fnname(&mut self, $fieldname: $fieldtype) {
            self.$fieldname = Some($fieldname);
        }
    };
    // --------------------------------------------------------------------------------------------
    ($vis: vis $fnname: ident => boxed $fieldname: ident, $fieldtype: ty) => {
        $vis fn $fnname(&mut self, $fieldname: $fieldtype) {
            self.$fieldname = Box::new($fieldname);
        }
    };
    // --------------------------------------------------------------------------------------------
    ($vis: vis $fnname: ident => boxed $fieldname: ident, into $fieldtype: ty) => {
        $vis fn $fnname<T>(&mut self, $fieldname: T)
        where
            T: Into<$fieldtype>,
        {
            self.$fieldname = Box::new($fieldname.into());
        }
    };
}

macro_rules! unsetter {
    ($vis: vis $fnname: ident => $fieldname: ident) => {
        $vis fn $fnname(&mut self) {
            self.$fieldname = None;
        }
    };
}

macro_rules! get_and_set {
    ($vis: vis $fieldname: ident, $set_fnname: ident => $fieldtype: ty) => {
        get_and_set!($vis $fieldname, $fieldname, $set_fnname => $fieldtype);
    };
    ($vis: vis $fieldname: ident, $get_fnname: ident, $set_fnname: ident => $fieldtype: ty) => {
        getter!($vis $fieldname => $get_fnname, $fieldtype);
        setter!($vis $set_fnname => $fieldname, $fieldtype);
    };
    // --------------------------------------------------------------------------------------------
    ($vis: vis $fieldname: ident, $set_fnname: ident => into $fieldtype: ty) => {
        get_and_set!($vis $fieldname, $fieldname, $set_fnname => into $fieldtype);
    };
    ($vis: vis $fieldname: ident, $get_fnname: ident, $set_fnname: ident => into $fieldtype: ty) => {
        getter!($vis $fieldname => $get_fnname, $fieldtype);
        setter!($vis $set_fnname => $fieldname, into $fieldtype);
    };
    // --------------------------------------------------------------------------------------------
    ($vis: vis $fieldname: ident, $set_fnname: ident => boxed $fieldtype: ty) => {
        get_and_set!($vis $fieldname, $fieldname, $set_fnname => boxed $fieldtype);
    };
    ($vis: vis $fieldname: ident, $get_fnname: ident, $set_fnname: ident => boxed $fieldtype: ty) => {
        getter!($vis $fieldname => $get_fnname, $fieldtype);
        setter!($vis $set_fnname => boxed $fieldname, $fieldtype);
    };
    // --------------------------------------------------------------------------------------------
    ($vis: vis $fieldname: ident, $set_fnname: ident => boxed into $fieldtype: ty) => {
        get_and_set!($vis $fieldname, $fieldname, $set_fnname => boxed into $fieldtype);
    };
    ($vis: vis $fieldname: ident, $get_fnname: ident, $set_fnname: ident => boxed into $fieldtype: ty) => {
        getter!($vis $fieldname => $get_fnname, $fieldtype);
        setter!($vis $set_fnname => boxed $fieldname, into $fieldtype);
    };
    // --------------------------------------------------------------------------------------------
    ($vis: vis $fieldname: ident, $set_fnname: ident, $unset_fnname: ident => optional $has_fnname: ident, $fieldtype: ty) => {
        get_and_set!($vis $fieldname, $fieldname, $set_fnname, $unset_fnname => optional $has_fnname, $fieldtype);
    };
    ($vis: vis $fieldname: ident, $get_fnname: ident, $set_fnname: ident, $unset_fnname: ident => optional $has_fnname: ident, $fieldtype: ty) => {
        getter!($vis $fieldname => optional $has_fnname, $get_fnname, $fieldtype);
        setter!($vis $set_fnname => optional $fieldname, $fieldtype);
        unsetter!($vis $unset_fnname => $fieldname);
    };
    // --------------------------------------------------------------------------------------------
    ($vis: vis $fieldname: ident, $set_fnname: ident, $unset_fnname: ident => optional copy $has_fnname: ident, $fieldtype: ty) => {
        get_and_set!($vis $fieldname, $fieldname, $set_fnname, $unset_fnname => optional copy $has_fnname, $fieldtype);
    };
    ($vis: vis $fieldname: ident, $get_fnname: ident, $set_fnname: ident, $unset_fnname: ident => optional copy $has_fnname: ident, $fieldtype: ty) => {
        getter!($vis $fieldname => optional copy $has_fnname, $get_fnname, $fieldtype);
        setter!($vis $set_fnname => optional $fieldname, $fieldtype);
        unsetter!($vis $unset_fnname => $fieldname);
    };
}

macro_rules! get_and_set_bool {
    ($vis: vis $fieldname: ident, $is_fname: ident, $set_fnname: ident) => {
        $vis fn $is_fname(&self) -> bool {
            self.$fieldname
        }
        setter!($vis $set_fnname => $fieldname, bool);
    };
}

macro_rules! get_and_set_vec {
    (
            $vis: vis
            is $empty: ident,
            $len: ident,
            $iter: ident,
            $iter_mut: ident,
            $push: ident,
            $extend: ident
        =>  $inner: ident,
            $membertype: ty
    ) => {
        $vis fn $empty(&self) -> bool {
            self.$inner.is_empty()
        }
        get_and_set_vec!($vis $len, $iter, $iter_mut, $push, $extend => $inner, $membertype);
    };
    (
            $vis: vis
            has $empty: ident,
            $len: ident,
            $iter: ident,
            $iter_mut: ident,
            $push: ident,
            $extend: ident
        =>  $inner: ident,
            $membertype: ty
    ) => {
        $vis fn $empty(&self) -> bool {
            !self.$inner.is_empty()
        }
        get_and_set_vec!($vis $len, $iter, $iter_mut, $push, $extend => $inner, $membertype);
    };
    (
            $vis: vis
            $len: ident,
            $iter: ident,
            $iter_mut: ident,
            $push: ident,
            $extend: ident
        =>  $inner: ident,
            $membertype: ty
    ) => {
        $vis fn $len(&self) -> usize {
            self.$inner.len()
        }

        $vis fn $iter(&self) -> impl Iterator<Item = &$membertype> {
            self.$inner.iter()
        }

        $vis fn $iter_mut(&mut self) -> impl Iterator<Item = &mut $membertype> {
            self.$inner.iter_mut()
        }

        $vis fn $push<I>(&mut self, value: I)
        where
            I: Into<$membertype>,
        {
            self.$inner.push(value.into())
        }

        $vis fn $extend<I>(&mut self, extension: I)
        where
            I: IntoIterator<Item = $membertype>,
        {
            self.$inner.extend(extension)
        }
     };
}

macro_rules! impl_as_sequence {
    ($vis: vis $type: ty => $membertype: ty) => {
        impl_as_sequence!($vis $type => values, $membertype);
    };
    ($vis: vis $type: ty => $inner: ident, $membertype: ty) => {
        impl $type {
            get_and_set_vec!(
                $vis
                is is_empty,
                len,
                iter,
                iter_mut,
                push,
                extend
                    => $inner, $membertype
            );
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
    ($vis: vis const $fnname: ident, $fntype: ty, $fieldname: ident $(, $paramname: ident => $paramtype: ty)* ) => {
        #[inline(always)]
        $vis const fn $fnname(&self $(, $paramname: $paramtype)*) -> $fntype {
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
    }; //($tyname: ty, $varname: ident, boxed $vartype: ty) => {
       //    impl_from_for_variant!($tyname, $varname, Box<$vartype>);
       //    impl From<$vartype> for $tyname {
       //        fn from(v: $vartype) -> Self {
       //            Self::$varname(Box::new(v))
       //        }
       //    }
       //};
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

macro_rules! is_variant {
    ($varname: ident => $fnname: ident) => {
        pub const fn $fnname(&self) -> bool {
            matches!(self, Self::$varname)
        }
    };
    ($varname: ident () => $fnname: ident) => {
        pub const fn $fnname(&self) -> bool {
            matches!(self, Self::$varname(_))
        }
    };
}

macro_rules! as_variant {
    ($varname: ident ($fntype: ty) => $fnname: ident) => {
        pub const fn $fnname(&self) -> Option<&$fntype> {
            match self {
                Self::$varname(v) => Some(v),
                _ => None,
            }
        }
    };
}

macro_rules! is_as_variant {
    ($varname: ident ($fntype: ty) => $is_name: ident, $as_name: ident) => {
        is_variant!($varname () => $is_name);
        as_variant!($varname ($fntype) => $as_name);
    };
}
