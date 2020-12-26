use crate::spectrogram::*;
use crate::numerics::*;


pub enum BaseDistance {
    L2,
    DTW,
}


pub struct Alignment {
    gap_penalty: f32,
    sakoe_chiba: usize,
    base_distance: BaseDistance
}


impl Alignment {

    pub fn from_params(gap_penalty: f32, sakoe_chiba: usize, base_distance: BaseDistance) -> Alignment {
	Alignment{ gap_penalty: gap_penalty, sakoe_chiba: sakoe_chiba, base_distance: base_distance }
    }
    
    pub fn align(&self, x: &Spectrogram, y: &Spectrogram) {
	let n = x.len();
	let m = y.len();
	let w = usize::max(self.sakoe_chiba, abs_sub(n, m)) + 2;	
	for i in 1 .. n + 1 {
	    for j in usize::max(1, save_sub(i, w)) .. usize::min(m + 1, i + w)  {
		let distance = match self.base_distance {
		    BaseDistance::DTW => dtw1d(x.vec(i - 1), y.vec(j - 1), x.n_bins / 10),
		    _                 => euclidean(x.vec(i - 1), y.vec(j - 1))
		};
		println!("{} {} {}", i, j, distance);
	    }
	}
    }

} 
