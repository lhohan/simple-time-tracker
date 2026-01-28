# Add CORS Configuration

**Severity**: HIGH | **Effort**: Easy | **Category**: Security

## Problem

No CORS headers or middleware configured. All API endpoints accept cross-origin requests by default.

## Files Affected

- `src/web/server.rs` (router configuration)

## Risk

Malicious websites can make requests to the API if exposed beyond localhost.

## Solution

Add tower-http CORS middleware:

```rust
use tower_http::cors::{CorsLayer, Any};

let cors = CorsLayer::new()
    .allow_origin(["http://localhost:3000".parse().unwrap()])
    .allow_methods([Method::GET]);

Router::new()
    // routes...
    .layer(cors)
```

## Acceptance Criteria

- [ ] CORS middleware added to router
- [ ] Only localhost origins allowed by default
- [ ] Cross-origin requests from other domains rejected
- [ ] `just test-web` passes
