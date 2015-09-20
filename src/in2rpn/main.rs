use std::io::prelude::*;
use std::io;

struct InfixNotation<'a> {
    text: &'a str,
    index: usize
}

impl<'a> InfixNotation<'a> {
    fn new(string: &'a str) -> InfixNotation<'a> {
        InfixNotation {
            text: string.trim().clone(),
            index: 0
        }
    }
}

impl<'a> Iterator for InfixNotation<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<&'a str> {
        let start_pos = self.index;
        if start_pos >= self.text.len() {
            return None;
        }
        let mut end_pos = self.index;
        for block in self.text.chars().skip(start_pos) {
            end_pos += 1;
            match block {
                '+' | '-' | '*' | '/' | '(' | ')' => {
                    end_pos -= 1;
                    break;
                },
                _ => continue
            }
        }
        if end_pos - start_pos == 0 {
            self.index = end_pos + 1;
        } else {
            self.index = end_pos;
        }
        Some(unsafe {
            self.text.slice_unchecked(start_pos, self.index)
        })
    }
}

fn is_digit(block: &str) -> bool {
    block.chars().nth(0).unwrap().is_digit(10)
}

fn get_priority(operator: &str) -> Option<i8> {
    match operator {
        "(" => Some(-1),
        "*" | "/" => Some(1),
        "+" | "-" => Some(2),
        _ => None
    }
}

fn can_pop(op1: &str, stack: &Vec<&str>) -> bool {
    if stack.len() == 0 {
        return false
    }
    let p1 = get_priority(op1).unwrap();
    let last = *stack.last().unwrap();
    let p2 = get_priority(last).unwrap();
    p1 >= 0 && p2 >= 0 && p1 >= p2
}

fn in2rpn(input: &str) -> String {
    let mut result = String::new();
    let it = InfixNotation::new(input);
    let mut func: Vec<&str> = Vec::new();
    for item in it {
        if is_digit(item) {
            result.push_str(item);
            result.push(' ');
        } else {
            if item == ")" {
                while func.len() > 0 && *func.last().unwrap() != "(" {
                    result.push_str(func.pop().unwrap());
                    result.push(' ');
                }
                func.pop();
            } else {
                while can_pop(item, &func) {
                    result.push_str(func.pop().unwrap());
                    result.push(' ');
                }
                func.push(item);
            }
        }
    }
    while let Some(item) = func.pop() {
        result.push_str(item);
        result.push(' ');
    }
    result
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
    println!("result: {}", in2rpn(&buffer));
}
