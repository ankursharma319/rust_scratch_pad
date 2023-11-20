

use mini_redis::{client, Result};

// cargo install mini-redis --root /Users/ankurs4/src/rust_scratch_pad/tokio_demo/_build
// _build/bin/mini-redis-server
// _build/bin/mini-redis-cli get foo
// cargo run --example hello-redis

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, world!");
    // open a connection to the mini-redis server
    let mut clien = client::connect("127.0.0.1:6379").await?;

    // Set the key "hello" with value "world"
    clien.set("hello", "world".into()).await?;

    let res = clien.get("hello").await?;
    println!("Done, got res={:?}", res);

    Ok(())
}
