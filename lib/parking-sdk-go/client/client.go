package client

import (
	"bytes"
	"encoding/json"
	"fmt"
	"net/http"
	"net/url"
	"time"
)

type Client struct {
	httpClient *http.Client
	BaseURL    string
}

// NewClient initializes and returns a new Client.
func NewClient(baseUrl string) *Client {
	return &Client{
		httpClient: &http.Client{
			Timeout: time.Second * 10,
		},
		BaseURL: baseUrl,
	}
}

func (c *Client) Get(path string, params url.Values, resObj any) error {
	// Ensure the BaseURL is always correctly joined with the path.
	fullURL, err := url.Parse(c.BaseURL)
	if err != nil {
		return err
	}
	relPath, err := url.Parse(path)
	if err != nil {
		return err
	}
	finalURL := fullURL.ResolveReference(relPath)

	// Append query parameters to the URL.
	finalURL.RawQuery = params.Encode()

	resp, err := c.httpClient.Get(finalURL.String())
	if err != nil {
		return err
	}
	defer resp.Body.Close()

	// Check for non-2xx status codes.
	if resp.StatusCode < 200 || resp.StatusCode >= 300 {
		return fmt.Errorf("API request error: %s", resp.Status)
	}

	if err := json.NewDecoder(resp.Body).Decode(&resObj); err != nil {
		return err
	}

	return nil
}

func (c *Client) Post(path string, reqObj any, resObj any) error {
	// Ensure the BaseURL is always correctly joined with the path.
	fullURL, err := url.Parse(c.BaseURL)
	if err != nil {
		return err
	}
	relPath, err := url.Parse(path)
	if err != nil {
		return err
	}
	finalURL := fullURL.ResolveReference(relPath).String()
	reqBody, err := json.Marshal(reqObj)
	if err != nil {
		return err
	}
	resp, err := c.httpClient.Post(finalURL, "application/json", bytes.NewBuffer(reqBody))
	if err != nil {
		return err
	}
	defer resp.Body.Close()
	// Check for non-2xx status codes.
	if resp.StatusCode < 200 || resp.StatusCode >= 300 {
		return fmt.Errorf("API request error: %s", resp.Status)
	}
	if err := json.NewDecoder(resp.Body).Decode(&resObj); err != nil {
		return err
	}
	return nil
}

func (c *Client) Put(path string, reqObj any, resObj any) error {
	// Ensure the BaseURL is always correctly joined with the path.
	fullURL, err := url.Parse(c.BaseURL)
	if err != nil {
		return err
	}
	relPath, err := url.Parse(path)
	if err != nil {
		return err
	}
	finalURL := fullURL.ResolveReference(relPath).String()
	reqBody, err := json.Marshal(reqObj)
	if err != nil {
		return err
	}
	req, err := http.NewRequest("PUT", finalURL, bytes.NewBuffer(reqBody))
	if err != nil {
		return err
	}
	req.Header.Set("Content-Type", "application/json")
	resp, err := c.httpClient.Do(req)
	if err != nil {
		return err
	}
	defer resp.Body.Close()
	// Check for non-2xx status codes.
	if resp.StatusCode < 200 || resp.StatusCode >= 300 {
		return fmt.Errorf("API request error: %s", resp.Status)
	}
	if err := json.NewDecoder(resp.Body).Decode(&resObj); err != nil {
		return err
	}
	return nil
}

func (c *Client) Delete(path string) error {
	// Ensure the BaseURL is always correctly joined with the path.
	fullURL, err := url.Parse(c.BaseURL)
	if err != nil {
		return err
	}
	relPath, err := url.Parse(path)
	if err != nil {
		return err
	}
	finalURL := fullURL.ResolveReference(relPath).String()
	req, err := http.NewRequest("DELETE", finalURL, nil)
	if err != nil {
		return err
	}
	resp, err := c.httpClient.Do(req)
	if err != nil {
		return err
	}
	defer resp.Body.Close()
	// Check for non-2xx status codes.
	if resp.StatusCode < 200 || resp.StatusCode >= 300 {
		return fmt.Errorf("API request error: %s", resp.Status)
	}
	return nil
}
