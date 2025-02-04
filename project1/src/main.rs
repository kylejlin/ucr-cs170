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
