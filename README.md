# SigmaOS Access Log

This crate is currently a minimal `no_std` component identity scaffold. Its
only behavior is a `Component` value with a fixed name; it does not read,
write, rotate, or inspect access logs.

Before adding log handling, define which events are recorded, access controls,
retention, redaction, rotation, and failure behavior. Logging must not expose
credentials or sensitive identifiers by default.

## Development

The crate has no third-party dependencies. `cargo check --lib` checks the
scaffold, not an access-log implementation. Its manifest declares Apache-2.0.
