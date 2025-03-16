use cs170_project2::*;

fn main() {
    println!("Welcome to Kyle Lin's Feature Selection Algorithm!");

    let file_content = io::ask_user_for_file_path_and_then_read_file();

    let dataset = match io::parse_dataset(&file_content) {
        Ok(d) => d,

        Err(e) => {
            println!("The file you specified was malformatted. Error: {:?}", e);
            std::process::exit(1);
        }
    };

    let algorithm = io::ask_user_for_algorithm();

    println!(
        "This dataset has {} features (not including the class attribute), with {} instances.",
        dataset.feature_count,
        dataset.instances.len(),
    );

    println!(
        "Running nearest neighbor with all {} features, using “leaving-one-out” evaluation, I get an accuracy of {:.1}%.",
        dataset.feature_count,
        leave_out_one_cross_validation(&dataset, dataset.features())
    );

    match algorithm {
        Algorithm::ForwardSearch => forward_search(&dataset),
        Algorithm::BackwardSearch => backward_search(&dataset),
    };
}
