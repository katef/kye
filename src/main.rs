use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::fmt;
use std::env;
use std::time;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use itertools::Itertools;

#[derive(Debug, Copy, Clone, FromPrimitive)]
enum Dir { N, NE, E, SE, S, SW, W, NW }

impl Dir {
	fn delta(self) -> (isize, isize) {
		use Dir::*;

		match self {
		N  => ( 0, -1),
		NE => ( 1, -1),
		E  => ( 1,  0),
		SE => ( 1,  1),
		S  => ( 0,  1),
		SW => (-1,  1),
		W  => (-1,  0),
		NW => (-1, -1),
		}
	}

	fn turn(self, i: i8) -> Dir {
		let n = (self as u8) as i8 + i;
		Dir::from_u8(n.rem_euclid(8) as u8).unwrap()
	}

	fn mirror(self, m: Dir) -> Dir {
		let n = m as i8 * 2 - self as i8;
		Dir::from_u8(n.rem_euclid(8) as u8).unwrap()
	}
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum State { Dead, Push, PushEsc, Char, CharEsc, Exec }

#[derive(Debug, Clone)]
struct Thread {
	x: usize,
	y: usize,
	dir: Dir,
	state: State,
	stack: Vec<u32>,
}

struct Kye {
	cells: Vec<Vec<char>>,
	width: usize,
	height: usize,
	threads: Vec<Thread>,
	exit_status: u32,
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
		Thread { x: 0, y: 0, dir: Dir::E, state: State::Exec, stack: vec![] }
	}

	fn fork(&self, i: i8) -> Thread {
		let mut new = self.clone();
		new.dir = self.dir.turn(i);
		new
	}

	fn r#move(&mut self, width: usize, height: usize) {
		let (dx, dy) = self.dir.delta();
		self.movexy(dx, dy, width, height);
	}

	fn movexy(&mut self, dx: isize, dy: isize, width: usize, height: usize) {
		self.x = (self.x as isize + dx).rem_euclid(width  as isize) as usize;
		self.y = (self.y as isize + dy).rem_euclid(height as isize) as usize;
	}

	fn push(&mut self, n: u32) {
		self.stack.push(n);
	}

	fn pop(&mut self) -> u32 {
		match self.stack.pop() {
			Some(n) => n,
			None => 0,
		}
	}
}

impl Kye {
	fn new(cells: Vec<Vec<char>>, width: usize, height: usize) -> Kye {
		let threads = vec![Thread::new()];

		Kye { cells: cells, width: width, height: height, threads: threads, exit_status: 0 }
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

	fn esc(c: char) -> u32 {
		match c {
		't' => '\t' as u32,
		'n' => '\n' as u32,
		'v' => 0x0B,
		'f' => 0x0C,
		'r' => '\r' as u32,
		'0'..='9' => c as u32 - '0' as u32,
		_ => c as u32,
		}
	}

	fn tick(&mut self) {
		let mut spawn = vec![];
		let mut quit = false;

		for thread in self.threads.iter_mut() {
			let c = self.cells[thread.y][thread.x];

			match thread.state {
			State::Dead => panic!("unexpected state"),

			State::Push => {
				match c {
				'\\' => thread.state = State::PushEsc,
				'\'' => thread.state = State::Exec,
				_ => thread.push(c as u32),
				}
			}

			State::PushEsc => {
				thread.push(Kye::esc(c));
				thread.state = State::Push;
			}

			State::Char => {
				thread.state = State::Exec;
				match c {
				'\\' => thread.state = State::CharEsc,
				_ => thread.push(c as u32),
				}
			}

			State::CharEsc => {
				thread.push(Kye::esc(c));
				thread.state = State::Exec;
			}

			State::Exec =>
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

				'C'  => thread.dir = thread.dir.turn( 2),
				'c'  => thread.dir = thread.dir.turn( 1),
				'A'  => thread.dir = thread.dir.turn(-2),
				'a'  => thread.dir = thread.dir.turn(-1),

				'|'  => thread.dir = thread.dir.mirror(Dir::N),
				'/'  => thread.dir = thread.dir.mirror(Dir::NE),
				'_'  => thread.dir = thread.dir.mirror(Dir::E),
				'\\' => thread.dir = thread.dir.mirror(Dir::SE),

				'\'' => thread.state = State::Push,
				'\"' => thread.state = State::Char,

				'#' => thread.r#move(self.width, self.height),
				'j' => for _ in 0..thread.pop() {
					thread.r#move(self.width, self.height);
				},
				't' => if thread.pop() == 0 {
					thread.r#move(self.width, self.height);
				},

				'z' => thread.push(0),

				'P' => {
					// TODO: pop all, print to stdout
					thread.stack.clear();
				},

				';' => loop {
					thread.r#move(self.width, self.height);

					// TODO: would prefer to land on the ';' here, but we don't have "peek" yet
					if self.cells[thread.y][thread.x] == ';' {
						break;
					}
				}

				'G' => spawn.push(thread.fork(-2)),
				'g' => spawn.push(thread.fork( 2)),

				'T' => {
					spawn.push(thread.fork(2));
					thread.dir = thread.dir.turn(-2);
				}

				'Y' => {
					spawn.push(thread.fork(1));
					thread.dir = thread.dir.turn(-1);
				}

				'@' => {
					thread.state = State::Dead;
					continue;
				},

				'Q' => {
					self.exit_status = thread.pop();
					quit = true;
					break;
				},

				_ => { }
				}
			}
		}

		if quit {
			self.threads.clear();
			return;
		}

		self.threads.retain(|t| t.state != State::Dead);
		self.threads.append(&mut spawn);

		for thread in self.threads.iter_mut() {
			thread.r#move(self.width, self.height)
		}
	}

	fn cells(&self) -> impl Iterator<Item = (usize, usize)> {
		(0..self.height).cartesian_product(0..self.width)
	}

	fn threads_at(&self, x: usize, y: usize) -> impl Iterator<Item = &Thread> {
		self.threads.iter().filter(move |t| (t.x, t.y) == (x, y))
	}

	fn print(&self) {
		fn esc(c: char) -> char {
			if (c as u32) < 32 || (c as u32) > 127 {
				'.'
			} else {
				c
			}
		}

		fn thread_color(state: State) -> i32 {
			match state {
			State::Dead => 0,
			State::Exec => 41,
			_ => 46,
			}
		}

		fn thread_arrow(dir: Dir) -> char {
			match dir {
			Dir::N  => '↑',
			Dir::NE => '↗',
			Dir::E  => '→',
			Dir::SE => '↘',
			Dir::S  => '↓',
			Dir::SW => '↙',
			Dir::W  => '←',
			Dir::NW => '↖',
			}
		}

		for (y, x) in self.cells() {
			let color = self.threads_at(x, y)
				.map(|t| thread_color(t.state))
				.max(); // TODO: .max() is a placeholder

			if color != None {
				eprint!("\x1b[1;{}m", color.unwrap());
			}
			eprint!("{}", esc(self.cells[y][x]));
			if color != None {
				eprint!("\x1b[0m");
			}
			if x == self.width - 1 {
				eprintln!("");
			}
		}

		for (i, thread) in self.threads.iter().enumerate() {
			let s: String = thread.stack.iter()
				.map(|n| match char::from_u32(*n) {
					Some(_c) if *n < 32 => format!("\\x{{{:X}}}", *n),
					Some(c) => String::from(c),
					None => format!("\\x{{{:X}}}", *n),
				}).collect();
			let color = thread_color(thread.state);
			let arrow = thread_arrow(thread.dir);

			eprint!("{:2},{:2} {} ", thread.x, thread.y, arrow);
			eprint!("\x1b[1;{}m", color);
			eprint!("{}", i);
			eprint!("\x1b[0m");
			eprintln!(": {}\x1b[0K", s);
		}
		eprintln!("\x1b[J");
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
