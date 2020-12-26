use hound::*;
use std::iter::FromIterator;

use crate::range::*;

#[derive(Clone)]
pub struct AudioData {
    pub spec: WavSpec,
    pub data: Vec<i16>
}


impl AudioData {

    
    pub fn from_file(file: &str) -> AudioData {
        let mut reader = WavReader::open(file).unwrap();
        let n_channels = reader.spec().channels as usize;
        let samples = reader
            .samples::<i16>()
            .enumerate()
            .filter_map(|(i, x)| {
                if i % n_channels == 0 {
                    Some(x.unwrap())
                } else {
                    None
                }
            })
            .collect();
        let mut spec = reader.spec();
        spec.channels = 1;
        AudioData {
            spec,
            data: samples,
        }
    }

    
    pub fn append(&mut self, insert_zeros: usize, audio: &mut AudioData) {
        for _i in 0..insert_zeros {
            self.data.push(0);
        }
        self.data.append(&mut audio.data);
    }

    
    pub fn write(&self, file: String) {
        let mut writer = hound::WavWriter::create(file, self.spec).unwrap();
        for sample in self.data.iter() {
            writer.write_sample(*sample).unwrap();
        }
	writer.finalize().unwrap();
    }

    pub fn write_alignment(&self, ranges: &[SpectrogramRange], file: String) {
        let mut writer = hound::WavWriter::create(file, self.spec).unwrap();
	for range in ranges {
	    for sample in self.data[range.start .. range.stop].iter() {
		if range.is_gap {
		    writer.write_sample(0).unwrap();
		} else {
		    writer.write_sample(*sample).unwrap();
		}
	    }
	}
	writer.finalize().unwrap();
    }
    
}
