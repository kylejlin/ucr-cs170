const PUZZLE_SIZE: usize = 3;

const GOAL_STATE: State = State {
    board: [
        [Tile(1), Tile(2), Tile(3)],
        [Tile(4), Tile(5), Tile(6)],
        [Tile(7), Tile(8), Tile(0)],
    ],
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Tile(u8);

#[derive(Clone, Copy, PartialEq, Eq)]
struct State {
    board: [[Tile; PUZZLE_SIZE]; PUZZLE_SIZE],
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Node {
    state: State,

    /// The cost to reach this node starting from the root.
    /// For the root node, this is `0`.
    ///
    /// This is `g(x)` in the A* algorithm.
    cost_to_reach: Cost,
}

type Cost = u32;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Algorithm {
    UniformCostSearch,
    MisplacedTileHeuristic,
    ManhattanDistanceHeuristic,
}

const DEFAULT_INITIAL_STATE: State = State {
    board: [
        [Tile(1), Tile(2), Tile(3)],
        [Tile(4), Tile(0), Tile(6)],
        [Tile(7), Tile(5), Tile(8)],
    ],
};

fn main() {
    println!("Welcome to Kyle's 8-Puzzle Solver!");

    let initial_state = ask_for_initial_state();
    let algorithm = ask_for_algorithm();

    search(initial_state, algorithm);
}

/// Returns `Some(solution)` if a solution was found, or `None` if no solution was found.
fn search(initial_state: State, algorithm: Algorithm) -> Option<State> {
    let initial_node = Node {
        state: initial_state,
        cost_to_reach: 0,
    };

    // We'll use a simple (albeit inefficient) queue design:
    // Store the nodes in a vector in descending order of cost.
    // - To dequeue, just `pop()` the last element.
    // - To enqueue, insert the new node at the correct position.
    // This means each enqueue operation is O(n), but the dequeues are O(1),
    // where n is the length of the queue.
    //
    // I'm sure there's a cleaner way to do this with trees or heaps or something,
    // but that's not the focus of this project, so I'm using this simple approach.
    let mut queue = Vec::new();

    queue.push(initial_node);

    loop {
        let Some(node) = queue.pop() else {
            // If the queue is empty, there is no solution.
            return None;
        };

        if node.state.is_goal() {
            return Some(node.state);
        }

        expand_queue(&node, &mut queue, algorithm);
    }
}

fn expand_queue(parent: &Node, queue: &mut Vec<Node>, algorithm: Algorithm) {
    parent.state.for_each_child(|child| {
        let child_cost = algorithm.cost(&child);

        // Calculate the index where the child should be inserted.
        let index = queue
            .iter()
            // Try to find the first index where the cost is greater than the child's cost...
            .position(|other| algorithm.cost(other) > child_cost)
            // ...but if all the costs are less than or equal to the child's cost,
            // then insert the child at the end.
            .unwrap_or(queue.len());

        queue.insert(index, child);
    })
}

impl Algorithm {
    /// This is `f(x) := g(x) + h(x)` in the A* algorithm.
    fn cost(self, node: &Node) -> Cost {
        node.cost_to_reach + self.estimate_cost_to_goal(node)
    }

    /// This is `h(x)` in the A* algorithm.
    fn estimate_cost_to_goal(self, node: &Node) -> Cost {
        match self {
            // With Uniform Cost Search, we simply hardcode h(x) to 0.
            Algorithm::UniformCostSearch => 0,

            Algorithm::MisplacedTileHeuristic => node.state.number_of_misplaced_tiles(),

            Algorithm::ManhattanDistanceHeuristic => node.state.manhattan_distance_to_goal(),
        }
    }
}

impl State {
    /// Iterates over every child,
    /// calling the visitor function `f` on each one.
    fn for_each_child(&self, mut f: impl FnMut(Node)) {
        todo!()
    }

    fn number_of_misplaced_tiles(&self) -> u32 {
        todo!()
    }

    fn manhattan_distance_to_goal(&self) -> u32 {
        todo!()
    }

    fn is_goal(&self) -> bool {
        // For this problem, there is only one goal state.
        *self == GOAL_STATE
    }
}

fn ask_for_initial_state() -> State {
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
        board: [[Tile(0); PUZZLE_SIZE]; PUZZLE_SIZE],
    };

    for row in 0..PUZZLE_SIZE {
        println!("Enter row {}:", row + 1);

        let line = read_line_from_stdin();
        let mut tiles = line.split_whitespace();

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

fn ask_for_algorithm() -> Algorithm {
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

fn exit_with_error() -> ! {
    println!("Exiting with error due to invalid user input. Please run this program again and enter valid inputs.");

    std::process::exit(1)
}

fn read_line_from_stdin() -> String {
    use std::io::BufRead;

    std::io::stdin().lock().lines().next().unwrap().unwrap()
}

// Pretty-printing functions to make debugging more pleasant.

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
