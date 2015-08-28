mod functions;
use functions::*;

fn f( x: f64 ) -> f64 {
    use std::f64::consts::PI;
    let pi4 = PI / 4.0;
    ( x - pi4 ).cos().powi( 3 ) + ( x - pi4 ).sin().powi( 3 )
}

fn print_tuple( text: &str, tuple: ( f64, f64 ) ) {
    let ( pi, err ) = tuple;
    println!( "{} pi = {:.15}, err = {:.15}", text, pi, err );
}

fn main() {
    print_tuple( "[dichotomy]", err_pi( dichotomy_1p( 3.0, 4.0, 1.0e-15_f64, f ) ) );
}
