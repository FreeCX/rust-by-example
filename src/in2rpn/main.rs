mod converter;
use std::io::prelude::*;
use std::io;
use converter::*;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn space() {
        assert_eq!("1 2 3 * 4 / +", converter::in2rpn(" 1  + 2   * 3 /  4 ").trim());
    }
    #[test]
    fn bracket() {
        assert_eq!("1 2 3 4 5 - / * +", converter::in2rpn("(1+(2*(3/(4-5))))").trim());
    }
}

fn main() {
    print!("enter expression: ");
    io::stdout().flush()
        .ok()
        .expect( "[error] Can't flush to stdout!" );
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)
        .ok()
        .expect( "[error] Can't read line from stdin!" );
    println!("result: {}", converter::in2rpn(&buffer));
}
