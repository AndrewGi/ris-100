#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Debug, Hash)]
pub enum Which {
	Acc,
	Bak,
	Port(Direction)
}
impl Which {
	pub fn as_str(self) -> &'static str {
		match self {
			Which::Acc => "ACC",
			Which::Bak => "BAK",
			Which::Port(dir) => dir.as_str(),
		}
	}
}
impl core::fmt::Display for Which {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.write_str(self.as_str())
	}
}
#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Debug, Hash)]
pub enum Direction {
	Up,
	Down,
	Left,
	Right
}
impl Direction {
	pub fn as_str(self) -> &'static str {
		match self {
			Direction::Up => "UP",
			Direction::Down => "DOWN",
			Direction::Left => "LEFT",
			Direction::Right => "RIGHT",
		}
	}
}

impl core::fmt::Display for Direction {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.write_str(self.as_str())
	}
}