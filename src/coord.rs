use crate::dir::Dir;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Coord {
	pub x: usize,
	pub y: usize,
}

impl Coord {
	pub fn new(x: usize, y: usize) -> Coord {
		Coord { x, y }
	}

	pub fn r#move(&self, dir: Dir, width: usize, height: usize) -> Coord {
		let (dx, dy) = dir.delta();
		let x = (self.x as isize + dx).rem_euclid(width  as isize) as usize;
		let y = (self.y as isize + dy).rem_euclid(height as isize) as usize;
		Coord::new(x, y)
	}

	pub fn moven(&self, dir: Dir, width: usize, height: usize, n: u32) -> Coord {
		let mut tmp = *self;
		for _ in 0..n {
			tmp = tmp.r#move(dir, width, height)
		}
		tmp
	}
}

