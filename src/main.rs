mod gantt;

use gantt::method;

fn main() {
	let problem = gantt::Problem::new(vec![
		("A", 0, 5),
		("B", 0, 4),
		("C", 2, 1),
		("D", 8, 3),
	]);
	method::fifo::run(&problem).print();
	method::rr::run(&problem, 2).print();
	method::sjf::run(&problem).print();
}
