---
description: Run full checks (build, test, clippy, fmt)
---

Run a comprehensive check of the codebase:

1. Run `cargo fmt --check` to verify formatting
2. Run `cargo clippy -- -D warnings` to check for linting issues
3. Run `cargo build` to check compilation
4. Run `cargo test` to run all tests

Report any failures and suggest fixes. If everything passes, confirm that the codebase is in good shape.
