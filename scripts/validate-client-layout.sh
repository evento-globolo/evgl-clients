#!/usr/bin/env bash
set -Eeuo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"

required=(
  clients/gleam/gleam.toml
  clients/erlang/rebar.config
  clients/elixir/mix.exs
  clients/dart/pubspec.yaml
  clients/rust/Cargo.toml
  clients/wasm/Cargo.toml
  clients/java/pom.xml
  clients/go/go.mod
  clients/python/pyproject.toml
  clients/ruby/evgl_client.gemspec
  clients/php/composer.json
  clients/typescript/package.json
  clients/typescript/nodejs/index.ts
  clients/typescript/deno/mod.ts
  clients/typescript/bun/index.ts
  clients/typescript/edge/index.ts
  clients/kotlin/build.gradle.kts
  clients/swift/Package.swift
)

missing=0
for path in "${required[@]}"; do
  if [[ ! -f "$path" ]]; then
    printf 'missing required client artifact: %s\n' "$path" >&2
    missing=1
  fi
done
((missing == 0)) || exit 1

for dep in \
  '"evento-globolo/evgl-interfaces" = "^0.1.0"' \
  '"evento-globolo/evgl-libs" = "^0.1.0"'; do
  grep -Fq "$dep" .zpkg.toml || { printf 'missing Zed dependency: %s\n' "$dep" >&2; exit 1; }
done

targets=(gleam erlang elixir dart rust rust-wasm java golang python ruby php nodejs kotlin swift)
for target in "${targets[@]}"; do
  grep -Fq "[targets.$target]" .zpkg.toml || { printf 'missing Zed target: %s\n' "$target" >&2; exit 1; }
done

grep -Fq 'dir = ".vendor/.zed"' .zpkg.toml
grep -Fq '.vendor/.zed/**' .zpkg.toml
printf 'client layout and Zed manifest contract validated\n'
