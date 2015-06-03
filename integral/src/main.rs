extern crate rand;
extern crate time;
use std::io;
use std::f64;
use std::io::prelude::*;
use rand::thread_rng;
use rand::distributions::{IndependentSample, Range};

fn f( x: f64 ) -> f64 {
    1.0 / ( x * x + 1.0 )
}

fn trapezoids_v1( a: f64, b: f64, n: i64 ) -> f64 {
    let mut sum: f64 = 0.0;
    let h = ( b - a ) / n as f64;
    let xk = |k: i64| a + k as f64 * h;
    for i in ( 0 .. n - 1 ) {
        let ( x0, x1 ) = ( xk( i ), xk( i + 1 ) );
        sum += 0.5 * ( f( x0 ) + f( x1 ) ) * ( x1 - x0 );
    }
    sum
}

fn trapezoids_v2( a: f64, b: f64, n: i64 ) -> f64 {
    let mut sum: f64 = 0.0;
    let h = ( b - a ) / n as f64;
    let xk = |k: i64| a + k as f64 * h;
    for i in ( 1 .. n - 1 ) {
        sum += f( xk( i ) );
    }
    h * ( 0.5 * ( f( a ) + f( b ) ) + sum )
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

fn rectangle( a: f64, b: f64, n: i64 ) -> ( f64, f64, f64 ) {
    let ( mut s_left, mut s_middle, mut s_right ) = ( 0.0, 0.0, 0.0 );
    let rectangle_left = |x0: f64, x1: f64| {
        f( x0 ) * ( x1 - x0 )
    };
    let rectangle_middle = |x0: f64, x1: f64| {
        f( 0.5 * ( x0 + x1 ) ) * ( x1 - x0 )
    };
    let rectangle_right = |x0: f64, x1: f64| {
        f( x0 ) * ( x0 - x1 )
    };
    let h = ( b - a ) / n as f64;
    let xk = |k: i64| a + k as f64 * h;
    for i in ( 0 .. n - 1 ) {
        let ( x0, x1 ) = ( xk( i + 0 ), xk( i + 1 ) );
        s_left += rectangle_left( x0, x1 );
        s_middle += rectangle_middle( x0, x1 );
        s_right += rectangle_right( x1, x0 );
    }
    ( s_left, s_middle, s_right )
}

fn simpson( a: f64, b: f64 ) -> f64 {
    ( ( b - a ) / 6.0 ) * ( f( a ) + 4.0 * f ( 0.5 * ( a + b ) ) + f( b ) )
}

fn simpson_kosates( a: f64, b: f64, n: i64 ) -> f64 {
    let h = ( b - a ) / n as f64;
    let xk = |k: i64| a + k as f64 * h;
    let mut sum = 0.0;
    for i in ( 1 .. n - 1 ).filter( |&x| x % 2 == 1 ) {
        let ( x0, x1, x2 ) = ( xk( i - 1 ), xk( i ), xk( i + 1 ) );
        sum += f( x0 ) + 4.0 * f( x1 ) + f( x2 );
    }
    ( h / 3.0 ) * sum
}

fn gauss( a: f64, b: f64 ) -> f64 {
    let bma = b - a;
    let bpa = b + a;
    let bpa2 = 0.5 * bpa;
    let tst = 2.0 * ( 3.0 as f64 ).sqrt();
    let tst2 = bma / tst;
    0.5 * bma * ( f( bpa2 - tst2 ) + f( bpa2 + tst2 ) )
}

fn print_tuple( text: &str, tuple: (f64, f64) ) {
    let ( pi, eps ) = tuple;
    println!( "{} pi = {:.50}, eps = {:.50}", text, pi, eps );
}

fn main() {
    let ( a, b ) = ( 0.0, 1.0 );
    let mut buffer = String::new();
    print!( "Enter iterations: " );
    io::stdout().flush()
        .ok()
        .expect( "Can't flush!" );
    io::stdin().read_line( &mut buffer )
        .ok()
        .expect( "Can't read line!" );
    let n = match buffer.trim().parse() {
        Ok( value )  => value,
        Err( error ) => {
            println!( "[error] {}; using default value.", error );
            1000
        }
    };
    println!( "Iteration count = {}", n );
    let pi = f64::consts::PI;
    let peps = |x: f64| ( x, ( x - pi ).abs() );
    let start_time = time::get_time();
    let tr1 = 4.0 * trapezoids_v1( a, b, n );
    let tr2 = 4.0 * trapezoids_v2( a, b, n );
    let mc = 4.0 * monte_carlo( a, b, n );
    let ( mut lr, mut mr, mut rr ) = rectangle( a, b, n );
    lr *= 4.0; mr *= 4.0; rr *= 4.0;
    let sm = 4.0 * simpson( a, b );
    let sk = 4.0 * simpson_kosates( a, b, n );
    let gm = 4.0 * gauss( a, b );
    let end_time = time::get_time();
    println!( "[     pi constant] pi = {:.50}", pi );
    print_tuple( "[   trapezoids v1]", peps( tr1 ) );
    print_tuple( "[   trapezoids v2]", peps( tr2 ) );
    print_tuple( "[     monte-carlo]", peps( mc ) );
    print_tuple( "[  rectangle left]", peps( lr ) );
    print_tuple( "[rectangle middle]", peps( mr ) );
    print_tuple( "[ rectangle right]", peps( rr ) );
    print_tuple( "[         simpson]", peps( sm ) );
    print_tuple( "[ simpson-kosates]", peps( sk ) );
    print_tuple( "[           gauss]", peps( gm ) );
    println!( "> {:?}", end_time - start_time );
}
