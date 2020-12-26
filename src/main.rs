#[macro_use]
pub mod numerics;
pub mod audio;
pub mod spectrogram;
pub mod align;
pub mod range;


fn main() {
    let fft_win     = 256;
    let fft_step    = 128;
    let gap_penalty = 2.0;
    let w           = 50;
    let in_x        = "data/x.wav";
    let in_y        = "data/y.wav";
    let out_x       = "aligned_x.wav";
    let out_y       = "aligned_y.wav";
    
    let raw_x = audio::AudioData::from_file(in_x);
    let raw_y = audio::AudioData::from_file(in_y);
    let spec_x = spectrogram::Spectrogram::from_audio(fft_win, fft_step, &raw_x);
    let spec_y = spectrogram::Spectrogram::from_audio(fft_win, fft_step, &raw_y);

    let aligner = align::Alignment::from_params(gap_penalty, w, align::BaseDistance::DTW(4));
    let (dp, xi, yi) = aligner.align(&spec_x, &spec_y);
    let ri = range::SpectrogramRange::from_path(&xi);
    let rj = range::SpectrogramRange::from_path(&yi);

    let ranges_x: Vec<range::SpectrogramRange> = ri.iter().map(|x| x.to_audio(128)).collect();
    let ranges_y: Vec<range::SpectrogramRange> = rj.iter().map(|x| x.to_audio(128)).collect();
    
    raw_x.write_alignment(&ranges_x, out_x.to_string());
    raw_y.write_alignment(&ranges_y, out_y.to_string());

    let frames_x = spec_x.len();
    let frames_y = spec_y.len();
    let score    = dp[&(frames_x, frames_y)];
    println!("alignment_score: {} length_x: {} length_y: {}", score, frames_x, frames_y)
}


