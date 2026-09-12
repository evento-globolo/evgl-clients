export type FetchLike = (input: string | URL | Request, init?: RequestInit) => Promise<Response>;

export interface ClientOptions {
  baseUrl: string;
  token?: string;
  fetchImpl?: FetchLike;
}

export interface CrossPostOptions {
  targets: unknown[];
  idempotencyKey: string;
}

export class ClientError extends Error {
  constructor(public readonly status: number, public readonly responseBody: string) {
    super(`HTTP ${status}: ${responseBody}`);
    this.name = "ClientError";
  }
}

export class Client {
  readonly baseUrl: string;
  readonly token?: string;
  readonly fetchImpl: FetchLike;

  constructor(options: ClientOptions) {
    this.baseUrl = options.baseUrl.replace(/\/+$/, "");
    this.token = options.token;
    const fetchImpl = options.fetchImpl ?? globalThis.fetch?.bind(globalThis);
    if (!fetchImpl) throw new Error("A Fetch API implementation is required");
    this.fetchImpl = fetchImpl;
  }

  health(): Promise<unknown> { return this.request("GET", "/healthz"); }
  getConfig(): Promise<unknown> { return this.request("GET", "/api/config"); }
  emitEvent(payload: unknown): Promise<unknown> { return this.request("POST", "/api/events", payload); }
  emitAlert(payload: unknown): Promise<unknown> { return this.request("POST", "/api/alerts", payload); }
  providers(): Promise<unknown> { return this.request("GET", "/v1/providers"); }
  connections(): Promise<unknown> { return this.request("GET", "/v1/connections"); }
  startOAuth(provider: string): Promise<unknown> {
    return this.request("POST", `/v1/oauth/${encodeURIComponent(provider)}/start`, {});
  }
  events(): Promise<unknown> { return this.request("GET", "/v1/events"); }
  createEvent(payload: unknown): Promise<unknown> {
    return this.request("POST", "/v1/events", payload);
  }
  job(jobId: string): Promise<unknown> {
    return this.request("GET", `/v1/jobs/${encodeURIComponent(jobId)}`);
  }
  crossPost(eventId: string, options: CrossPostOptions): Promise<unknown> {
    return this.request(
      "POST",
      `/v1/events/${encodeURIComponent(eventId)}/cross-post`,
      { targets: options.targets },
      { "idempotency-key": options.idempotencyKey },
    );
  }
  jobWebSocketUrl(jobId: string): string {
    const url = new URL(`/v1/jobs/${encodeURIComponent(jobId)}/ws`, `${this.baseUrl}/`);
    url.protocol = url.protocol === "https:" ? "wss:" : "ws:";
    return url.toString();
  }

  async request(
    method: string,
    path: string,
    payload?: unknown,
    extraHeaders: Record<string, string> = {},
  ): Promise<unknown> {
    const headers = new Headers({ accept: "application/json" });
    if (this.token) headers.set("authorization", `Bearer ${this.token}`);
    for (const [name, value] of Object.entries(extraHeaders)) headers.set(name, value);
    let body: string | undefined;
    if (payload !== undefined) {
      headers.set("content-type", "application/json");
      body = JSON.stringify(payload);
    }
    const response = await this.fetchImpl(`${this.baseUrl}${path}`, { method, headers, body });
    const text = await response.text();
    if (!response.ok) throw new ClientError(response.status, text);
    if (!text) return undefined;
    const contentType = response.headers.get("content-type") ?? "";
    return contentType.includes("json") ? JSON.parse(text) : text;
  }
}
