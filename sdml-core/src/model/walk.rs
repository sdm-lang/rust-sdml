/*!
Provides the capability to walk the in-memory model of an SDML module.

To use the model walker:

1. Provide a type, say `MyModuleWalker`.
2. Provide an implementation of `SimpleModuleVisitor` for `MyModuleWalker`.
2. Implement any methods from the trait `SimpleModuleVisitor` of interest to you.
3. Use the function `walk_module_simple` and provide the module you wish to walk and your walker.

```rust,ignore
#[derive(Debug, Default)]
pub struct MyModuleWalker {}

impl SimpleModuleVisitor for MyModuleWalker {
    // implement some methods...
}

walk_module_simple(
    &some_module,  // module to walk
    &mut MyModuleWalker::default(),
    false,         // ignore constraints
    true           // include members/variants
);
```

*/

use crate::error::Error;
use crate::model::annotations::{Annotation, AnnotationProperty, HasAnnotations};
use crate::model::constraints::{ConstraintBody, ControlledLanguageString, FormalConstraint};
use crate::model::definitions::{
    DatatypeDef, Definition, EntityDef, EnumDef, EventDef, HasMembers, HasVariants, PropertyDef,
    RdfDef, StructureDef, TypeVariant, UnionDef, ValueVariant,
};
use crate::model::identifiers::{IdentifierReference, QualifiedIdentifier};
use crate::model::members::{Member, MemberDef, MemberKind};
use crate::model::modules::{Import, ModuleImport};
use crate::model::modules::{ImportStatement, Module};
use crate::model::{HasBody, HasOptionalBody};
use tracing::info;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The trait that captures the callbacks that [`walk_module_simple`] uses as it traverses the module.
///
/// Some functions return a boolean, this indicates whether the walker should continue into any
/// nested structure for that model element. For example if `structure_start` returns `false` then
/// no annotations or members within that structure instance will be walked. Note that this also
/// removes the corresponding `structure_end` as well.
///
pub trait SimpleModuleVisitor {
    const INCLUDE_NESTED: Result<bool, Error> = Ok(true);
    const NO_NESTED: Result<bool, Error> = Ok(false);

    // --------------------------------------------------------------------------------------------
    // Module-level
    // --------------------------------------------------------------------------------------------

    ///
    /// Called to denote the start of a `Module` instance.
    ///
    /// # Nested Calls
    ///
    /// - `import` once for each import statement
    /// - `annotation_start` once for each annotation on the module
    /// - `definition_start` once for each definition in the module
    /// - `module_end` once, when imports, annotations, and definitions are complete
    ///
    fn module_start(&mut self, _thing: &Module) -> Result<bool, Error> {
        info!("SimpleModuleWalker::module_start(..) -- skipped");
        Self::INCLUDE_NESTED
    }

    ///
    /// Called to denote the end of a `Module` instance.
    ///
    /// # Nested
    ///
    /// None.
    ///
    fn module_end(&mut self, _thing: &Module) -> Result<(), Error> {
        info!("SimpleModuleWalker::module_end(..) -- skipped");
        Ok(())
    }

    ///
    /// Called to denote the start of an `ImportStatement` instance.
    ///
    /// # Nested
    ///
    /// - `module_import`
    /// - `member_import`
    ///
    fn import_statement_start(&mut self, _thing: &ImportStatement) -> Result<bool, Error> {
        info!("SimpleModuleWalker::import_statement_start(..) -- skipped");
        Self::INCLUDE_NESTED
    }

    ///
    /// Called to denote the end of an `ImportStatement` instance.
    ///
    /// # Nested
    ///
    /// None.
    ///
    fn import_statement_end(&mut self, _thing: &ImportStatement) -> Result<(), Error> {
        info!("SimpleModuleWalker::import_statement_end(..) -- skipped");
        Ok(())
    }

    ///
    /// Called to handle a `ModuleImport` instance.
    ///
    /// # Nested
    ///
    /// None.
    ///
    fn module_import(&mut self, _thing: &ModuleImport) -> Result<(), Error> {
        info!("SimpleModuleWalker::module_import(..) -- skipped");
        Ok(())
    }

    ///
    /// Called to handle a `Qualifiedidentifier` instance.
    ///
    /// # Nested
    ///
    /// None.
    ///
    fn member_import(&mut self, _thing: &QualifiedIdentifier) -> Result<(), Error> {
        info!("SimpleModuleWalker::member_import(..) -- skipped");
        Ok(())
    }

    // --------------------------------------------------------------------------------------------
    // Annotations
    // --------------------------------------------------------------------------------------------

    ///
    /// Called to denote the start of an `Annotation` instance.
    ///
    /// # Nested
    ///
    /// - `annotation_property`
    /// - `informal_constraint`
    /// - `formal_constraint`
    /// - `annotation_end`
    ///
    fn annotation_start(&mut self, _thing: &Annotation) -> Result<bool, Error> {
        info!("SimpleModuleWalker::annotation_start(..) -- skipped");
        Self::INCLUDE_NESTED
    }

    ///
    /// Called to denote the end of an `Annotation` instance.
    ///
    /// # Nested
    ///
    /// None.
    ///
    fn annotation_end(&mut self, _thing: &Annotation) -> Result<(), Error> {
        info!("SimpleModuleWalker::annotation_end(..) -- skipped");
        Ok(())
    }

    ///
    /// Called to handle a `AnnotationProperty` instance.
    ///
    /// # Nested
    ///
    /// None.
    ///
    fn annotation_property(&mut self, _thing: &AnnotationProperty) -> Result<(), Error> {
        info!("SimpleModuleWalker::annotation_property(..) -- skipped");
        Ok(())
    }

    ///
    /// Called to handle a `ControlledLanguageString` instance.
    ///
    /// # Nested
    ///
    /// None.
    ///
    fn informal_constraint(&mut self, _thing: &ControlledLanguageString) -> Result<(), Error> {
        info!("SimpleModuleWalker::informal_constraint(..) -- skipped");
        Ok(())
    }

    ///
    /// Called to handle a `FormalConstraint` instance.
    ///
    /// # Nested
    ///
    /// None.
    ///
    fn formal_constraint(&mut self, _thing: &FormalConstraint) -> Result<(), Error> {
        info!("SimpleModuleWalker::formal_constraint(..) -- skipped");
        Ok(())
    }

    // --------------------------------------------------------------------------------------------
    // Definitions
    // --------------------------------------------------------------------------------------------

    ///
    /// Called to denote the start of a `Definition` instance.
    ///
    /// # Nested
    ///
    /// - `datatype_start`
    /// - `entity_start`
    /// - `enum_start`
    /// - `event_start`
    /// - `property_start`
    /// - `rdf_start`
    /// - `structure_start`
    /// - `union_start`
    /// - `definition_end`
    ///
    fn definition_start(&mut self, _thing: &Definition) -> Result<bool, Error> {
        info!("SimpleModuleWalker::definition_start(..) -- skipped");
        Self::INCLUDE_NESTED
    }

    ///
    /// Called to denote the end of a `Definition` instance.
    ///
    /// # Nested
    ///
    /// None
    ///
    fn definition_end(&mut self, _thing: &Definition) -> Result<(), Error> {
        info!("SimpleModuleWalker::definition_end(..) -- skipped");
        Ok(())
    }

    ///
    /// Called to denote the start of a `DatatypeDef` instance.
    ///
    /// # Nested
    ///
    /// - `annotation_start`
    /// - `datatype_end`
    ///
    fn datatype_start(&mut self, _thing: &DatatypeDef) -> Result<bool, Error> {
        info!("SimpleModuleWalker::datatype_start(..) -- skipped");
        Self::INCLUDE_NESTED
    }

    ///
    /// Called to denote the end of a `DatatypeDef` instance.
    ///
    /// # Nested
    ///
    /// None.
    ///
    fn datatype_end(&mut self, _thing: &DatatypeDef) -> Result<(), Error> {
        info!("SimpleModuleWalker::datatype_end(..) -- skipped");
        Ok(())
    }

    ///
    /// Called to denote the start of an `EntityDef` instance.
    ///
    /// # Nested
    ///
    /// - `annotation_start`
    /// - `identity_member_start`
    /// - `member_start`
    /// - `entity_end`
    ///
    fn entity_start(&mut self, _thing: &EntityDef) -> Result<bool, Error> {
        info!("SimpleModuleWalker::import(..) -- skipped");
        Self::INCLUDE_NESTED
    }

    ///
    /// Called to denote the end of an `EntityDef` instance.
    ///
    /// # Nested
    ///
    /// None.
    ///
    fn entity_end(&mut self, _thing: &EntityDef) -> Result<(), Error> {
        info!("SimpleModuleWalker::import(..) -- skipped");
        Ok(())
    }

    ///
    /// Called to denote the start of an `EnumDef` instance.
    ///
    /// # Nested
    ///
    /// - `annotation_start`
    /// - `value_variant_start`
    /// - `enum_end`
    ///
    fn enum_start(&mut self, _thing: &EnumDef) -> Result<bool, Error> {
        info!("SimpleModuleWalker::enum_start(..) -- skipped");
        Self::INCLUDE_NESTED
    }

    ///
    /// Called to denote the end of an `EnumDef` instance.
    ///
    /// # Nested
    ///
    /// None.
    ///
    fn enum_end(&mut self, _thing: &EnumDef) -> Result<(), Error> {
        info!("SimpleModuleWalker::enum_end(..) -- skipped");
        Ok(())
    }

    ///
    /// Called to denote the start of an `EventDef` instance.
    ///
    /// # Nested
    ///
    /// - `annotation_start`
    /// - `member_start`
    /// - `event_end`
    ///
    fn event_start(&mut self, _thing: &EventDef) -> Result<bool, Error> {
        info!("SimpleModuleWalker::event_start(..) -- skipped");
        Self::INCLUDE_NESTED
    }

    ///
    /// Called to denote the end of an `EventDef` instance.
    ///
    /// # Nested
    ///
    /// None.
    ///
    fn event_end(&mut self, _thing: &EventDef) -> Result<(), Error> {
        info!("SimpleModuleWalker::event_end(..) -- skipped");
        Ok(())
    }

    ///
    /// Called to denote the start of a `PropertyDev` instance.
    ///
    /// # Nested
    ///
    /// - `member_definition_start`
    /// - `property_end`
    ///
    fn property_start(&mut self, _thing: &PropertyDef) -> Result<bool, Error> {
        info!("SimpleModuleWalker::property_start(..) -- skipped");
        Self::INCLUDE_NESTED
    }

    ///
    /// Called to denote the end of a `PropertyDef` instance.
    ///
    /// # Nested
    ///
    /// None.
    ///
    fn property_end(&mut self, _thing: &PropertyDef) -> Result<(), Error> {
        info!("SimpleModuleWalker::property_end(..) -- skipped");
        Ok(())
    }

    ///
    /// Called to denote the start of a `RdfDef` instance.
    ///
    /// # Nested
    ///
    /// - `annotation_start`
    /// - `rdf_end`
    ///
    fn rdf_start(&mut self, _thing: &RdfDef) -> Result<bool, Error> {
        info!("SimpleModuleWalker::rdf_start(..) -- skipped");
        Self::INCLUDE_NESTED
    }

    ///
    /// Called to denote the end of a `RdfDef` instance.
    ///
    /// # Nested
    ///
    /// None.
    ///
    fn rdf_end(&mut self, _thing: &RdfDef) -> Result<(), Error> {
        info!("SimpleModuleWalker::rdf_end(..) -- skipped");
        Ok(())
    }

    ///
    /// Called to denote the start of a `StructureDef` instance.
    ///
    /// # Nested
    ///
    /// - `annotation_start`
    /// - `member_start`
    /// - `structure_end`
    ///
    fn structure_start(&mut self, _thing: &StructureDef) -> Result<bool, Error> {
        info!("SimpleModuleWalker::structure_start(..) -- skipped");
        Self::INCLUDE_NESTED
    }

    ///
    /// Called to denote the end of a `StructureDef` instance.
    ///
    /// # Nested
    ///
    /// None.
    ///
    fn structure_end(&mut self, _thing: &StructureDef) -> Result<(), Error> {
        info!("SimpleModuleWalker::structure_end(..) -- skipped");
        Ok(())
    }

    ///
    /// Called to denote the start of an `UnionDef` instance.
    ///
    /// # Nested
    ///
    /// - `annotation_start`
    /// - `type_variant_start`
    /// - `union_end`
    ///
    fn union_start(&mut self, _thing: &UnionDef) -> Result<bool, Error> {
        info!("SimpleModuleWalker::union_start(..) -- skipped");
        Self::INCLUDE_NESTED
    }

    ///
    /// Called to denote the end of an `UnionDef` instance.
    ///
    /// # Nested
    ///
    /// None.
    ///
    fn union_end(&mut self, _thing: &UnionDef) -> Result<(), Error> {
        info!("SimpleModuleWalker::union_end(..) -- skipped");
        Ok(())
    }

    // --------------------------------------------------------------------------------------------
    // Members and Variants
    // --------------------------------------------------------------------------------------------

    ///
    /// Called to denote the start of a `Member` instance.
    ///
    /// # Nested
    ///
    /// - `property_reference_start`
    /// - `member_definition_start`
    /// - `member_end`
    ///
    fn member_start(&mut self, _thing: &Member) -> Result<bool, Error> {
        info!("SimpleModuleWalker::member_start(..) -- skipped");
        Self::INCLUDE_NESTED
    }

    ///
    /// Called to denote the end of a `Member` instance.
    ///
    /// # Nested
    ///
    /// None.
    ///
    fn member_end(&mut self, _thing: &Member) -> Result<(), Error> {
        info!("SimpleModuleWalker::member_end(..) -- skipped");
        Ok(())
    }

    ///
    /// Called to denote the start of an identity `Member` instance.
    ///
    /// # Nested
    ///
    /// - `property_reference_start`
    /// - `member_definition_start`
    /// - `identity_member_end`
    ///
    fn identity_member_start(&mut self, _thing: &Member) -> Result<bool, Error> {
        info!("SimpleModuleWalker::identity_member_start(..) -- skipped");
        Self::INCLUDE_NESTED
    }

    ///
    /// Called to denote the end of an identity `Member` instance.
    ///
    /// # Nested
    ///
    /// None.
    ///
    fn identity_member_end(&mut self, _thing: &Member) -> Result<(), Error> {
        info!("SimpleModuleWalker::identity_member_end(..) -- skipped");
        Ok(())
    }

    ///
    /// Called to denote the start of a `MemberDef` instance.
    ///
    /// # Nested
    ///
    /// - `annotation_start`
    /// - `member_definition_end`
    ///
    fn member_definition_start(&mut self, _thing: &MemberDef) -> Result<bool, Error> {
        info!("SimpleModuleWalker::member_definition_start(..) -- skipped");
        Self::INCLUDE_NESTED
    }

    ///
    /// Called to denote the end of a `MemberDef` instance.
    ///
    /// # Nested
    ///
    /// None.
    ///
    fn member_definition_end(&mut self, _thing: &MemberDef) -> Result<(), Error> {
        info!("SimpleModuleWalker::member_definition_end(..) -- skipped");
        Ok(())
    }

    ///
    /// Called to denote the start of a member reference `IdentifierReference` instance.
    ///
    /// # Nested
    ///
    /// - `property_reference_end`
    ///
    fn property_reference_start(&mut self, _thing: &IdentifierReference) -> Result<bool, Error> {
        info!("SimpleModuleWalker::property_reference_start(..) -- skipped");
        Self::INCLUDE_NESTED
    }

    ///
    /// Called to denote the end of a member reference `IdentifierReference` instance.
    ///
    /// # Nested
    ///
    /// None.
    ///
    fn property_reference_end(&mut self, _thing: &IdentifierReference) -> Result<(), Error> {
        info!("SimpleModuleWalker::property_reference_end(..) -- skipped");
        Ok(())
    }

    ///
    /// Called to denote the start of a `ValueVariant` instance.
    ///
    /// # Nested
    ///
    /// - `annotation_start`
    /// - `value_variant_end`
    ///
    fn value_variant_start(&mut self, _thing: &ValueVariant) -> Result<bool, Error> {
        info!("SimpleModuleWalker::value_variant_start(..) -- skipped");
        Self::INCLUDE_NESTED
    }

    ///
    /// Called to denote the end of a `ValueVariant` instance.
    ///
    /// # Nested
    ///
    /// None.
    ///
    fn value_variant_end(&mut self, _thing: &ValueVariant) -> Result<(), Error> {
        info!("SimpleModuleWalker::value_variant_end(..) -- skipped");
        Ok(())
    }

    ///
    /// Called to denote the start of a `TypeVariant` instance.
    ///
    /// # Nested
    ///
    /// - `annotation_start`
    /// - `type_variant_end`
    ///
    fn type_variant_start(&mut self, _thing: &TypeVariant) -> Result<bool, Error> {
        info!("SimpleModuleWalker::type_variant_start(..) -- skipped");
        Self::INCLUDE_NESTED
    }

    ///
    /// Called to denote the end of a `TypeVariant` instance.
    ///
    /// # Nested
    ///
    /// None.
    ///
    fn type_variant_end(&mut self, _thing: &TypeVariant) -> Result<(), Error> {
        info!("SimpleModuleWalker::type_variant_end(..) -- skipped");
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

macro_rules! walk_annotations {
    ($walker: expr, $iterator: expr, $visit_annotations: expr) => {
        if $visit_annotations {
            for annotation in $iterator {
                if $walker.annotation_start(annotation)? {
                    match annotation {
                        Annotation::Property(property) => {
                            $walker.annotation_property(&property)?;
                        }
                        Annotation::Constraint(cons) => match cons.body() {
                            ConstraintBody::Informal(constraint) => {
                                $walker.informal_constraint(&constraint)?;
                            }
                            ConstraintBody::Formal(constraint) => {
                                $walker.formal_constraint(&constraint)?;
                            }
                        },
                    }
                    $walker.annotation_end(annotation)?;
                }
            }
        }
    };
}
// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Walk the module `module` calling the relevant methods on `walker`.
///
pub fn walk_module_simple(
    module: &Module,
    walker: &mut impl SimpleModuleVisitor,
    visit_annotations: bool,
    visit_members_and_variants: bool,
) -> Result<(), Error> {
    if walker.module_start(module)? {
        let body = module.body();

        for import in body.imports() {
            if walker.import_statement_start(import)? {
                for import in import.imports() {
                    match import {
                        Import::Module(v) => walker.module_import(v)?,
                        Import::Member(v) => walker.member_import(v)?,
                    }
                }
                walker.import_statement_end(import)?;
            }
        }

        walk_annotations!(walker, body.annotations(), visit_annotations);

        for type_def in body.definitions() {
            if walker.definition_start(type_def)? {
                match &type_def {
                    Definition::Datatype(def) => walk_datatype_def(def, walker, visit_annotations)?,
                    Definition::Entity(def) => {
                        walk_entity_def(def, walker, visit_annotations, visit_members_and_variants)?
                    }
                    Definition::Enum(def) => {
                        walk_enum_def(def, walker, visit_annotations, visit_members_and_variants)?
                    }
                    Definition::Event(def) => {
                        walk_event_def(def, walker, visit_annotations, visit_members_and_variants)?
                    }
                    Definition::Property(def) => walk_property_def(def, walker)?,
                    Definition::Rdf(def) => walk_rdf_def(def, walker, visit_annotations)?,
                    Definition::Structure(def) => walk_structure_def(
                        def,
                        walker,
                        visit_annotations,
                        visit_members_and_variants,
                    )?,
                    Definition::TypeClass(_) => todo!(),
                    Definition::Union(def) => {
                        walk_union_def(def, walker, visit_annotations, visit_members_and_variants)?
                    }
                }
                walker.definition_end(type_def)?;
            }
        }

        walker.module_end(module)?;
    }
    Ok(())
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn walk_datatype_def(
    thing: &DatatypeDef,
    walker: &mut impl SimpleModuleVisitor,
    visit_annotations: bool,
) -> Result<(), Error> {
    if walker.datatype_start(thing)? {
        if let Some(body) = thing.body() {
            walk_annotations!(walker, body.annotations(), visit_annotations);
        }

        walker.datatype_end(thing)?;
    }
    Ok(())
}

fn walk_entity_def(
    thing: &EntityDef,
    walker: &mut impl SimpleModuleVisitor,
    visit_annotations: bool,
    visit_members_and_variants: bool,
) -> Result<(), Error> {
    if walker.entity_start(thing)? {
        if let Some(body) = thing.body() {
            walk_identity_member(body.identity(), walker, visit_annotations)?;

            walk_annotations!(walker, body.annotations(), visit_annotations);

            if visit_members_and_variants {
                for member in body.members() {
                    walk_member(member, walker, visit_annotations)?;
                }
            }
        }

        walker.entity_end(thing)?;
    }
    Ok(())
}

fn walk_enum_def(
    thing: &EnumDef,
    walker: &mut impl SimpleModuleVisitor,
    visit_annotations: bool,
    visit_members_and_variants: bool,
) -> Result<(), Error> {
    if walker.enum_start(thing)? {
        if let Some(body) = thing.body() {
            walk_annotations!(walker, body.annotations(), visit_annotations);
            if visit_members_and_variants {
                for variant in body.variants() {
                    walk_value_variant(variant, walker, visit_annotations)?;
                }
            }
        }

        walker.enum_end(thing)?;
    }
    Ok(())
}

fn walk_event_def(
    thing: &EventDef,
    walker: &mut impl SimpleModuleVisitor,
    visit_annotations: bool,
    visit_members_and_variants: bool,
) -> Result<(), Error> {
    if walker.event_start(thing)? {
        if let Some(body) = thing.body() {
            walk_annotations!(walker, body.annotations(), visit_annotations);

            if visit_members_and_variants {
                for member in body.members() {
                    walk_member(member, walker, visit_annotations)?;
                }
            }
        }

        walker.event_end(thing)?;
    }
    Ok(())
}

fn walk_property_def(
    thing: &PropertyDef,
    walker: &mut impl SimpleModuleVisitor,
) -> Result<(), Error> {
    if walker.property_start(thing)? {
        let defn = thing.member_def();
        if walker.member_definition_start(defn)? {
            walker.member_definition_end(defn)?;
        }
        walker.property_end(thing)?;
    }
    Ok(())
}

fn walk_rdf_def(
    thing: &RdfDef,
    walker: &mut impl SimpleModuleVisitor,
    visit_annotations: bool,
) -> Result<(), Error> {
    if walker.rdf_start(thing)? {
        walk_annotations!(walker, thing.body().annotations(), visit_annotations);
        walker.rdf_end(thing)?;
    }
    Ok(())
}

fn walk_structure_def(
    thing: &StructureDef,
    walker: &mut impl SimpleModuleVisitor,
    visit_annotations: bool,
    visit_members_and_variants: bool,
) -> Result<(), Error> {
    if walker.structure_start(thing)? {
        if let Some(body) = thing.body() {
            walk_annotations!(walker, body.annotations(), visit_annotations);

            if visit_members_and_variants {
                for member in body.members() {
                    walk_member(member, walker, visit_annotations)?;
                }
            }
        }

        walker.structure_end(thing)?;
    }
    Ok(())
}

fn walk_union_def(
    thing: &UnionDef,
    walker: &mut impl SimpleModuleVisitor,
    visit_annotations: bool,
    visit_members_and_variants: bool,
) -> Result<(), Error> {
    if walker.union_start(thing)? {
        if let Some(body) = thing.body() {
            walk_annotations!(walker, body.annotations(), visit_annotations);
            if visit_members_and_variants {
                for variant in body.variants() {
                    walk_type_variant(variant, walker, visit_annotations)?;
                }
            }
        }

        walker.union_end(thing)?;
    }
    Ok(())
}

fn walk_member(
    thing: &Member,
    walker: &mut impl SimpleModuleVisitor,
    visit_annotations: bool,
) -> Result<(), Error> {
    if walker.member_start(thing)? {
        walk_member_common(thing, walker, visit_annotations)?;
        walker.member_end(thing)?;
    }
    Ok(())
}

fn walk_identity_member(
    thing: &Member,
    walker: &mut impl SimpleModuleVisitor,
    visit_annotations: bool,
) -> Result<(), Error> {
    if walker.identity_member_start(thing)? {
        walk_member_common(thing, walker, visit_annotations)?;
        walker.identity_member_end(thing)?;
    }
    Ok(())
}

fn walk_member_common(
    thing: &Member,
    walker: &mut impl SimpleModuleVisitor,
    visit_annotations: bool,
) -> Result<(), Error> {
    match thing.kind() {
        MemberKind::Reference(v) => {
            if walker.property_reference_start(v)? {
                walker.property_reference_end(v)?;
            }
        }
        MemberKind::Definition(v) => {
            if walker.member_definition_start(v)? {
                if let Some(body) = v.body() {
                    walk_annotations!(walker, body.annotations(), visit_annotations);
                }
                walker.member_definition_end(v)?;
            }
        }
    }

    Ok(())
}

fn walk_value_variant(
    thing: &ValueVariant,
    walker: &mut impl SimpleModuleVisitor,
    visit_annotations: bool,
) -> Result<(), Error> {
    walker.value_variant_start(thing)?;

    if let Some(body) = thing.body() {
        walk_annotations!(walker, body.annotations(), visit_annotations);
    }

    walker.value_variant_end(thing)
}

fn walk_type_variant(
    thing: &TypeVariant,
    walker: &mut impl SimpleModuleVisitor,
    visit_annotations: bool,
) -> Result<(), Error> {
    walker.type_variant_start(thing)?;

    if let Some(body) = thing.body() {
        walk_annotations!(walker, body.annotations(), visit_annotations);
    }

    walker.type_variant_end(thing)
}
