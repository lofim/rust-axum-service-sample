# Rust Axum Service Sample

This repository holds a sample service written in Rust & Axum.
The main intend to showcase production ready code with clearly defined architecture.
This service follows the [12factor](https://12factor.net/) methodology.

## Features

- [x] Basic architecture, hexagonal - swappable components
- [x] HTTP routing using Axum
- [x] Configurable logging format, JSON for log aggregation/KVP for local development (noone wants to read raw json logs)
- [ ] Configuration via environment vars (leverage config-rs)
- [x] Error handling (basic)
- [x] Error handling on Axum extractors
- [x] Database layer - [sqlx](https://github.com/launchbadge/sqlx) (SQL)
- [x] Database migrations
- [x] Payload validation [Keats/validator](https://github.com/Keats/validator)
- [ ] Unit testing (basic)
- [ ] Integration testing (Testcontainers)
- [ ] Authentication (middleware JWT, oauth2)
- [ ] Service graceful shutdown
- [ ] Service health probe
- [ ] Service readiness probe
- [ ] Kafka client
- [ ] [Distributed tracing]()

## Tooling

- [x] Linting (cargo clippy works out of the box)
- [x] Formatting (cargo fmt works out of the box)
- [x] Use "Problem Details" standard for API error responses (https://tools.ietf.org/html/rfc7807)
- [ ] [Openapi spec]
- [ ] Gzip responses [tower_http](https://github.com/tower-rs/tower-http)
- [ ] Http client usage [reqwest?]
- [ ] [Circuit Breaker]
- [ ] Docker-compose + depdendencies for local development

## Tool stack

- [Axum(web framework)](https://github.com/tokio-rs/axum)
- [Tracing(logging)](https://docs.rs/tracing/latest/tracing/index.html)
- [http-api-problem(errors)](https://crates.io/crates/http-api-problem)
