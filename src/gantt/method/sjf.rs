use crate::gantt::{Answer, Problem};

pub fn run(problem: &Problem) -> Answer {
	Answer::new("sjf", problem, vec![])
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
