# Contributing

Contributions from people and AI-assisted contributors go through the same
review. Contributions are accepted under the project license, MPL-2.0
(inbound = outbound). No CLA.

1. Open an issue for anything larger than a small fix.
2. Use synthetic test files. Never include secrets or private documents.
3. Add tests. Before opening a PR, run:

       cargo fmt --all
       cargo clippy --workspace --all-targets --locked -- -D warnings
       cargo test --workspace --locked

4. Put every user-visible string in `locales/en-US/main.ftl` and use its
   message ID. Do not concatenate translated fragments.
5. Disclose AI assistance and third-party code in the PR description.

Maintainer review is required; there are no automatic merges. Critique code,
not people.
