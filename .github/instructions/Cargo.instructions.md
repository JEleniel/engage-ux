---
description: 'Agent directives for Cargo.toml metadata, dependencies, and package configuration.'
applyTo: '**/Cargo.toml'
---

# Cargo.toml Best Practices

- Prefer `mcp_cargo-mcp_*` for Cargo operations when available.
- If unavailable, use the standard `cargo` CLI.

## Application Metadata

- For applications targeting Apple or mobile packaging toolchains that require an application identifier, include a single reverse-DNS app ID in `Cargo.toml` as package/workspace metadata (not a top-level Cargo key).
- Only add this metadata when a packaging or bundling toolchain will consume it.
- Use a domain you control for reverse-DNS identifiers.
- The `crystultima` namespace below is an example owned by the repo owner, not a required metadata key. Rename it to match your project or toolchain.
- For package-level metadata:

```toml
[package.metadata.crystultima]
app_id = "org.crystultima.<package_name>"
```

- If the app ID is workspace-wide:

```toml
[workspace.metadata.crystultima]
app_id = "org.crystultima.<workspace_name>"
```

## Core Settings

- Set `edition` to the current stable value supported by the repo or toolchain (`2024` at the time of writing) and declare `rust-version` (MSRV).
- Include a clear `description`.
- Include `license`, `repository`, and `readme` if those are available.
- Do not use the deprecated `authors` field.
- Use SPDX license identifiers. Default to MIT/Apache dual licensing.
- Use explicit semver requirements; never `"*"`. Prefer not to pin the patch number unless a specific fix or compatibility need requires it.
- Minimize dependency count; separate `dev-` and `build-` dependencies.
- Disable defaults when not required: `default-features = false`.
- Features must be additive only; never change existing behavior.
- Gate optional/heavy deps behind features.
- Keep `default` small and broadly usable.
- Avoid git dependencies in releases.
- Avoid `[patch]` except as a temporary override.
- Centralize versions in `[workspace.dependencies]`.
- Keep dependency scope as narrow as possible. Promote dependencies to workspace only if they are actually shared.
- Tune release builds intentionally (`lto`, `codegen-units`, `panic = "abort"` when appropriate).
- Optimize `[profile.dev]` for faster iteration.
- Regularly audit dependencies.
- Avoid hidden behavior in defaults or build scripts.

## Approved libraries

The following libraries are approved for use. Sublibraries include crates that share the parent prefix or are designed as companions.
This section governs dependency allowance and selection only; language-specific instruction files define how approved crates are used.

- `anyhow`, `thiserror` for error handling
- `axum` (and sublibraries), `tower` (and sublibraries), `hyper` (and sublibraries) for web servers
- `base64`, `hex`, `num-traits`, `regex`, `unicode-normalization`, `uuid` for utilities
- `chrono` for time and date handling
- `clap` for CLI interfaces
- `config` for configuration file handling
- `ctrlc` for signal handling
- `dirs` (preferred) or `directories` for standard config/data/cache directories
- `fern` for logging
- `log` for logging API
- `r2d2`, `r2d2_sqlite`, `rusqlite` for SQLite (use `rusqlite` with the `bundled` feature)
- `reqwest` for HTTP client calls
- `serde` (and sublibraries), `serde_json` for serialization
- `tokio` (and sublibraries) for async runtime
- `url`, `urlencoding` for URL handling
- `sha2`, and `hmac` for hashing
