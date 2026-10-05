# alguna

Rust SDK for the [Alguna API](https://alguna.com/docs): usage-based billing,
subscriptions, invoicing and payments.

## Install

```bash
cargo add alguna
cargo add tokio --features macros,rt-multi-thread
```

The crate uses `rustls` for TLS. It needs an async runtime such as `tokio`.

## Client

Create an API key in the dashboard under **Settings → Connections → Developers → API
keys**. A key looks like `<key_id>.<key_secret>`. The key is a secret, so use the SDK on
a server only.

```rust
use alguna::prelude::*;
use alguna::ApiClientBuilder;

fn clients() -> Result<(), ApiError> {
    // Reads the key from the ALGUNA_API_KEY environment variable.
    let from_env = ApiClientBuilder::default().build()?;
    let explicit = ApiClientBuilder::default()
        .api_key("<key_id>.<key_secret>")
        .build()?;
    let from_struct = Alguna::new(ClientConfig {
        api_key: Some("<key_id>.<key_secret>".to_string()),
        ..Default::default()
    })?;
    Ok(())
}
```

If there is no key, `build()` and `Alguna::new` return `ApiError::Configuration` and send
nothing.

| Setting     | Builder method   | Default                                                    |
| ----------- | ---------------- | ---------------------------------------------------------- |
| API key     | `api_key`        | the `ALGUNA_API_KEY` environment variable                  |
| API version | `api_version`    | `2026-04-01`, the version this release targets |
| Base URL    | `new(url)`       | `https://api.alguna.io`                                    |
| Timeout     | `timeout`        | 60 seconds for each attempt                                |
| Retries     | `max_retries`    | 2                                                          |
| HTTP client | `reqwest_client` | one built from the timeout and user agent                  |

The SDK sends the key as `Authorization: Bearer <key>`. Every request carries an
`Alguna-Version` header, so the response types match the API. For the sandbox, use
`ApiClientBuilder::new("https://api.sandbox.alguna.io")`.

## Requests

Each resource is a field of the client: `client.customers`, `client.invoices` and so
on. A method takes its request and an optional `RequestOptions`, which overrides the
client settings for that call. Every list filter is optional.

```rust
use alguna::prelude::*;

async fn requests(client: &Alguna) -> Result<(), ApiError> {
    let request = CreateCustomerRequest {
        name: "Acme".to_string(),
        currency: "USD".to_string(),
        ..Default::default()
    };
    let customer = client.customers.create(&request, None).await?;
    println!("{}", customer.id);

    let filter = ProductsListQueryRequest {
        fee_type: Some(vec![ListProductsRequestFeeTypeItem::Metered]),
        ..Default::default()
    };
    let options = RequestOptions::new().api_version("2026-04-01");
    for product in client.products.list(&filter, Some(options)).await?.data {
        println!("{}", product.name);
    }
    Ok(())
}
```

## Pagination

`list` returns one page. `list_paginated` returns a `Stream` of every item. It loads the
next page when the stream needs it, from `offset` (default 0). Add `futures = "0.3"` for
`StreamExt`.

```rust
use alguna::prelude::*;
use futures::StreamExt;

async fn every_customer(client: &Alguna) -> Result<(), ApiError> {
    let mut customers = client
        .customers
        .list_paginated(&CustomersListQueryRequest::default(), None)
        .await?;
    while let Some(customer) = customers.next().await {
        let customer: CustomerResponse = customer?;
        println!("{}", customer.name);
    }
    Ok(())
}
```

## Idempotency and retries

Every request other than a GET carries an `Idempotency-Key` header. The SDK makes a
UUID v4 key for each call and sends the same key on every retry of that call, so the
API acts once. To make a call safe to repeat later, for example after your process
stops, set your own key:

```rust
use alguna::prelude::*;

async fn create_once(client: &Alguna, request: &CreateCustomerRequest) -> Result<(), ApiError> {
    let options = RequestOptions::new().idempotency_key("create-customer-acme");
    client.customers.create(request, Some(options)).await?;
    Ok(())
}
```

`client.events.ingest`, `client.credits.check` and `client.credits.track` are different.
The API identifies their work by the request body, so the SDK makes no key for them and
still retries them. The API removes duplicate events by `unique_id`, and
`credits.track` draws credit once for the same `unique_id`s. A key replaces that
identity on `credits.track`, so a new key on each call would make a redelivered batch
draw credit twice. A key that you set is sent on every attempt. `credits.check` with
`reserve` needs your key, because the key identifies the hold.

By default the SDK retries a call 2 times after:

- a network error: the connection is refused, reset or closed before the response, or
  the attempt times out;
- a 408, 429, 502, 503 or 504 response;
- a 409 response with `Retry-After`. The API sends it while a request with the same
  idempotency key is still running. Any other 409 is a conflict, and the SDK does not
  retry it.

A retry waits 1 second, then 2 seconds, and so on, each ±10%. If the response has
`Retry-After` (seconds or an HTTP date), the retry waits that long, up to 60 seconds.
Set `max_retries` on the builder or on `RequestOptions`. 0 turns retries off.

To stop a call, drop its future, for example with `tokio::time::timeout`. A dropped
call makes no more attempts.

## Timeouts

`timeout` on the builder limits each attempt. The default is 60 seconds.
`timeout_seconds` on `RequestOptions` replaces it for one call. A custom
`reqwest_client` keeps its own timeout.

```rust
use alguna::prelude::*;
use alguna::ApiClientBuilder;
use std::time::Duration;

async fn tuned() -> Result<(), ApiError> {
    let client = ApiClientBuilder::default()
        .timeout(Duration::from_secs(30))
        .max_retries(4)
        .build()?;
    let options = RequestOptions::new().timeout_seconds(10).max_retries(0);
    client.customers.get("cus_123", Some(options)).await?;
    Ok(())
}
```

## Errors

Every call returns `Result<_, ApiError>`. The enum is `#[non_exhaustive]`, so keep a
wildcard arm. An error prints the API's reason: the body's `detail`, else its `title`,
else its `message`, else the HTTP status text. For example:
`NotFoundError: Resource not found - customer cus_1 not found`.

```rust
use alguna::prelude::*;

async fn find(client: &Alguna, id: &str) -> Result<Option<CustomerResponse>, ApiError> {
    match client.customers.get(id, None).await {
        Ok(customer) => Ok(Some(customer)),
        Err(ApiError::NotFoundError { .. }) => Ok(None),
        Err(error) => {
            // status(): the HTTP status, if the API answered. detail(): its reason.
            eprintln!(
                "{:?} {:?} timeout={}",
                error.status(),
                error.detail(),
                error.is_timeout()
            );
            Err(error)
        }
    }
}
```

A network failure is `ApiError::Network`. Its cause is the error's `source()`.

## Webhooks

Alguna sends webhooks through Svix. Verify the signature before you trust a payload:
give the raw request body and its `svix-id`, `svix-timestamp` and `svix-signature`
headers to the [`svix`](https://docs.rs/svix) crate
(`Webhook::new(secret)?.verify(body, &headers)?`). Then read the event. Each event
`type` has a payload type in this crate, such as `WebhookPaymentCreatedPayload` for
`payment.created`. The example also needs `serde_json`.

```rust
use alguna::prelude::*;

// `body` is the raw request body, after its signature is verified.
fn handle(body: &[u8]) -> Result<(), serde_json::Error> {
    let event: Value = serde_json::from_slice(body)?;
    match event["type"].as_str() {
        Some("payment.created") => {
            let event: WebhookPaymentCreatedPayload = serde_json::from_value(event)?;
            println!("payment {}", event.data.amount);
        }
        Some("invoice.paid") => {
            let event: WebhookInvoicePaidPayload = serde_json::from_value(event)?;
            println!("invoice {} paid", event.data.id);
        }
        // Alguna can add event types before you upgrade.
        _ => {}
    }
    Ok(())
}
```

## Reference

Every endpoint and type is in [reference.md](reference.md). The API documentation is at
[alguna.com/docs](https://alguna.com/docs).

## About this repo

Alguna generates this crate from the Alguna API spec in a private repository. Open
issues here. Alguna ports code changes to that repository, so a pull request here is
not merged as it is.
