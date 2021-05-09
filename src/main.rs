pub mod shell;
pub mod glass;

use std::io::{self, Write};

fn main() {
	shell::run().debug();
}

pub fn input(msg: &str) -> String {
	print!("{}", msg);
	let mut buf: String = String::new();

	io::stdout().flush().unwrap();
	io::stdin().read_line(&mut buf).unwrap();

	buf.trim().to_string()
}

pub fn debug(msg: String) { println!("debug: {}", msg); }