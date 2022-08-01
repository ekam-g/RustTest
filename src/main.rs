extern crate core;

use std::fmt::Error;
use std::io::stdin;
use std::io::stdout;
use std::str::FromStr;


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

        println!("write a number");
        stdin().read_line(&mut a).unwrap();
        if a == "exit" {
            break;
        }
        let a: Result<i32, Error> = a.trim().parse().unwrap();
        let a = match a {
            Ok(a) => a,
            Err(e) => {
                panic!("{}", e);
            },
        };

        stdin().read_line(&mut b).unwrap();


        let b: i32 = b.trim().parse().unwrap();
        println!("{}", a + b);
    }
}