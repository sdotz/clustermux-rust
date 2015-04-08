extern crate getopts;
use getopts::Options;
use std::env;


fn tmux(stack: &str) {
	println!("You chose stack {}", stack);
}

fn print_usage(program: &str, opts: Options) {
	let brief = format!("Usage: {} [options]", program);
	print!("{}", opts.usage(&brief));
}

fn main() {
    let args : Vec<String> = env::args().collect();
	let program = args[0].clone();

	let mut opts = Options::new();

	opts.optopt("s", "stack", "specify stack to access", "STACK");
	opts.optflag("h", "help", "print this help menu");


	let matches = match opts.parse(&args[1..]) {
		Ok(m) => { m }
		Err(f) => { panic!(f.to_string()) }
	};

	if matches.opt_present("h") {
			print_usage(&program, opts);
			return;
	}

	let stack = if matches.opt_str("s").is_some() {
		matches.opt_str("s").unwrap()
	} else {
		print_usage(&program, opts);
		return;
	};

	tmux(&stack);
}
