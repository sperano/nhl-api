---
description: Fix all clippy warnings in the codebase
---

Fix all clippy warnings:

1. Run `cargo clippy -- -D warnings` to identify all warnings
2. Fix each warning by:
   - Understanding the issue
   - Applying the recommended fix
   - Ensuring the fix doesn't break functionality
3. Re-run clippy after each batch of fixes
4. If any warnings are intentional, add `#[allow(clippy::warning_name)]` with a comment explaining why

Focus on meaningful improvements, not just suppressing warnings.
