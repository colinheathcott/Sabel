# Sabel

A systems-level programming language that aims to be a refreshing and cleaner approach to performance-critical programming and manual memory management. Intended for use in data science and scientific computation. 

## Goals

1. Strongly-implemented manual memory management that supplies the programmer many options for pointer/reference rules.
2. Capable type system that allows for intuitive encapsulation, composition, overloading, and good support for various kinds of abstract data types.
3. Modern and clean syntax that prioritizes readability of code rather than expressiveness while remaining familair to programmers of all backgrounds.

## Structure

1. **ast**: contains all code related to the structure of the Abstract Syntax Tree itself.
    - *expr.rs*: contains data structures and methods for AST expressions
    - *stmt.rs*: contains data structures and methods for AST statements
    - *format.rs*: contains methods for formatting the AST to be printed
    - *tree.rs*: contains high level organizational structures and methods for the AST
2. **common**: contains basic and shared data structures and methods that are used around the crate.
    - *handle.rs*: contains the `Handle` data structure and it's methods
    - *file.rs*: contains `File`, `Position`, and `Substring` structures
    - *diagnostic.rs*: contains all diganostic-related structures and methods, including the `Diag` structure
    - *operator.rs*: contains all operators and the `Operator` sutrcture
    - *token.rs*: contains the token structure and variants

## Implementation

### File & Position Handling

The `Position` struct is implemented using a basic span-based type containing `offset` and `len`. `Substring` struct offers an effectively free way to refer to small snippets of source code without allocations or references.

### Diagnostics

Diagnostics are a generic kind of message to be emitted to the user via the `Diag` struct. Specifics are determined by the kind of the diagnostic, encoded through the `DiagKind` enum.

### Handle

Since Rust hates un-collected heap allocations and maintaining references to heap allocated items, the `Handle` struct provides a type-safe way to index into collections of items like AST nodes.

### Scanner

Sabel uses an on-demand byte-based scanner that returns `Result<Token, Diag>` to the parser.

1. ~~operators~~
2. ~~identifiers~~
3. ~~string literals~~
4. ~~keywords~~
5. ~~integer literals~~
6. ~~floating point literals~~
7. raw string literals
8. interpolated string literals

### Operator

Operators are implemented in a very C-like manner to avoid oddities with Rust enums and bitflags. Operators are instances of the `Operator` struct, which exposes methods like `is()` for determining if an arbitrary operator has a certain flag.

Operators of any kind are contained as compile-time constants in `operator::ops` and their bitflags are in `operator::flags`.