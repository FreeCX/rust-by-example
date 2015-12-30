struct InfixNotation<'a> {
    text: &'a str,
    index: usize,
}

impl<'a> InfixNotation<'a> {
    fn new(string: &'a str) -> InfixNotation<'a> {
        InfixNotation {
            text: string.trim().clone(),
            index: 0,
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
                }
                _ => continue,
            }
        }
        if end_pos - start_pos == 0 {
            self.index = end_pos + 1;
        } else {
            self.index = end_pos;
        }
        Some(unsafe { self.text.slice_unchecked(start_pos, self.index).trim() })
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
        _ => None,
    }
}

fn can_pop(op1: &str, stack: &Vec<&str>) -> bool {
    if stack.len() == 0 {
        return false;
    }
    let p1 = match get_priority(op1) {
        Some(value) => value,
        None => panic!("[error]: unknown operator '{}'", op1),
    };
    let last = match stack.last() {
        Some(value) => *value,
        None => panic!("[error]: function stack is empty"),
    };
    let p2 = match get_priority(last) {
        Some(value) => value,
        None => panic!("[error]: unknown operator '{}'", last),
    };
    p1 >= 0 && p2 >= 0 && p1 >= p2
}

pub fn in2rpn(input: &str) -> String {
    let mut result = String::new();
    let it = InfixNotation::new(input);
    let mut func: Vec<&str> = Vec::new();
    for item in it {
        if is_digit(item) {
            result.push_str(&format!("{} ", item));
        } else {
            if item == ")" {
                while func.len() > 0 && *func.last().unwrap() != "(" {
                    let function = match func.pop() {
                        Some(value) => value,
                        None => panic!("[error]: function stack is empty"),
                    };
                    result.push_str(&format!("{} ", function));
                }
                func.pop();
            } else {
                while can_pop(item, &func) {
                    let function = match func.pop() {
                        Some(value) => value,
                        None => panic!("[error]: function stack is empty"),
                    };
                    result.push_str(&format!("{} ", function));
                }
                func.push(item);
            }
        }
    }
    while let Some(item) = func.pop() {
        result.push_str(&format!("{} ", item));
    }
    result
}
