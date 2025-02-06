use cs170_project1::*;

fn main() {
    println!("Welcome to Kyle's 8-Puzzle Solver!");

    let initial_state = ui::ask_for_initial_state();
    let algorithm = ui::ask_for_algorithm();

    search(initial_state, algorithm);
}
