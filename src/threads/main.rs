extern crate rand;
extern crate time;
use std::io;
use std::f64;
use std::thread;
use std::sync::mpsc::{self, Sender, Receiver};
use std::io::prelude::*;
use std::str::FromStr;
use rand::thread_rng;
use rand::distributions::{IndependentSample, Range};

fn f(x: f64) -> f64 {
    (x.powf(2.0) + 1.0).recip()
}

fn monte_carlo(a: f64, b: f64, n: i64) -> f64 {
    let between = Range::new(a, b);
    let mut rng = rand::thread_rng();
    let mut sum = 0.0;
    for _ in 0..n {
        let x = between.ind_sample(&mut rng);
        sum += f(x);
    }
    (b - a) * sum / n as f64
}

fn read_line<T: FromStr>(text: &str) -> Result<T, T::Err> {
    let mut buffer = String::new();
    print!("{}", text);
    io::stdout()
        .flush()
        .ok()
        .expect("[error] Can't flush to stdout!");
    io::stdin()
        .read_line(&mut buffer)
        .ok()
        .expect("[error] Can't read line!");
    buffer.trim().parse::<T>()
}

fn main() {
    let (a, b) = (0.0, 1.0);
    let iterations = read_line("Enter iteration count: ").unwrap_or(10000);
    let thread_count = read_line("Enter thread count: ").unwrap_or(8);
    let h = (b - a) / thread_count as f64;
    let step = iterations / thread_count;
    let mut threads = Vec::new();
    let (tx, rx): (Sender<f64>, Receiver<f64>) = mpsc::channel();
    println!("iteration count: {}", iterations);
    let start_time = time::get_time();
    for i in (0..iterations).filter(|&x| x % step == 0) {
        let thread_tx = tx.clone();
        threads.push(thread::spawn(move || {
            let xk = |k: i64| a + k as f64 * h;
            let (x0, x1) = (xk(i / step), xk(i / step + 1));
            let result_thread = monte_carlo(x0, x1, step);
            thread_tx.send(result_thread).unwrap();
            println!("#{} result = {:.50}", i / step, result_thread);
        }));
    }
    let mut result = 0.0;
    for thread in threads {
        thread.join()
              .ok()
              .expect("Can't join to thread!");
        result += rx.recv().unwrap();
    }
    let end_time = time::get_time();
    let pi = result * 4.0;
    println!("> real pi = {:.50}", f64::consts::PI);
    println!(">> result = {:.50}", pi);
    println!("> epsilon = {:.50}", (pi - f64::consts::PI).abs());
    println!("> {:?}", end_time - start_time);
}
