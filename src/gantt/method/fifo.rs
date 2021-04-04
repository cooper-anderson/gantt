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
	}
}
