pub mod io;

use io::*;

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

    for _ in 0..dataset.feature_count {
        let mut best_feature: Option<FeatureStartingFrom1> = None;
        let mut best_accuracy = -1.0;

        for candidate_feature in dataset.features() {
            // Don't add the feature if it's already in the set.
            if current_set.contains(&candidate_feature) {
                continue;
            }

            let accuracy =
                leave_out_one_cross_validation(&dataset, &current_set, candidate_feature);

            println!(
                "    Using feature(s) {} accuracy is {:.1}%",
                (current_set.as_slice(), candidate_feature).pretty(),
                accuracy * 100.0
            );

            if accuracy > best_accuracy {
                best_accuracy = accuracy;
                best_feature = Some(candidate_feature);
            }
        }

        let best_feature = best_feature.unwrap();
        current_set.push(best_feature);
        println!(
            "Feature set {} was best, accuracy is {:.1}%",
            current_set.pretty(),
            best_accuracy * 100.0
        );
    }

    current_set
}

pub fn leave_out_one_cross_validation(
    dataset: &Dataset,
    current_set: &[FeatureStartingFrom1],
    candidate_feature: FeatureStartingFrom1,
) -> f64 {
    todo!()
}

impl Dataset {
    pub fn features(&self) -> impl Iterator<Item = FeatureStartingFrom1> {
        (1..=self.feature_count).map(FeatureStartingFrom1)
    }
}
