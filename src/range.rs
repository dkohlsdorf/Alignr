#[derive(Debug)]
pub struct SpectrogramRange {
    pub start: usize,
    pub stop: usize,
    pub is_gap: bool 
}

impl SpectrogramRange {

    pub fn to_audio(&self, fft_step: usize) -> SpectrogramRange {
	let start = self.start * fft_step;
	let stop  = self.stop * fft_step;
	SpectrogramRange { start: start, stop: stop, is_gap: self.is_gap}
    }
    
    pub fn from_path(path: &[usize]) -> Vec<SpectrogramRange> {
	let mut ranges = Vec::<SpectrogramRange>::new();
	let mut start = 0;
	let mut is_gap = false;
	for i in 1 ..  path.len() {
	    let current_gap = path[i] == path[i - 1];
	    if is_gap && !current_gap {
		let r = SpectrogramRange {start: 0, stop: i - start, is_gap: true};
		ranges.push(r);
		is_gap = current_gap;
		start = i;
	    }
	    if !is_gap && current_gap {
		let r = SpectrogramRange {start: path[start], stop: path[i], is_gap: false};
		ranges.push(r);
		is_gap = current_gap;
		start = i;
	    }
	}
	if is_gap {
	    let r = SpectrogramRange {start: 0, stop: path.len() - start, is_gap: true};
	    ranges.push(r);
	} else {
            let r = SpectrogramRange {start: path[start], stop: path[path.len() - 1], is_gap: false};
            ranges.push(r);	    
	}
	
	ranges
    }

}
