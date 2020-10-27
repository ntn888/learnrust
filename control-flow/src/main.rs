use std::io;

fn main() {
    println!("Hello, world!");
    // let mut number = 3;

    // while number != 0 {
    //     println!("{}!", number);

    //     number -= 1;
    // }

    // println!("LIFTOFF!!!");

    // let a = [10, 20, 30, 40, 50];
    // let mut index = 0;

    // while index < 5 {
    //     println!("the value is: {}", a[index]);
    //     index += 1;
    // }

    // for element in a.iter() {
    //     println!("the value is: {}", element);
    // }

    // main

    println!("1: convert f to c");
    println!("2: convert c t0 f");

    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Failed to read line");

    let x: u32 = x.trim().parse().expect("Please type a number!");

    if x == 1 {
        println!("enter value in farenheit");

        let mut x = String::new();
        io::stdin().read_line(&mut x).expect("Failed to read line");

        let x: u32 = x.trim().parse().expect("Please type a number!");

        println!("{} in degress celcius", (x - 32) * 5 / 9);
    } else {
        println!("enter value in celcius");

        let mut x = String::new();
        io::stdin().read_line(&mut x).expect("Failed to read line");

        let x: u32 = x.trim().parse().expect("Please type a number!");

        println!("{} in degress farenheit", x * 9 / 5 + 32);
    }
}
