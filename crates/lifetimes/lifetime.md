## lifetime

- Lifetime refers to the duration for which a reference is valid and 
accessible. Lifetimes are associated with references rather than
values.
- Lifetimes are denoted by a single apostrophe `'` followed by a name that
can be any valid identifier, such as `'a`, `'b`, `'c`, etc.


Lifetime is a concept in Rust that helps the compiler ensure that 
references are valid for as long as they are used. Lifetimes are 
a way of annotating the relationships between the lifetimes of 
various references. 

The Rust compiler uses this information to ensure that all references
are valid for as long as they are used.

## Lifetime annotations

Lifetime annotations indicate the relationships between the input references
and the return value.
Syntax: 
- 'a; represents a lifetime named 'a
- 'xyz: represents a lifetime named 'xyz
- 'b: represents a lifetime named 'b
- '_: represents an anonymous lifetime

consts have a lifetime of 'static, which means they are valid for the
entire duration of the program.

### Lifetime elision rules

There are three lifetime elision rules in Rust:
1. Each input parameter that is a reference gets its own lifetime.
2. If there is exactly one input lifetime parameter, assign it to all output lifetimes.
3. If there is a &self or &mut self input parameter, its lifetime will be assigned to all output lifetimes.

