pub mod io;

use io::*;

#[derive(Debug, Clone, Copy)]
pub enum Algorithm {
    ForwardSelection,
    BackwardElimination,
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

pub fn forward_selection(dataset: &Dataset) -> Vec<FeatureStartingFrom1> {
    println!("Beginning forward selection.");

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

            // For performance reasons, rather than pass a vector of features
            // (which we would have to iterate over for each instance),
            // we pass a feature of coefficients (which are either `1.0` or `0.0`) instead.
            // See the documentation comments for `leave_out_one_cross_validation` for more information.
            let accuracy = leave_out_one_cross_validation(
                &dataset,
                // Use the union of the current set of features and the candidate feature.
                &dataset.feature_coefficients_to_use_only(
                    current_set
                        .iter()
                        .cloned()
                        .chain(std::iter::once(candidate_feature)),
                ),
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

pub fn backward_elimination(dataset: &Dataset) -> Vec<FeatureStartingFrom1> {
    panic!("TODO: Backward search is not implemented yet.");
}

/// Performs leave-one-out cross validation on the dataset
/// and returns the accuracy rate.
/// When computing the distance between two instances,
/// we multiply each feature value by the corresponding
/// coefficient and sum the results.
/// You can set the coefficients of the features you want to use to 1,
/// and set the rest of the coefficients to 0.
pub fn leave_out_one_cross_validation(dataset: &Dataset, feature_coefficients: &[f64]) -> f64 {
    // TODO
    0.123456789
}

pub fn leave_out_one_cross_validation_with_all_features(dataset: &Dataset) -> f64 {
    let feature_coefficients = vec![1.0; dataset.feature_count];
    leave_out_one_cross_validation(dataset, &feature_coefficients)
}

impl Dataset {
    pub fn features(&self) -> impl Iterator<Item = FeatureStartingFrom1> + Clone {
        (1..=self.feature_count).map(FeatureStartingFrom1)
    }

    pub fn feature_coefficients_to_use_only(
        &self,
        features_to_include: impl IntoIterator<Item = FeatureStartingFrom1>,
    ) -> Vec<f64> {
        let mut coefficients = vec![0.0; self.feature_count];
        for feature in features_to_include {
            coefficients[feature.0 - 1] = 1.0;
        }
        coefficients
    }

    pub fn feature_coefficients_to_use_all_except(
        &self,
        features_to_exclude: impl IntoIterator<Item = FeatureStartingFrom1>,
    ) -> Vec<f64> {
        let mut coefficients = vec![1.0; self.feature_count];
        for feature in features_to_exclude {
            coefficients[feature.0 - 1] = 0.0;
        }
        coefficients
    }
}
