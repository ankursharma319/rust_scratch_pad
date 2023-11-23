
use tokio::net::{TcpListener, TcpStream};
use std::hash::{Hash,Hasher};
//
//
// cargo install mini-redis --root /Users/ankurs4/src/rust_scratch_pad/tokio_demo/_build
// _build/bin/mini-redis-server
// _build/bin/mini-redis-cli get foo
// cargo run
// cargo run --example hello-redis

type ShardedDB = std::sync::Arc<std::vec::Vec<std::sync::Mutex<std::collections::HashMap<String, bytes::Bytes>>>>;

fn new_sharded_db(num_shards: usize) -> ShardedDB {
    let mut my_vec = std::vec::Vec::with_capacity(num_shards);
    for _ in 0..num_shards {
        my_vec.push(std::sync::Mutex::new(std::collections::HashMap::new()));
    }
    return std::sync::Arc::new(my_vec);
}

#[tokio::main]
async fn main() {
    println!("Hello, listening!");
    // Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    let db: ShardedDB = new_sharded_db(4);

    loop {
        // The second item contains the IP and port of the new connection.
        let (socket, addr) = listener.accept().await.unwrap();
        println!("new client accepted: {:?}", addr);
        let db = db.clone();
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}

async fn process(socket: TcpStream, db: ShardedDB) {
    let mut redis_connection = mini_redis::Connection::new(socket);

    while let Some(frame) = redis_connection.read_frame().await.unwrap() {
        println!("Got input frame = {:?}", frame);
        let response_frame = match mini_redis::Command::from_frame(frame).unwrap() {
            mini_redis::Command::Get(cmd) => {
                println!("Got get command");
                let db = get_correct_db(&db, cmd.key()).lock().unwrap();
                match db.get(cmd.key()) {
                    Some(val) => { mini_redis::Frame::Bulk(val.clone()) }
                    _ => { mini_redis::Frame::Null }
                }
            },
            mini_redis::Command::Set(cmd) => {
                println!("Got set command");
                let mut db = get_correct_db(&db, cmd.key()).lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                mini_redis::Frame::Simple("OK".to_string())
            },
            _ => panic!("Got another command"),
        };
        redis_connection.write_frame(&response_frame).await.unwrap();
    }

}

fn get_correct_db<'a>(db: &'a ShardedDB, key: &'a str) -> &'a std::sync::Mutex<std::collections::HashMap<String, bytes::Bytes>>{
    let mut s = std::collections::hash_map::DefaultHasher::new();
    key.hash(&mut s);
    let tmp_res : usize = s.finish().try_into().unwrap();
    return &db[tmp_res % db.len()];
}

