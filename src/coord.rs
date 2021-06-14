use crate::dir::Dir;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Coord {
	pub x: usize,
	pub y: usize,
}

impl From<(usize, usize)> for Coord {			
	fn from(coord: (usize, usize)) -> Coord {
		Coord { x: coord.0, y: coord.1 }
	}
}

impl Coord {
	pub fn r#move(&mut self, dir: Dir, width: usize, height: usize) {
		let (dx, dy) = <(_, _)>::from(dir);
		let x = (self.x as isize + dx).rem_euclid(width  as isize) as usize;
		let y = (self.y as isize + dy).rem_euclid(height as isize) as usize;
		*self = Coord::from((x, y))
	}

	pub fn moven(&mut self, dir: Dir, width: usize, height: usize, n: u32) {
		for _ in 0..n {
			self.r#move(dir, width, height)
		}
	}
}

