use std::f64::consts::PI;

// dichotomy method for functions of one variable
// more info:
//  * https://en.wikipedia.org/wiki/Bisection_method
//  * https://ru.wikipedia.org/wiki/Дихотомия
fn dichotomy_1p<F>(a: f64, b: f64, eps: f64, f: F) -> f64
    where F: Fn(f64) -> f64
{
    let (mut x0, mut x1) = (a, b);
    let mut x: f64;
    while (x1 - x0).abs() > eps {
        x = 0.5 * (x0 + x1);
        if f(x1) * f(x) < 0.0 {
            x0 = x;
        } else {
            x1 = x;
        }
    }
    0.5 * (x0 + x1)
}

// error calculation function
fn err_pi(x: f64) -> (f64, f64) {
    (x, (x - PI).abs())
}

// calculated function
fn f( x: f64 ) -> f64 {
    let pi4 = PI / 4.0;
    (x - pi4).cos().powi(3) + (x - pi4).sin().powi(3)
}

// auxiliary function
fn print_tuple( text: &str, tuple: ( f64, f64 ) ) {
    let (pi, err) = tuple;
    println!("{} pi = {:.15}, err = {:.15}", text, pi, err);
}

fn main() {
    // [a, b] -- interval
    //      n -- precision
    let (a, b, n) = (3.0, 4.0, 1.0e-15_f64);
    print_tuple("[dichotomy]", err_pi(dichotomy_1p(a, b, n, f)));
}
