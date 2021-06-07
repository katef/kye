use crate::dir::Dir;
use crate::coord::Coord;

#[derive(Clone)]
pub struct Automaton {
	pub coord: Coord,
	pub dir: Dir,
}

impl Automaton {
	pub fn is_automaton(c: char) -> bool {
		c == '^' || c == '>' || c == 'v' || c == '<'
	}

	pub fn is_pushable(c: char) -> bool {
		c != 'O'
	}

	pub fn char_to_dir(c: char) -> Option<Dir> {
		match c {
		'^' => Some(Dir::N),
		'>' => Some(Dir::E),
		'v' => Some(Dir::S),
		'<' => Some(Dir::W),
		_ => None
		}
	}

	pub fn new(x: usize, y: usize, dir: Dir) -> Automaton {
		let coord = Coord::new(x, y);
		Automaton { coord, dir }
	}
}

