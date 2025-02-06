use cs170_project1::*;

fn main() {
    println!("Welcome to Kyle's 8-Puzzle Solver!");

    let initial_state = io::ask_for_initial_state();
    let algorithm = io::ask_for_algorithm();

    let mut tracer = PrintTracer {
        max_queue_size: 0,
        nodes_expanded: 0,
    };

    let possible_solution: Option<Node> = search(initial_state, algorithm, &mut tracer);

    if let Some(solution) = possible_solution {
        println!(
            "Solution found!\nSolution depth: {}",
            solution.cost_to_reach
        );
    } else {
        println!("No solution found.");
    }

    println!(
        "Max queue size: {}\nNodes expanded: {}",
        tracer.max_queue_size, tracer.nodes_expanded
    );
}

struct PrintTracer {
    max_queue_size: usize,
    nodes_expanded: usize,
}

impl io::SearchTracer for PrintTracer {
    fn on_dequeue(&mut self, node: &Node) {
        self.nodes_expanded += 1;

        println!(
            "The best state to expand with a g(n) = {} and h(n) = {} is...\n{:?}",
            node.cost_to_reach,
            node.total_cost - node.cost_to_reach,
            node.state,
        );
    }

    fn on_enqueue(&mut self, _: &Node, queue: &min_heap::MinHeap<Node>) {
        self.max_queue_size = self.max_queue_size.max(queue.len());
    }
}
