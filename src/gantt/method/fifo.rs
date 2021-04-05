use std::collections::{HashMap, VecDeque};

use crate::gantt::{Answer, Problem, Process};

pub fn run(problem: &Problem) -> Answer {
	let mut order: Vec<String> = Vec::new();
	let mut queue: VecDeque<&Process> = VecDeque::new();
	let mut counts: HashMap<&String, usize> = HashMap::new();

	for i in 0..problem.length {
		for proc in &problem.processes {
			if i == proc.arrival { queue.push_back(proc); }
		}

		match queue.pop_front() {
			Some(p) => {
				let count = counts.entry(&p.label).or_insert(0);
				order.push(p.label.clone());
				*count += 1;
				if *count != p.service { queue.push_front(p) }
			}
			None => {}
		}
	}

	Answer::new("fifo", &problem, order)
}

#[cfg(test)]
mod example {
	use crate::gantt::Problem;

	#[test]
	fn order() {
		let problem = Problem::example();
		let answer = super::run(&problem);
		let order = vec![
			"A", "A", "A", "B", "B", "B", "B", "B", "B", "C",
			"C", "C", "C", "D", "D", "D", "D", "D", "E", "E"
		];

		assert_eq!(order, answer.order);
	}

	#[test]
	fn turnaround() {
		let problem = Problem::example();
		let answer = super::run(&problem);
		assert_eq!(8.6, answer.turnaround);
	}

	#[test]
	fn response() {
		let problem = Problem::example();
		let answer = super::run(&problem);
		assert_eq!(4.6, answer.response);
	}
}
