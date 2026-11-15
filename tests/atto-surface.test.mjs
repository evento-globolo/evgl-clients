import assert from "node:assert/strict";
import test from "node:test";
import { readFile } from "node:fs/promises";

const root = new URL("../", import.meta.url);
const source = await readFile(new URL("clients/atto/src/routes.atto", root), "utf8");
const matrix = JSON.parse(await readFile(new URL("clients/matrix.json", root), "utf8"));
const manifest = await readFile(new URL(".zpkg.toml", root), "utf8");

test("AttoLang is a canonical, discoverable client target", () => {
  assert.deepEqual(matrix.languages.attolang, {
    dir: "clients/atto",
    sentinel: "src/routes.atto",
  });
  assert.match(manifest, /\[targets\.atto\]\s+dir = "clients\/atto"/);
});

test("pure Atto path helpers match the existing SDK routes", () => {
  const declarations = new Map(
    [...source.matchAll(/pub fnc (\w+_path)\(\): string \{ return "([^"]+)"; \}/g)]
      .map(([, name, route]) => [name, route]),
  );
  assert.deepEqual(Object.fromEntries(declarations), {
    health_path: "/healthz",
    config_path: "/api/config",
    emit_event_path: "/api/events",
    emit_alert_path: "/api/alerts",
    providers_path: "/v1/providers",
    connections_path: "/v1/connections",
    list_events_path: "/v1/events",
    create_event_path: "/v1/events",
  });
  assert.ok([...declarations.values()].every(path => path.startsWith("/") && !path.includes("..")));
});

test("Atto authorization helper documents the transport boundary", () => {
  assert.match(source, /pub fnc authorization_value\(string token\): string \{ return "Bearer " \+ token; \}/);
  assert.match(source, /Consumers must validate destination origins/);
  assert.doesNotMatch(source, /pub fnc (?:job|cross_post|start_oauth)_path\(/);
});
