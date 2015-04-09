extern crate getopts;
extern crate rustc_serialize;

use std::env;
use std::fs::File;
use std::path::Path;
use getopts::Options;
use std::error::Error;
use std::io::prelude::*;
use std::collections::BTreeMap;
use rustc_serialize::json;

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
	/*let home = match env::home_dir() {
		Some(ref p) => p.as_path().to_str(),
		None => println!("Can't get home dir")
	};*/

	let path_str = match inv {
		Inventory::Path(p) => p,
		Inventory::Nil => "/Users/206637/inventory.json".to_string(),
	};

	let path = Path::new(&path_str);

	let display = path.display();

	let mut file = match File::open(path) {
		Err(why) => panic!("couldn't open {}: {}",
		display,
		Error::description(&why)),
		Ok(file) => file,
	};

	let mut s = String::new();
	match file.read_to_string(&mut s) {
			Err(why) => panic!("couldn't read {}: {}", display,
							   Error::description(&why)),
			Ok(_) => print!("{} contains:\n{}", display, s),
	}
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

