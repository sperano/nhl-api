---
description: Add a new NHL API endpoint with types and client method
---

Add support for the {{endpoint_name}} NHL API endpoint.

Steps:
1. Research the endpoint structure by checking NHL API documentation or example responses
2. Create appropriate types in `src/types/` (or add to existing files)
3. Add the client method to `src/client.rs`
4. Add HTTP handling in `src/http_client.rs` if needed
5. Write comprehensive tests for deserialization
6. Add integration test examples

Follow the existing patterns:
- Use serde with `#[serde(rename = "camelCase")]` for field names
- Use `Option<T>` for fields that may be missing
- Derive `Debug, Clone, Serialize, Deserialize, PartialEq`
- All client methods should be async and return `Result<T, NHLApiError>`
- Add doc comments with endpoint URLs and examples
