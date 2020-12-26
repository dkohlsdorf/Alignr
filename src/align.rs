use crate::spectrogram::*;
use crate::numerics::*;

use std::collections::HashMap;


pub enum BaseDistance {
    L2,
    DTW(usize),
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

    pub fn align(&self, x: &Spectrogram, y: &Spectrogram) -> (HashMap<(usize, usize), f32>, Vec<usize>, Vec<usize>) {
	let n = x.len();
	let m = y.len();
	let w = usize::max(self.sakoe_chiba, abs_sub(n, m)) + 2;	

	let mut dp = HashMap::<(usize, usize), f32>::new();
	let mut bp = HashMap::<(usize, usize), (usize, usize)>::new();
	
	dp.insert((0,0), 0.0);
	for i in 1 .. n + 1 {
	    for j in usize::max(1, save_sub(i, w)) .. usize::min(m + 1, i + w)  {
		let distance = match self.base_distance {
		    BaseDistance::DTW(warp) => dtw1d(x.vec(i - 1), y.vec(j - 1), warp),
		    _                       => euclidean(x.vec(i - 1), y.vec(j - 1))
		};
		let match_score = match dp.get(&(i-1, j-1)) {
		    Some(score) => *score,
		    None => std::f32::INFINITY
		};
		let insert_score = match dp.get(&(i-1, j)) {
                    Some(score) => *score,
                    None => std::f32::INFINITY
                };
                let delete_score = match dp.get(&(i, j-1)) {
                    Some(score) => *score,
                    None => std::f32::INFINITY
                };
		
		if delete_score < match_score && delete_score < insert_score {
		    bp.insert((i, j), (i, j - 1));
		    dp.insert((i,j), delete_score + self.gap_penalty + distance);
		} else if insert_score < match_score && insert_score < delete_score {
		    bp.insert((i, j), (i - 1, j));
		    dp.insert((i,j), insert_score + self.gap_penalty + distance);
		} else {
		    bp.insert((i, j), (i - 1, j - 1));
		    dp.insert((i,j), match_score + distance);
		}
	    }	    
	}
	let mut path_i = Vec::<usize>::new();
	let mut path_j = Vec::<usize>::new();
	let mut i = n;
	let mut j = m;
	while i > 0 && j > 0 {
	    let (_i, _j) = bp[&(i,j)];
	    path_i.push(i);
	    path_j.push(j);
	    i = _i;
	    j = _j;
	}
	path_i.push(0);
	path_j.push(0);
	path_i.reverse();
	path_j.reverse();
	(dp, path_i, path_j)
    }

} 
