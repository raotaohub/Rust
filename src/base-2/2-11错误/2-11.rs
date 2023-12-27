use std::io;

fn main() {
    // panic!("crash and burn");

    let myst = String::from("hello");

    println!("{}", myst);

    io::stdin().read_line()
}
