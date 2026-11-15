export class EvglClient {
  constructor({ baseUrl, token, fetchImpl = fetch, WebSocketImpl = WebSocket }) {
    this.baseUrl = new URL(baseUrl);
    this.token = token;
    this.fetchImpl = fetchImpl;
    this.WebSocketImpl = WebSocketImpl;
  }

  providers() { return this.#request("GET", "/v1/providers"); }
  connections() { return this.#request("GET", "/v1/connections"); }
  startOAuth(provider) {
    return this.#request("POST", `/v1/oauth/${encodeURIComponent(provider)}/start`, {});
  }
  createEvent(event) { return this.#request("POST", "/v1/events", event); }
  job(jobId) { return this.#request("GET", `/v1/jobs/${encodeURIComponent(jobId)}`); }

  crossPost(eventId, { targets, idempotencyKey }) {
    return this.#request("POST", `/v1/events/${encodeURIComponent(eventId)}/cross-post`,
      { targets }, { "idempotency-key": idempotencyKey });
  }

  watchJob(jobId, onUpdate) {
    const url = new URL(`/v1/jobs/${encodeURIComponent(jobId)}/ws`, this.baseUrl);
    url.protocol = url.protocol === "https:" ? "wss:" : "ws:";
    const socket = new this.WebSocketImpl(url, []);
    socket.addEventListener("open", () => {
      // Browser WebSockets cannot set Authorization headers. Deployments should use
      // the same-site MASH proxy or an ephemeral WS ticket endpoint.
    });
    socket.addEventListener("message", (event) => onUpdate(JSON.parse(event.data)));
    return socket;
  }

  async #request(method, path, body, extraHeaders = {}) {
    const response = await this.fetchImpl(new URL(path, this.baseUrl), {
      method,
      headers: {
        authorization: `Bearer ${this.token}`,
        ...(body === undefined ? {} : { "content-type": "application/json" }),
        ...extraHeaders
      },
      body: body === undefined ? undefined : JSON.stringify(body)
    });
    const text = await response.text();
    const parsed = text ? JSON.parse(text) : null;
    if (!response.ok) {
      throw new Error(`Evento Globolo API ${response.status}: ${text}`);
    }
    return parsed;
  }
}
