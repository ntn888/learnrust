fn main() {
    let mut a: u32 = 0;
    println!("Hello, world!");
    test_fn();
    test_fn();
    while true {
        println!("vim is fun!");
    }
}

fn test_fn() {
    let mut i: i32 = 0;

    i += 1;
    let s = String::from("hello again");
    let q = String::from("hello ag");
    println!("{}, {}", s, i);
}
