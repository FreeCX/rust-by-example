// inspired by http://habrahabr.ru/post/250817/
extern crate rand;
use std::io;
use std::io::prelude::*;
use rand::{thread_rng, Rng};
use std::str::FromStr;

fn read_line<T: FromStr>(text: &str) -> Result<T, T::Err> {
    let mut buffer = String::new();
    print!("{}", text);
    io::stdout()
        .flush()
        .expect("Couldn't flush to stdout!");
    io::stdin()
        .read_line(&mut buffer)
        .expect("Couldn't read line!");
    buffer.trim().parse::<T>()
}

fn main() {
    let max_prisoners: usize = read_line("Input prisoners count: ").unwrap_or(0);
    let max_attempt: usize = read_line("Input number of attempts: ").unwrap_or(0);
    let max_experiment = read_line("Input experiments count: ").unwrap_or(0);
    let mut life_counter: i16 = 0;
    let mut death_counter: i16 = 0;
    for _ in 0..max_experiment {
        let mut number: Vec<usize> = (0..max_prisoners).collect();
        let mut pbox: Vec<usize> = Vec::new();
        let mut life_status: bool = false;
        // shuffle numbers array
        thread_rng().shuffle(&mut *number);
        for i in 0..max_prisoners {
            // put number from array to box
            pbox.push(number[i]);
        }
        // loop by prisoners
        for prisoner in 0..max_prisoners {
            let mut next = prisoner;
            // loop by attempts
            for _ in 0..max_attempt {
                // if prisoner number in the box,
                // then go to the next
                if pbox[next] == prisoner {
                    life_status = true;
                    break;
                } else {
                    // otherwise we continue to open the box
                    next = pbox[next];
                    life_status = false;
                }
            }
        }
        match life_status {
            true => life_counter += 1,
            false => death_counter += 1,
        }
    }
    println!("Experiments count {}", max_experiment);
    println!(" > Number of survival - {} [ {} % ]",
             life_counter, (life_counter * 100) as f32 / max_experiment as f32);
    println!(" > Number of deaths   - {} [ {} % ]",
             death_counter, (death_counter * 100) as f32 / max_experiment as f32);
}
