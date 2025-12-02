---
description: Add or run benchmarks for performance-critical code
---

Work with benchmarks for {{target}}:

1. Check if benchmarks exist in `benches/` directory
2. If creating new benchmarks:
   - Set up criterion benchmark harness
   - Add realistic test cases
   - Measure relevant operations (deserialization, API calls, etc.)
3. If running benchmarks:
   - Execute `cargo bench`
   - Analyze results
   - Compare with previous runs if available
4. Suggest optimization opportunities based on results

Focus on meaningful performance metrics.
