use std::collections::{HashMap, VecDeque};

use crate::gantt::{Answer, Problem, Process};

pub fn run(problem: &Problem, q: usize) -> Answer {
	let mut order: Vec<String> = Vec::new();
	let mut queue: VecDeque<&Process> = VecDeque::new();
	let mut counts: HashMap<&String, usize> = HashMap::new();

	for i in 0..problem.length + 1 {
		match queue.pop_front() {
			Some(p) => {
				let count = counts.entry(&p.label).or_insert(0);
				order.push(p.label.clone());
				*count += 1;
				for proc in &problem.processes {
					if i == proc.arrival { queue.push_back(proc); }
				}
				if *count != p.service {
					if *count % q == 0 { queue.push_back(p) }
					else { queue.push_front(p) }
				}
			}
			None => {
				for proc in &problem.processes {
					if proc.arrival == 0 { queue.push_back(proc); }
				}
			}
		}
	}

	Answer::new(format!("rr q={}", q).as_str(), &problem, order)
}

#[cfg(test)]
mod example_q4 {
	use crate::gantt::Problem;

	#[test]
	fn order() {
		let problem = Problem::example();
		let answer = super::run(&problem, 4);
		let order = vec![
			"A", "A", "A", "B", "B", "B", "B", "C", "C", "C",
			"C", "D", "D", "D", "D", "B", "B", "E", "E", "D"
		];

		assert_eq!(order, answer.order);
	}

	#[test]
	fn turnaround() {
		let problem = Problem::example();
		let answer = super::run(&problem, 4);
		assert_eq!(10.0, answer.turnaround);
	}

	#[test]
	fn response() {
		let problem = Problem::example();
		let answer = super::run(&problem, 4);
		assert_eq!(3.6, answer.response);
	}
}

#[cfg(test)]
mod example_q1 {
	use crate::gantt::Problem;

	#[test]
	fn order() {
		let problem = Problem::example();
		let answer = super::run(&problem, 1);
		let order = vec![
			"A", "A", "B", "A", "B", "C", "B", "D", "C", "B",
			"E", "D", "C", "B", "E", "D", "C", "B", "D", "D"
		];

		assert_eq!(order, answer.order);
	}

	#[test]
	fn turnaround() {
		let problem = Problem::example();
		let answer = super::run(&problem, 1);
		assert_eq!(10.8, answer.turnaround);
	}

	#[test]
	fn response() {
		let problem = Problem::example();
		let answer = super::run(&problem, 1);
		assert_eq!(0.8, answer.response);
	}
}
