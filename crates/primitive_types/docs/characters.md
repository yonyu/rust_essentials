## Character type

Rust has a character type, `char`, which represents a single Unicode character, as a 32-bit value.
It is always four bytes in size.

Rust uses the `char` type for single characters in isolation, but uses the UTF-8 encoding
for strings and streams of text.

So, a String represents its text as a sequence of UTF-8 bytes, not as an array of characters.

Examples:

```rust
let c = 'z';
let z: char = 'ℤ'; // with explicit type annotation
let heart_eyed_cat = '😻';
```

