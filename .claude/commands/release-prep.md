---
description: Prepare for a new release (version bump, changelog, tests)
---

Prepare for releasing version {{version}}:

1. Update version in `Cargo.toml`
2. Run full test suite (`cargo test`)
3. Run `cargo clippy -- -D warnings`
4. Run `cargo fmt`
5. Check `README.md` for accuracy
6. Review public API changes since last release
7. Update CHANGELOG.md with:
   - New features
   - Bug fixes
   - Breaking changes
   - Deprecations
8. Verify examples still work
9. Check that all public items have documentation
10. Suggest git commands for tagging the release

Provide a checklist of what's ready and what needs attention.
