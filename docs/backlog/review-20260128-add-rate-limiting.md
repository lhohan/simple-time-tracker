# Add Rate Limiting

**Severity**: MEDIUM | **Effort**: Easy | **Category**: Security

## Problem

No rate limiting middleware. Unlimited requests accepted from single client.

## Files Affected

- `src/web/server.rs` (router configuration)
- `Cargo.toml` (new dependency)

## Risk

- DoS attacks
- Resource exhaustion
- Brute-force on any future authentication

## Solution

Add tower rate limiting middleware:

```rust
use tower_governor::{GovernorLayer, GovernorConfigBuilder};

let governor_conf = GovernorConfigBuilder::default()
    .per_second(10)
    .burst_size(50)
    .finish()
    .unwrap();

Router::new()
    // routes...
    .layer(GovernorLayer::new(&governor_conf))
```

## Acceptance Criteria

- [ ] Rate limiting middleware added
- [ ] Reasonable limits configured (e.g., 10 req/sec, burst 50)
- [ ] 429 Too Many Requests returned when exceeded
- [ ] `just test-web` passes
