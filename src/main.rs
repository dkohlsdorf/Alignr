#[macro_use]
extern crate image;

pub mod numerics;
pub mod audio;
pub mod spectrogram;
pub mod align;
pub mod range;

use std::fs::File;
// 1) Output alignment paths with distances
// 2) Output aligned audio


pub fn plot(file: String, pixels: &[u8], rows: u32, cols: u32) {
    let output = File::create(format!("{}", file)).unwrap();
    let encoder = image::png::PNGEncoder::new(output);
    encoder.encode(&pixels, cols, rows, image::ColorType::L8).unwrap();
}


fn main() {
    println!("Hello, world!");
    let x = vec![0,0,0,1,2,3,3,3,4,5,6];
    let ranges = range::SpectrogramRange::from_path(&x);
    println!("{:?}", ranges);
    println!("{:?}", ranges[0].to_audio(128));
    //let x    = vec![0.1,0.2,0.3,0.4];
    //let y    = vec![0.2,0.2,0.3,0.4];
    //let raw  = audio::AudioData::from_file("data/test.wav");
    //let spec = spectrogram::Spectrogram::from_audio(256, 128, &raw); 
    //let _ = plot(
    //   "spec.png".to_string(),
    //    &spec.img(),
    //    spec.len() as u32,
    //    spec.n_bins as u32,
    //);
    //let a = align::Alignment::from_params(0.25, 2, align::BaseDistance::DTW);
    //let (w, pi, pj) = a.align(&spec, &spec);
    //let dtwdist = w[&(10, 9)];
    //println!("{}", numerics::euclidean(&x, &y));
    //println!("{}", numerics::dtw1d(&x, &y, 0));
    //println!("{:?}", numerics::hamming(11));
    //println!("{:?}", &raw.data[0..10]);
    //println!("{}", dtwdist);
    //println!("{:?}", pi);
    //println!("{:?}", pj);
}
