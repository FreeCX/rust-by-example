extern crate time;
use std::sync::{Arc, mpsc};
use std::thread;

type Float = f64;
type Integer = u32;
type Callback = Box<Fn(Float) -> Float + Send + Sync + 'static>;
type FnThread = Fn() -> Callback + Send + Sync;

#[derive(Clone)]
struct Integrator {
    f: Arc<FnThread>,
    a: Float,
    b: Float,
    n: Integer,
}

impl Integrator {
    fn new(func: Arc<FnThread>, a: Float, b: Float, iteration: Integer) -> Integrator {
        Integrator {
            f: func,
            a: a,
            b: b,
            n: iteration,
        }
    }
    fn call(&self, x: Float) -> Float {
        (self.f)()(x)
    }
    fn monte_carlo(&self, threads: Integer) -> Float {
        let mut thread_list = Vec::new();
        let h_step = (self.b - self.a) / self.n as Float;
        let t_step = self.n / threads;
        let (tx, rx) = mpsc::channel::<Float>();
        for i in 0..threads {
            let local_tx = tx.clone();
            let local_self = self.clone();
            thread_list.push(thread::spawn(move || {
                let u_i = |i: Integer| -> Float { local_self.a + h_step * i as Float };
                let (x0, x1) = (t_step * i, t_step * (i+1));
                let sum = (x0..x1).fold(0.0, |acc, i| acc + local_self.call(u_i(i)));
                local_tx.send(sum)
                    .ok()
                    .expect("Data not sended!");
            }));
        }
        let mut result = 0.0;
        for thread in thread_list {
            thread.join()
                  .ok()
                  .expect("Thread can't joined!");
            result += rx.recv()
                        .ok()
                        .expect("Data not recieved!");
        }
        result * h_step
    }
}

fn monte_carlo_linear(f: Arc<FnThread>, a: Float, b: Float, n: Integer) -> Float {
    let h = (b - a) / n as Float;
    let u_i = |i: Integer| -> Float { a + h * i as Float };
    (0..n).fold(0.0, |acc, x| acc + (f())(u_i(x))) * h
}

fn f() -> Callback {
    Box::new(|x: Float| -> Float {
        (x.powf(2.0) + 1.0).recip()
    })
}

fn main() {
    let (a, b, n) = (0.0, 1.0, 1_000_000_000);
    let f_a = Integrator::new(Arc::new(f), a, b, n);
    println!("# Iteration count: {:E}", n as Float);
    let start = time::get_time();
    let pi = monte_carlo_linear(Arc::new(f), a, b, n) * 4.0;
    let duration = time::get_time() - start;
    println!("# Linear code");
    println!("result = {:+.16}", pi);
    println!("   err = {:+.16}", std::f64::consts::PI - pi);
    println!("  time = {} ms\n", duration.num_milliseconds());
    for threads in (1..9).filter(|&x| x % 2 == 0) {
        println!("# Thread count: {}", threads);
        let start = time::get_time();
        let pi = f_a.monte_carlo(threads) * 4.0;
        let duration = time::get_time() - start;
        println!("result = {:+.16}", pi);
        println!("   err = {:+.16}", std::f64::consts::PI - pi);
        println!("  time = {} ms\n", duration.num_milliseconds());
    }
}