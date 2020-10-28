fn main() {
    // let mut s = String::from("hello");

    // s.push_str(", world!"); // appends literal to a String

    // println!("{}", s); // will print "hello, world!"

    let s = String::from("hello");

    takes_ownership(s);

    //rintln!("{}", s);

    let x = 5;

    makes_copy(x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
