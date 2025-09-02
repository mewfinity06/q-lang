# Q-lang

Q-lang is a new language written in Rust. It aims to be a fast compiled language

## TODOs

- [ ] Lexer
- [ ] Parser
- [ ] Codegen
  - LLVM
  - JVM Bytecode?
  - Own backend?
- [ ] Optimization

## Basics

### Variables & Functions

```rust
// const, let, & let mut
// variables are declared similar to Rust's variable system
const Foo: usize = 42;

let x = 55;
let mut y = 100;
y -= x;


// functions have the same structure as all other variables
const add : fn(a: u8, b: u8) -> u8 = { a + b };
const sub : fn(a, b: u8)     -> u8 = { return a + b; };

// since functions are declared in this way,
//     anon functions are declared much same
const apply_math(
  f: fn(u8, u8) -> u8,
  a: u8,
  b: u8
) -> u8 = { f(a, b) };
    

```

