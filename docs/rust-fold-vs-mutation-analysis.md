# Fold vs Mutation in Rust: Performance Analysis

## Summary

**Key Finding**: Rust does NOT have efficient immutable data structures like Scala/Haskell. The functional `fold` pattern with standard collections creates O(N²) performance due to expensive deep clones, while mutable accumulation is both faster and idiomatic.

While functional patterns like fold are elegant, they can become performance traps if the accumulator is expensive to clone. In performance-sensitive Rust, a well-structured imperative loop that mutates state is often the most efficient and readable solution. You correctly identified that this was not a place for functional purity at the cost of millions of allocations.

## The Question

When facing O(N²) performance in a parser using `fold` with cloning, should we:
1. Replace with mutable accumulation, or
2. Use immutable data structures to keep the `fold` approach?

## Analysis

### Why Fold Works in Functional Languages

```scala
// Scala - Immutable collections with structural sharing
entries.foldLeft(Map.empty[Date, List[Entry]]) { (acc, entry) =>
  acc.updated(entry.date, entry :: acc.getOrElse(entry.date, Nil))
  // ↑ O(log n) update due to structural sharing
}
```

**Scala/Haskell advantages:**
- Persistent data structures with structural sharing
- "Updates" reuse existing structure (O(log n) instead of O(n))
- Immutability is the primary paradigm
- Language/runtime optimized for this pattern

### Why Fold Fails in Rust

```rust
// Current problematic Rust code
content.lines().fold(ParseState::default(), |state, line| {
    ParseState {
        current_date: new_date,
        ..state.clone()  // ← Full deep copy every time!
    }
})
```

**Rust limitations:**
- Standard `HashMap`/`Vec` are mutable-first with expensive `.clone()`
- No built-in persistent/immutable collections with structural sharing
- Each clone creates entirely new memory allocations
- O(N²) complexity as data structures grow

### Available Alternatives

#### Option 1: `im` Crate
```rust
// Using im crate for immutable collections
use im::HashMap;

let result = lines.fold(ParseState::default(), |mut state, line| {
    state.entries = state.entries.update(date, entries);  // O(log n)
    state
});
```

**Drawbacks:**
- External dependency
- Different API surface than standard collections
- Performance unclear for this specific use case
- Still not as optimized as languages built for immutability

#### Option 2: Mutable Accumulation (Recommended)
```rust
// Idiomatic Rust approach
let mut state = ParseState::default();
for line in lines {
    process_line_mut(&line, &mut state, filter, file_name);
}
```

**Advantages:**
- Zero unnecessary allocations
- Optimal cache locality
- Leverages Rust's ownership system as designed
- Idiomatic Rust code

## Performance Implications

### Fold with Clone (Current)
- **Time Complexity**: O(N²) - each line clones increasingly large structures
- **Memory**: Quadratic allocations, fragmented memory
- **Cache Performance**: Poor due to frequent allocations

### Mutable Accumulation
- **Time Complexity**: O(N) - each line adds constant work
- **Memory**: Linear growth, single allocation per data structure
- **Cache Performance**: Excellent due to locality

## Conclusion

**The mutable approach is not just faster—it's the correct Rust solution.**

Rust's design philosophy centers on zero-cost abstractions through controlled mutation and ownership, not immutable data structures. The current fold pattern fights against Rust's strengths and creates unnecessary performance overhead.

**Recommendation**: Replace fold with mutable accumulation. This aligns with Rust idioms and provides optimal performance characteristics.

## Context

This analysis was performed during investigation of O(N²) parser performance in a time-tracking CLI tool, where parsing 10k lines created ~50 million unnecessary allocations due to state cloning in a fold operation.

**Date**: 2025-09-25
**Source**: Performance analysis during time-tracker parser optimization
