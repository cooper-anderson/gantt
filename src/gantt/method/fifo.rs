use crate::gantt::{Answer, Problem};

pub fn run(problem: Problem) -> Answer {
	let mut order: Vec<String> = Vec::new();
	Answer::new("fifo", vec![], 0.0, 0.0)
}

#[cfg(test)]
mod example {
	use crate::gantt::Problem;

	#[test]
	fn order() {
		let problem = Problem::example();
		let answer = super::run(problem);
		let order = vec![
			"A", "A", "A", "B", "B", "B", "B", "B", "B", "C",
			"C", "C", "C", "D", "D", "D", "D", "D", "E", "E"
		];

		assert_eq!(order, answer.order);
	}

	#[test]
	fn turnaround() {
		let problem = Problem::example();
		let answer = super::run(problem);
		assert_eq!(8.6, answer.turnaround);
	}

	#[test]
	fn response() {
		let problem = Problem::example();
		let answer = super::run(problem);
		assert_eq!(4.6, answer.response);
	}
}
