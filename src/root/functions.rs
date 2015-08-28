pub fn dichotomy_1p<F>( a: f64, b: f64, eps: f64, f: F ) -> f64 where F: Fn( f64 ) -> f64 {
    let ( mut x0, mut x1 ) = ( a, b );
    let mut x: f64;
    while ( x1 - x0 ).abs() > eps {
        x = 0.5 * ( x0 + x1 );
        if f( x1 ) * f( x ) < 0.0 {
            x0 = x;
        } else {
            x1 = x;
        }
    }
    0.5 * ( x0 + x1 )
}

pub fn err_pi( x: f64 ) -> ( f64, f64 ) {
    use std::f64::consts::PI;
    ( x, ( x - PI ).abs() )
}