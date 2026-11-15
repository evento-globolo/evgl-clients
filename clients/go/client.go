package client

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"net/url"
	"strings"
)

type Client struct {
	BaseURL string
	Token   string
	HTTP    *http.Client
}

func New(baseURL string) (*Client, error) {
	baseURL = strings.TrimRight(strings.TrimSpace(baseURL), "/")
	if !strings.HasPrefix(baseURL, "https://") && !strings.HasPrefix(baseURL, "http://localhost") {
		return nil, fmt.Errorf("base URL must use HTTPS or localhost HTTP")
	}
	return &Client{BaseURL: baseURL, HTTP: http.DefaultClient}, nil
}

func (c *Client) Health(ctx context.Context) (*http.Response, error) {
	return c.Do(ctx, http.MethodGet, "/healthz", nil)
}

func (c *Client) EmitEvent(ctx context.Context, payload any) (*http.Response, error) {
	return c.Do(ctx, http.MethodPost, "/api/events", payload)
}

func (c *Client) Providers(ctx context.Context) (*http.Response, error) {
	return c.Do(ctx, http.MethodGet, "/v1/providers", nil)
}

func (c *Client) Connections(ctx context.Context) (*http.Response, error) {
	return c.Do(ctx, http.MethodGet, "/v1/connections", nil)
}

func (c *Client) StartOAuth(ctx context.Context, provider string) (*http.Response, error) {
	return c.Do(ctx, http.MethodPost, "/v1/oauth/"+url.PathEscape(provider)+"/start", struct{}{})
}

func (c *Client) Events(ctx context.Context) (*http.Response, error) {
	return c.Do(ctx, http.MethodGet, "/v1/events", nil)
}

func (c *Client) CreateEvent(ctx context.Context, payload any) (*http.Response, error) {
	return c.Do(ctx, http.MethodPost, "/v1/events", payload)
}

func (c *Client) Job(ctx context.Context, jobID string) (*http.Response, error) {
	return c.Do(ctx, http.MethodGet, "/v1/jobs/"+url.PathEscape(jobID), nil)
}

func (c *Client) CrossPost(
	ctx context.Context,
	eventID string,
	idempotencyKey string,
	targets any,
) (*http.Response, error) {
	if idempotencyKey == "" || len(idempotencyKey) > 200 {
		return nil, fmt.Errorf("idempotency key must contain 1..=200 bytes")
	}
	return c.DoWithHeaders(
		ctx,
		http.MethodPost,
		"/v1/events/"+url.PathEscape(eventID)+"/cross-post",
		map[string]any{"targets": targets},
		http.Header{"Idempotency-Key": []string{idempotencyKey}},
	)
}

func (c *Client) JobWebSocketURL(jobID string) (string, error) {
	endpoint, err := url.Parse(c.BaseURL)
	if err != nil {
		return "", err
	}
	if endpoint.Scheme == "https" {
		endpoint.Scheme = "wss"
	} else {
		endpoint.Scheme = "ws"
	}
	endpoint.Path = "/v1/jobs/" + url.PathEscape(jobID) + "/ws"
	return endpoint.String(), nil
}

func (c *Client) Do(ctx context.Context, method, path string, payload any) (*http.Response, error) {
	return c.DoWithHeaders(ctx, method, path, payload, nil)
}

func (c *Client) DoWithHeaders(
	ctx context.Context,
	method string,
	path string,
	payload any,
	extraHeaders http.Header,
) (*http.Response, error) {
	var body io.Reader
	if payload != nil {
		encoded, err := json.Marshal(payload)
		if err != nil {
			return nil, err
		}
		body = bytes.NewReader(encoded)
	}
	req, err := http.NewRequestWithContext(ctx, method, c.BaseURL+path, body)
	if err != nil {
		return nil, err
	}
	req.Header.Set("Accept", "application/json")
	if payload != nil {
		req.Header.Set("Content-Type", "application/json")
	}
	if c.Token != "" {
		req.Header.Set("Authorization", "Bearer "+c.Token)
	}
	for name, values := range extraHeaders {
		for _, value := range values {
			req.Header.Add(name, value)
		}
	}
	return c.HTTP.Do(req)
}
