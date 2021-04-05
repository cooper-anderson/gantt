use crate::gantt::{Answer, Problem};

pub fn run(problem: Problem) -> Answer {
	Answer::new("stcf", problem, vec![])
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
