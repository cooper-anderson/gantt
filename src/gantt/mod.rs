pub mod method;

#[derive(PartialEq, Eq, Hash)]
pub struct Process {
	pub label: String,
	pub arrival: usize,
	pub service: usize
}

impl Process {
	fn new(raw: (&str, usize, usize)) -> Self {
		Process { label: raw.0.to_string(), arrival: raw.1, service: raw.2 }
	}
}

pub struct Problem {
	pub processes: Vec<Process>,
	pub length: usize
}

impl Problem {
	pub fn new(raw: Vec<(&str, usize, usize)>) -> Self {
		let mut processes: Vec<Process> = Vec::new();
		let mut length: usize = 0;

		for p in raw {
			length += p.2;
			processes.push(Process::new(p));
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
	turnaround: f32,
	response: f32
}

impl Answer {
	pub fn new(name: &str, problem: Problem, order: Vec<String>) -> Self {
		let mut turnaround: f32 = 0.0;
		let mut response: f32 = 0.0;
		let size = problem.processes.len() as f32;
		for process in problem.processes {
			turnaround += Answer::get_turnaround(&order, &process) as f32;
			response += Answer::get_response(&order, &process) as f32;
		}
		turnaround /= size;
		response /= size;
		Answer {name: name.to_string(), order, turnaround, response}
	}

	fn get_turnaround(order: &Vec<String>, process: &Process) -> usize {
		match order.iter().rposition(|s| s.as_str() == process.label) {
			Some(i) => i - process.arrival + 1,
			None => 0
		}
	}

	fn get_response(order: &Vec<String>, process: &Process) -> usize {
		match order.iter().position(|s| s.as_str() == process.label) {
			Some(i) => i - process.arrival,
			None => 0
		}
	}
}
