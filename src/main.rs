extern crate getopts;

use getopts::Options;
use std::env;
use std::fs::File;

enum Inventory {
	Path(String),
	Nil
}

fn tmux(stack: &str) {
	println!("You chose stack {}", stack);
}

fn print_usage(program: &str, opts: Options) {
	let brief = format!("Usage: {} [options]", program);
	print!("{}", opts.usage(&brief));
}

fn get_inventory(inv: Inventory) {
	let path = match inv {
			Inventory::Path(p) => p
			Inventory::Nil => "~/inventory.yaml",
		};
	let mut f = try!(File::open(path));
}

fn main() {
    let args : Vec<String> = env::args().collect();
	let program = args[0].clone();

	let mut opts = Options::new();

	opts.optopt("s", "stack", "specify stack to access", "STACK");
	opts.optopt("i", "inventory", "specify inventory if not in ~/", "PATH");
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

	let inventory = if matches.opt_str("i").is_some() {
		let path = matches.opt_str("s").unwrap().to_string();
		Inventory::Path(path)
	} else {
		Inventory::Nil
	};

	get_inventory(inventory);
}

