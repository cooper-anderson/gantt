use std::collections::HashSet;

pub mod method;

#[derive(PartialEq, Eq, Hash)]
pub struct Process {
	pub label: String,
	pub arrival: i32,
	pub service: i32
}

impl Process {
	fn new(raw: (&str, i32, i32)) -> Self {
		Process { label: raw.0.to_string(), arrival: raw.1, service: raw.2 }
	}
}

pub struct Problem {
	pub processes: HashSet<Process>,
	pub length: i32
}

impl Problem {
	pub fn new(raw: Vec<(&str, i32, i32)>) -> Self {
		let mut processes: HashSet<Process> = HashSet::new();
		let mut length: i32 = 0;

		for p in raw {
			length += p.2;
			processes.insert(Process::new(p));
		}

		Problem {processes, length}
	}

	pub fn example() -> Self {
		Problem::new(vec![
			("A", 0, 3),
			("B", 2, 6),
			("C", 4, 4),
			("D", 6, 5),
			("E", 8, 2)
		])
	}
}

pub struct Answer {
	name: String,
	order: Vec<String>,
	turnaround: i32,
	response: i32
}

impl Answer {
	pub fn new(
		name: &str, order: Vec<String>, turnaround: i32, response: i32
	) -> Self {
		Answer {name: name.to_string(), order, turnaround, response}
	}
}
