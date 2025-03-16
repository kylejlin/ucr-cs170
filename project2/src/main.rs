use cs170_project2::*;

fn main() {
    println!("Welcome to Kyle's 8-Puzzle Solver!");

    let initial_state = io::ask_for_initial_state();

    let algorithm = io::ask_for_algorithm();

    let mut tracer = io::PrintTracer {
        max_queue_size: 1,
        nodes_expanded: 0,
    };

    let solution: Option<Node> = search(initial_state, algorithm, &mut tracer);

    io::print_solution_status(&solution);

    tracer.print_stats();
}
