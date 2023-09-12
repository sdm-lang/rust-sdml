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
    ($type: ty => variants $($varname: ident),+) => {
        impl $crate::model::HasSourceSpan for $type {
            #[inline]
            fn with_source_span(self, span: Span) -> Self {
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
    ($type: ty => variants $($varname: ident),+) => {
        impl $crate::model::members::HasType for $type {
            #[inline]
            fn target_type(&self) -> &$crate::model::members::TypeReference {
                match self {
                    $(
                        Self::$varname(v) => v.target_type(),
                    )+
                }
            }

            #[inline]
            fn set_target_type(&mut self, target_type: $crate::model::members::TypeReference) {
                match self {
                    $(
                        Self::$varname(v) => v.set_target_type(target_type),
                    )+
                }
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
    }; //    ($type: ty => variants $($varname: ident),+) => {
       //        impl $crate::model::members::HasCardinality for $type {
       //            #[inline]
       //            fn target_cardinality(&self) -> &Cardinality {
       //                match self {
       //                    $(
       //                        Self::$varname(v) => v.target_cardinality(),
       //                    )+
       //                }
       //            }
       //
       //            #[inline]
       //            fn set_target_cardinality(&mut self, target_cardinality: Cardinality) {
       //                match self {
       //                    $(
       //                        Self::$varname(v) => v.set_target_cardinality(target_cardinality),
       //                    )+
       //                }
       //            }
       //        }
       //    };
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
    ($type: ty => variants $($varname: ident),+) => {
        impl_has_optional_body_for!(
            $type,
            $crate::model::annotations::AnnotationOnlyBody
                => variants $( $varname ),+
        );
    };
    ($type: ty, $bodytype: ty => variants $($varname: ident),+) => {
        impl $crate::model::HasOptionalBody<$bodytype> for $type {
            fn body(&self) -> Option<&$bodytype> {
                match self {
                    $(
                        Self::$varname(v) => v.body(),
                    )+
                }
            }

            fn set_body(&mut self, body: $bodytype) {
                match self {
                    $(
                        Self::$varname(v) => v.set_body(body),
                    )+
                }
            }

            fn unset_body(&mut self) {
                match self {
                    $(
                        Self::$varname(v) => v.unset_body(),
                    )+
                }
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
    ($type: ty => todo!) => {
        impl $crate::model::check::Validate for $type {
            fn is_complete(&self, _: &$crate::model::modules::Module) -> Result<bool, $crate::error::Error> {
                todo!();
            }

            fn is_valid(&self, _: bool, _: &$crate::model::modules::Module) -> Result<bool, $crate::error::Error> {
                todo!();
            }
        }
    };
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

macro_rules! impl_validate_for_annotations_and_members {
    ($type: ty) => {
        impl Validate for $type {
            fn is_complete(&self, top: &Module) -> Result<bool, Error> {
                Ok(self
                    .annotations()
                    .map(|ann| ann.is_complete(top))
                    .chain(self.members().map(|m| m.is_complete(top)))
                    .collect::<Result<Vec<bool>, Error>>()?
                    .into_iter()
                    .all(::std::convert::identity))
            }

            fn is_valid(&self, check_constraints: bool, top: &Module) -> Result<bool, Error> {
                Ok(self
                    .annotations()
                    .map(|ann| ann.is_valid(check_constraints, top))
                    .chain(self.members().map(|m| m.is_complete(top)))
                    .collect::<Result<Vec<bool>, Error>>()?
                    .into_iter()
                    .all(::std::convert::identity))
            }
        }
    };
}

macro_rules! impl_validate_for_annotations_and_variants {
    ($type: ty) => {
        impl Validate for $type {
            fn is_complete(&self, top: &Module) -> Result<bool, Error> {
                Ok(self
                    .annotations()
                    .map(|ann| ann.is_complete(top))
                    .chain(self.variants().map(|m| m.is_complete(top)))
                    .collect::<Result<Vec<bool>, Error>>()?
                    .into_iter()
                    .all(::std::convert::identity))
            }

            fn is_valid(&self, check_constraints: bool, top: &Module) -> Result<bool, Error> {
                Ok(self
                    .annotations()
                    .map(|ann| ann.is_valid(check_constraints, top))
                    .chain(self.variants().map(|m| m.is_complete(top)))
                    .collect::<Result<Vec<bool>, Error>>()?
                    .into_iter()
                    .all(::std::convert::identity))
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
    ($type: ty => variants $($varname: ident),+) => {
        impl HasAnnotations for $type {
            fn has_annotations(&self) -> bool {
                match self {
                    $(
                        Self::$varname(v) => v.has_annotations(),
                    )+
                }
            }

            fn annotations_len(&self) -> usize {
                match self {
                    $(
                        Self::$varname(v) => v.annotations_len(),
                    )+
                }
            }

            fn annotations(&self) -> Box<dyn Iterator<Item = &Annotation> + '_> {
                match self {
                    $(
                        Self::$varname(v) => v.annotations(),
                    )+
                }
            }

            fn annotations_mut(&mut self) -> Box<dyn Iterator<Item = &mut Annotation> + '_> {
                match self {
                    $(
                        Self::$varname(v) => v.annotations_mut(),
                    )+
                }
            }

            fn add_to_annotations<I>(&mut self, value: I)
            where
                I: Into<Annotation> {
                match self {
                    $(
                        Self::$varname(v) => v.add_to_annotations(value),
                    )+
                }
            }

            fn extend_annotations<I>(&mut self, extension: I)
            where
                I: IntoIterator<Item = Annotation> {
                match self {
                    $(
                        Self::$varname(v) => v.extend_annotations(extension),
                    )+
                }
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
// Public Macros ❱ Fields
// ------------------------------------------------------------------------------------------------

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
    ($vis: vis $fieldname: ident => copy $fnname: ident, $fieldtype: ty) => {
        $vis const fn $fnname(&self) -> $fieldtype {
            self.$fieldname
        }
    };
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
    ($vis: vis $fieldname: ident, $set_fnname: ident => copy $fieldtype: ty) => {
        get_and_set!($vis $fieldname, $fieldname, $set_fnname => copy $fieldtype);
    };
    ($vis: vis $fieldname: ident, $get_fnname: ident, $set_fnname: ident => copy $fieldtype: ty) => {
        getter!($vis $fieldname => copy $get_fnname, $fieldtype);
        setter!($vis $set_fnname => $fieldname, $fieldtype);
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
