use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::fmt;

#[derive(Debug)]
struct Thread {
	x: usize,
	y: usize,
}

struct Kye {
	cells: Vec<Vec<char>>,
	height :usize,
	width :usize,
	threads: Vec<Thread>,
}

fn hashbang(i :usize, line: &str) -> bool {
	return i == 0 && line.starts_with("#")
}

impl fmt::Debug for Kye {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut s = Vec::new();

		for row in &self.cells {
			s.push(row.iter().collect::<String>());
		}

		f.debug_struct("Kye")
			.field("cells", &s)
			.field("width", &self.width)
			.field("height", &self.height)
			.finish()
	}
}

impl Thread {
	fn new() -> Thread {
		Thread { x: 0, y: 0, }
	}

	fn cmd(&mut self, cells: &mut Vec<Vec<char>>) {
		let c = cells[self.y][self.x];

		match c {
		'Q' => { }
		_ => { }
		}
dbg!(&self);
dbg!(&cells[self.y][self.x]);

		self.r#move()
	}

	fn r#move(&mut self) {
		self.x += 1;
	}
}

impl Kye {
	fn new(cells: Vec<Vec<char>>, height: usize, width: usize) -> Kye {
		let threads = vec![Thread::new()];

		Kye { cells: cells, width: width, height: height, threads: threads, }
	}

	fn open(path :&str) -> Kye {
		let f = File::open(path).unwrap(); // XXX
		let f = BufReader::new(f);

		let mut height :usize = 0;
		let mut width  :usize = 0;

		let lines: Vec<_> = f.lines()
			.enumerate()
			.filter_map(|(i, line)|
				if hashbang(i, line.as_ref().unwrap().as_str()) {
					None
				} else {
					let line = line.unwrap();
					if line.len() > width {
						width = line.len()
					}
					height += 1;
					Some(line)
				}
			)
			.collect();

		let mut cells = Vec::with_capacity(height);
		for line in lines {
			cells.push(format!("{:1$}", line, width).chars().collect::<Vec<_>>());
		}

		Kye::new(cells, width, height)
	}

	fn tick(&mut self) {
		for thread in self.threads.iter_mut() {
			thread.cmd(&mut self.cells);
		}
	}
}

fn main() -> io::Result<()> {
	let mut kye = Kye::open("examples/delay1.kye");

dbg!(&kye);

	kye.tick();
	kye.tick();
	kye.tick();

	Ok(())
}
