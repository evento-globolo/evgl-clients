# Architecture

`evgl-clients` contains generated and hand-written Evento Globolo clients for supported application runtimes.

## Canonical package fleet

- `evgl-interfaces` owns wire formats and generated contract types.
- `evgl-libs` owns reusable, runtime-light domain behavior.
- `evgl-clients` consumes versioned interfaces and exposes language-specific SDKs.
- `evgl-sync` owns offline-first reconciliation boundaries.
- `evgl-api` and the MASH, Leptos, and Dioxus web runtimes own deployment behavior.
- `evgl-cli` composes clients, interfaces, and libraries for operator workflows.
- `evgl-infra` owns deployment configuration.
- `evgl-monorepo` coordinates pinned revisions without becoming a second package identity.

The long-name bootstrap repositories are historical aliases, not package sources. New dependencies must use the short `evento-globolo/evgl-*` coordinates.

## Zed and Git submodules

Prefer Zed for reusable dependency resolution. A repository retained as a Git submodule must have an explicit editable-workspace, inventory, embedded-source, experiment-reference, or legacy role. Do not represent the same repository as both a Zed dependency and a gitlink in one composition.

`zed overtake --git-submodules` imports each initialized submodule that declares its own `.zpkg.toml` into the root manifest and lockfile while retaining `.gitmodules` as a reversible transport mirror. Edge code is allowlisted and never a generic proxy.
