use super::*;

pub fn ask_for_initial_state() -> State {
    println!("Type \"1\" to use the default initial state, or \"2\" to enter your own custom initial state.");

    match read_line_from_stdin().trim() {
        "1" => DEFAULT_INITIAL_STATE,

        "2" => ask_for_custom_initial_state(),

        bad_response => {
            println!("Invalid input: {bad_response}. You must enter \"1\" or \"2\".");
            exit_with_error()
        }
    }
}

fn ask_for_custom_initial_state() -> State {
    println!("Enter your custom initial state, using a zero to represent the blank. Please only enter valid 8-puzzles. Please enter a space in between the numbers. Type RETURN only when finished with a row.");

    let mut out = State {
        // Use an obviously illegal dummy value to indicate that the board is uninitialized.
        // This will make bugs more obvious.
        board: [[Tile(9); PUZZLE_SIZE]; PUZZLE_SIZE],
    };

    for row in 0..PUZZLE_SIZE {
        println!("Enter row {}:", row + 1);

        let line = read_line_from_stdin();
        let mut tiles = line.trim().split_whitespace();

        for col in 0..PUZZLE_SIZE {
            let tile: u8 = tiles
                .next()
                .expect("Invalid input: You must enter 3 numbers per row.")
                .trim()
                .parse()
                .expect("Invalid input: You must enter a number.");
            out.board[row][col] = Tile(tile);
        }
    }

    validate_custom_initial_state(out);

    out
}

fn validate_custom_initial_state(state: State) {
    let mut seen = [false; 9];

    for row in 0..PUZZLE_SIZE {
        for col in 0..PUZZLE_SIZE {
            let tile = state.board[row][col].0;

            if tile > 8 {
                println!("Invalid input: \"{tile}\" is not a valid tile.");
                exit_with_error();
            }

            if seen[tile as usize] {
                println!("Invalid input: \"{tile}\" is used more than once.");
                exit_with_error();
            }

            seen[tile as usize] = true;
        }
    }
}

pub fn ask_for_algorithm() -> Algorithm {
    println!("Please select an algorithm. Type \"1\" for Uniform Cost Search, \"2\" for the Misplaced Tile Heuristic, or \"3\" the Manhattan Distance Heuristic.");

    match read_line_from_stdin().trim() {
        "1" => Algorithm::UniformCostSearch,

        "2" => Algorithm::MisplacedTileHeuristic,

        "3" => Algorithm::ManhattanDistanceHeuristic,

        bad_response => {
            println!("Invalid input: {bad_response}. You must enter \"1\", \"2\", or \"3\".");
            exit_with_error()
        }
    }
}

pub fn print_solution_status(solution: &Option<Node>) {
    if let Some(s) = solution {
        println!("Solution found!\nSolution depth: {}", s.cost_to_reach);
    } else {
        println!("No solution found.");
    }
}

pub struct PrintTracer {
    pub max_queue_size: usize,
    pub nodes_expanded: usize,
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

impl PrintTracer {
    pub fn print_stats(&self) {
        println!(
            "Nodes expanded: {}\nMax queue size: {}",
            self.nodes_expanded, self.max_queue_size
        );
    }
}

fn exit_with_error() -> ! {
    println!("Exiting with error due to invalid user input. Please run this program again and enter valid inputs.");

    std::process::exit(1)
}

fn read_line_from_stdin() -> String {
    use std::io::BufRead;

    std::io::stdin().lock().lines().next().unwrap().unwrap()
}

pub trait SearchTracer {
    fn on_dequeue(&mut self, node: &Node);

    fn on_enqueue(&mut self, node: &Node, queue: &MinHeap<Node>);
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..PUZZLE_SIZE {
            for col in 0..PUZZLE_SIZE {
                write!(f, "{:?} ", self.board[row][col])?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
