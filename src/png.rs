use std::fs::File;
use std::fs;
use std::io::Read;


pub struct PNGFile {
	pub buffer: Vec<u8>,
	pub is_valid: bool,
}

const PNG_SIGNATURE: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];

pub fn png_is_valid(data: &Vec<u8>) -> bool {
	let signature = &data[0..8];
	for i in 0..8 {
		if signature[i] != PNG_SIGNATURE[i] {
			return false;
		} 
	}
	true
}

pub fn read_png(filename: &String) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");
    buffer
}

pub fn open_png(filename: &String) -> PNGFile {
	let buffer = read_png(filename);
	let valid = png_is_valid(&buffer); 
	PNGFile {
		buffer: buffer,
		is_valid: valid
	}
}