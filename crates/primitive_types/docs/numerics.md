## Integers

Rust has signed integer types and unsigned integer types. 

- The signed integer types are `i8`, `i16`, `i32`, `i64`, `i128`, and `isize`.
- The unsigned integer types are `u8`, `u16`, `u32`, `u64`, `u128`, and `usize`.

The number in the type name indicates the number of bits in the integer type.
For example, `i32` is a signed integer type that is 32 bits in size.

Examples:

```rust
let var1: i8 = 10; // full declaration
let var2 = 11; // type inference, i32
let var3: i8; // uninitialized variable
let var4; // type inference and uninitialized
var3 = 12; // delayed initialization
var4 = 13; // delayed initialization, i32
```

**Rust defaults to i32 when you write integer literals without specifying a type.**

In case of `isize` and `usize`, the size of the integer is the same as the size of a pointer on the platform.
For example, on a 64 bits machine, `isize` and `usize` are 64 bits in size.

## Floating-point numbers

Rust has two floating-point types: `f32` and `f64`.

**Rust defaults to `f64` when you write floating-point literals without specifying a type.**

Examples:

```rust
let a = 9.109_383_56e-31f64; //f64
let b = 1e2f32; //f32
let d = -1.5625; //f64
```
