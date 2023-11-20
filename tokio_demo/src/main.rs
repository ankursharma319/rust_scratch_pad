
use tokio::net::{TcpListener, TcpStream};
//
//
// cargo install mini-redis --root /Users/ankurs4/src/rust_scratch_pad/tokio_demo/_build
// _build/bin/mini-redis-server
// _build/bin/mini-redis-cli get foo
// cargo run
// cargo run --example hello-redis

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    // Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        // The second item contains the IP and port of the new connection.
        let (socket, addr) = listener.accept().await.unwrap();
        println!("new client: {:?}", addr);
        tokio::spawn(async move {
            process(socket).await;
        });
    }
}

async fn process(socket: TcpStream) {
    let mut redis_connection = mini_redis::Connection::new(socket);
    let mut db: std::collections::HashMap<String, Vec<u8>>= std::collections::HashMap::new();

    while let Some(frame) = redis_connection.read_frame().await.unwrap() {
        println!("Got input frame = {:?}", frame);
        let response_frame = match mini_redis::Command::from_frame(frame).unwrap() {
            mini_redis::Command::Get(cmd) => {
                println!("Got get command");
                match db.get(cmd.key()) {
                    Some(val) => { mini_redis::Frame::Bulk(val.clone().into()) }
                    _ => { mini_redis::Frame::Null }
                }
            },
            mini_redis::Command::Set(cmd) => {
                println!("Got set command");
                db.insert(cmd.key().to_string(), cmd.value().to_vec());
                mini_redis::Frame::Simple("OK".to_string())
            },
            _ => panic!("Got another command"),
        };
        redis_connection.write_frame(&response_frame).await.unwrap();
    }

}

