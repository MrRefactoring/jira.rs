# Contributing to jira

Bug reports, questions and pull requests are welcome. Questions go to
[Discussions](https://github.com/MrRefactoring/jira.rs/discussions), bugs and requests to
[Issues](https://github.com/MrRefactoring/jira.rs/issues).

## What is generated and what is not

Most of `src/` is generated from Atlassian's OpenAPI documents. A file that starts with `// @generated` is overwritten
on the next generation, so a change made there by hand is lost. The generator lives in
[apis-code-gen](https://github.com/MrRefactoring/apis-code-gen), and that is where such a change belongs: a wrong
type, a missing parameter, a renamed method. If you are not sure where a fix goes, open an issue with the endpoint and
what the server actually answers; that is enough to act on.

Hand-written, and changed in this repository: `src/core/`, `src/jql.rs`, `src/webhooks/`, `src/lib.rs`, `tests/`,
`xtask/`, `examples/` and the documentation.

## Before opening a pull request

Rust 1.91 or newer. These are the checks CI runs:

```sh
cargo fmt --all --check
cargo clippy --locked --all-features --all-targets -- -D warnings
cargo test --locked --workspace --all-features
cargo test --locked --doc
RUSTDOCFLAGS="-D warnings" cargo doc --locked --all-features --no-deps
RUSTFLAGS="-D warnings" cargo hack check --each-feature --no-dev-deps
cargo xtask parity --check
cargo xtask streams --check
```

A change that a caller can see gets a line in `CHANGELOG.md`.

## Live tests

The tests under `tests/live/` talk to a real Jira and are ignored by default. They need a site of your own and a
`.env` file with `JIRA_BASE_URL`, `JIRA_EMAIL` and `JIRA_API_TOKEN`; the Cloud suite expects a company-managed scrum
project with the key `AUTOTEST`.

```sh
cargo live          # Jira Cloud
cargo live-server   # Jira Data Center, see docker/jira-dc
cargo live-jsm      # Jira Service Management Data Center, see docker/jsm-dc
```

They are not required for a pull request: CI runs the Cloud suite against the project's own site.

## License

By contributing you agree that your work is released under the [MIT license](LICENSE).
