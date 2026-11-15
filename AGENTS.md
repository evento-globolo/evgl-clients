# AGENTS.md

Owner: `evento-globolo`  
Tracking: `DEN-1889`

Use focused pull requests, preserve interface compatibility, add tests with behavior changes, never commit credentials or customer data, and resolve conflicts semantically using both sides and relevant history.

The canonical repository and Zed package are `evento-globolo/evgl-clients`. Do not revive the superseded `evento-globolo-clients` package identity.

## Rust-first tests and scripts

Follow the [Evento Globolo organization-wide Rust-first policy](https://github.com/evento-globolo/.github/pull/37).
Write new first-party scripts, tests, contract audits, CI helpers and code generation in Rust (`cargo test`, `tests/*.rs`, or `cargo xtask`), **not Python**. Existing Python automation is legacy and must not be extended. Migrate it only after equivalent Rust checks, flags, error behavior, security gates, and exact-head CI are established. Do not remove or bypass a required check to make the policy pass. Python product code needs a narrowly documented application exception.
