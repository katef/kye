use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::fmt;
use std::env;
use std::time;
use itertools::Itertools;

#[derive(Debug)]
enum Dir { N, NE, E, SE, S, SW, W, NW }

#[derive(Debug)]
struct Thread {
	x: usize,
	y: usize,
	dir: Dir,
}

struct Kye {
	cells: Vec<Vec<char>>,
	width: usize,
	height: usize,
	threads: Vec<Thread>,
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
			.field("threads", &self.threads)
			.finish()
	}
}

impl Thread {
	fn new() -> Thread {
		Thread { x: 0, y: 0, dir: Dir::E }
	}

	fn r#move(&mut self, width: usize, height: usize) {
		fn delta(dir: &Dir) -> (isize, isize) {
			match dir {
			Dir::N  => ( 0, -1),
			Dir::NE => ( 1, -1),
			Dir::E  => ( 1,  0),
			Dir::SE => ( 1,  1),
			Dir::S  => ( 0,  1),
			Dir::SW => (-1,  1),
			Dir::W  => (-1,  0),
			Dir::NW => (-1, -1),
			}
		}

		let (dx, dy) = delta(&self.dir);
		self.movexy(dx, dy, width, height);
	}

	fn movexy(&mut self, dx: isize, dy: isize, width: usize, height: usize) {
		self.x = (self.x as isize + dx).rem_euclid(width  as isize) as usize;
		self.y = (self.y as isize + dy).rem_euclid(height as isize) as usize;
	}
}

impl Kye {
	fn new(cells: Vec<Vec<char>>, width: usize, height: usize) -> Kye {
		let threads = vec![Thread::new()];

		Kye { cells: cells, width: width, height: height, threads: threads, }
	}

	fn read<R: BufRead>(buf: R) -> Kye {
		let mut width: usize = 0;
		let mut height: usize = 0;

		fn hashbang(i: usize, line: &str) -> bool {
			return i == 0 && line.starts_with("#")
		}

		let lines: Vec<_> = buf.lines()
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
			let c = self.cells[thread.y][thread.x];

			match c {
			'1' => thread.dir = Dir::SW,
			'2' => thread.dir = Dir::S,
			'3' => thread.dir = Dir::SE,
			'4' => thread.dir = Dir::W,
			'5' => { },
			'6' => thread.dir = Dir::E,
			'7' => thread.dir = Dir::NW,
			'8' => thread.dir = Dir::N,
			'9' => thread.dir = Dir::NE,

			'Q' => { }
			_ => { }
			}

			thread.r#move(self.width, self.height)
		}
	}

	fn cells(&self) -> impl Iterator<Item = (usize, usize)> {
		(0..self.height).cartesian_product(0..self.width)
	}

	fn thread_at(&self, x: usize, y: usize) -> bool {
		self.threads.iter()
			.filter(|t| (x, y) == (t.x, t.y)).count() > 0
	}

	fn print(&mut self) {
		fn esc(c: char) -> char {
			if (c as u32) < 32 || (c as u32) > 127 {
				'.'
			} else {
				c
			}
		}

		for (y, x) in self.cells() {
			let cursor = self.thread_at(x, y);
			if cursor {
				eprint!("\x1b[1;41m");
			}
			eprint!("{}", esc(self.cells[y][x]));
			if cursor {
				eprint!("\x1b[0m");
			}
			if x == self.width - 1 {
				eprintln!("");
			}
		}
	}
}

fn main() -> io::Result<()> {
	let args: Vec<String> = env::args().collect();

	let f = File::open(&args[1]).unwrap(); // XXX
	let buf = io::BufReader::new(f);

	let mut kye = Kye::read(buf);

	eprint!("\x1b[?25l\x1b[2J");
	loop {
		eprint!("\x1b[0;0H");
		kye.print();
		std::thread::sleep(time::Duration::from_millis(250));
		kye.tick();
	}
}
