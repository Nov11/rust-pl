extern crate num;
extern crate image;

use image::ColorType;
use image::png::PNGEncoder;
use std::fs::File;
use std::str::FromStr;
use std::io::Write;

use num::Complex;

fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };

    for i in 0..limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
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
        Some((re, im)) => Some(Complex { re, im }),
        None => None
    }
}

fn pixel_to_point(bounds: (usize, usize), pixel: (usize, usize), upper_left: Complex<f64>, lower_right: Complex<f64>) -> Complex<f64> {
    let (w, h) = (lower_right.re - upper_left.re, upper_left.im - lower_right.im);

    //say :
    //..
    //h
    //2
    //1
    //0 -> 1 -> 2 -> .. w
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

fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;

    let encoder = PNGEncoder::new(output);

    encoder.encode(&pixels, bounds.0 as u32, bounds.1 as u32, ColorType::Gray(8))?;
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 5 {
        writeln!(std::io::stderr(), "blablabla").unwrap();
        std::process::exit(1);
    }

    let bounds = parse_pair(&args[2], 'x').expect("image dimensions");
    let upper_left = parse_complex(&args[3]).expect("upper left");
    let lower_right = parse_complex(&args[4]).expect("lower right");

    let mut pixels = vec![0; bounds.0 * bounds.1];

//    render(&mut pixels, bounds, upper_left, lower_right);

    let threads = 8;
    let rows_per_band = (bounds.1 + threads - 1) / threads;
    let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * bounds.0).collect();

    crossbeam::scope(|spawner| {
        for (i, band) in bands.into_iter().enumerate() {
            let top = rows_per_band * i;
            let height = band.len() / bounds.0;
            let band_bounds = (bounds.0, height);
            let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
            let band_lower_right = pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);

            spawner.spawn(move || {
                render(band, band_bounds, band_upper_left, band_lower_right);
            });
        }
    });

    write_image(&args[1], &pixels, bounds).expect("error write file");
}
