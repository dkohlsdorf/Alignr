use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;
use rustfft::FFTplanner;

use crate::audio::*;
use crate::numerics::*;

pub struct Spectrogram {
    pub n_bins: usize,
    pub spec: Vec<f32>
}


impl Spectrogram {

    
    pub fn from_audio(fft_size: usize, fft_step: usize, raw_audio: &AudioData) -> Spectrogram {
	let samples: Vec<Complex<f32>> = raw_audio
            .data
            .iter()
            .map(|x| Complex::new(f32::from(*x), 0.0))
            .collect();

	let hamming = hamming(fft_size);
	let mut planner_dft = FFTplanner::new(false);
        let fft = planner_dft.plan_fft(fft_size);
	let n = samples.len();

	let mut spectrogram: Vec<f32> = Vec::new();        
        for i in (fft_size .. n).step_by(fft_step) {
            let start = i - fft_size;
            let stop  = i;
            let mut output: Vec<Complex<f32>> = vec![Complex::zero(); fft_size];
            let mut input: Vec<Complex<f32>>  = samples[start..stop]
                .iter()
                .enumerate()
                .map(|(i, x)| x * hamming[i])
                .collect();
            fft.process(&mut input[..], &mut output);
            let result: Vec<f32> = output
                .iter()
                .map(|complex| f32::sqrt(complex.norm_sqr()))
                .take(fft_size / 2)
                .collect();
	    let mu_spec  = mean(&result[0..result.len()]);
            let std_spec = f32::max(std(&result[0..result.len()], mu_spec), 1.0);
            for result in result.iter() {
                spectrogram.push((result - mu_spec) / std_spec);
            }
	}

	Spectrogram {
            n_bins: fft_size / 2,
            spec: spectrogram
        }
    }

    
    pub fn vec(&self, t: usize) -> &[f32] {
        &self.spec[t * self.n_bins..(t + 1) * self.n_bins]
    }

    
    pub fn len(&self) -> usize {
        self.spec.len() / (self.n_bins as usize)
    }

}
