use std::env;

fn main() {
    let mut i = 0;
    for arg in env::args() {
        match i {
            0 => println!( "program -- {}", arg ),
            _ => println!( "[{:05}] -- {}", i, arg ),
        }
        i += 1;
    }
}
