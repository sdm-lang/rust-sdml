# SDML Tests Package

This package is internal-only, it is not and must not be published. The purpose
is to act as a central set of test cases shared across different parsers,
generators and so forth. It consists of a corpus of small SDML example files and
a set of *format* directories that contain a corresponding representation of the
example in that format. For example, given an example `empty_module.sdm` example
file there will be a `empty_module.json` in the `json` format directory and a
`empty_module.ttl` file in the `ttl` directory.

## Unit Test Integration

Client packages integrate by first adding `sdml-tests` to their `dev-dependencies`
and then, by convention, creating a test module names `test_standard_corpus.rs`
that looks something like this:

``` rust
use sdml_core::{
    model::modules::Module, repr::RepresentationWriter,
    store::InMemoryModuleCache,
};

sdml_tests::test_setup! {
    "source_full",       // name of results format/directory
    standard,            // name of suite in sdml-tests
    module_to_string_fn, // name of stringify function
}

fn module_to_string_fn(module: &Module, cache: &InMemoryModuleCache) -> String {
    todo!()
}
```

Completing the function at the end will allow the test framework to parse each
example file, pass it to your module which generates some result which is then
compared to the stored result in the corresponding results file. 

## Tools

There are two shell scripts in the `corpus` directory used to maintain the
examples themselves and the corresponding results. 

### cpsource.sh

In general test cases are (at least *should be*) authored in the `tree-sitter-sdml`
repository to ensure all features are tested there first. This script will copy
any test file from the `test/corpus` directory of the `tree-sitter-sdml` repository
into the `sdml-tests` corpus. The `tree-sitter` test tool puts all it's test
meta-data into the example file, this tool will strip it out so as to leave **only**
the SDML itself.

### mkresults.sh

This tool will create results files, in the corresponding results directory, for
any tests in the corpus that have no results yet. It can also be used to
initialize a new format directory with all tests. 
