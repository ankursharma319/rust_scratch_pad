use futures::executor::block_on;


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

async fn dance() {
    println!("Start dancing!");
    let fut = sing();
    // Unlike block_on, await doesn't block the current thread,
    // but instead asynchronously waits for the future to complete
    fut.await;
    drum().await;
    println!("Finished dancing!");
}
async fn some_fun() {
    println!("Hello world!");
    dance().await;
}
