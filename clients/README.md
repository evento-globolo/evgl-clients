# evgl client SDKs

These runtime-specific SDK baselines depend on the `evgl-interfaces`
and `evgl-lib` Zed packages. Existing product bindings are preserved;
missing targets receive a transport-neutral client configuration baseline.
# Evento Globolo client matrix

This Zed package depends on `evento-globolo/evgl-interfaces` and `evento-globolo/evgl-libs` and exposes isolated
targets for Gleam, Erlang, Elixir, Dart, Rust, Java, Go, Python 3, Ruby, PHP, and
TypeScript for Node.js, Deno, Bun, and edge runtimes. Kotlin and Swift are not advertised until this family has a mobile surface.

Run `python3 scripts/validate-client-matrix.py` before publishing.
