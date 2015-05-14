use std::fmt;

trait LiquidContainer {
	fn capacity(&self) -> u32;
}


#[derive(Debug)]
struct Kettle {
	capacity: u32,
	colour: (u8, u8, u8)
}

#[derive(Debug)]
enum CupSize {
	Small, Medium, Large
}

#[derive(Debug)]
struct Cup {
	size: CupSize
}

impl LiquidContainer for Kettle {
	fn capacity(&self) -> u32 {
		self.capacity
	}
}

impl LiquidContainer for Cup {
	fn capacity(&self) -> u32 {
		match self.size {
			CupSize::Small => 100,
			CupSize::Medium => 250,
			CupSize::Large => 500
		}
	}
}

impl fmt::Display for LiquidContainer {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Container with {} ml capacity", self.capacity())
    }
}

#[cfg(test)]
mod tests;
