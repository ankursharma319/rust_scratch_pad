use futures::{executor::block_on, Future};

fn main() {
    let future = some_fun();
    block_on(future);
}

async fn sing() {
    println!("Singing!");
}

async fn drum() {
    println!("Drumming!");
}

async fn dance() -> i32 {
    println!("Start dancing!");
    let fut = sing();
    futures::join!(fut, drum());
    println!("Finished dancing!");
    return 2;
}
async fn some_fun() {
    println!("Hello world!");
    // Unlike block_on, await doesn't block the current thread,
    // but instead asynchronously waits for the future to complete
    let res: i32 = dance().await;
    println!("res = {}", res);
}
