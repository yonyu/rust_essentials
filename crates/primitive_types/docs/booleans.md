## Boolean types

Rust has a boolean type, `bool`, which has two possible values: `true` and `false`.

Although a bool needs only a single bit to represent it, Rust uses an entire byte for a
bool value in memory, so you can create a pointer to it.

Examples:

```rust
let t = true;
let f: bool = false; // with explicit type annotation
let c = !t; // false

