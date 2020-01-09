use rand::Rng;
use std::env;

const CHARACTORS: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabdefghijklmnopqrstuvwxyz0123456789_-";
const DEFAULT_SIZE: u32 = 10;

fn main() {
	let mut rng = rand::thread_rng();
	let args: Vec<String> = env::args().collect();
	let size = &args
		.get(1)
		.unwrap_or(&DEFAULT_SIZE.to_string())
		.parse()
		.unwrap();

	let mut result = "".to_string();

	for _ in 0..*size {
		let i: usize = rng.gen_range(0, CHARACTORS.len() - 1);
		result += &CHARACTORS
			.chars()
			.nth(i)
			.map(|char| char.to_string())
			.unwrap();
	}

	print!("{}", result);
}
