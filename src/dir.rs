use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(Debug, Copy, Clone, FromPrimitive)]
pub enum Dir { N, NE, E, SE, S, SW, W, NW }

impl Dir {
	pub fn delta(self) -> (isize, isize) {
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

	pub fn turn(&mut self, i: i8) {
		let n = (*self as u8) as i8 + i;
		*self = Dir::from_u8(n.rem_euclid(8) as u8).unwrap();
	}

	pub fn mirror(&mut self, m: Dir) {
		let n = m as i8 * 2 - *self as i8;
		*self = Dir::from_u8(n.rem_euclid(8) as u8).unwrap();
	}

	pub fn bounce(&mut self) {
		self.turn(4);
	}
}

