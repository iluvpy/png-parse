use std::env;

mod png;

fn main() {
    let args: Vec<String> = env::args().collect();
	if args.len() > 1 {
		let png_file = png::open_png(&args[1]);
		if png_file.is_valid {
			println!("Valid png file!");
		} else {
			println!("Invalid png file!");
		}
	}
}
