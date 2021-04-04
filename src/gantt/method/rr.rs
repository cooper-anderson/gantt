use crate::gantt::{Answer, Problem};

pub fn run(problem: Problem, q: i32) -> Answer {
	Answer::new(format!("rr q={}", q).as_str(), vec![], 0.0, 0.0)
}

#[cfg(test)]
mod example {
	use crate::gantt::Problem;

	#[test]
	fn order() {
		let problem = Problem::example();
		let answer = super::run(problem, 4);
	}
}
