# Fuzz Test the Parser

**Task**: Use `cargo-fuzz` to throw randomized, malformed input at `parse_content`.

**Context**:

This is an incredibly effective way to find panic-inducing edge cases that you'd never think to write a unit test for. The current parsing logic is robust for well-formed input, but we lack confidence in how it handles completely unexpected or malicious input. Fuzzing will help us harden the parser against crashes and improve overall robustness.
