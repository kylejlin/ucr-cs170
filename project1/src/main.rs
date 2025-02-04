const PUZZLE_SIZE: usize = 3;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Tile(u8);

#[derive(Clone, Copy, PartialEq, Eq)]
struct State {
    board: [[Tile; PUZZLE_SIZE]; PUZZLE_SIZE],
}

#[derive(Clone, Copy, PartialEq, Eq)]
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

fn search(initial_state: State, algorithm: Algorithm) {
    todo!()
}

fn ask_for_initial_state() -> State {
    println!("Type \"1\" to use the default initial state, or \"2\" to enter your own custom initial state.");

    match read_line_from_stdin().trim() {
        "1" => DEFAULT_INITIAL_STATE,

        "2" => ask_for_custom_puzzle(),

        bad_response => {
            println!("Invalid input: {bad_response}. You must enter \"1\" or \"2\".");
            exit_with_error()
        }
    }
}

fn ask_for_custom_puzzle() -> State {
    todo!()
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
