use crate::gantt::{Answer, Problem};

pub fn run(problem: Problem, q: i32) -> Answer {
	Answer::new(format!("rr q={}", q).as_str(), problem, vec![])
}

#[cfg(test)]
mod example_q4 {
	use crate::gantt::Problem;

	#[test]
	fn order() {
		let problem = Problem::example();
		let answer = super::run(problem, 4);
		let order = vec![
			"A", "A", "A", "B", "B", "B", "B", "C", "C", "C",
			"C", "D", "D", "D", "D", "B", "B", "E", "E", "D"
		];

		assert_eq!(order, answer.order);
	}

	#[test]
	fn turnaround() {
		let problem = Problem::example();
		let answer = super::run(problem, 4);
		assert_eq!(10.0, answer.turnaround);
	}

	#[test]
	fn response() {
		let problem = Problem::example();
		let answer = super::run(problem, 4);
		assert_eq!(3.6, answer.response);
	}
}

#[cfg(test)]
mod example_q1 {
	use crate::gantt::Problem;

	#[test]
	fn order() {
		let problem = Problem::example();
		let answer = super::run(problem, 4);
		let order = vec![
			"A", "A", "B", "A", "B", "C", "B", "D", "C", "B",
			"E", "D", "C", "B", "E", "D", "C", "B", "D", "D"
		];

		assert_eq!(order, answer.order);
	}

	#[test]
	fn turnaround() {
		let problem = Problem::example();
		let answer = super::run(problem, 4);
		assert_eq!(10.8, answer.turnaround);
	}

	#[test]
	fn response() {
		let problem = Problem::example();
		let answer = super::run(problem, 4);
		assert_eq!(0.8, answer.response);
	}
}
