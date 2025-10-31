# Contributing

This document includes repository-specific guidance and should be read afer the
organization's overall [Contributing](https://raw.githubusercontent.com/sdm-lang/.github/main/profile/CONTRIBUTING.md)
guidelines. Most of the content in here is specific to the fact the libraries and
tools in this repository are developed in Rust.

## Submitting changes

Fork, then clone the repo:

``` bash
❯ git clone https://github.com/sdm-lang/rust-sdml.git
```

Ensure you have a good Rust install, usually managed by [rustup](https://rustup.rs/).
You can ensure the latest tools with the following:

``` bash
❯ rustup update
```

Make your change. Add tests, and documentation, for your change. Ensure not only
that tests pass, but the following all run successfully.

``` bash
❯ cargo check --workspace --all-targets --all-features
❯ cargo fmt --all -- --check
❯ cargo clippy --workspace --all-targets --all-features -- -D warnings
```

Make sure the tests pass. Note the three test passes which check for weird cases
which can creap in with features.

``` bash
❯ cargo test --workspace --no-fail-fast --all-features
❯ cargo test --workspace --no-fail-fast --all-targets --all-features
❯ cargo test --workspace --no-fail-fast --no-default-targets --all-features
❯ cargo doc --workspace --all-features --no-deps

If you have made any changes to `Cargo.toml`, also check:

``` bash
❯ cargo audit
❯ cargo outdated --workspace --depth 1
```

Push to your fork and [submit a pull request](../../compare/) using our
[PR template](https://raw.githubusercontent.com/sdm-lang/.github/main/profile/pull_request_template.md).

At this point you're waiting on us. We like to at least comment on pull requests
within three business days (and, typically, one business day). We may suggest
some changes or improvements or alternatives.

Some things that will increase the chance that your pull request is accepted:

1. Write unit tests.
2. Write API documentation.
3. Write a [good commit message](https://cbea.ms/git-commit/https://cbea.ms/git-commit/).

## Coding conventions

The primary tool for coding conventions is rustfmt, and specifically `cargo
fmt` is a part of the build process and will cause Actions to fail.

**DO NOT** create or update any existing `rustfmt.toml` file to change the default
formatting rules.

**DO NOT** alter any `warn` or `deny` library attributes.

**DO NOT** add any `feature` attributes that would prohibit building on the stable
channel. In some cases new crate-level features can be used to introduce an
unstable feature dependency but these **MUST** be clearly documented and optional.

**DO NOT** introduce `unsafe` code without a discussion on alternatives first.
Ideally any unsafe code will be introduced as optional features.
