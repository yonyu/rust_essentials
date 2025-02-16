# Ownership and borrowing in Rust

One of the biggest differences between Rust and other programming
languages is the enforcement of a few very important rules about how data
can be accessed and dependencies between different forms of data access.
These rules are not overly complicated, but they are different from many
other languages, which have no enforcement of such rules. 

Here are the rules for ownership:
- Each value in Rust has a variable that’s called its owner
- There can only be one owner at a time
- When the owner goes out of scope, the value will be dropped

The Rust compiler is enforcing these rules.

