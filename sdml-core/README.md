# Package sdml-core

Rust in-Memory model of the Simple Domain Modeling Language (SDML).

[![Crates.io](https://img.shields.io/crates/v/sdml_core.svg)](https://crates.io/crates/sdml_core)
[![Docs.rs](https://img.shields.io/docsrs/sdml-core.svg)](https://docs.rs/sdml_core)

This package is part of the Rust SDML project and specifically defines the
in-memory model of an SDML module. The project's intent is to provide an
idiomatic implementation of the in-memory model, parser, generators, and the CLI
tool.

The following figure demonstrates this package in the broader project context.

![Package Overview](https://raw.githubusercontent.com/sdm-lang/rust-sdml/refs/heads/main/doc/overview-core.png)

Note that other tools can use the =sdml_core= API to create or manipulate models.

## Changes

### Version 0.4.1

Another major release with language features; these have been building up in the
mainline branch for the grammar for a while, but getting the new build working
for v0.4 has taken longer than expected.

1. Feature: add `from` clause to import statements to allow import paths.
1. Feature: removed `self` from the rule `quantified_variable`.
1. Feature: add new syntax for datatype restrictions.
1. Feature: add ability to externally define the name of reserved library module
   names and datatypes.
1. Feature: simplified functions (in constraints) and methods (in type classes)
   to share grammar so implementation is easier and cleaner.
1. Style: consistent style for `use` statements with nesting.

### Version 0.4.0

The primary aim of this release is to introduce a new definition type, a
*dimension*. This may be seen as a violation of SDML's goal of being technology or
implementation independent however it is a pragmatic decision based on usage
experience. Modeling the data managed by a business in terms of entities solves
many operational purposes but ignores a major purpose of this data -- reporting.

```sdml
module example is

  import [ sales stores xsd ]

  dimension Region is
    ;; define an identifier for this dimension
    identity region -> long

    ;; add members
    name -> string
  end
  
  dimension Location is
    ;; tightly bind this dimension to the Sale entity
    source sales:Sale

    ;; define a hierarchy by noting one or more parent dimensions
    parent region -> Region

    ;; reuse members from the source entity
    store from sales:Sale
    city from sales:Sale

    ;; add additional members not on the source entity
    state -> stores:State
    country -> stores:Country
  end

end
```

Detailed Changes:

* Add new `DimensionDef` to the model which has a source clause, set of parents
  and set of members.
* Add a `source` claused based on, but extending, the one on an event definition.
* Add new `DimensionParent` structure with name and entity name reference.

The new syntax for the `source` keyword also allows the inclusion of members from
the source entity rather than requiring duplicate members. This new source
clause has been added to the event definition as a part of the body of an event
thus allowing incomplete event definitions.

``` sdml
module example is

  entity Thing is
    identity id -> long
    name -> string
  end
  
  event Empty

  event NewThing is
    source Thing with name
  end

end

Additionally, this version of the grammar allows module's to rename imports,
both modules and members. This allows then client module to avoid always using
qualified names, and to use short, or more meaningful, names as appropriate.

``` sdml
module example is

  import rentals_billing as billing

  import billing:Invoice as Invoice

end
```

### Version 0.3.1

* Feature: move use of tree-sitter crate behind a feature.
* Feature: added iterators to trait `ModuleStore`.

### Version 0.3.0

* Feature: update for the latest grammar.
  * Updated `Member` to be a union of `MemberDef` and `IdentifierReference~`
  * Updated entity `identity` to be just a `Member`.
  * Updated `PropertyDef` to be a wrapper around `MemberDef`.
  * Fixed corresponding validation rules.
* Refactor: simplified the walker/visitor to pass model members rather than
  de-constructing them.
* Refactor: renamed and made some changes in the `cache` (now `store`) and `load`
  modules.

### Version 0.2.17

* Feature: inverted the logic for determining whether a definition is incomplete.
  * Rename: trait `MaybeInvalid` to `MaybeIncomplete`.
* Feature: added method `has_source` to the trait `ModuleLoader`.
* Feature: added implementation of `FromStr` for both `QualifiedReference` and
  `IdentifierReference`.
* Build: bump version of sdml-errors.

### Version 0.2.16

* Fix: [sdml:srcLabel lacks surrounding quotation in Turtle serialization](https://github.com/sdm-lang/rust-sdml/issues/11) (#11).

### Version 0.2.15

* Fix: `Identifier::from_str` should allow type names.

### Version 0.2.14

* Feature: add new `ModuleStore` trait, implemented by `ModuleCache`.

While not advantageous immediately, it mirrors the separation of trait and
implementation that worked well for `ModuleLoader` and `ModuleResolver`.

### Version 0.2.13

* Feature: add new validation for `IdentifierNotPreferredCase`, to enforce case
  conventions.

### Version 0.2.12

* Feature: more term validation, mainly to reduce the number of `todo!` panics.

### Version 0.2.11

* Refactor: moved errors and diagnostics to new crate `sdml_error`.
* Feature: started on diagnostics and verification, working but incomplete.
* Added: =deprecated= terminology validation.

### Version 0.2.10

* Feature: Add new stdlib modules `iso_3166` for country codes, and `iso_4217` for
  currency codes.
* Style: Shortened stdlib constant names for readability.
* Fix: Correct the regex for identifiers, now in sync with the grammar.

### Version 0.2.9

* Fix: Cardinality parser set incorrect default values.
  * Update: the `with_` constructors on `Cardinality` to take option types.

### Version 0.2.8

* Feature: Add more to the SDML standard library module as needed for RDF
  generator.

### Version 0.2.7

Clean-up release.

* Added new `import!` macro for stdlib modules.
* Removed debugging `println!` calls.
* Fixed compiler warnings and fmt issues.

### Version 0.2.6

* Feature: Added more to the `sdml` stdlib module.
* Feature: Added helpers `is_stdlib_property` and `is_datatype_facet` to
  `AnnotationProperty`.
* Feature: Added new `AnnotationBuilder` trait and impls on most definitions to
  allow easy adding of annotation properties.
* Feature: Added helper methods to `ModuleCache` to make it more collection-like.

### Version 0.2.5

* Feature: Implemented the core standard library modules.
  * `dc` (elements) -- Complete.
  * `dc_terms` -- Not started.
  * `dc_am` -- Not started.
  * `dc_type` -- Not started.
  * `owl` -- Complete.
  * `rdf` -- Complete.
  * `rdfs` -- Complete.
  * `sdml` -- Mostly complete.
  * `skos` -- Complete.
  * `xsd` (part 2) -- Complete.

This change affects the =ModuleCache= as well, it's `with_stdlib` constructor will
include all the library modules and their definitions. This can be checked out
with the command-line tool to either draw diagrams of the standard library
modules or convert into s-expressions, etc.

### Version 0.2.4

* Feature: add new stdlib modules with standard layout.
* Feature: minor refactor of cache and loader.

### Version 0.2.3

* Feature: Update to latest grammar for version URIs and RDF definitions.

### Version 0.2.2

* Feature: Add initial support for versioned modules.
  * Remove `base` keyword.
  * Add new optional `version` keyword after module URI with:
    * optional version string that becomes `owl:versionInfo`.
    * version URI that becomes `owl:verionIRI`.
* Feature: Add new RDF structure/property definitions.
  * Add new keyword `rdf` followed by either `structure` or `property` with name
    and annotation body.
  * Extended `SimpleModuleWalker` with support for RDF class/property definitions.

### Version 0.2.1

* Feature: Remove member groups.

### Version 0.2.0

* Feature: Update to latest grammar.
  * Remove Value Variant numeric values.
  * Update formal constraints.
  * Add type classes.

### Version 0.1.11

* Feature: Update `Cardinality::to_uml_string` to output constraints.
* Fix: Missing features in mapping types and values.

### Version 0.1.10

* Feature: Added support for `mapping_type` and `mapping_value` rules.

### Version 0.1.9

* Style: Run Cargo format and clippy.

### Version 0.1.8

* Feature: Made the name for constraints required, not `Option`.
* Style: Remove most macros from the model.

### Version 0.1.7

* Fix: Minor fixes.

### Version 0.1.6

* Build: Updated parser with grammar changes in `tree-sitter-sdml` version
  `0.1.29`

### Version 0.1.5

* Created a `stdlib` module and moved all the SDML and relevant RDF files into it.
* Updated model to the same level as `tree-sitter-sdml` version `0.1.21`.
* Updated `tree-sitter-sdml` dependency with updated constraints.
  * Renamed `TypeDefinition` to =Definition= to address the fact that property
    definitions aren't types.
  * Renamed `EnumVariant` to `ValueVariant` to align with `TypeVariant` on unions.
    This required change to walker methods.

### Version 0.1.4

Previously part of a single crate [sdml](https://crates.io/crates/sdml).
