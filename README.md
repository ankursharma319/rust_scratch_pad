# Rust

## Usage

```bash
nix develop
cargo build
cargo run
cargo test
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


## Packages and crates

A crate is the smallest amount of code that the Rust compiler considers at a time. Crates can contain modules, and the modules may be defined in other files that get compiled with the crate.

Binary crates are executable programs with `fn main` & library crates are not executable.

A package is a bundle of one or more crates. A package contains Cargo.toml file which describes how to build those crates. A package can contain as many binary crates as you like, but at most only one library crate. By default, cargo looks at `src/main.rs` for binary crate, `src/lib.rs` for library crate and `src/bin/` for additional binary crates.

## Modules

In the root crate file, we can declare modules using `mod ModuleName;` or `pub mod ModuleName` for making it public.
The implentation of the module must be provided either inline in the root crate file or by a matching file at `src/ModuleName.rs` or `src/ModuleName/mod.rs`.

So modules are basically equivalent to files.

Once a module is part of your crate, you can refer to code in that module from anywhere else in that same crate, as long as the privacy rules allow. For e.g. using an absolute path `crates::ModuleName::SomeFunction` or relative paths like `ModuleName::SomeFunction`

Items in a parent module can’t use the private items inside child modules, but items in child and sibling modules can use all items in their ancestor or sibling modules.

Marking an enum `pub` makes all the varients public too. But marking a struct `pub` does not. You also need to specifically mark specific fields `pub` and can leave some fields private.

`use` is nothing but a way to not have to namespace explicitly

## Strings

`String` type is mutable, growable, owned type and part of std library, while the `str&` is not mutable and for string literals and string slices and is part of the language core.

`to_string` method which is available on any type that implements the `Display` trait can be used to convert to String.

Can use `push_str` method which can be used to append to String. There is also the `+` operator which essentially calls the add function and the signature is `fn add(self, s: &str) -> String`, so the first arg is no longer valid after calling the add function.

The compiler can coerce the &String argument into a &str.

Accessing an index into a String with `[]` is not allowed because it would not be clear if we are asking for nth byte or nth utf8 u8 or u16 or u24 or u32 char. But indexing slices is allowed and it will return bytes. Slicing at a bad utf boundary will result in a runtime error.

For iterating over Strings, use .chars() or .bytes() explicitly.

Values can be moved into containers such as Hashmaps and vectors or references can be stored. If storing references, the lifetime of reference must be greater or equal to that of the container.

## Errors

For recoverable errors, Rust uses `Result<T, E>` and a macro called `panic!` for unrecoverable errors.
For returning the error type from a function, there is `?` operator which short circuits if the result was an error and returns the error. Can make things less verbose than match statements.

`?` operator is allowed in a function that returns `Result`, `Option`, or another type that implements `FromResidual`.

## Generics

Compile time things, generics are turned into specific impls in compiled code, just like c++
E.g.:

```rust
// only accepts T which implement the PartialOrd trait
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {...}
```

## Traits

Similar to interfaces. But after implementing traits for a type, must bring the traits and the types into the scope. So that means that a trait is not implemented permanently like it an interface would be implemented in c++

We can implement a trait on a type only if at least one of the trait or the type is local to our crate.
Can return types which implement a specific trait or multiple traits (using `+` syntax).

Can also conditionally implement additional methods on our types based on traits of the args to the structs or the methods.
`ToString` is an example of a trait with a "blanket implementation" because it essentially has a single implementation that applies to all types which have `Display` trait.

## Lifetimes

Most of the time, lifetimes of references are implicit. Sometimes, we must annotate lifetimes when the lifetimes of references could be related in a few different way.

In the below code, it will be an compiler error if we dont specify the lifetime relation

```rust
// ' is used to start a lifetime annotation, a is just a label we chose
// the concrete lifetime that is substituted for 'a is the part of the scope of x that overlaps with the scope of y
// 'a on the return value signifies that the lifetime of returned value will be smaller of the
// the lifetimes of the passed parameters
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```


When we specify the lifetime parameters in this function signature, we’re not changing the lifetimes of any values passed in or returned. Rather, we’re specifying that the borrow checker should reject any values that don’t adhere to these constraints.

If the function takes in only one arg and returns reference of the same type, then we dont need to manually specify this lifetime annotation beause rust compiler will do it for us since its pretty obvious that the lifetime of the return value will be same as that of the single arg.

Also a similar rule exists for methods which take Self as the first arg. The lifetime of returned value is assumed to be same as that of self.

Lifetime annotations must also be used if storing references in structs.

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}
```

