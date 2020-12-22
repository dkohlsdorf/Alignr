#[macro_use]
extern crate image;

pub mod numerics;
pub mod audio;
pub mod spectrogram;

use std::fs::File;

pub fn plot(file: String, pixels: &[u8], rows: u32, cols: u32) {
    let output = File::create(format!("{}", file)).unwrap();
    let encoder = image::png::PNGEncoder::new(output);
    encoder.encode(&pixels, cols, rows, image::ColorType::L8).unwrap();
}

fn main() {
    println!("Hello, world!");
    let x    = vec![0.1,0.2,0.3,0.4];
    let y    = vec![0.2,0.2,0.3,0.4];
    let raw  = audio::AudioData::from_file("data/test.wav");
    let spec = spectrogram::Spectrogram::from_audio(256, 128, &raw); 
    let _ = plot(
       "spec.png".to_string(),
        &spec.img(),
        spec.len() as u32,
        spec.n_bins as u32,
    );
    println!("{}", numerics::euclidean(&x, &y));
    println!("{}", numerics::dtw1d(&x, &y, 0));
    println!("{:?}", numerics::hamming(11));
    println!("{:?}", &raw.data[0..10]);
}
