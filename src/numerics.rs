pub fn min(x: &[f32]) -> f32 {
    let mut min = std::f32::INFINITY;
    for frame in x {
        if *frame < min && frame.is_finite() {
            min = *frame;
        }
    }
    min
}


pub fn max(x: &[f32]) -> f32 {
    let mut max = std::f32::NEG_INFINITY;
    for frame in x {
        if *frame > max && frame.is_finite() {
            max = *frame;
        }
    }
    max
}


pub fn mean(x: &[f32]) -> f32 {
    let mut mean = 0.0;
    for value in x.iter() {
        mean += value;
    }
    mean / (x.len() as f32)
}


pub fn std(x: &[f32], mu: f32) -> f32 {
    let mut std = 0.0;
    for value in x.iter() {
        std += f32::powf(value - mu, 2.0);
    }
    f32::sqrt(std / (x.len() as f32))
}


pub fn hamming(len: usize) -> Vec<f32> {
    let mut hamming = Vec::new();
    for i in 0 .. len {
	hamming.push(0.54 + 0.46 * f32::cos((2.0 * std::f32::consts::PI * i as f32) / (len - 1) as f32));
    }
    hamming
}


pub fn euclidean(x: &[f32], y: &[f32]) -> f32 {
    let mut distance = 0.0;
    for i in 0 .. x.len() {
	distance += f32::powf(x[i] - y[i], 2.0);
    }
    f32::sqrt(distance)
}


pub fn min3(x: f32, y: f32, z: f32) -> f32 {
    let mut min = std::f32::INFINITY;
    if x < min {
	min = x;
    }
    if y < min {
	min = y;
    }
    if z < min {
	min = z;
    }
    min
}


pub fn save_sub(x: usize, y: usize) -> usize {
    if x >= y {
	x - y
    } else {
	0
    }
}


pub fn abs_sub(x: usize, y: usize) -> usize {
    usize::max(save_sub(x, y), save_sub(y, x))
}


pub fn dtw1d(x: &[f32], y: &[f32], band: usize) -> f32 {
    let n = x.len();
    let m = y.len();
    let w = usize::max(band, abs_sub(n, m)) + 2;

    let mut dp = vec![std::f32::INFINITY; (n + 1)  * (m + 1)];
    dp[0] = 0.0;
    for i in 1 .. n + 1 {
	
	for j in usize::max(1, save_sub(i, w)) .. usize::min(m + 1, i + w)  {
	    let distance = f32::powf(x[i - 1] - y[j - 1], 2.0);
	    dp[i * m + j] = distance + min3(
		dp[(i - 1) * m + j],
		dp[(i - 1) * m + (j - 1)],
		dp[i * m + (j - 1 )]
	    );
	}
    }
    dp[n * m + m]   
}
