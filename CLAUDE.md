# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Test Commands

```bash
cargo build                    # Build
cargo test                     # Run all tests
cargo test <module_name>       # Run tests for a specific module (e.g. cargo test api)
cargo clippy -- -D warnings    # Lint (treat warnings as errors)
cargo install --path .         # Install binary as `gbpcli`
```

## Environment

- Rust 1.92 (pinned via `.mise.toml`), edition 2024
- Edition 2024 makes `env::set_var`/`remove_var` unsafe — avoid mutating env vars in tests; use struct construction instead
- Binary name is `gbpcli` (defined in `[[bin]]` section of Cargo.toml), package name is `gbpcli_rs`
- Requires `.env` file with: `GOOGLE_BUSINESS_API_CLIENT_ID`, `GOOGLE_BUSINESS_API_CLIENT_SECRET`, `GOOGLE_BUSINESS_API_REFRESH_TOKEN`

## Architecture

CLI wrapping the Google Business Profile API. Flow: load `.env` → refresh OAuth token → call API → print JSON.

- **main.rs** — CLI definition (clap derive) and orchestration. Subcommands defined in `Commands` enum. Each subcommand loads config, refreshes token, calls API, prints result.
- **config.rs** — `Config::from_env()` reads the three OAuth env vars.
- **auth.rs** — `refresh_access_token()` posts to `https://oauth2.googleapis.com/token` with refresh_token grant type.
- **api.rs** — API client functions and response types. Currently implements `accounts.list` (`GET https://mybusinessaccountmanagement.googleapis.com/v1/accounts`). All response structs use `Option<T>` fields with `#[serde(rename_all = "camelCase")]`.

When adding new API endpoints, follow the pattern in `api.rs`: define response structs with serde, add an async function taking `&reqwest::Client` + `&str` access token, and wire it up as a new clap subcommand in `main.rs`.
