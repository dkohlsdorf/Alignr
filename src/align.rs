use crate::spectrogram::*;
use crate::numerics::*;

use std::sync::Arc;
use std::thread;
use std::collections::HashMap;


#[derive(Copy, Clone, Debug)]
pub enum BaseDistance {
    L2,
    DTW(usize),
}


pub struct Alignment {
    gap_penalty: f32,
    sakoe_chiba: usize,
    base_distance: BaseDistance,
    n_workers: usize
}


impl Alignment {

    pub fn from_params(gap_penalty: f32, sakoe_chiba: usize, base_distance: BaseDistance, n_workers: usize) -> Alignment {
	Alignment{ gap_penalty: gap_penalty, sakoe_chiba: sakoe_chiba, base_distance: base_distance, n_workers: n_workers }
    }

    pub fn precompute(spec_x: Arc<Spectrogram>, spec_y: Arc<Spectrogram>, base_distance: BaseDistance, n_workers: usize, w: usize) -> HashMap<(usize, usize), f32> {
	let n = spec_x.len();
	let m = spec_y.len();

	let mut jobs = vec![];
	for i in 1 .. n + 1 {
	    for j in usize::max(1, save_sub(i, w)) .. usize::min(m + 1, i + w)  {
		jobs.push((i, j))
	    }
	}
	let batch_size = (jobs.len() / n_workers) + 1;
	let n_jobs = jobs.len();
	let distance_jobs: Arc<Vec<(usize, usize)>> = Arc::from(jobs);
	let mut children = vec![];
        for batch in 0 .. n_workers {
	    let distance_jobs = Arc::clone(&distance_jobs);
	    let spec_x = Arc::clone(&spec_x);
	    let spec_y = Arc::clone(&spec_y);

            let start = batch * batch_size;
            let stop = usize::min((batch + 1) * batch_size, n_jobs);
	    let th = thread::spawn(move || {
		let mut dist = HashMap::<(usize, usize), f32>::new();
		for job in start .. stop {		    
		    let (i, j) = (*distance_jobs)[job];
		    let distance = match base_distance {
			BaseDistance::DTW(warp) => dtw1d(spec_x.vec(i - 1), spec_y.vec(j - 1), warp),
			_                       => euclidean(spec_x.vec(i - 1), spec_y.vec(j - 1))
		    };
		    dist.insert((i,j), distance); 
		}
		dist
	    });
	    children.push(th);
	}
	let mut dist = HashMap::<(usize, usize), f32>::new();
	for child in children {
            let job = child.join().unwrap();
	    dist = dist.into_iter().chain(job).collect();
        }
	dist
    }
    
    pub fn align(&self, x: Arc<Spectrogram>, y: Arc<Spectrogram>) -> (HashMap<(usize, usize), f32>, Vec<usize>, Vec<usize>) {
	let n = x.len();
	let m = y.len();
	let w = usize::max(self.sakoe_chiba, abs_sub(n, m)) + 2;	

	let mut dp = HashMap::<(usize, usize), f32>::new();
	let mut bp = HashMap::<(usize, usize), (usize, usize)>::new();
	let distances = Alignment::precompute(x, y, self.base_distance, self.n_workers, w);
	dp.insert((0,0), 0.0);
	for i in 1 .. n + 1 {
	    for j in usize::max(1, save_sub(i, w)) .. usize::min(m + 1, i + w)  {
		let distance = distances[&(i,j)];
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
