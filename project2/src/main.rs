use cs170_project2::*;

fn main() {
    println!("Welcome to Kyle Lin's Feature Selection Algorithm!");

    let dataset = io::ask_user_for_dataset_file_path_and_then_parse();
    let algorithm = io::ask_user_for_algorithm();

    println!(
        "This dataset has {} features (not including the class attribute), with {} instances.",
        dataset.feature_count,
        dataset.instances.len(),
    );

    println!(
        "Running nearest neighbor with all {} features, using “leaving-one-out” evaluation, I get an accuracy of {:.1}%.",
        dataset.feature_count,
        leave_out_one_cross_validation(&dataset, &dataset.complete_feature_set()) * 100.0
    );

    match algorithm {
        Algorithm::ForwardSelection => forward_selection(&dataset),
        Algorithm::BackwardElimination => backward_elimination(&dataset),
    };
}
