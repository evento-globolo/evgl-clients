# evento-globolo/evgl-clients#12 — chore: nightly polyglot client hardening

head: automation/nightly-client-hardening  base: main  author: ORESoftware  updated: 2026-08-28T22:26:28Z
dir: /Users/maca5/codes/.claude-fleet/scratch/merge/evento-globolo_evgl-clients__12

## conflicted files
- .zpkg.toml
- clients/.api-surface.sha256
- clients/api-surface.json
- clients/c/.zed-api-surface.sha256
- clients/c/.zed-client-contract.json
- clients/client-api.schema.json
- clients/contract-manifest.json
- clients/cpp/.zed-api-surface.sha256
- clients/cpp/.zed-client-contract.json
- clients/dart/.zed-api-surface.sha256
- clients/dart/.zed-client-contract.json
- clients/elixir/.zed-api-surface.sha256
- clients/elixir/.zed-client-contract.json
- clients/erlang/.zed-api-surface.sha256
- clients/erlang/.zed-client-contract.json
- clients/gleam/.zed-api-surface.sha256
- clients/gleam/.zed-client-contract.json
- clients/golang/.zed-api-surface.sha256
- clients/golang/.zed-client-contract.json
- clients/java/.zed-api-surface.sha256
- clients/java/.zed-client-contract.json
- clients/kotlin/.zed-api-surface.sha256
- clients/kotlin/.zed-client-contract.json
- clients/php/.zed-api-surface.sha256
- clients/php/.zed-client-contract.json
- clients/python/.zed-api-surface.sha256
- clients/python/.zed-client-contract.json
- clients/ruby/.zed-api-surface.sha256
- clients/ruby/.zed-client-contract.json
- clients/rust/.zed-api-surface.sha256
- clients/rust/.zed-client-contract.json
- clients/swift/.zed-api-surface.sha256
- clients/swift/.zed-client-contract.json
- clients/typescript/.zed-contracts/nodejs/.zed-api-surface.sha256
- clients/typescript/.zed-contracts/nodejs/.zed-client-contract.json
- clients/typescript/bun/.zed-api-surface.sha256
- clients/typescript/bun/.zed-client-contract.json
- clients/typescript/deno/.zed-api-surface.sha256
- clients/typescript/deno/.zed-client-contract.json
- clients/typescript/edge/.zed-api-surface.sha256
- clients/typescript/edge/.zed-client-contract.json
- clients/wasm/.zed-api-surface.sha256
- clients/wasm/.zed-client-contract.json
- clients/zig/.zed-api-surface.sha256
- clients/zig/.zed-client-contract.json

## base (main) last 8 commits
a5c18ff feat(validation): consume public lib-core SDKs (#17)
fb029d2 Merge pull request #15 from evento-globolo/agent/den-3583-canonical-client-schema
e157a42 DEN-3583 Adopt the canonical JSON Schema client contract.
f46cba0 Merge branch 'agent-sync/20260823/main' into main
8517f79 ci: add pre-build JS and Rust source lint
42f9b18 ci: add pre-build JS and Rust source lint
8fee270 ci: add pre-build JS and Rust source lint
d6a747f chore: ignore tmp/temp worktree scratch directories

## head (automation/nightly-client-hardening) last 8 commits
b6679ff feat: harden canonical polyglot client contract
081e890 Merge remote:agent/full-polyglot-client-matrix into main with semantic hunk reconciliation
81d6786 Merge remote:agent/polyglot-client-matrix-20260805 into main with semantic hunk reconciliation
a4f85db Merge remote:agent/zed-dependency-graph into main with canonical policy reconciliation
376dff4 Merge remote:agent/standardize-zed-client-matrix-20260805 into main
f201d65 chore: ignore generated client build caches
a196f0d Prefer primary branches and avoid agent worktrees
7a61364 feat(clients): add C, C++, and Zig SDK slices (#10)

## merge-base: 081e890882c8831f12eb3a2ad95682c861d898e5

## PR diff stat (merge-base..head)
 clients/golang/.zed-api-surface.sha256             |   1 +
 clients/golang/.zed-client-contract.json           |  10 +
 clients/java/.zed-api-surface.sha256               |   1 +
 clients/java/.zed-client-contract.json             |  10 +
 clients/kotlin/.zed-api-surface.sha256             |   1 +
 clients/kotlin/.zed-client-contract.json           |  10 +
 clients/php/.zed-api-surface.sha256                |   1 +
 clients/php/.zed-client-contract.json              |  10 +
 clients/python/.zed-api-surface.sha256             |   1 +
 clients/python/.zed-client-contract.json           |  10 +
 clients/ruby/.zed-api-surface.sha256               |   1 +
 clients/ruby/.zed-client-contract.json             |  10 +
 clients/rust/.zed-api-surface.sha256               |   1 +
 clients/rust/.zed-client-contract.json             |  10 +
 clients/sdk-matrix.json                            | 109 ++++
 clients/swift/.zed-api-surface.sha256              |   1 +
 clients/swift/.zed-client-contract.json            |  10 +
 .../.zed-contracts/nodejs/.zed-api-surface.sha256  |   1 +
 .../nodejs/.zed-client-contract.json               |  10 +
 clients/typescript/bun/.zed-api-surface.sha256     |   1 +
 clients/typescript/bun/.zed-client-contract.json   |  10 +
 clients/typescript/deno/.zed-api-surface.sha256    |   1 +
 clients/typescript/deno/.zed-client-contract.json  |  10 +
 clients/typescript/edge/.zed-api-surface.sha256    |   1 +
 clients/typescript/edge/.zed-client-contract.json  |  10 +
 clients/wasm/.zed-api-surface.sha256               |   1 +
 clients/wasm/.zed-client-contract.json             |  10 +
 clients/zig/.zed-api-surface.sha256                |   1 +
 clients/zig/.zed-client-contract.json              |  10 +
 47 files changed, 1716 insertions(+), 30 deletions(-)

## base diff stat (merge-base..base)
 clients/typescript/bun/.zed-api-surface.sha256     |    1 +
 clients/typescript/bun/.zed-client-contract.json   |   10 +
 clients/typescript/deno/.zed-api-surface.sha256    |    1 +
 clients/typescript/deno/.zed-client-contract.json  |   10 +
 clients/typescript/edge/.zed-api-surface.sha256    |    1 +
 clients/typescript/edge/.zed-client-contract.json  |   10 +
 clients/wasm/.zed-api-surface.sha256               |    1 +
 clients/wasm/.zed-client-contract.json             |   10 +
 clients/zig/.zed-api-surface.sha256                |    1 +
 clients/zig/.zed-client-contract.json              |   10 +
 schemas/client-api.schema.json                     |  718 +++++++++
 scripts/check-validation-imports.py                |   27 +
 scripts/client_contract_boundary.py                |  123 ++
 scripts/harden_client_contract.py                  | 1573 ++++++++++++++++++++
 scripts/verify_client_contract.py                  |  279 ++++
 tests/test_client_contract_boundary.py             |   50 +
 validation-consumer/README.md                      |    7 +
 validation-consumer/gleam/gleam.toml               |    9 +
 .../gleam/src/evgl_validation_consumer.gleam       |    6 +
 .../gleam/test/evgl_validation_consumer_test.gleam |   17 +
 validation-consumer/golang/consumer.go             |   23 +
 validation-consumer/golang/consumer_test.go        |   14 +
 validation-consumer/golang/go.mod                  |    7 +
 validation-consumer/rust/Cargo.toml                |   13 +
 validation-consumer/rust/src/lib.rs                |   35 +
 validation-consumer/typescript/package.json        |   17 +
 validation-consumer/typescript/src/index.ts        |   19 +
 .../typescript/test/consumer.test.ts               |   18 +
 validation-consumer/typescript/tsconfig.json       |   15 +
 74 files changed, 4968 insertions(+), 25 deletions(-)

## merge output
Auto-merging .zpkg.toml
CONFLICT (content): Merge conflict in .zpkg.toml
Auto-merging clients/.api-surface.sha256
CONFLICT (add/add): Merge conflict in clients/.api-surface.sha256
Auto-merging clients/api-surface.json
CONFLICT (add/add): Merge conflict in clients/api-surface.json
Auto-merging clients/c/.zed-api-surface.sha256
CONFLICT (add/add): Merge conflict in clients/c/.zed-api-surface.sha256
Auto-merging clients/c/.zed-client-contract.json
CONFLICT (add/add): Merge conflict in clients/c/.zed-client-contract.json
Auto-merging clients/client-api.schema.json
CONFLICT (add/add): Merge conflict in clients/client-api.schema.json
Auto-merging clients/contract-manifest.json
CONFLICT (add/add): Merge conflict in clients/contract-manifest.json
Auto-merging clients/cpp/.zed-api-surface.sha256
CONFLICT (add/add): Merge conflict in clients/cpp/.zed-api-surface.sha256
Auto-merging clients/cpp/.zed-client-contract.json
CONFLICT (add/add): Merge conflict in clients/cpp/.zed-client-contract.json
Auto-merging clients/dart/.zed-api-surface.sha256
CONFLICT (add/add): Merge conflict in clients/dart/.zed-api-surface.sha256
Auto-merging clients/dart/.zed-client-contract.json
CONFLICT (add/add): Merge conflict in clients/dart/.zed-client-contract.json
Auto-merging clients/elixir/.zed-api-surface.sha256
CONFLICT (add/add): Merge conflict in clients/elixir/.zed-api-surface.sha256
Auto-merging clients/elixir/.zed-client-contract.json
CONFLICT (add/add): Merge conflict in clients/elixir/.zed-client-contract.json
Auto-merging clients/erlang/.zed-api-surface.sha256
CONFLICT (add/add): Merge conflict in clients/erlang/.zed-api-surface.sha256
Auto-merging clients/erlang/.zed-client-contract.json
CONFLICT (add/add): Merge conflict in clients/erlang/.zed-client-contract.json
Auto-merging clients/gleam/.zed-api-surface.sha256
CONFLICT (add/add): Merge conflict in clients/gleam/.zed-api-surface.sha256
Auto-merging clients/gleam/.zed-client-contract.json
CONFLICT (add/add): Merge conflict in clients/gleam/.zed-client-contract.json
Auto-merging clients/golang/.zed-api-surface.sha256
CONFLICT (add/add): Merge conflict in clients/golang/.zed-api-surface.sha256
Auto-merging clients/golang/.zed-client-contract.json
CONFLICT (add/add): Merge conflict in clients/golang/.zed-client-contract.json
Auto-merging clients/java/.zed-api-surface.sha256
CONFLICT (add/add): Merge conflict in clients/java/.zed-api-surface.sha256
Auto-merging clients/java/.zed-client-contract.json
CONFLICT (add/add): Merge conflict in clients/java/.zed-client-contract.json
Auto-merging clients/kotlin/.zed-api-surface.sha256
CONFLICT (add/add): Merge conflict in clients/kotlin/.zed-api-surface.sha256
Auto-merging clients/kotlin/.zed-client-contract.json
CONFLICT (add/add): Merge conflict in clients/kotlin/.zed-client-contract.json
Auto-merging clients/php/.zed-api-surface.sha256
CONFLICT (add/add): Merge conflict in clients/php/.zed-api-surface.sha256
Auto-merging clients/php/.zed-client-contract.json
CONFLICT (add/add): Merge conflict in clients/php/.zed-client-contract.json
Auto-merging clients/python/.zed-api-surface.sha256
CONFLICT (add/add): Merge conflict in clients/python/.zed-api-surface.sha256
Auto-merging clients/python/.zed-client-contract.json
CONFLICT (add/add): Merge conflict in clients/python/.zed-client-contract.json
Auto-merging clients/ruby/.zed-api-surface.sha256
CONFLICT (add/add): Merge conflict in clients/ruby/.zed-api-surface.sha256
Auto-merging clients/ruby/.zed-client-contract.json
CONFLICT (add/add): Merge conflict in clients/ruby/.zed-client-contract.json
Auto-merging clients/rust/.zed-api-surface.sha256
CONFLICT (add/add): Merge conflict in clients/rust/.zed-api-surface.sha256
Auto-merging clients/rust/.zed-client-contract.json
CONFLICT (add/add): Merge conflict in clients/rust/.zed-client-contract.json
Auto-merging clients/swift/.zed-api-surface.sha256
CONFLICT (add/add): Merge conflict in clients/swift/.zed-api-surface.sha256
Auto-merging clients/swift/.zed-client-contract.json
CONFLICT (add/add): Merge conflict in clients/swift/.zed-client-contract.json
Auto-merging clients/typescript/.zed-contracts/nodejs/.zed-api-surface.sha256
CONFLICT (add/add): Merge conflict in clients/typescript/.zed-contracts/nodejs/.zed-api-surface.sha256
Auto-merging clients/typescript/.zed-contracts/nodejs/.zed-client-contract.json
CONFLICT (add/add): Merge conflict in clients/typescript/.zed-contracts/nodejs/.zed-client-contract.json
Auto-merging clients/typescript/bun/.zed-api-surface.sha256
CONFLICT (add/add): Merge conflict in clients/typescript/bun/.zed-api-surface.sha256
Auto-merging clients/typescript/bun/.zed-client-contract.json
CONFLICT (add/add): Merge conflict in clients/typescript/bun/.zed-client-contract.json
Auto-merging clients/typescript/deno/.zed-api-surface.sha256
CONFLICT (add/add): Merge conflict in clients/typescript/deno/.zed-api-surface.sha256
Auto-merging clients/typescript/deno/.zed-client-contract.json
CONFLICT (add/add): Merge conflict in clients/typescript/deno/.zed-client-contract.json
Auto-merging clients/typescript/edge/.zed-api-surface.sha256
CONFLICT (add/add): Merge conflict in clients/typescript/edge/.zed-api-surface.sha256
Auto-merging clients/typescript/edge/.zed-client-contract.json
CONFLICT (add/add): Merge conflict in clients/typescript/edge/.zed-client-contract.json
Auto-merging clients/wasm/.zed-api-surface.sha256
CONFLICT (add/add): Merge conflict in clients/wasm/.zed-api-surface.sha256
Auto-merging clients/wasm/.zed-client-contract.json
CONFLICT (add/add): Merge conflict in clients/wasm/.zed-client-contract.json
Auto-merging clients/zig/.zed-api-surface.sha256
CONFLICT (add/add): Merge conflict in clients/zig/.zed-api-surface.sha256
Auto-merging clients/zig/.zed-client-contract.json
CONFLICT (add/add): Merge conflict in clients/zig/.zed-client-contract.json
Automatic merge failed; fix conflicts and then commit the result.
