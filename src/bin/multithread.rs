use std::sync::MutexGuard;
use std::sync::Mutex;
use std::sync::Arc;
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    println!("Starting 10 threads to increment count");
    let mut x: i32 = 30;
    let y : Rc<RefCell<i32>>= Rc::new(RefCell::new(20));
    let z : Arc<Mutex<i32>> = Arc::new(Mutex::new(50));

    let mut handles = Vec::new();
    for i in 0..10 {
        let y_copy : Rc<RefCell<i32>> = Rc::clone(&y);
        // cant do this one inside the lambda passed to spawn because
        // Sync trait not implemented on RefCell
        *y_copy.borrow_mut() += 2;

        let z_copy = Arc::clone(&z);
        let handle = std::thread::spawn(move || {
            println!("Inside thread #{}", i);
            // x is copied, so each thread has its own x which it increments to 31
            x = x + 1;
            let mut z_num: MutexGuard<i32> = z_copy.lock().expect("Couldnt lock mutex successfully");
            *z_num += 1;
            println!("Done inside thread #{}, x = {}, z = {}", i, x, z_num);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Done, final x = {}, y = {}", x, *y.borrow());
}

