pub mod io;

use io::*;

#[derive(Debug, Clone, Copy)]
pub enum Algorithm {
    ForwardSearch,
    BackwardSearch,
}

#[derive(Debug)]
pub struct Dataset {
    pub class_count: usize,
    pub feature_count: usize,
    pub instances: Vec<Instance>,
}

/// We create a new type to help us remember
/// that the classes are counted starting from 1.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct ClassStartingFrom1(pub usize);

/// We create a new type to help us remember
/// that the features are counted starting from 1.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct FeatureStartingFrom1(pub usize);

#[derive(Debug)]
pub struct Instance {
    /// Starts from 1.
    pub class: ClassStartingFrom1,
    pub feature_values: Vec<f64>,
}

pub fn forward_search(dataset: &Dataset) -> Vec<FeatureStartingFrom1> {
    println!("Beginning forward search.");

    let mut current_set = vec![];
    let mut best_set = vec![];
    let mut best_accuracy = -1.0;

    for _ in 0..dataset.feature_count {
        let mut best_feature: Option<FeatureStartingFrom1> = None;
        let mut best_accuracy_for_current_outer_iteration = -1.0;

        for candidate_feature in dataset.features() {
            // Don't add the feature if it's already in the set.
            if current_set.contains(&candidate_feature) {
                continue;
            }

            let accuracy = leave_out_one_cross_validation(
                &dataset,
                current_set
                    .iter()
                    .cloned()
                    .chain(std::iter::once(candidate_feature)),
            );

            println!(
                "    Using feature(s) {} accuracy is {:.1}%",
                (current_set.as_slice(), candidate_feature).pretty(),
                accuracy * 100.0
            );

            if accuracy > best_accuracy_for_current_outer_iteration {
                best_accuracy_for_current_outer_iteration = accuracy;
                best_feature = Some(candidate_feature);
            }
        }

        let best_feature = best_feature.unwrap();
        current_set.push(best_feature);
        println!(
            "Feature set {} was best, accuracy is {:.1}%",
            current_set.pretty(),
            best_accuracy_for_current_outer_iteration * 100.0
        );

        if best_accuracy_for_current_outer_iteration > best_accuracy {
            best_accuracy = best_accuracy_for_current_outer_iteration;
            best_set = current_set.clone();
        }
    }

    println!(
        "Finished search! The best feature subset is {}, which has an accuracy of {:.1}%.",
        best_set.pretty(),
        best_accuracy * 100.0
    );

    best_set
}

pub fn backward_search(dataset: &Dataset) -> Vec<FeatureStartingFrom1> {
    panic!("TODO: Backward search is not implemented yet.");
}

/// Performs leave-one-out cross validation on the dataset,
/// but only using the features in `features_to_use`.
/// Returns the accuracy (between 0 and 1).
pub fn leave_out_one_cross_validation(
    dataset: &Dataset,
    features_to_use: impl Iterator<Item = FeatureStartingFrom1> + Clone,
) -> f64 {
    // Stub: for now, we just return a pseudo-random number.

    let mut x: u32 = 45298425;

    for feature in features_to_use.clone() {
        x <<= 1;
        x ^= (feature.0 as u32) * 9845245;
    }

    (x as f64) / (std::u32::MAX as f64)
}

impl Dataset {
    pub fn features(&self) -> impl Iterator<Item = FeatureStartingFrom1> + Clone {
        (1..=self.feature_count).map(FeatureStartingFrom1)
    }
}
