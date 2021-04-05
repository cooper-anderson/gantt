use std::collections::HashMap;
use priority_queue::PriorityQueue;

use crate::gantt::{Answer, Problem, Process};

pub fn run(problem: &Problem) -> Answer {
	let mut order: Vec<String> = Vec::new();
	let mut stack: PriorityQueue<&Process, usize> = PriorityQueue::new();
	let mut counts: HashMap<&String, usize> = HashMap::new();
	let mut current: Option<(&Process, usize)> = None;
	let mut dirty = true;

	for i in 0..problem.length {
		for proc in &problem.processes {
			if i == proc.arrival { stack.push(proc, problem.length - proc.service); }
		}

		if dirty {
			current = stack.pop();
			dirty = false;
		}

		match current {
			Some((process, _)) => {
				let count = counts.entry(&process.label).or_insert(0);
				order.push(process.label.clone());
				*count += 1;
				if *count == process.service { dirty = true }
			}
			None => {}
		}
	}
	
	Answer::new("sjf", problem, order)
}

#[cfg(test)]
mod example {
	use crate::gantt::Problem;

	#[test]
	fn order() {
		let problem = Problem::example();
		let answer = super::run(&problem);
		let order = vec![
			"A", "A", "A", "B", "B", "B", "B", "B", "B", "E",
			"E", "C", "C", "C", "C", "D", "D", "D", "D", "D"
		];

		assert_eq!(order, answer.order);
	}

	#[test]
	fn turnaround() {
		let problem = Problem::example();
		let answer = super::run(&problem);
		assert_eq!(7.6, answer.turnaround);
	}

	#[test]
	fn response() {
		let problem = Problem::example();
		let answer = super::run(&problem);
		assert_eq!(3.6, answer.response);
	}
}
