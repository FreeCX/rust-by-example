use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

#[cfg(test)]
mod test {
    use super::executor;

    #[test]
    fn hello_world() {
        let code: Vec<u8> = concat!("++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++",
                                    ".>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.",
                                    "------.--------.>+.>.")
                                .bytes()
                                .collect();
        assert_eq!(executor(code, 5), "Hello World!\n");
    }
}

fn read_line(text: &str) -> String {
    let mut buffer = String::new();
    print!("{}", text);
    io::stdout()
        .flush()
        .ok()
        .expect("Не удалось очистить stdout!");
    io::stdin()
        .read_line(&mut buffer)
        .ok()
        .expect("Невозможно прочитать строку!");
    buffer.trim_right().to_string()
}

fn read_file(filename: String) -> Vec<u8> {
    let mut code: Vec<u8> = Vec::new();
    let path = Path::new(&filename);
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(error) => panic!("[ошибка]: {}", error),
    };
    file.read_to_end(&mut code)
        .ok()
        .expect("Невозможно прочитать файл!");
    code
}

fn executor(code: Vec<u8>, mem_size: usize) -> String {
    let mut io_buffer = String::new();
    let mut code_index: usize = 0;
    let mut cell_index: usize = 0;
    let mut func_index: Vec<usize> = Vec::new();
    let mut memory: Vec<i64> = (0..mem_size).map(|_| 0).collect();
    while code_index < code.len() {
        match code[code_index] as char {
            '+' => memory[cell_index] += 1,
            '-' => memory[cell_index] -= 1,
            '>' => {
                cell_index += if cell_index < mem_size {
                    1
                } else {
                    0
                }
            }
            '<' => {
                cell_index -= if cell_index > 0 {
                    1
                } else {
                    0
                }
            }
            '.' => io_buffer.push((memory[cell_index] as u8) as char),
            ',' => memory[cell_index] = io::stdin().bytes().next().unwrap().unwrap() as i64,
            '[' => func_index.push(code_index),
            ']' => {
                if memory[cell_index] > 0 {
                    code_index = func_index.pop().unwrap();
                    continue;
                } else {
                    func_index.pop();
                }
            }
            _ => {}
        }
        code_index += 1;
    }
    io_buffer
}

fn main() {
    let filename = read_line("Исполняемый файл: ");
    let code: Vec<u8> = read_file(filename);
    println!("{}", executor(code, 256));
}
