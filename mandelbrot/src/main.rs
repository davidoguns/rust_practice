use num::Complex;

#[allow(dead_code)]
fn complex_square_add_loop(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }

    Option::None{}
}

fn main() {
    println!("Hello, world!");
}
