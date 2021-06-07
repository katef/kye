use std::io;
use std::env;
use std::fs::File;
use std::time;

use crate::kye::Kye;

mod aut;
mod coord;
mod dir;
mod kye;
mod thread;

fn main() -> io::Result<()> {
	let args: Vec<String> = env::args().collect();

	let f = File::open(&args[1])?;
	let buf = io::BufReader::new(f);

	let mut kye = Kye::read(buf);

	eprint!("\x1b[?25l\x1b[2J");
	loop {
		eprint!("\x1b[0;0H");
		kye.print();

		if kye.threads.is_empty() {
			break;
		}

		std::thread::sleep(time::Duration::from_millis(200));
		kye.tick();
	}

	if kye.exit_status != 0 {
		std::process::exit(kye.exit_status as i32);
	}

	Ok(())
}
