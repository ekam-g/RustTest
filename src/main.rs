extern crate core;


// use std::intrinsics::likely;
use std::io::stdin;
use std::os::unix::raw::time_t;
use std::thread;
use std::time::Duration;
use std::sync::mpsc;


fn main() {
//     // app_test();
//     // Yes().cool();
//     let (tx, rx) = mpsc::channel();
//
//     loop {
//         let x = rx.recv().unwrap();
//         println!("{}", x);
//         thread::sleep(Duration::from_millis(1000));
//     }
// }
//
// fn test(mut y: i32){
//     let testfunc = thread::spawn( move|| {
//         loop {
//             y = y + 1;
//             thread::sleep(Duration::from_millis(1000));
//             tx.send(y).unwrap();
//         }
//     });
    let mut buf = String::new();

    {
        let mut obj = write_json::object(&mut buf);
        obj.string("name", "Peter").number("favorite number", 92.0);
        obj.array("films")
            .string("Drowning By Numbers")
            .string("A Zed & Two Noughts");
        obj.null("suitcase");
    }

    assert_eq!(
        buf,
        r#"{"name":"Peter","favorite number":92,"films":["Drowning By Numbers","A Zed & Two Noughts"],"suitcase":null}"#
    )

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
