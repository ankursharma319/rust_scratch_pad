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

## Memory

When a variable goes out of scope, Rust calls a special function for us. This function is called drop. Similar to c++ RAII.

```rust
// creates copy
let x = 5;
let y = x;


// strings len, capacity, ptr which are stored on the stack are copied
// the memory pointed to be ptr (on the heap) is not copied
let s1 = String::from("hello");
let s2 = s1;

// how does rust avoid double free error when both s2 and s1 go out of scope
// actually s1 is considered no longer valid (kinda like a std::moved val)
// using s1 after the assignment to s2 will be an error at compile time

```

Design choice : Rust will never automatically create “deep” copies of your data. Use clone() method explicitly to create a deep copy.
Primitives stored on stock are fully copied though.

If a type implements the Copy trait, variables that use it do not move, but rather are trivially copied, making them still valid after assignment to another variable.
Not allowed to add copy trait to a type which implements the Drop trait (which is similar to a non trivial destructor in c++).

This rule for copy for primitives that implement the Copy trait and rule of move for copy of other types also applies when passing to functions. Returning values can also transfer ownership.

There is also references, very similar to c++. The action of creating references is called borrowing. References do not allow to mutate by default. Need to use mutable references to allow mutation.

Mutable references have one big restriction: if you have a mutable reference to a value, you can have no other references to that value (which are alive at the same time).

```rust
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s; // the error persists whether or not this one is mut or const

println!("{}, {}", r1, r2);
```

this restriction helps avoid data races, at compile time, because two different pointers are not accessing the same data, while one is writing to the memory.

note that calling any method which mutates the original string also passes a reference to self, so this will also fail to compile:

```rust
let mut s = String::from("  hello  ");

let r1 = &s;
let r2 = &s;

s.push_str("hello there");
```

Also, the compiler guarantees that references will never be dangling references. You cannot return references to a local variable from a function, for e.g.

A string slice is like a std::string_view and is writen as:

```rust
let s = String::from("hello");

let slice:&str = &s[0..2];
```

## Structs

`struct Color(i32, i32, i32);` is a tuple struct where fields are accessed as `color.1`, `color.2` etc.

Creating struct from fields in another struct may move it or copy it depending on if the fields have Copy trait or not.

e.g.

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
let user1 = User {
    active: true,
    username: String::from("someusername123"),
    email: String::from("someone@example.com"),
    sign_in_count: 1,
};

let user2 = User {
    email: String::from("another@example.com"),
    ..user1
};
```

So the email field will not be moved, username field will be moved, and active & sign_in_count will be copied.

Unit structs behave similar to empty tuples `()`. To define a unit struct: `struct AlwaysEqual;`

In order to print structs using `println!("{:#?}")` or `!dbg` macro, it must implement Debug trait. There is a default impl which we can opt in to by using `#[derive(Debug)]` above the struct declaration.

Methods can be added to structs in impl blocks. The first argument must be `self`, `&self` or `&mut self`. If the first argument is not self, its kinda like a static method in c++.

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```

`&self` is actually short for `self: &Self` and `Self` is an alias for the type that the impl block is for. `&self` is for reading the data in the struct but not mutating it, `&mut self` is for reading and mutating the data in the struct and `self` is for taking the ownership of the struct instance and is is used rarely, when the method transforms self into something else and you want to prevent the caller from using the original instance after the transformation.

## Enums

It is possible to attach data to enums. They actually behave like variants and data can be attached to them. e.g.

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

Another difference compared to c++ is that enums are allowed to have methods defined for them, just like structs.

Normally would use `match` to alter behaviour depending on value of enum.
There is also an `if let` syntax which is less boilerplaty when we want to do something only if it matches one pattern.
