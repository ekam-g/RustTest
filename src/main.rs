fn main() {
    let a = 5;
    let b: i32 = 25;
    for i in 0..10 {
        let sum = return_sum(a, b);
        println!("The sum of a &  b is = {}", sum);
        println!("{}",i)
    }
}

fn return_sum(i: i32, j: i32) -> i32 {
    i + j
}