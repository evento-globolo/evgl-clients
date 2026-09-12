# evgl-clients

Polyglot client SDKs for **Evento Globolo**. Generated and handwritten clients share the OpenAPI contract in `openapi/openapi.yaml`, while every language keeps idiomatic authentication, transport injection, retries, and error handling.

## Zed package graph

This repository is a Zed package installed under `.vendor/.zed`. It imports both canonical package layers:

- `evento-globolo/evgl-interfaces` — wire contracts and shared interfaces
- `evento-globolo/evgl-libs` — shared product behavior and helpers

Each language is also exposed as a Zed target from the root `.zpkg.toml`. A `.zpkg.lock` is committed only when produced by a real resolver run.

## Client matrix

| Ecosystem | Path | Package metadata |
|---|---|---|
| Gleam | `clients/gleam` | `gleam.toml` |
| Erlang | `clients/erlang` | `rebar.config` |
| Elixir | `clients/elixir` | `mix.exs` |
| Dart | `clients/dart` | `pubspec.yaml` |
| Rust | `clients/rust` | `Cargo.toml` |
| Rust/WASM | `clients/wasm` | `Cargo.toml` |
| Java | `clients/java` | `pom.xml` |
| Go | `clients/go` | `go.mod` |
| Python 3 | `clients/python` | `pyproject.toml` |
| Ruby | `clients/ruby` | `evgl_client.gemspec` |
| PHP | `clients/php` | `composer.json` |
| TypeScript | `clients/typescript` | `package.json` |
| TypeScript / Node.js | `clients/typescript/nodejs` | package export |
| TypeScript / Deno | `clients/typescript/deno` | package export |
| TypeScript / Bun | `clients/typescript/bun` | package export |
| TypeScript / edge runtimes | `clients/typescript/edge` | package export |
| Kotlin | `clients/kotlin` | `build.gradle.kts` |
| Swift | `clients/swift` | `Package.swift` |

The SDKs preserve the shared edge methods (`health`, `getConfig`, `emitEvent`, and `emitAlert`) and expose the canonical provider surface: capability discovery, connections, OAuth start, event create/list, idempotent cross-posting, jobs, and job WebSocket URL construction. Browser clients return the WebSocket URL but do not pretend they can attach a bearer header; deployments must use the same-site proxy or a reviewed ephemeral-ticket flow. Every implementation accepts an injectable or caller-owned transport so retries, telemetry, platform networking, and deterministic tests remain outside the contract core.

The exact recovered clients remain under `recovery/provider-cross-posting/` as provenance. Their compatible behavior has been ported into the canonical TypeScript, Rust, Go, and Dart slices rather than published as a competing package layout.

## Validation

```bash
./scripts/validate-client-layout.sh
./scripts/test.sh
```

`test.sh` runs every locally available toolchain and reports explicit skips for unavailable ecosystems. CI can install the full matrix without changing the repository contract.
