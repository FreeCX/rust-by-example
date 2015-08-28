use std::io::prelude::*;
use std::io;

fn print_stack( stack: &Vec<f64> ) {
    print!( "стек < " );
    for e in stack {
        print!( "{} ", e );
    }
    println!( ">" );
}

fn main() {
    let mut stack: Vec<f64> = Vec::new();
    print!( "введите выражение в ОПН: " );
    io::stdout().flush()
        .ok()
        .expect( "[error] Can't flush to stdout!" );
    let mut buffer = String::new();
    io::stdin().read_line( &mut buffer )
        .ok()
        .expect( "[error] Can't read line from stdin!" );
    for token in buffer.trim().split( " " ) {
        match token {
            "+" | "-" | "*" | "/" => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                let mut result: f64;
                match token {
                    "+" => result = b + a,
                    "-" => result = b - a,
                    "*" => result = b * a,
                    "/" => result = b / a,
                    _   => continue
                };
                println!( "{} {} {} -> {}", b, token, a, result );
                stack.push( result );
            },
            _ => {
                stack.push( match token.parse() {
                    Ok( value ) => {
                        println!( "{} -> stack", value );
                        value
                    }
                    Err( why )  => {
                        println!( "{}", why );
                        continue;
                    }
                } );
            }
        }
    }
    print_stack( &stack );
}
