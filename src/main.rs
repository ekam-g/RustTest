extern crate core;

use std::io::stdin;
use std::io::stdout;

fn main() {
    app_test();
}


fn cool()
{
    let a = 5;
    let b: i32 = 25;
    for i in 0..10 {
        let sum = return_sum(a, b);
        println!("The sum of a &  b is = {}", sum);
        println!("{}", i)
    }
}

fn return_sum(i: i32, j: i32) -> i32 {
    i + j
}


fn app_test() {
    loop {
        let mut a = String::new();
        let mut b = String::new();
        ;
        println!("write a number");
        stdin().read_line(&mut a).unwrap();
        stdin().read_line(&mut b).unwrap();
        if a == "exit" {
            break;
        }

        let a: i32 = a.trim().parse().unwrap();
        let b: i32 = b.trim().parse().unwrap();
        println!("{}", a + b);
    }
}