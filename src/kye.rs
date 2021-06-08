use std::fmt;
use std::io::prelude::*;
use itertools::Itertools;

use crate::coord::Coord;
use crate::aut::Automaton;
use crate::thread::State;
use crate::thread::Thread;
use crate::dir::Dir;

pub struct Kye {
	cells: Vec<Vec<char>>,
	width: usize,
	height: usize,
	pub threads: Vec<Thread>,
	automata: Vec<Automaton>,
	pub exit_status: u32,
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

impl Kye {
	fn new(cells: Vec<Vec<char>>, threads: Vec<Thread>, automata: Vec<Automaton>, width: usize, height: usize) -> Kye {
		Kye { cells, width, height, threads, automata, exit_status: 0 }
	}

	pub fn read<R: BufRead>(buf: R) -> Kye {
		let mut width: usize = 0;
		let mut height: usize = 0;
		let mut threads = vec![];
		let mut automata = vec![];

		fn hashbang(i: usize, line: &str) -> bool {
			i == 0 && line.starts_with('#')
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
		for (y, line) in lines.iter().enumerate() {
			let mut row = vec![];
			for (x, c) in format!("{:1$}", line, width).chars().enumerate() {
				if c == '$' {
					row.push('$');
					threads.push(Thread::new(Coord::new(x, y)));
				} else if Automaton::is_automaton(c) {
					row.push(' ');
					automata.push(Automaton::new(x, y, Automaton::char_to_dir(c).unwrap()));
				} else {
					row.push(c);
				}
			}
			cells.push(row);
		}

		Kye::new(cells, threads, automata, width, height)
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

	fn codepoint_print(codepoint: u32) {
		if let Some(character) = char::from_u32(codepoint) {
			print!("{}", character);
		} else {
			print!("\\u{{{:04X}}}", codepoint);
		}
	}

	pub fn tick(&mut self) {
		let mut spawn = vec![];
		let mut quit = false;

		for thread in self.threads.iter_mut() {
			let c = self.cells[thread.coord.y][thread.coord.x];

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

				'C'  => thread.dir.turn( 2),
				'c'  => thread.dir.turn( 1),
				'A'  => thread.dir.turn(-2),
				'a'  => thread.dir.turn(-1),

				'|'  => thread.dir.mirror(Dir::N),
				'/'  => thread.dir.mirror(Dir::NE),
				'_'  => thread.dir.mirror(Dir::E),
				'\\' => thread.dir.mirror(Dir::SE),

				'\'' => thread.state = State::Push,
				'\"' => thread.state = State::Char,

				'#' => thread.coord.r#move(thread.dir, self.width, self.height),
				'j' => for _ in 0..thread.pop() {
					thread.coord.r#move(thread.dir, self.width, self.height);
				},
				't' => if thread.pop() == 0 {
					thread.coord.r#move(thread.dir, self.width, self.height);
				},

				'z' => thread.push(0),
				'[' => { let c = thread.pop() + 1; thread.push(c); },
				']' => { let c = thread.pop() - 1; thread.push(c); },

				',' => Kye::codepoint_print(thread.pop()),
				'P' => for _ in 0 ..= thread.stack.iter().count() {
					Kye::codepoint_print(thread.pop());
				},

				'm' => {
					let c = char::from_u32(thread.pop()).unwrap_or('.');
					let mut peek = thread.coord;
					let mut right = thread.dir;
					right.turn(2);
					peek.r#move(right, self.width, self.height);
					self.automata.retain(|a| a.coord != peek);
					if Automaton::is_automaton(c) {
						self.cells[peek.y][peek.x] = ' ';
						self.automata.push(Automaton::new(peek.x, peek.y, Automaton::char_to_dir(c).unwrap()));
					} else {
						self.cells[peek.y][peek.x] = c;
					}
				},

				'M' => {
					let c = char::from_u32(thread.pop()).unwrap_or('.');
					let mut peek = thread.coord;
					let mut left = thread.dir;
					left.turn(-2);
					peek.r#move(left, self.width, self.height);
					self.automata.retain(|a| a.coord != peek);
					if Automaton::is_automaton(c) {
						self.cells[peek.y][peek.x] = ' ';
						self.automata.push(Automaton::new(peek.x, peek.y, Automaton::char_to_dir(c).unwrap()));
					} else {
						self.cells[peek.y][peek.x] = c;
					}
				},

				'H' => {
					let n = thread.pop();
					let c = char::from_u32(thread.pop()).unwrap_or('.');
					let mut peek = thread.coord;
					peek.moven(thread.dir, self.width, self.height, n);
					self.automata.retain(|a| a.coord != peek);
					if Automaton::is_automaton(c) {
						self.cells[peek.y][peek.x] = ' ';
						self.automata.push(Automaton::new(peek.x, peek.y, Automaton::char_to_dir(c).unwrap()));
					} else {
						self.cells[peek.y][peek.x] = c;
					}
				},

				';' => loop {
					thread.coord.r#move(thread.dir, self.width, self.height);

					// TODO: would prefer to land on the ';' here, but we don't have "peek" yet
					if self.cells[thread.coord.y][thread.coord.x] == ';' {
						break;
					}
				}

				'G' => spawn.push(thread.fork(-2)),
				'g' => spawn.push(thread.fork( 2)),

				'T' => {
					spawn.push(thread.fork(2));
					thread.dir.turn(-2);
				}

				'Y' => {
					spawn.push(thread.fork(1));
					thread.dir.turn(-1);
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
			loop {
				thread.coord.r#move(thread.dir, self.width, self.height);
				if thread.state != State::Exec || self.cells[thread.coord.y][thread.coord.x] != ' ' {
					break;
				}
			}
		}

		for automaton in self.automata.iter_mut() {
			automaton.coord.r#move(automaton.dir, self.width, self.height);
		}

		let collisions = self.automata.clone();
		for automaton in self.automata.iter_mut() {
			let instr_collision = self.cells[automaton.coord.y][automaton.coord.x] != ' ';
			let automata_collision = collisions.iter().filter(|a| a.coord == automaton.coord).count() > 1;
			if instr_collision || automata_collision {
				if !automata_collision {
					if Automaton::is_pushable(self.cells[automaton.coord.y][automaton.coord.x]) {
						let mut peek = automaton.coord;
						peek.r#move(automaton.dir, self.width, self.height);
// TODO: and there aren't any automata in the "peek" space?
// TODO: deal with >| < situation, which should produce < |>
						if self.cells[peek.y][peek.x] == ' ' {
							self.cells[peek.y][peek.x] = self.cells[automaton.coord.y][automaton.coord.x];
							self.cells[automaton.coord.y][automaton.coord.x] = ' ';
						}
					}
				}

				automaton.dir.bounce();
				automaton.coord.r#move(automaton.dir, self.width, self.height);

				let mut back = automaton.coord;
				back.r#move(automaton.dir, self.width, self.height);
				if self.cells[back.y][back.x] == ' ' {
					automaton.coord = back;
				}
			}
		}
	}

	fn cells(&self) -> impl Iterator<Item = (usize, usize)> {
		(0..self.height).cartesian_product(0..self.width)
	}

	fn threads_at(&self, x: usize, y: usize) -> impl Iterator<Item = &Thread> {
		self.threads.iter().filter(move |t| (t.coord.x, t.coord.y) == (x, y))
	}

	fn automata_at(&self, x: usize, y: usize) -> impl Iterator<Item = &Automaton> {
		self.automata.iter().filter(move |a| (a.coord.x, a.coord.y) == (x, y))
	}

	pub fn print(&self) {
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

		fn automata_char(dir: Dir) -> char {
			match dir {
			Dir::N  => '^',
			Dir::E  => '>',
			Dir::S  => 'v',
			Dir::W  => '<',
			_ => panic!("unrecognised automata direction"),
			}
		}

		for (y, x) in self.cells() {
			let color = self.threads_at(x, y)
				.map(|t| thread_color(t.state))
				.max(); // TODO: .max() is a placeholder

			if color != None {
				eprint!("\x1b[1;{}m", color.unwrap());
			}
			if self.cells[y][x] != ' ' {
				eprint!("{}", esc(self.cells[y][x]));
			} else if let Some(automata) = self.automata_at(x, y).next() {
				eprint!("{}", automata_char(automata.dir));
			} else {
				eprint!(" ");
			}
			if color != None {
				eprint!("\x1b[0m");
			}
			if x == self.width - 1 {
				eprintln!();
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

			eprint!("{:2},{:2} {} ", thread.coord.x, thread.coord.y, arrow);
			eprint!("\x1b[1;{}m", color);
			eprint!("{}", i);
			eprint!("\x1b[0m");
			eprintln!(": {}\x1b[0K", s);
		}
		eprintln!("\x1b[J");
	}
}

