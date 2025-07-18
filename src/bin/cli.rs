// idea: add clap
// idea: provide config as file
// idea: add modes: dot/dash mode, string mode etc

use cwos::{core::apps::AppLauncher, prelude::*, std_context::StdContext};
use std::io::stdin;

pub fn main() {
	let mut controller = AppLauncher::default();

	loop {
		print!(">> ");
		let mut input_str = String::new();
		stdin().read_line(&mut input_str).unwrap();
		let input_str = input_str.trim();

		let input = CwString::from(input_str);
		let mut ctx = StdContext;
		let output = String::from(&controller.tick(&mut ctx, input));

		println!("{output}");

		println!();
	}
}
