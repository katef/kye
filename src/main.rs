use std::io::{self, BufReader, BufRead};
use std::fs::File;
use std::path::PathBuf;
use std::time;
use std::num::ParseIntError;
use clap::Clap;

use crate::kye::Kye;

mod aut;
mod coord;
mod dir;
mod kye;
mod thread;

fn parse_duration(src: &str) -> Result<time::Duration, ParseIntError> {
	let ms = src.parse::<u64>()?;
	Ok(time::Duration::from_millis(ms))
}

#[derive(Clap, Debug)]
#[clap(name = "kye")]
struct Opts {
	#[clap(parse(from_os_str))]
	input_file: Option<PathBuf>,

	#[clap(short, long, about = "Hide debug output")]
	quiet: bool,

	#[clap(short, long, parse(try_from_str = parse_duration),
		default_value = "100",
		about = "Tick delay for debug output (ms)")]
	delay: time::Duration,

	#[clap(short, long, about = "Show threads in debug output")]
	threads: bool,
}

fn main() -> io::Result<()> {
	let opts: Opts = Opts::parse();

	let buf: Box<dyn BufRead> = match opts.input_file {
	Some(input_file) => Box::new(BufReader::new(File::open(input_file)?)),
	None => Box::new(BufReader::new(std::io::stdin()))
	};

	let mut kye = Kye::read(buf);

	if !opts.quiet {
		eprint!("\x1b[?25l\x1b[2J");
	}
	loop {
		eprint!("\x1b[0;0H");
		if !opts.quiet {
			kye.print(opts.threads);
		}

		if kye.threads.is_empty() {
			break;
		}

		if !opts.quiet {
			std::thread::sleep(opts.delay);
		}
		kye.tick();
	}

	if kye.exit_status != 0 {
		std::process::exit(kye.exit_status as i32);
	}

	Ok(())
}
