import test from "node:test";
import assert from "node:assert/strict";
import { EvglClient } from "../src/index.js";

test("cross-post sends JWT and idempotency key", async () => {
  const seen = [];
  const client = new EvglClient({
    baseUrl: "https://api.example.test",
    token: "jwt",
    WebSocketImpl: class {},
    fetchImpl: async (url, init) => {
      seen.push({ url: String(url), init });
      return new Response(JSON.stringify({ id: "job-1" }), {
        status: 202, headers: { "content-type": "application/json" }
      });
    }
  });
  await client.crossPost("event-1", {
    idempotencyKey: "idem-1",
    targets: [{ provider: "meetup", connection_id: "connection-1", options: {} }]
  });
  assert.equal(seen[0].init.headers.authorization, "Bearer jwt");
  assert.equal(seen[0].init.headers["idempotency-key"], "idem-1");
});
