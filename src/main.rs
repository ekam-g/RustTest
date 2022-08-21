extern crate core;


// use std::intrinsics::likely;
use std::io::stdin;
use std::os::unix::raw::time_t;
use std::thread;
use std::time::Duration;
use std::sync::mpsc;


fn main() {
    // app_test();
    // Yes().cool();
    let (tx, rx) = mpsc::channel();
    let testfunc = thread::spawn( move|| {
        let mut y: i32= 0;
        loop {
            y = test(y);
            thread::sleep(Duration::from_millis(1000));
            tx.send(y).unwrap();
        }
    });
    loop {
        let x = rx.recv().unwrap();
        println!("{}", x);
        thread::sleep(Duration::from_millis(1000));
    }
}

fn test(mut i: i32) -> i32 {
    i += 1;
    return i;
}


struct Yes();


trait Cool {
    fn cool(&self);
}

impl Cool for Yes {
    fn cool(&self)
    {
        let a = 5;
        let b: i32 = 25;
        for i in 0..10 {
            let sum = return_sum(a, b);
            println!("The sum of a &  b is = {}", sum);
            println!("{}", i)
        }
    }
}


fn return_sum(i: i32, j: i32) -> i32 {
    i + j
}

trait App {
    fn app_test(&self);
}

impl App for Yes {
    fn app_test(&self) {
        loop {
            let mut a = String::new();
            let mut b = String::new();

            println!("write a number");
            stdin().read_line(&mut a).unwrap();
            if a == "exit" {
                break;
            }
            let a: i32 = a.trim().parse().unwrap();
            stdin().read_line(&mut b).unwrap();


            let b: i32 = b.trim().parse().unwrap();
            println!("{}", a + b);
        }
    }
}
