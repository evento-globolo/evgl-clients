package client

import (
	"context"
	"io"
	"net/http"
	"strings"
	"testing"
)

type roundTripFunc func(*http.Request) (*http.Response, error)

func (fn roundTripFunc) RoundTrip(request *http.Request) (*http.Response, error) {
	return fn(request)
}

func TestNewNormalizesURL(t *testing.T) {
	client, err := New("https://api.example.com/")
	if err != nil {
		t.Fatal(err)
	}
	if client.BaseURL != "https://api.example.com" {
		t.Fatalf("unexpected URL: %s", client.BaseURL)
	}
}

func TestNewRejectsCleartextRemoteURL(t *testing.T) {
	if _, err := New("http://example.com"); err == nil {
		t.Fatal("expected validation error")
	}
}

func TestCrossPostCarriesAuthAndIdempotency(t *testing.T) {
	client, err := New("https://api.example.com")
	if err != nil {
		t.Fatal(err)
	}
	client.Token = "jwt"
	client.HTTP = &http.Client{Transport: roundTripFunc(func(request *http.Request) (*http.Response, error) {
		if request.URL.EscapedPath() != "/v1/events/event%2F1/cross-post" {
			t.Fatalf("unexpected path: %s", request.URL.EscapedPath())
		}
		if request.Header.Get("Authorization") != "Bearer jwt" {
			t.Fatal("missing bearer token")
		}
		if request.Header.Get("Idempotency-Key") != "idem-1" {
			t.Fatal("missing idempotency key")
		}
		return &http.Response{
			StatusCode: http.StatusAccepted,
			Body:       io.NopCloser(strings.NewReader("{}")),
			Header:     make(http.Header),
		}, nil
	})}
	response, err := client.CrossPost(context.Background(), "event/1", "idem-1", []any{})
	if err != nil {
		t.Fatal(err)
	}
	response.Body.Close()
}
