use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::fmt;

struct Kye {
	cells: Vec<Vec<char>>,
	width :usize,
	height :usize,
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

impl Kye {
	fn open(path :&str) -> Kye {
		let f = File::open(path).unwrap(); // XXX
		let f = BufReader::new(f);

		let mut width  :usize = 0;
		let mut height :usize = 0;

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

		Kye { cells: cells, width: width, height: height }
	}
}

fn main() -> io::Result<()> {
	let mut kye = Kye::open("examples/delay1.kye");

dbg!(kye);

	Ok(())
}
