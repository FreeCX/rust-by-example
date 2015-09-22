mod calculate;
mod converter;
use calculate::calculate;
use converter::in2rpn;
use std::io::prelude::*;
use std::io;

fn main() {
    print!("enter expression: ");
    io::stdout().flush()
        .ok()
        .expect( "[error] Can't flush to stdout!" );
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)
        .ok()
        .expect( "[error] Can't read line from stdin!" );
    println!("result: {:?}", calculate(&in2rpn(&buffer)));
}