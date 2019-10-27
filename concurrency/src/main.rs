extern crate num;

use std::str::FromStr;

use num::Complex;

fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex { re: 0, im: 0 };

    for i in 0..limit {
        z = z * z + c;
        if z.norm_sqe() > 4 {
            return Some(i);
        }
    }

    None
}

fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(i) => {
            match (T::from_str(&s[..i]), T::from_str(&s[i + 1..])) {
                (Ok(v1), Ok(v2)) => {
                    Some((v1, v2))
                }
                _ => None
            }
        }
    }
}

fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Complex { re, im },
        None => None
    }
}


fn pixel_to_point(bounds: (useize, usize), pixel: (usize, usize), upper_left: Complex<f64>, lower_right: Complex<f64>) -> Complex<f64> {
    let (w, h) = (lower_right.re - upper_left.re, upper_left.im - lower_right.im);

    //say using 4th Quadrant
    //0 -> 1 -> 2 -> .. w
    //1
    //2
    //..
    //h
    Complex {
        re: upper_left.re + pixel.0 as f64 * w / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * h / bounds.1 as f64,
    }
}

fn render(pixels: &mut [u8], bounds: (usize, usize), upper_left: Complex<f64>, lower_right: Complex<f64>) {
    for row in 0..bounds.1 {
        for col in 0..bounds.0 {
            let point = pixel_to_point(bounds, (col, row), upper_left, lower_right);
            pixels[row * bounds.0 + col] =
                match escape_time(point, 255) {
                    None => 0,
                    Some(c) => 255 - c as u8
                }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
