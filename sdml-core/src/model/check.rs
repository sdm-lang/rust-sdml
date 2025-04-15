/*!
Provides types for model checking.

*/

use super::{definitions::HasMultiMembers, identifiers::Identifier, HasSourceSpan};
use crate::{
    load::ModuleLoader,
    model::{definitions::Definition, identifiers::IdentifierReference, modules::Module, HasName},
    store::ModuleStore,
};
use sdml_errors::diagnostics::functions::{duplicate_member, member_is_incomplete};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub trait MaybeIncomplete {
    fn is_incomplete(&self, top: &Module, cache: &impl ModuleStore) -> bool;
}

pub trait Validate {
    fn validate(
        &self,
        top: &Module,
        cache: &impl ModuleStore,
        loader: &impl ModuleLoader,
        check_constraints: bool,
    );
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn find_definition<'a>(
    name: &IdentifierReference,
    current: &'a Module,
    cache: &'a impl ModuleStore,
) -> Option<&'a Definition> {
    cache.resolve_or_in(name, current.name())
}

pub fn validate_is_incomplete<E>(
    model_element: &E,
    top: &Module,
    cache: &impl ModuleStore,
    loader: &impl ModuleLoader,
) where
    E: HasName + HasSourceSpan + MaybeIncomplete,
{
    validate_is_incomplete_named(model_element, model_element.name(), top, cache, loader)
}

pub fn validate_is_incomplete_named<E, S>(
    model_element: &E,
    name: S,
    top: &Module,
    cache: &impl ModuleStore,
    loader: &impl ModuleLoader,
) where
    E: HasSourceSpan + MaybeIncomplete,
    S: Into<String>,
{
    if model_element.is_incomplete(top, cache) {
        loader
            .report(&member_is_incomplete(
                top.file_id().copied().unwrap_or_default(),
                model_element.source_span().map(|span| span.byte_range()),
                name.into(),
            ))
            .unwrap()
    }
}

pub fn validate_multiple_method_duplicates<E>(
    model_element: &E,
    top: &Module,
    _cache: &impl ModuleStore,
    loader: &impl ModuleLoader,
) where
    E: HasSourceSpan + HasMultiMembers,
{
    let mut all_names: Vec<&Identifier> = model_element.all_member_names().collect();
    all_names.sort();
    for pair in all_names.windows(2) {
        if pair[0] == pair[1] {
            loader
                .report(&duplicate_member(
                    top.file_id().copied().unwrap_or_default(),
                    pair[0]
                        .source_span()
                        .map(|span| span.byte_range())
                        .unwrap_or_default(),
                    pair[1]
                        .source_span()
                        .map(|span| span.byte_range())
                        .unwrap_or_default(),
                ))
                .unwrap()
        }
    }
}

// TODO: need a new version of this --v

// pub fn validate_value(
//     _a_value: &Value,
//     a_type: &TypeReference,
//     current: &Module,
//     cache: &ModuleCache,
//     _check_constraints: bool,
//     _errors: &mut Vec<Error>,
// ) {
//     match a_type {
//         TypeReference::Unknown => {
//             panic!("no value allowed for unknown");
//         }
//         TypeReference::Type(id_ref) => {
//             if let Some(_defn) = find_definition(id_ref, current, cache) {
//                 // todo: check it's an actual type
//                 todo!()
//             } else {
//                 panic!("not a valid type reference");
//             }
//         }
//         TypeReference::FeatureSet(_id_ref) => todo!(),
//         TypeReference::MappingType(_map_type) => todo!(),
//     }
// }

#[cfg(feature = "terms")]
pub mod terms {
    use crate::{
        load::ModuleLoader,
        model::{
            annotations::*, constraints::*, definitions::*, identifiers::*, members::*, modules::*,
            values::*, *,
        },
    };
    use sdml_errors::{diagnostics::functions::deprecated_term_used, Error};
    use serde::{Deserialize, Serialize};
    use std::collections::{HashMap, HashSet};

    // --------------------------------------------------------------------------------------------
    // Public Types
    // --------------------------------------------------------------------------------------------

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct TermSet {
        name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        terms: HashMap<String, Term>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Term {
        #[serde(skip_serializing_if = "Option::is_none", with = "serde_regex")]
        regex: Option<regex::Regex>,
        alternative_terms: Vec<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        reason: Option<String>,
    }

    // --------------------------------------------------------------------------------------------
    // Public Functions
    // --------------------------------------------------------------------------------------------

    const DEFAULT_RULES: &str = include_str!("default_terms.json");

    pub fn default_term_set() -> Result<TermSet, Error> {
        Ok(serde_json::from_str(DEFAULT_RULES).unwrap())
    }

    pub fn validate_module_terms(module: &Module, term_set: &TermSet, loader: &impl ModuleLoader) {
        let mut validator = Validator::from(term_set);
        module.name().validate_terms(&mut validator, module, loader);
        for annotation in module.annotations() {
            annotation.validate_terms(&mut validator, module, loader);
        }
        for definition in module.definitions() {
            definition.validate_terms(&mut validator, module, loader);
        }
    }

    // --------------------------------------------------------------------------------------------
    // Private Types
    // --------------------------------------------------------------------------------------------

    #[derive(Clone, Debug)]
    struct TermInfo<'a> {
        regex: regex::Regex,
        alternative_terms: &'a Vec<String>,
        reason: &'a Option<String>,
    }

    #[derive(Clone, Debug)]
    struct Validator<'a> {
        term_map: HashMap<String, TermInfo<'a>>,
        seen: HashSet<String>,
    }

    // --------------------------------------------------------------------------------------------
    // Implementations ❱ Validator
    // --------------------------------------------------------------------------------------------

    impl<'a> From<&'a TermSet> for Validator<'a> {
        fn from(term_set: &'a TermSet) -> Self {
            let mut term_map: HashMap<String, TermInfo<'a>> = Default::default();
            for (term, info) in &term_set.terms {
                let regex = if let Some(regex) = &info.regex {
                    regex.clone()
                } else {
                    regex::Regex::new(&format!("(?i)\\b{}\\b", term)).unwrap()
                };
                let new_info = TermInfo {
                    regex,
                    alternative_terms: &info.alternative_terms,
                    reason: &info.reason,
                };
                term_map.insert(term.clone(), new_info);
            }
            Self {
                term_map,
                seen: Default::default(),
            }
        }
    }

    impl Validator<'_> {
        fn check_for_matches<S>(
            &mut self,
            value: S,
            span: Option<&Span>,
            top: &Module,
            loader: &impl ModuleLoader,
        ) where
            S: Into<String>,
        {
            let value = value.into();
            if self.seen.insert(value.clone()) {
                for (term, info) in &self.term_map {
                    if info.regex.is_match(value.as_ref()) {
                        loader
                            .report(&deprecated_term_used(
                                top.file_id().copied().unwrap_or_default(),
                                span.map(|span| span.byte_range()),
                                &value,
                                term,
                                info.alternative_terms,
                                info.reason.as_ref(),
                            ))
                            .unwrap()
                    }
                }
            }
        }
    }

    // --------------------------------------------------------------------------------------------

    trait ValidateTerms {
        fn validate_terms(
            &self,
            validator: &mut Validator<'_>,
            top: &Module,
            loader: &impl ModuleLoader,
        );
    }

    impl ValidateTerms for Identifier {
        fn validate_terms(
            &self,
            validator: &mut Validator<'_>,
            top: &Module,
            loader: &impl ModuleLoader,
        ) {
            validator.check_for_matches(self, self.source_span(), top, loader);
        }
    }

    impl ValidateTerms for QualifiedIdentifier {
        fn validate_terms(
            &self,
            validator: &mut Validator<'_>,
            top: &Module,
            loader: &impl ModuleLoader,
        ) {
            self.module().validate_terms(validator, top, loader);
            self.member().validate_terms(validator, top, loader);
        }
    }

    impl ValidateTerms for IdentifierReference {
        fn validate_terms(
            &self,
            validator: &mut Validator<'_>,
            top: &Module,
            loader: &impl ModuleLoader,
        ) {
            match self {
                Self::Identifier(v) => v.validate_terms(validator, top, loader),
                Self::QualifiedIdentifier(v) => v.validate_terms(validator, top, loader),
            }
        }
    }

    impl ValidateTerms for Annotation {
        fn validate_terms(
            &self,
            validator: &mut Validator<'_>,
            top: &Module,
            loader: &impl ModuleLoader,
        ) {
            match self {
                Self::Property(v) => v.validate_terms(validator, top, loader),
                Self::Constraint(v) => v.validate_terms(validator, top, loader),
            }
        }
    }

    impl ValidateTerms for AnnotationProperty {
        fn validate_terms(
            &self,
            validator: &mut Validator<'_>,
            top: &Module,
            loader: &impl ModuleLoader,
        ) {
            self.name_reference().validate_terms(validator, top, loader);
            self.value().validate_terms(validator, top, loader);
        }
    }

    impl ValidateTerms for Value {
        fn validate_terms(
            &self,
            validator: &mut Validator<'_>,
            top: &Module,
            loader: &impl ModuleLoader,
        ) {
            match self {
                Self::Simple(v) => v.validate_terms(validator, top, loader),
                Self::ValueConstructor(v) => v.validate_terms(validator, top, loader),
                Self::Mapping(v) => v.validate_terms(validator, top, loader),
                Self::Reference(v) => v.validate_terms(validator, top, loader),
                Self::Sequence(v) => v.validate_terms(validator, top, loader),
            }
        }
    }

    impl ValidateTerms for SimpleValue {
        fn validate_terms(
            &self,
            validator: &mut Validator<'_>,
            top: &Module,
            loader: &impl ModuleLoader,
        ) {
            if let SimpleValue::String(value) = self {
                validator.check_for_matches(value.value(), value.source_span(), top, loader);
            }
        }
    }

    impl ValidateTerms for ValueConstructor {
        fn validate_terms(
            &self,
            validator: &mut Validator<'_>,
            top: &Module,
            loader: &impl ModuleLoader,
        ) {
            self.type_name().validate_terms(validator, top, loader);
            self.value().validate_terms(validator, top, loader);
        }
    }

    impl ValidateTerms for SequenceOfValues {
        fn validate_terms(
            &self,
            validator: &mut Validator<'_>,
            top: &Module,
            loader: &impl ModuleLoader,
        ) {
            for value in self.iter() {
                value.validate_terms(validator, top, loader);
            }
        }
    }

    impl ValidateTerms for SequenceMember {
        fn validate_terms(
            &self,
            validator: &mut Validator<'_>,
            top: &Module,
            loader: &impl ModuleLoader,
        ) {
            match self {
                Self::Simple(v) => v.validate_terms(validator, top, loader),
                Self::ValueConstructor(v) => v.validate_terms(validator, top, loader),
                Self::Reference(v) => v.validate_terms(validator, top, loader),
                Self::Mapping(v) => v.validate_terms(validator, top, loader),
            }
        }
    }

    impl ValidateTerms for MappingValue {
        fn validate_terms(
            &self,
            validator: &mut Validator<'_>,
            top: &Module,
            loader: &impl ModuleLoader,
        ) {
            self.domain().validate_terms(validator, top, loader);
            self.range().validate_terms(validator, top, loader);
        }
    }

    impl ValidateTerms for AnnotationOnlyBody {
        fn validate_terms(
            &self,
            validator: &mut Validator<'_>,
            top: &Module,
            loader: &impl ModuleLoader,
        ) {
            for annotation in self.annotations() {
                annotation.validate_terms(validator, top, loader);
            }
        }
    }

    impl ValidateTerms for Constraint {
        fn validate_terms(
            &self,
            validator: &mut Validator<'_>,
            top: &Module,
            loader: &impl ModuleLoader,
        ) {
            self.name().validate_terms(validator, top, loader);
            match self.body() {
                ConstraintBody::Informal(v) => v.validate_terms(validator, top, loader),
                ConstraintBody::Formal(v) => v.validate_terms(validator, top, loader),
            }
        }
    }

    impl ValidateTerms for ControlledLanguageString {
        fn validate_terms(
            &self,
            validator: &mut Validator<'_>,
            top: &Module,
            loader: &impl ModuleLoader,
        ) {
            validator.check_for_matches(self.value(), self.source_span(), top, loader);
        }
    }

    impl ValidateTerms for FormalConstraint {
        fn validate_terms(
            &self,
            _validator: &mut Validator<'_>,
            _top: &Module,
            _loader: &impl ModuleLoader,
        ) {
            todo!()
        }
    }

    impl ValidateTerms for Definition {
        fn validate_terms(
            &self,
            validator: &mut Validator<'_>,
            top: &Module,
            loader: &impl ModuleLoader,
        ) {
            match self {
                Self::Datatype(v) => v.validate_terms(validator, top, loader),
                Self::Dimension(v) => v.validate_terms(validator, top, loader),
                Self::Entity(v) => v.validate_terms(validator, top, loader),
                Self::Enum(v) => v.validate_terms(validator, top, loader),
                Self::Event(v) => v.validate_terms(validator, top, loader),
                Self::Property(v) => v.validate_terms(validator, top, loader),
                Self::Rdf(v) => v.validate_terms(validator, top, loader),
                Self::Structure(v) => v.validate_terms(validator, top, loader),
                Self::TypeClass(v) => v.validate_terms(validator, top, loader),
                Self::Union(v) => v.validate_terms(validator, top, loader),
            }
        }
    }

    impl ValidateTerms for DatatypeDef {
        fn validate_terms(
            &self,
            validator: &mut Validator<'_>,
            top: &Module,
            loader: &impl ModuleLoader,
        ) {
            self.name().validate_terms(validator, top, loader);
            self.base_type().validate_terms(validator, top, loader);
            if let Some(body) = self.body() {
                body.validate_terms(validator, top, loader);
            }
        }
    }

    impl ValidateTerms for DimensionDef {
        fn validate_terms(
            &self,
            validator: &mut Validator<'_>,
            top: &Module,
            loader: &impl ModuleLoader,
        ) {
            self.name().validate_terms(validator, top, loader);
            if let Some(body) = self.body() {
                for annotation in body.annotations() {
                    annotation.validate_terms(validator, top, loader);
                }
                match body.identity() {
                    DimensionIdentity::Source(source) => {
                        source.validate_terms(validator, top, loader)
                    }
                    DimensionIdentity::Identity(member) => {
                        member.validate_terms(validator, top, loader)
                    }
                }
                for parent in body.parents() {
                    parent.validate_terms(validator, top, loader);
                }
                for member in body.members() {
                    member.validate_terms(validator, top, loader);
                }
            }
        }
    }

    impl ValidateTerms for SourceEntity {
        fn validate_terms(
            &self,
            validator: &mut Validator<'_>,
            top: &Module,
            loader: &impl ModuleLoader,
        ) {
            self.target_entity().validate_terms(validator, top, loader);
            for member in self.members() {
                member.validate_terms(validator, top, loader);
            }
        }
    }

    impl ValidateTerms for DimensionParent {
        fn validate_terms(
            &self,
            validator: &mut Validator<'_>,
            top: &Module,
            loader: &impl ModuleLoader,
        ) {
            self.name().validate_terms(validator, top, loader);
            self.target_entity().validate_terms(validator, top, loader);
        }
    }

    impl ValidateTerms for EntityDef {
        fn validate_terms(
            &self,
            validator: &mut Validator<'_>,
            top: &Module,
            loader: &impl ModuleLoader,
        ) {
            self.name().validate_terms(validator, top, loader);
            if let Some(body) = self.body() {
                for annotation in body.annotations() {
                    annotation.validate_terms(validator, top, loader);
                }
                for member in body.members() {
                    member.validate_terms(validator, top, loader);
                }
            }
        }
    }

    impl ValidateTerms for EnumDef {
        fn validate_terms(
            &self,
            validator: &mut Validator<'_>,
            top: &Module,
            loader: &impl ModuleLoader,
        ) {
            self.name().validate_terms(validator, top, loader);
            if let Some(body) = self.body() {
                for annotation in body.annotations() {
                    annotation.validate_terms(validator, top, loader);
                }
                for variant in body.variants() {
                    variant.validate_terms(validator, top, loader);
                }
            }
        }
    }

    impl ValidateTerms for ValueVariant {
        fn validate_terms(
            &self,
            validator: &mut Validator<'_>,
            top: &Module,
            loader: &impl ModuleLoader,
        ) {
            self.name().validate_terms(validator, top, loader);
            if let Some(body) = self.body() {
                for annotation in body.annotations() {
                    annotation.validate_terms(validator, top, loader);
                }
            }
        }
    }

    impl ValidateTerms for EventDef {
        fn validate_terms(
            &self,
            validator: &mut Validator<'_>,
            top: &Module,
            loader: &impl ModuleLoader,
        ) {
            self.name().validate_terms(validator, top, loader);
            if let Some(body) = self.body() {
                // TODO: put back -- body.event_source().validate_terms(validator, top, loader);
                for annotation in body.annotations() {
                    annotation.validate_terms(validator, top, loader);
                    for member in body.members() {
                        member.validate_terms(validator, top, loader);
                    }
                }
            }
        }
    }

    impl ValidateTerms for PropertyDef {
        fn validate_terms(
            &self,
            validator: &mut Validator<'_>,
            top: &Module,
            loader: &impl ModuleLoader,
        ) {
            self.member_def().validate_terms(validator, top, loader);
        }
    }

    impl ValidateTerms for RdfDef {
        fn validate_terms(
            &self,
            validator: &mut Validator<'_>,
            top: &Module,
            loader: &impl ModuleLoader,
        ) {
            self.name().validate_terms(validator, top, loader);
            for annotation in self.body().annotations() {
                annotation.validate_terms(validator, top, loader);
            }
        }
    }

    impl ValidateTerms for StructureDef {
        fn validate_terms(
            &self,
            validator: &mut Validator<'_>,
            top: &Module,
            loader: &impl ModuleLoader,
        ) {
            self.name().validate_terms(validator, top, loader);
            if let Some(body) = self.body() {
                for annotation in body.annotations() {
                    annotation.validate_terms(validator, top, loader);
                }
                for member in body.members() {
                    member.validate_terms(validator, top, loader);
                }
            }
        }
    }

    impl ValidateTerms for TypeClassDef {
        fn validate_terms(
            &self,
            validator: &mut Validator<'_>,
            top: &Module,
            loader: &impl ModuleLoader,
        ) {
            self.name().validate_terms(validator, top, loader);
            if let Some(body) = self.body() {
                for annotation in body.annotations() {
                    annotation.validate_terms(validator, top, loader);
                }
            }
            todo!("validate all")
        }
    }

    impl ValidateTerms for UnionDef {
        fn validate_terms(
            &self,
            validator: &mut Validator<'_>,
            top: &Module,
            loader: &impl ModuleLoader,
        ) {
            self.name().validate_terms(validator, top, loader);
            if let Some(body) = self.body() {
                for annotation in body.annotations() {
                    annotation.validate_terms(validator, top, loader);
                }
                for variant in body.variants() {
                    variant.validate_terms(validator, top, loader);
                }
            }
        }
    }

    impl ValidateTerms for TypeVariant {
        fn validate_terms(
            &self,
            validator: &mut Validator<'_>,
            top: &Module,
            loader: &impl ModuleLoader,
        ) {
            self.name_reference().validate_terms(validator, top, loader);
            if let Some(rename) = self.rename() {
                rename.validate_terms(validator, top, loader);
            }
            if let Some(body) = self.body() {
                for annotation in body.annotations() {
                    annotation.validate_terms(validator, top, loader);
                }
            }
        }
    }

    impl ValidateTerms for Member {
        fn validate_terms(
            &self,
            validator: &mut Validator<'_>,
            top: &Module,
            loader: &impl ModuleLoader,
        ) {
            match self.kind() {
                MemberKind::Reference(v) => v.validate_terms(validator, top, loader),
                MemberKind::Definition(v) => v.validate_terms(validator, top, loader),
            }
        }
    }

    impl ValidateTerms for MemberDef {
        fn validate_terms(
            &self,
            validator: &mut Validator<'_>,
            top: &Module,
            loader: &impl ModuleLoader,
        ) {
            self.name().validate_terms(validator, top, loader);
            self.target_type().validate_terms(validator, top, loader);
            if let Some(body) = self.body() {
                body.validate_terms(validator, top, loader);
            }
        }
    }

    impl ValidateTerms for TypeReference {
        fn validate_terms(
            &self,
            validator: &mut Validator<'_>,
            top: &Module,
            loader: &impl ModuleLoader,
        ) {
            match self {
                Self::Unknown => {}
                Self::Type(v) => v.validate_terms(validator, top, loader),
                Self::MappingType(v) => {
                    v.domain().validate_terms(validator, top, loader);
                    v.range().validate_terms(validator, top, loader);
                }
            }
        }
    }
}
