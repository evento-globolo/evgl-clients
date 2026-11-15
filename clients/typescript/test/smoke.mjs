import assert from "node:assert/strict";
import { Client } from "../dist/src/index.js";

let observed;
const client = new Client({
  baseUrl: "https://api.example.com/",
  token: "secret",
  fetchImpl: async (input, init) => {
    observed = { input: String(input), init };
    return new Response('{"ok":true}', { status: 200, headers: { "content-type": "application/json" } });
  },
});
assert.deepEqual(await client.health(), { ok: true });
assert.equal(observed.input, "https://api.example.com/healthz");
assert.equal(observed.init.method, "GET");
assert.equal(observed.init.headers.get("authorization"), "Bearer secret");
await client.crossPost("event/1", {
  idempotencyKey: "idem-1",
  targets: [{ provider: "meetup", connection_id: "connection-1", options: {} }],
});
assert.equal(observed.input, "https://api.example.com/v1/events/event%2F1/cross-post");
assert.equal(observed.init.method, "POST");
assert.equal(observed.init.headers.get("idempotency-key"), "idem-1");
assert.equal(client.jobWebSocketUrl("job/1"), "wss://api.example.com/v1/jobs/job%2F1/ws");
console.log("typescript client contract ok");
