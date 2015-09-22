mod calculate;
use std::io::prelude::*;
use std::io;
use calculate::*;

fn main() {
    print!( "введите выражение в ОПН: " );
    io::stdout().flush()
        .ok()
        .expect( "[error] Can't flush to stdout!" );
    let mut buffer = String::new();
    io::stdin().read_line( &mut buffer )
        .ok()
        .expect( "[error] Can't read line from stdin!" );
    println!("stack {:?}", calculate::calculate(&buffer));
}
