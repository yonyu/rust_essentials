# Data types in Rust

Rust is a statically typed language, it must know the types of all variables at compile time.

## Primitive types

The Rust language has a number of built-in types that are considered
primitive. These types are the most basic types in the language and
are provided by the language itself rather than by libraries. They
represent single values rather than collections of values.

Here are the primitive types in Rust:

- [Numeric types](docs/numerics.md)
- [Booleans](docs/booleans.md)
- [Characters](characters.md)

The primitive types are also called scalar types as they represent a single value.
 
## Compound types

Rust also has compound types that can group multiple values into one type. 
These types can be used to create collections of values. They are also called primitive
compound types as they are provided by the language itself.

These include tuples and arrays. Tuples can hold multiple values of different
types, and arrays hold multiple values of the same type.

- [Tuples](tuples.md)
- [Arrays](arrays.md)

## Reference Types

Rust has reference types that allow you to refer to a value without taking ownership of it.
These involve pointers and borrowing.

Here are the reference types in Rust:

- References (&T and &mut T):

    * Immutable Reference (&T): Allows reading but not modifying the borrowed data. 
    * Mutable Reference (&mut T): Allows reading and modifying the borrowed data. Only one mutable reference can exist at a time to ensure data safety.

- Raw Pointers (*const T and *mut T):

    * Const Raw Pointer (*const T): Similar to an immutable reference but without the borrow checker guarantees. Mainly used for interfacing with C code or when performing low-level memory manipulation. 
    * Mutable Raw Pointer (*mut T): Similar to a mutable reference but without the borrow checker guarantees. Also used for low-level operations.

- Smart Pointers:

    * Box (Box<T>): A heap-allocated smart pointer that owns the data it points to. It ensures that the data it points to is dropped when the Box goes out of scope.

    * Rc (Rc<T>): A reference-counting smart pointer for single-threaded scenarios. Allows multiple immutable references to the same data.

    * Arc (Arc<T>): An atomic reference-counting smart pointer for multi-threaded scenarios. Safe to use across threads.

    * RefCell (RefCell<T>): Allows mutable borrowing checked at runtime. Useful when you need interior mutability.

    * Cell (Cell<T>): Allows copying types to be mutated inside an immutable struct. Provides interior mutability.

    * Weak (Weak<T>): A non-owning reference to data managed by Rc or Arc to break reference cycles.

- Slices (&[T] and &mut [T]):

    * Immutable Slice (&[T]): A reference to a contiguous sequence of elements in a collection, read-only.

    * Mutable Slice (&mut [T]): A reference to a contiguous sequence of elements in a collection, read and write access.

- String Slices (&str and &mut str):

    * Immutable String Slice (&str): A reference to a sequence of UTF-8 encoded characters, read-only.
    
    * Mutable String Slice (&mut str): A reference to a sequence of UTF-8 encoded characters, read and write access.

## Collection Types

Collection types are used to store multiple values in one data structure. 
They are provided by the standard library.

Here are the collection types in Rust:
These are some of the most commonly used collection types in Rust. 
Each collection type has its own strengths and use cases, helping you efficiently 
manage and organize data in various scenarios.

1. Vectors
   * Vec<T>: A resizable array that stores elements of type T on the heap. Commonly used for dynamic arrays.

   * VecDeque<T>: A double-ended queue that allows efficient insertion and removal of elements from both ends.

2. Strings
   * String: A growable, heap-allocated string. It stores UTF-8 encoded text.

   * &str: A slice of a string, which can refer to either a part of a String or a string literal.

3. Arrays
   * [T; N]: A fixed-size array with elements of type T and length N.

4. Hash Maps
   * HashMap<K, V>: A hash table where keys of type K are mapped to values of type V.

5. BTree Maps
    * BTreeMap<K, V>: An ordered map based on a B-tree where keys of type K are mapped to values of type V.

6. Hash Sets
    * HashSet<T>: A hash table that stores unique elements of type T.

7. BTree Sets
    * BTreeSet<T>: An ordered set based on a B-tree that stores unique elements of type T.

8. Linked Lists
    * LinkedList<T>: A doubly-linked list storing elements of type T.

9. Binary Heaps
    * BinaryHeap<T>: A priority queue implemented with a binary heap, where elements of type T are sorted based on a comparator.

10. Ranges
    * Range<T>: Represents a sequence of values from start to end, with a step size of 1.

    * RangeInclusive<T>: Similar to Range<T>, but includes both the start and end values.

    * RangeFrom<T>: A sequence that starts from a specific value and continues indefinitely.

    * RangeTo<T>: A sequence that includes all values up to, but not including, a specific value.

    * RangeToInclusive<T>: A sequence that includes all values up to and including a specific value.

11. Iterators
    * Iterator Trait: Defines methods for iterating over a sequence of items.

    * IntoIterator Trait: Allows a collection to be converted into an iterator.

    * DoubleEndedIterator Trait: Allows iterating from both ends of a collection.

## Custom Types
These are user-defined types. Examples are structs, enums, and unions.

## Function Types
These include function pointers, closures, and traits.

## Advanced Types
These include associated types, type aliases, and type inference.

1. Associated Types
   * Associated types are a way of associating a type placeholder with a trait. 
   Instead of specifying a concrete type when implementing a trait, you define 
   a type placeholder within the trait definition and then specify the concrete 
   type in the trait implementation. This can make the trait definition and 
   usage more flexible and easier to read.

2. Type Aliases
   * Type aliases allow you to create an alias for an existing type. This can make your code more readable and easier to maintain, especially if you have complex types or use the same type frequently.

3. Type Inference
   * Type inference is Rust's ability to infer the types of variables and expressions based on the context in which they are used. This means that in many cases, you don't need to explicitly specify the type of a variable because the compiler can deduce it for you.

