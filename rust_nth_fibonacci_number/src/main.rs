extern crate num_bigint;
extern crate num_traits;

use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::mem::replace;

// Calculate large fibonacci numbers.
fn fib(n: usize) -> BigUint {
    let mut f0: BigUint = Zero::zero();
    let mut f1: BigUint = One::one();
    //let mut counter = 1;
    for _ in 0..n {
        let f2 = f0 + &f1;
        //counter += 1;
        //let x_f = x as f64;
        //let n_f = n as f64;
        //let percent = x_f / n_f  * 100.0;
        
        
        // swapping f0 with f1 and f1 with f2.
        f0 = replace(&mut f1, f2);
        //println!("{}%", percent);
    }
    f0
}

fn main() {
    println!("fib(1000000) = {}", fib(1000000));
}
