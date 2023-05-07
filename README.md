# Rust

## Usage

```bash
nix develop
cargo build
cargo run --bin rust_scratch_pad
cargo run --bin multithread
IGNORE_CASE=1 cargo run --bin minigrep -- hello ./README.md
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

## Closures

E.g. -

```rust
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };`
```

It captures by immutable reference, mutable reference or ownership automatically depending on how the closure uses the captured variable.
If want to force capture by ownership, can use the `move` keyword like so:

```rust
use std::thread;

fn main() {
    let list = vec![1, 2, 3];
    thread::spawn(move || println!("From thread: {:?}", list)).join().unwrap();
}
```

If what we want to do doesn’t require capturing a value from the environment, we can use the name of a function rather than a closure.

The way a closure captures and handles values from the environment affects which traits the closure implements, and traits are how functions and structs can specify what kinds of closures they can use. For e.g. `FnOnce() -> T`, which means F must be able to be called once, take no arguments, and return a T

FnOnce trait - a closure that can be called atleast once - applies to all closures - if a closures body moves value out (gives ownership to something else), it can only be called once and will have no other Fn related trait other than this one

FnMut trait - dont move the values out but might mutate the values - can be called multipe times

Fn trait - dont move captured values out and dont mutate - can be called multiples times - also can be called concurrently

## Iterators

`vec.iter()` returns a type which has the `Iterator` trait implemented. There is also `into_iter()` and `iter_mut()` which instead of const ref, return ownership of items and mutable reference respectively.

```rust
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    // methods with default implementations elided
}
```

Calling `next()` mutates the iterator itself and eventually it will run out of items.

The `Iterator` definition above hides the other methods which have default implementations. These go into 2 categories: those which consume the iterator by calling next (e.g. - `sum()`) and those which return a new iterator based on self (e.g. `map()`).

`collect()` method also consumes an iterator, it basically produces a new collection.

## Smart pointers

`Box<T>` types are those whose data is allocated on heap instead of stack. What remains on stack is pointer to the heap data. Useful when data size is not known at compile time, large amount of data. When a `Box<T>` value goes out of scope, the heap data that the box is pointing to is cleaned up as well because of the `Drop` trait implementation.

Can allow recursive types because otherwise the size of struct needed in struct will not be possible to compute .

References in rust can be dereferenced using the * operator, just like in C. For our custom types (for e.g. if we were implementing a smart pointer like Box), we can implement the `Deref` trait to make it dereferenceable and specify what dereferencing does. (implement `DerefMut` trait to override the * operator on mutable references). `&T` will return a `&U` when `T: Deref<Target=U>`

Deref coercion converts a reference to a type that implements the `Deref` trait into a reference to another type. e.g. - `&String` to `&str` because `String` implements the `Deref` trait such that it returns `&str`.

`Drop` trait can be used to specify what should happen when a value goes out of scope.

`Rc<T>` (short for reference counting) is useful for when we want to have multiple owners. `Rc::new` to move some data into this pointer and `Rc::clone` to return a new instance of `Rc` and increase the reference count. `Rc` only allows immutable access to the data.

`RefCell<T>` is a lot like `Box<T>`except you can cheat and mutate data even through a immutable reference to RefCell. Interior mutability is a design pattern in Rust that allows you to mutate data (using some unsafe code) even when there are immutable references to that data; normally, this action is disallowed by the borrowing rules. RefCell checks our borrowing rules at runtime instead of compile time.

E.g. can mutate an object internally in a struct even though only have a immutable reference to it.

RefCell has methods `borrow` and `borrow_mut` which return `Ref<T>` and `RefMut<T>` respectively.

For mutating data in something like shared_ptrs, we can use `Rc<T>` together with `RefCell<T>`. If you have an Rc<T> that holds a RefCell<T>, you can get a value that can have multiple owners and that you can mutate.

Rc, refcell are all only suitable for single threaded scenarios.

You can also create a weak reference to the value within an `Rc<T>` instance by calling `Rc::downgrade` and passing a reference to the `Rc<T>`. Strong references are how you can share ownership of an `Rc<T>` instance. Weak references don’t express an ownership relationship, and their count doesn’t affect when an `Rc<T>` instance is cleaned up.

## Concurrency

Can spawn a thread using

```rust
let v = vec![1, 2, 3];
let handle = thread::spawn(move || {
    println!("Here's a vector: {:?}", v);
});

handle.join().unwrap();
```

Note that we moved v into the new thread using `move ||` syntax.

Its better to communicate between threads via message passing rather than using shared memory. Example of creating a multiple producer single consumer channel:

```rust
let (tx, rx) = mpsc::channel();

// for multiple producers, can clone tx and move that into more threads
thread::spawn(move || {
    let val = String::from("hi");
    tx.send(val).unwrap();
});

// recv blocks until got msg, try_recv wont block
let received = rx.recv().unwrap();
println!("Got: {}", received);
```

For shared memory access, there is mutexes `std::sync::Mutex<T>`.
In order to send data into the lambda for `thread::spawn`, the data must implement `Send` or `Sync` traits.
Send trait indicates that ownership of values of the type implementing Send can be transferred between threads. Most are, except `Rc`.
Sync trait indicates that it is safe for the type implementing Sync to be referenced from multiple threads. Most are, but some like `Rc`, `RefCell` are not. `Mutex` implements Sync.

`Arc<T>` (atomic reference counting) is like `Rc<T>` but thread safe (well the increment and decrement of the reference counters is thread safe, not necessarily operations on thing it points to) and can be moved into other threads.

## Trait objects

A trait object points to both an instance of a type implementing our specified trait and a table used to look up trait methods on that type at runtime. Therefore trait objects must be specified as pointers or smart pointers and with `dyn` keyword. E.g. `Box<dyn Draw>` where `Draw` is a trait that we want the arg to have implemented. A slight runtime cost compared to pure compile time generics for e.g.

Suppose we want to have a container (vector) which can contain multiple types. Can have a container of enums or have a container of these trait objects. If we already have all the types which container can contain in our crate, enums maybe better. But sometimes, we want users of our library to add their own types. Trait objects useful in those cases.

Doing this `struct Screen<T: Draw> { components: Vec<T> }` would have limited T to be a single type.

## Unsafe code

Allow programmer to write some code that would otherwise not be safe with full rust safety.

- Dereference a pointer
- Calling unsafe function or method, including functions in extern languages such as C
- Accessing or modifying a mutable static variable, including functions in extern languages such as C
    - using immutable static variables is safe, but mutable static variables are unsafe
- implement an unsafe trait
- work with union types (usually when working with a C library interface which likes using unions)

## Types

Can create "newtypes" like this `struct Wrapper(Vec<String>);`. Can create type aliases like this `type Kilometers = i32;`.

For a function that will never return (maybe bcoz it will cause a crash or its a forever loop), there is the never type:

```rust
fn bar() -> ! {
    // --snip--
}
```

`str` is actually a DST - dynamically sized type.
So although a `&T` is a single value that stores the memory address of where the `T` is located, a `&str` is two values: the address of the `str` and its length.

`Sized` trait is used to determine whether or not the size of a struct is known at compile time and is automatically/implicitly inserted in most places. E.g.

```rust
fn generic<T>(t: T) {
    // --snip--
}

// is actually treated as though we had written this:

fn generic<T: Sized>(t: T) {
    // --snip--
}
```

The `fn` type is called a function pointer. Not to be confused with `Fn` closure trait. E.g.:

```rust
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}
```

`fn` implements all three of `FnOnce`, `FnMut` and `Fn`. Normally, we want to accept closures, so that the user can choose to pass in a function pointer or a closure. But if interfacing with C code, then would need to accept function pointers only.

## Macros

Metaprogramming, before the real compilation. Macros can take a variable number of parameters. Also, macros must be brought into scope (or defined locally) in order to be used, unlike functions which can be called directly.

```rust
// shows how vec! macro might be implemented
// macro_export : this macro should be made available when this crate is brought into scope.
// macro_rules! is general syntax name to define a macro
// vec is the name of the macro
#[macro_export]
macro_rules! vec {
    // Like a match expression, if expression matches this pattern
    // the following code will be emitted
    // $ to declare a variable that will contain the code matching the pattern
    // $x:expr matches any expression and gives it name x
    // like in regex, * means zero or more of whatever precedes the *
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
```

There are also procedural macros which are a bit like function annotations in python.

## Async

Other than using threads explicitly like shown above, can also use async, especially for io bound tasks because there will be less overhead.
Probably

No heap allocations and dynamic dispatch needed for async - low cost.

Need to chose an async runtime from community implementations.  An async runtime uses a small amount of (expensive) threads to handle a large amount of (cheap) tasks.

No extra threads are created for running the two downloads below:
```rust
async fn get_two_sites_async() {
    // Create two different "futures" which, when run to completion,
    // will asynchronously download the webpages.
    let future_one = download_async("https://www.foo.com");
    let future_two = download_async("https://www.bar.com");

    // Run both futures to completion at the same time.
    join!(future_one, future_two);
}
```

