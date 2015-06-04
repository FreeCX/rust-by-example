extern crate rand;
use std::f64;
use std::thread;
use std::sync::{Arc, Mutex};
use rand::thread_rng;
use rand::distributions::{IndependentSample, Range};

fn f( x: f64 ) -> f64 {
    1.0 / ( x * x + 1.0 )
}

fn monte_carlo( a: f64, b: f64, n: i64 ) -> f64 {
    let between = Range::new( a, b );
    let mut rng = rand::thread_rng();
    let mut sum = 0.0;
    for _ in ( 0 .. n ) {
        let x = between.ind_sample( &mut rng );
        sum += f( x );
    }
    ( b - a ) * sum / n as f64 
}

fn main() {
    let ( a, b ) = ( 0.0, 1.0 );
    let iterations = 10000;
    let thread_count = 8;
    let h = ( b - a ) / thread_count as f64;
    let step = iterations / thread_count;
    let result: Arc<Mutex<f64>> = Arc::new( Mutex::new( 0.0 ) );
    let mut threads = Vec::new();
    println!( "iterations count: {}", iterations );
    for i in ( 0 .. iterations ).filter( |&x| x % step == 0 ) {
        let result = result.clone();
        threads.push( thread::spawn( move || {
            let xk = |k: i64| a + k as f64 * h;
            let ( x0, x1 ) = ( xk( i / step ), xk( i / step + 1 ) );
            let result_thread = monte_carlo( x0, x1, step );
            let mut result = result.lock().unwrap();
            *result += result_thread;
            println!( "#{} result = {:.50}", i / step, result_thread );
        }));
    }
    for thread in threads {
        thread.join()
            .ok()
            .expect( "Can't join to thread!" );
    }
    let result = result.lock().unwrap();
    let pi = *result * 4.0;
    println!( "> real pi = {:.50}", f64::consts::PI );
    println!( ">> result = {:.50}", pi );
    println!( "> epsilon = {:.50}", ( pi - f64::consts::PI).abs() );
}
