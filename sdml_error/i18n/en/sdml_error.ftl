count_of_bugs = {$count} bugs
count_of_errors ={$count}  errors
count_of_informational = {$count} informational
count_of_warnings = {$count} warnings
counts_generated_summary = generated {$counts}

help_alternative_terms = help: consider one of: {$terms}
help_datatype_invalid_base_type = help: A type reference in this position must refer to a datatype definition
help_deprecated_term_reason = help: {$reason}
help_error_node = help: encountered a tree-sitter ERROR node in the parse tree
help_feature_set_not_a_union = help: A type reference in this position must refer to a union definition
help_more_details_url = help: for more details, see <{$url}>
help_property_reference_not_property = help: A type reference in this position must refer to a property definition
help_type_definition_not_found = help: did you forget to add an import for this type, or qualify it's name

lbl_actual_node_kind = actual node kind: {$kind}
lbl_actual_this_version_uri = module contained this version URI
lbl_actual_version_uri = actual version URI: `<{$url}>`
lbl_definition_name = definition name: `{$name}`
lbl_expected_this_version_uri = expected this version URI
lbl_expected_version_uri = expected version URI: `<{$url}>`
lbl_expecting_node_kind = expecting node kind: {$kind}
lbl_expecting_one_of_node_kind = expecting one of node kinds: {$kind}
lbl_here = here
lbl_identifier = identifier: `{$name}`
lbl_in_this = in_this: `{$val}`
lbl_in_grammar_rule = in grammar rule: `{$name}`
lbl_missing_node_kind = missing node of kind: `{$kind}`
lbl_missing_node_kind_in_variable = missing node of kind: `{$kind}`, in variable: `{$variable}`
lbl_member_name = member name: `{$name}`
lbl_module_name_short = module `{$name}`
lbl_module_name = module name: `{$name}`
lbl_not_valid_for_type = not valid for this type
lbl_parser = parser
lbl_previously_defined_here = was previously defined here
lbl_previously_imported_here = was previously imported here
lbl_property_name = property name: `{$name}`
lbl_rdf_name = RDF name: `{$name}`
lbl_term_name = found term: `{$name}`
lbl_this_definition = this definition
lbl_this_definition_name = this definition name
lbl_this_identifier = this identifier
lbl_this_import = this import
lbl_this_language_tag = this language tag
lbl_this_member = this member
lbl_this_member_name = this member name
lbl_this_module = this module
lbl_this_reference = this reference
lbl_this_type = this type
lbl_this_usage = this usage
lbl_this_value = this value
lbl_this_variant = this variant
lbl_this_variant_name = this variant name
lbl_type_name = type name: `{$name}`
lbl_typeclass_name = type-class name: `{$name}`
lbl_value = value: `{$val}`
lbl_expected_case = expected {$case}

lbl_case_module = snake case (snake_case)
lbl_case_member = snake case (snake_case)
lbl_case_imported_member = snake (snake_case) or upper camel case (UpperCamelCase)
lbl_case_datatype = snake (snake_case) or upper camel case (UpperCamelCase)
lbl_case_rdf = snake case (snake_case)
lbl_case_type_defn = upper camel case (UpperCamelCase)
lbl_case_value_variant= upper camel (UpperCamelCase) or shouty snake case (SHOUTY_SNAKE_CASE)

msg_datatype_invalid_base = invalid type for datatype base, not a datatype
msg_definition_not_found = definition not found in module
msg_deprecated_term_used = found a deprecated term, consider an alternative
msg_double_underscored_identifier = identifiers should avoid using double underscores
msg_duplicate_definition_import = duplicate import of definition
msg_duplicate_definition_name = a definition with this name already exists in this module
msg_duplicate_member_name = a member with this name already exists in this definition
msg_duplicate_module_import = duplicate import of module
msg_duplicate_variant_name = a variant with this name already exists in this definition
msg_featureset_not_union = invalid type for feature set, not a union
msg_incomplete_definition = this definition is incomplete
msg_incomplete_member = this member is incomplete
msg_incomplete_module = this module is incomplete
msg_invalid_identifier = invalid value for an identifier
msg_invalid_language_tag = invalid value for a language tag
msg_invalid_module_base_url = module base URL is invalid or not absolute
msg_invalid_module_version_url = module base URL is invalid or not absolute
msg_invalid_value_for_type = invalid value literal for type
msg_imported_module_not_found = module named in import statement not found
msg_module_not_found = module not found
msg_module_version_info_empty = module's version info string is empty
msg_module_version_mismatch = actual module URI does not match import requirement
msg_module_version_not_found = imported module has no version URI
msg_property_incompatible = a property definition is not compatible in this location
msg_property_reference_not_property = member references a non-property as a property
msg_rdf_definition_incompatible = an RDF definition is not compatible in this location
msg_string_without_language = this string value has no language tag
msg_treesitter_error_node = tree-sitter parse error encountered
msg_treesitter_missing_node = missing an expected tree-sitter node
msg_treesitter_missing_variable = missing an expected tree-sitter variable
msg_treesitter_unexpected_node = encountered an unexpected tree-sitter node
msg_type_definition_not_found = type definition not found in module
msg_typeclass_incompatible = a type-class definition is not compatible in this location
msg_unconstrained_datatype = this datatype is used without any constraint
msg_validation_incomplete = validation may not be complete for this type
msg_not_preferred_case = identifier not using preferred casing

word_bug = bug
word_error = error
word_help = help
word_note = note
word_warning = warning
