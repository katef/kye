use crate::dir::Dir;
use crate::coord::Coord;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum State { Dead, Push, PushEsc, Char, CharEsc, Exec }

#[derive(Debug, Clone)]
pub struct Thread {
	pub coord: Coord,
	pub dir: Dir,
	pub state: State,
	pub stack: Vec<u32>,
}

impl Thread {
	pub fn new() -> Thread {
		let coord = Coord::new(0, 0);
		Thread { coord, dir: Dir::E, state: State::Exec, stack: vec![] }
	}

	pub fn fork(&self, i: i8) -> Thread {
		let mut new = self.clone();
		new.dir = self.dir.turn(i);
		new
	}

	pub fn push(&mut self, n: u32) {
		self.stack.push(n);
	}

	pub fn pop(&mut self) -> u32 {
		self.stack.pop().unwrap_or(0)
	}
}

