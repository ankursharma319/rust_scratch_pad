
use mini_redis::{client, Result};

// cargo run --bin client

/// Provided by the requester and used by the manager task to send
/// the command response back to the requester.
type Responder<T> = tokio::sync::oneshot::Sender<mini_redis::Result<T>>;

#[derive(Debug)]
enum Command {
    Get {
        key: String,
        resp: Responder<Option<bytes::Bytes>>
    },
    Set {
        key: String,
        val: bytes::Bytes,
        resp: Responder<()>
    }
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    // Create a new channel with a capacity of at most 32.
    let (tx, mut rx) = tokio::sync::mpsc::channel(32);
    let tx2 = tx.clone();

    let t1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = tokio::sync::oneshot::channel();
        tx.send(Command::Get { key: "hello".to_string(), resp: resp_tx }).await.unwrap();
        // Await the response
        let res = resp_rx.await;
        println!("GOT = {:?}", res);
    });

    let t2 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = tokio::sync::oneshot::channel();
        tx2.send(Command::Set { key: "hello".to_string(), val: "world".into(), resp: resp_tx}).await.unwrap();
        // Await the response
        let res = resp_rx.await;
        println!("GOT = {:?}", res);
    });

    let manager = tokio::spawn(async move {
        println!("Inside Message consumer");

        // open a connection to the mini-redis server
        let mut clien = client::connect("127.0.0.1:6379").await.unwrap();

        while let Some(cmd) = rx.recv().await {
            use Command::*;
            match cmd {
                Get{key, resp} => {
                    println!("Get key={}", key);
                    let res = clien.get(&key).await;
                    resp.send(res);
                }
                Set {key, val, resp} => {
                    println!("Set key={}, val={:?}", key, val);
                    let res = clien.set(&key, val).await;
                    resp.send(res);
                },
            }
        }
    });

    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
}

