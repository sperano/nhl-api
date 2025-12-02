---
description: Research NHL API endpoint structure (e.g., /api-research /v1/roster/BOS/current)
---

Research the NHL API endpoint: {{endpoint_path}}

Steps:
1. Fetch the endpoint using curl: `https://api-web.nhle.com{{endpoint_path}}`
2. Analyze the JSON response structure
3. Identify all fields and their types
4. Note any optional fields or variations
5. Suggest Rust struct definitions with appropriate serde attributes
6. Recommend where to add this in the codebase (new file vs existing)
7. Suggest a good client method name and signature

This helps plan implementation before writing code.
