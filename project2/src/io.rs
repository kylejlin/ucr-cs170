use super::*;

pub struct DatasetSyntaxError {
    pub message: String,
}

pub struct Pretty<T>(pub T);

pub trait ToPretty {
    fn pretty(&self) -> Pretty<&Self> {
        Pretty(self)
    }
}

impl DatasetSyntaxError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl std::fmt::Debug for DatasetSyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BadDatasetSyntaxError: {}", self.message)
    }
}

impl ToPretty for [FeatureStartingFrom1] {}

impl ToPretty for (&[FeatureStartingFrom1], FeatureStartingFrom1) {}

impl std::fmt::Display for Pretty<&[FeatureStartingFrom1]> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Pretty(features) = self;

        write!(f, "{{")?;

        for (i, feature) in features.iter().enumerate() {
            if i != 0 {
                write!(f, ",")?;
            }
            write!(f, "{}", feature.0)?;
        }

        write!(f, "}}")
    }
}

impl std::fmt::Display for Pretty<&(&[FeatureStartingFrom1], FeatureStartingFrom1)> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Pretty((current_features, candidate_feature)) = self;

        write!(f, "{{")?;

        for (i, feature) in current_features
            .iter()
            .chain(std::iter::once(candidate_feature))
            .enumerate()
        {
            if i != 0 {
                write!(f, ",")?;
            }
            write!(f, "{}", feature.0)?;
        }

        write!(f, "}}")
    }
}

/// This function parses a string and returns a dataset.
/// It enforces the following constraints:
/// - The class of each instance must be at least `1.0`. Classes are floored to integers.
/// - The number of features in each instance must be consistent.
pub fn parse_dataset(s: &str) -> Result<Dataset, DatasetSyntaxError> {
    let mut instances = vec![];
    let mut class_count = 0;
    let mut data_set_feature_count = 0;

    for (line_index, line) in s.lines().enumerate() {
        let line_number = line_index + 1;

        let mut parts = line.split_whitespace();

        let class = parts.next().ok_or(DatasetSyntaxError::new(&format!(
            "missing class on line {line_number}"
        )))?;
        let class: f64 = class
            .parse::<f64>()
            .map_err(|_| {
                DatasetSyntaxError::new(&format!("invalid class on line {line_number}: {class}"))
            })?
            .floor();
        if class < 1.0 {
            return Err(DatasetSyntaxError::new(&format!(
                "invalid class on {line_number}: {class}. class must be no less than 1"
            )));
        }
        let class = ClassStartingFrom1(class as usize);

        class_count = class_count.max(class.0);

        let instance_feature_count = parts.clone().count();

        if data_set_feature_count == 0 {
            data_set_feature_count = instance_feature_count;
        }

        if instance_feature_count != data_set_feature_count {
            return Err(DatasetSyntaxError::new(&format!(
                "inconsistent feature count on line {line_number}. expected {data_set_feature_count} features, but got {instance_feature_count}"
            )));
        }

        let feature_values = parts
            .map(|part| {
                part.parse::<f64>().map_err(|_| {
                    DatasetSyntaxError::new(&format!(
                        "invalid feature value on line {line_number}: {part}"
                    ))
                })
            })
            .collect::<Result<Vec<f64>, DatasetSyntaxError>>()?;

        instances.push(Instance {
            class,
            feature_values,
        });
    }

    Ok(Dataset {
        class_count,
        feature_count: data_set_feature_count,
        instances,
    })
}

/// Asks the user for a file path, and then reads the file at that path.
/// If the file cannot be read (e.g., because the path points to a nonexistent location),
/// then this function repeatedly asks the user for a file path until a valid file is found.
pub fn ask_user_for_file_path_and_then_read_file() -> String {
    loop {
        print!("Type in the path to the file to test: ");

        let path = read_line_from_stdin();
        let path = std::path::Path::new(&path);

        match std::fs::read_to_string(path) {
            Ok(c) => return c,

            Err(e) => {
                println!("Error reading file: {:?}", e);
                continue;
            }
        }
    }
}

pub fn ask_user_for_algorithm() -> Algorithm {
    loop {
        println!("Type the number of the algorithm you want to run.");
        println!("1) Forward Selection");
        println!("2) Backward Elimination");
        print!("Enter the number of the algorithm you want to run: ");

        let choice = read_line_from_stdin();

        match choice.trim() {
            "1" => return Algorithm::ForwardSearch,

            "2" => return Algorithm::BackwardSearch,

            _ => {
                println!("Invalid choice. Please enter 1 or 2.");
                continue;
            }
        }
    }
}

fn read_line_from_stdin() -> String {
    use std::io::BufRead;

    std::io::stdin().lock().lines().next().unwrap().unwrap()
}
