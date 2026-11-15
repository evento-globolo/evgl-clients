# AttoLang client surface (preview)

The source file [`src/routes.atto`](src/routes.atto) adds pure, snake_case
AttoLang route helpers for the established Evento Globolo SDK endpoints.
It makes AttoLang a discoverable source target in the Zed manifest and
machine-readable client matrix.

The functions return **path literals** or assemble a bearer authorization
value. They do **not** perform HTTP requests, validate URL origins,
validate tokens, encode JSON, or implement retries. Application code must
use an independently reviewed transport and never log credentials.
Dynamic identifier-based routes (OAuth provider, job ID, cross-post event)
are intentionally omitted until path-segment validation can be enforced.

The authoritative event wire schema and AttoLang event field binding live
in `evento-globolo/evgl-interfaces`. The tests here verify source and
manifest consistency only, not native/wasm compilation or execution.
Atto compiler/backend parity needs separate verified runs.
