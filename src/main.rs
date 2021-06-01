use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::fmt;

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
	w: usize,
	h: usize,
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
			.field("width", &self.w)
			.field("height", &self.h)
			.field("threads", &self.threads)
			.finish()
	}
}

impl Thread {
	fn new() -> Thread {
		Thread { x: 0, y: 0, dir: Dir::E }
	}

	fn r#move(&mut self, w: usize, h: usize) {
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
		self.movexy(dx, dy, w, h);
	}

	fn movexy(&mut self, dx: isize, dy: isize, w: usize, h: usize) {
		self.x = (self.x as isize + dx).rem_euclid(w as isize) as usize;
		self.y = (self.y as isize + dy).rem_euclid(h as isize) as usize;
	}
}

impl Kye {
	fn new(cells: Vec<Vec<char>>, w: usize, h: usize) -> Kye {
		let threads = vec![Thread::new()];

		Kye { cells: cells, w: w, h: h, threads: threads, }
	}

	fn open(path :&str) -> Kye {
		let f = File::open(path).unwrap(); // XXX
		let f = BufReader::new(f);

		let mut w: usize = 0;
		let mut h: usize = 0;

		fn hashbang(i: usize, line: &str) -> bool {
			return i == 0 && line.starts_with("#")
		}

		let lines: Vec<_> = f.lines()
			.enumerate()
			.filter_map(|(i, line)|
				if hashbang(i, line.as_ref().unwrap().as_str()) {
					None
				} else {
					let line = line.unwrap();
					if line.len() > w {
						w = line.len()
					}
					h += 1;
					Some(line)
				}
			)
			.collect();

		let mut cells = Vec::with_capacity(h);
		for line in lines {
			cells.push(format!("{:1$}", line, w).chars().collect::<Vec<_>>());
		}

		Kye::new(cells, w, h)
	}

	fn tick(&mut self) {
		for thread in self.threads.iter_mut() {
			let c = self.cells[thread.y][thread.x];

dbg!(&thread);
dbg!(&c);

			match c {
			'1' => thread.dir = Dir::SW,
			'2' => thread.dir = Dir::S,
			'3' => thread.dir = Dir::SE,
			'4' => thread.dir = Dir::W,
			'5' => { }, // noop
			'6' => thread.dir = Dir::E,
			'7' => thread.dir = Dir::NW,
			'8' => thread.dir = Dir::N,
			'9' => thread.dir = Dir::NE,

			'Q' => { }
			_ => { }
			}

			thread.r#move(self.w, self.h)
		}
	}
}

fn main() -> io::Result<()> {
	let mut kye = Kye::open("test/go.kye");

dbg!(&kye);

	kye.tick();
	kye.tick();
	kye.tick();

	Ok(())
}
