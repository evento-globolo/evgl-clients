package evgl_clients

import (
    "context"
    "encoding/json"
    "fmt"
    "net/http"
    "net/url"
    "strings"
)

type Client struct {
    BaseURL *url.URL
    Token string
    HTTP *http.Client
}

type Health struct {
    Status string `json:"status"`
    Service string `json:"service"`
}

func New(baseURL, token string) (*Client, error) {
    parsed, err := url.Parse(baseURL)
    if err != nil { return nil, err }
    return &Client{BaseURL: parsed, Token: token, HTTP: http.DefaultClient}, nil
}

func (c *Client) Health(ctx context.Context) (Health, error) {
    endpoint := *c.BaseURL
    endpoint.Path = strings.TrimRight(endpoint.Path, "/") + "/healthz"
    req, err := http.NewRequestWithContext(ctx, http.MethodGet, endpoint.String(), nil)
    if err != nil { return Health{}, err }
    if c.Token != "" { req.Header.Set("Authorization", "Bearer " + c.Token) }
    response, err := c.HTTP.Do(req)
    if err != nil { return Health{}, err }
    defer response.Body.Close()
    if response.StatusCode >= 300 {
        return Health{}, fmt.Errorf("API request failed: %s", response.Status)
    }
    var health Health
    err = json.NewDecoder(response.Body).Decode(&health)
    return health, err
}
