extern crate clap;
use clap::{Arg, App};
use std::sync::Arc;

pub mod numerics;
pub mod audio;
pub mod spectrogram;
pub mod align;
pub mod range;


fn main() {
    let matches = App::new("Alignr")
	.version("1.0")
	.author("Daniel Kohlsdorf <dkohlsdorf@gmail.com>")
	.about("Computes pairwise alignment between two audio files")
	.arg(Arg::with_name("fft_win")
	     .short("w")
	     .long("fft_win")
	     .help("Windoe size for spectrogram computation DEFAULT 256")
	     .takes_value(true))
	.arg(Arg::with_name("fft_step")
	     .short("s")
	     .long("fft_step")
	     .help("Step size if window for spectrogram computation DEFAULT 128")
	     .takes_value(true))
	.arg(Arg::with_name("gap")
	     .short("g")
	     .long("gap")
	     .help("Gap Penalty for DTW computation DEFAULT 0.0")
	     .takes_value(true))	
	.arg(Arg::with_name("n_workers")
	     .short("n")
	     .long("n_workers")
	     .help("Number of parallel workers DEFAULT 1")
	     .takes_value(true))	
	.arg(Arg::with_name("sakoe_chiba")
	     .short("b")
	     .long("band")
	     .help("Sakoe Chiba band for DTW computation DEFAULT 1.0")
	     .takes_value(true))
	.arg(Arg::with_name("in1")
	     .long("in1")
	     .help("First input file")
	     .required(true)
	     .takes_value(true))
	.arg(Arg::with_name("in2")
	     .long("in2")
	     .help("Second input file")
	     .required(true)
	     .takes_value(true))
	.arg(Arg::with_name("out1")
	     .long("out1")
	     .help("First output file DEFAULT align_x.wav")
	     .takes_value(true))
	.arg(Arg::with_name("out2")
	     .long("out2")
	     .help("Second output file DEFAULT align_y.wav")
	     .takes_value(true))
	.arg(Arg::with_name("dtw_frame")
	     .long("dynamic")
	     .help("[Otional] Allow freuqncy shift by using dtw to compute the frame distance. This parameter sets the wraping band or the allowed shift. If not set or set to 0, euclidean distance is used.")
	     .takes_value(true))
	.get_matches();
    
    let fft_win     = matches.value_of("fft_win").unwrap_or("256").parse::<usize>().unwrap();
    let fft_step    = matches.value_of("fft_step").unwrap_or("128").parse::<usize>().unwrap();
    let gap_penalty = matches.value_of("gap").unwrap_or("0.0").parse::<f32>().unwrap();
    let warp_perc   = matches.value_of("sakoe_chiba").unwrap_or("1.0").parse::<f32>().unwrap();
    let n_workers   = matches.value_of("n_workers").unwrap_or("1").parse::<usize>().unwrap();
    let in_x        =  matches.value_of("in1").unwrap();
    let in_y        =  matches.value_of("in2").unwrap();
    let out_x       =  matches.value_of("out1").unwrap_or("align_x.wav");
    let out_y       =  matches.value_of("out2").unwrap_or("align_y.wav");

    let frame_dtw_band = matches.value_of("dtw_frame").unwrap_or("0").parse::<usize>().unwrap();
    let base_dist      = if frame_dtw_band > 0 { align::BaseDistance::DTW(frame_dtw_band) } else { align::BaseDistance::L2 };
    println!("Params: fft (win: {} step: {}) alignment (gap: {} warp: {}) base {:?}",fft_win, fft_step, gap_penalty, warp_perc, base_dist);
    println!("Aligning: {} and {} to {} and {}", in_x, in_y, out_x, out_y);
    let raw_x = audio::AudioData::from_file(in_x);
    let raw_y = audio::AudioData::from_file(in_y);

    let spec_x = spectrogram::Spectrogram::from_audio(fft_win, fft_step, &raw_x);
    let spec_y = spectrogram::Spectrogram::from_audio(fft_win, fft_step, &raw_y);

    let frames_x = spec_x.len();
    let frames_y = spec_y.len();
    let w = ((usize::max(frames_x, frames_y) as f32) * warp_perc) as usize;
    
    let x = Arc::from(spec_x);
    let y = Arc::from(spec_y);
    let aligner = align::Alignment::from_params(gap_penalty, w, base_dist, n_workers);

    let (dp, xi, yi) = aligner.align(x, y);
    let ri = range::SpectrogramRange::from_path(&xi);
    let rj = range::SpectrogramRange::from_path(&yi);

    let ranges_x: Vec<range::SpectrogramRange> = ri.iter().map(|x| x.to_audio(128)).collect();
    let ranges_y: Vec<range::SpectrogramRange> = rj.iter().map(|x| x.to_audio(128)).collect();
    
    raw_x.write_alignment(&ranges_x, out_x.to_string());
    raw_y.write_alignment(&ranges_y, out_y.to_string());

    let score    = dp[&(frames_x, frames_y)];
    println!("alignment_score: {} length_x: {} length_y: {}", score, frames_x, frames_y)
}
