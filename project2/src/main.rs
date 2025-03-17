use cs170_project2::*;

fn main() {
    println!("Welcome to Kyle Lin's Feature Selection Algorithm!");

    let dataset = io::ask_user_for_dataset_file_path_and_then_parse();
    let algorithm = io::ask_user_for_algorithm();

    println!(
        "This dataset has {} classes and {} features (not including the class attribute), with {} instances.",
        dataset.class_count,
        dataset.feature_count,
        dataset.instances.len(),
    );

    println!(
        "The default rate is {:.1}% (using class {}).",
        dataset.default_rate() * 100.0,
        dataset.most_common_class(),
    );

    println!(
        "Running nearest neighbor with all {} features, using “leaving-one-out” evaluation, I get an accuracy of {:.1}%.",
        dataset.feature_count,
        leave_out_one_cross_validation(&dataset, &dataset.complete_feature_set()) * 100.0
    );

    let start_time = std::time::Instant::now();

    match algorithm {
        Algorithm::ForwardSelection => forward_selection(&dataset),
        Algorithm::BackwardElimination => backward_elimination(&dataset),
    };

    let elapsed = start_time.elapsed();
    println!("The search took {:.1?} to complete.", elapsed);
}
