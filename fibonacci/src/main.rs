use std::io::BufRead;

fn main() {
    println!("Hello, world!");

    //let x = 9;
    println!("Please input n:");

    let mut n = String::new();

    io::stdin().read_line(&mut n).expect("Failed to read line");

    let n: i32 = n.trim().parse().expect("Please type a number!");
    println!("{}", fib(n));
}

fn fib(x: i32) -> i32 {
    if x <= 1 {
        return x;
    };
    return fib(x - 1) + fib(x - 2);
}
