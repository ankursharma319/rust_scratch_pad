# Rust

## Usage

```bash
nix develop
cargo build
cargo run
```

`rustc` is the compiler. `cargo` is the build system and package manager. `cargo run` does the compilation and runs the binary.


## Scalar types

There is 4 scalar types: integer types (e.g. i32, u8), floating point numbers, booleans, and chars.
In case of overflow, there is wrapping behaviour. It is possible to use checked_types which return None, saturating_types which cap at the max or min value.

f64 is the default floating type, f32 can be explicitly mentioned.

char type is actually not like u8, like it is in c++. char is 4 bytes (32 bits) and can hold more than simple ascii.
It can actually represent all of Unicode character set, except for some characters called surrogates. [See here for details.](https://stackoverflow.com/a/48465266)

```rust
    // this is a valid char
    let heart_eyed_cat = '😻';
```

## Compound types

Tuples e.g.

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
```

Arrays are fixed length and allocated on stack. Also, out of bounds access is a runtime error.

```rust
let a = [1, 2, 3, 4, 5];
let first = a[0];
```

## Statements vs expressions

Statements are instructions that perform some action and do not return a value.
Expressions evaluate to a resultant value

On the right side of equals (=), there must be an expression, otherwise its a compile time error. Unlike in c where this is legal `x = y = 5`.

```rust
let y = {
    let x = 3;
    x + 1
};
```

This is legal in rust because the thing inside the block is an expression.

It is also legal to not have to use `return` keyword in a function (but must be done without a semicolon at the end).

## Control flow

can return values from loop like this

```rust
let result = loop {
    counter += 1;

    if counter == 10 {
        break counter * 2;
    }
};
```

while is also a thing, works as you would expect

There is also for loop

let a = [10, 20, 30, 40, 50];

```rust
for element in a {
    println!("the value is: {element}");
}
```

