# Workflows

## audit

This crate is a library, so downstream builds never read its `Cargo.lock`. The audit runs `cargo update` first and
then audits the result. A failure means an advisory survives the newest versions our `Cargo.toml` ranges allow, so
fixing it takes a manifest bump here and a release. Lockfile-only fixes belong to the downstream repos' own audits.

It runs `cargo audit` directly rather than `rustsec/audit-check`, so it opens no issues. A failed scheduled run
notifies by email instead.
