use std::sync::MutexGuard;
use std::sync::Mutex;
use std::sync::Arc;
use std::rc::Rc;
use std::cell::RefCell;

unsafe fn unsafe_func() {
    // do something unsafe here
}

extern "C" {
    fn abs(input: i32) -> i32;
}

static mut COUNTER: u32 = 0;

fn demo_unsafe_pointers() {
    let mut num1 = 34;
    let r1 : &i32 = &num1;
    let p1 : *const i32 = r1;
    let r2 : &mut i32 = &mut num1;
    let p2 : *mut i32 = r2;
    let _p3 : *const i32 = 0x012345 as *const i32;
    unsafe {
        *p2 = 37;
        println!("p1 is {}", *p1);
        println!("p2 is {}", *p2);
        // will cause segfault
        // println!("p3 is {}", *_p3);
        unsafe_func();
        println!("Absolute value of -3 according to C: {}", abs(-3));
        COUNTER += 1;
        println!("COUNTER = {}", COUNTER);
    }
}

fn main() {
    demo_unsafe_pointers();

    println!("Starting 10 threads to increment count");
    let mut x: i32 = 30;
    let y : Rc<RefCell<i32>>= Rc::new(RefCell::new(20));
    let z : Arc<Mutex<i32>> = Arc::new(Mutex::new(50));

    let mut handles = Vec::new();
    for i in 0..10 {
        let y_copy : Rc<RefCell<i32>> = Rc::clone(&y);
        // cant do this one inside the lambda passed to spawn because
        // Send or Sync trait not implemented on RefCell
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

