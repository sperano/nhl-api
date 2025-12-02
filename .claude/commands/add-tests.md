---
description: Add comprehensive tests for a type or function
---

Add comprehensive tests for {{target}} in the codebase.

Steps:
1. Find the implementation of {{target}}
2. Check existing tests to understand coverage gaps
3. Add tests covering:
   - Happy path scenarios
   - Edge cases
   - Error conditions
   - Deserialization with missing/optional fields (for types)
   - Boundary conditions
4. Follow the project's test naming convention: `test_{component}_{scenario}`
5. Use realistic JSON examples matching actual API responses for deserialization tests
6. Run the tests to verify they pass

Ensure tests are thorough but not redundant.
