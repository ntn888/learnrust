//use std::io;

// fn main() {
//     println!("Hello, world!");
//     // let mut number = 3;

//     // while number != 0 {
//     //     println!("{}!", number);

//     //     number -= 1;
//     // }

//     // println!("LIFTOFF!!!");

//     // let a = [10, 20, 30, 40, 50];
//     // let mut index = 0;

//     // while index < 5 {
//     //     println!("the value is: {}", a[index]);
//     //     index += 1;
//     // }

//     // for element in a.iter() {
//     //     println!("the value is: {}", element);
//     // }

//     // main

//     println!("1: convert f to c");
//     println!("2: convert c t0 f");

//     let mut x = String::new();
//     io::stdin().read_line(&mut x).expect("Failed to read line");

//     let x: u32 = x.trim().parse().expect("Please type a number!");

//     if x == 1 {
//         println!("enter value in farenheit");

//         let mut x = String::new();
//         io::stdin().read_line(&mut x).expect("Failed to read line");

//         let x: u32 = x.trim().parse().expect("Please type a number!");

//         println!("{} in degress celcius", (x - 32) * 5 / 9);
//     } else {
//         println!("enter value in celcius");

//         let mut x = String::new();
//         io::stdin().read_line(&mut x).expect("Failed to read line");

//         let x: u32 = x.trim().parse().expect("Please type a number!");

//         println!("{} in degress farenheit", x * 9 / 5 + 32);
//     }
// }

fn main() {
    //println!("test");

    for x in 1..13 {
        print_xmas(x);
        println!("");
    }
}

fn print_xmas(verse: i32) {
    if verse == 1 {
        println!("1st  day of Christmas my true love sent to me")
    } else if verse == 2 {
        println!("2nd day of Christmas my true love sent to me")
    } else if verse == 3 {
        println!("3rd day of Christmas my true love sent to me")
    } else {
        println!("{}th day of Christmas my true love sent to me", verse)
    }

    if verse >= 12 {
        println!("12 drummers drumming,");
    }
    if verse >= 11 {
        println!("11 pipers piping,");
    }
    if verse >= 10 {
        println!("10 lords a-leaping,");
    }
    if verse >= 9 {
        println!("9 ladies dancing,");
    }
    if verse >= 8 {
        println!("8 maids a-milking,");
    }
    if verse >= 7 {
        println!("7 swans a-swimming,");
    }
    if verse >= 6 {
        println!("6 geese a-laying,");
    }
    if verse >= 5 {
        println!("5 gold rings,");
    }
    if verse >= 4 {
        println!("4 calling birds,");
    }
    if verse >= 3 {
        println!("Three French hens,");
    }
    if verse >= 2 {
        println!("Two turtle doves,");
    }
    if verse >= 1 {
        println!("A partridge in a pear tree.");
    }
}
