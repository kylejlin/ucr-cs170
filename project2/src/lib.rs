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

/// We implement a feature set of an array _B_ of booleans.
/// The _i_-th feature is in the set if and only if _B\[i-1\]_ is `true`.
#[derive(Debug, Clone)]
pub struct FeatureSet(pub Vec<bool>);

#[derive(Debug)]
pub struct Instance {
    /// Starts from 1.
    pub class: ClassStartingFrom1,
    pub feature_values: Vec<f64>,
}

pub fn forward_selection(dataset: &Dataset) -> FeatureSet {
    println!("Beginning forward selection.");

    let mut current_set = dataset.empty_feature_set();
    let mut best_set = current_set.clone();
    let mut best_accuracy = -1.0;

    for _ in 0..dataset.feature_count {
        let mut best_feature: Option<FeatureStartingFrom1> = None;
        let mut best_accuracy_for_current_outer_iteration = -1.0;

        for candidate_feature in dataset.features() {
            // Don't add the feature if it's already in the set.
            if current_set.contains(candidate_feature) {
                continue;
            }

            let extended_set = current_set.adding(candidate_feature);

            let accuracy = leave_out_one_cross_validation(&dataset, &extended_set);

            println!(
                "    Using feature(s) {} accuracy is {:.1}%",
                extended_set.pretty(),
                accuracy * 100.0
            );

            if accuracy > best_accuracy_for_current_outer_iteration {
                best_accuracy_for_current_outer_iteration = accuracy;
                best_feature = Some(candidate_feature);
            }
        }

        let best_feature = best_feature.unwrap();
        current_set.add(best_feature);
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

pub fn backward_elimination(dataset: &Dataset) -> FeatureSet {
    panic!("TODO: Backward search is not implemented yet.");
}

/// Performs leave-one-out cross validation on the dataset
/// and returns the accuracy rate.
/// Only the features in `feature_set` are used.
pub fn leave_out_one_cross_validation(dataset: &Dataset, feature_set: &FeatureSet) -> f64 {
    let mut correct_count = 0;

    for (instance_index, instance) in dataset.instances.iter().enumerate() {
        let mut closest_square_distance = f64::INFINITY;
        let mut closest_class = ClassStartingFrom1(1);

        for (other_index, other) in dataset.instances.iter().enumerate() {
            // Don't compare an instance to itself, since the distance is always 0.
            if instance_index == other_index {
                continue;
            }

            let square_distance = instance.square_distance_to(other, feature_set);

            if square_distance < closest_square_distance {
                closest_square_distance = square_distance;
                closest_class = other.class;
            }
        }

        if closest_class == instance.class {
            correct_count += 1;
        }
    }

    correct_count as f64 / dataset.instances.len() as f64
}

impl Dataset {
    pub fn features(&self) -> impl Iterator<Item = FeatureStartingFrom1> + Clone {
        (1..=self.feature_count).map(FeatureStartingFrom1)
    }

    pub fn complete_feature_set(&self) -> FeatureSet {
        FeatureSet(vec![true; self.feature_count])
    }

    pub fn empty_feature_set(&self) -> FeatureSet {
        FeatureSet(vec![false; self.feature_count])
    }
}

impl FeatureSet {
    pub fn contains(&self, feature: FeatureStartingFrom1) -> bool {
        self.0[feature.0 - 1]
    }

    pub fn add(&mut self, feature: FeatureStartingFrom1) {
        self.0[feature.0 - 1] = true;
    }

    pub fn adding(&self, feature: FeatureStartingFrom1) -> FeatureSet {
        let mut out = self.clone();
        out.add(feature);
        out
    }

    pub fn remove(&mut self, feature: FeatureStartingFrom1) {
        self.0[feature.0 - 1] = false;
    }

    pub fn removing(&self, feature: FeatureStartingFrom1) -> FeatureSet {
        let mut out = self.clone();
        out.remove(feature);
        out
    }
}

impl Instance {
    pub fn square_distance_to(&self, other: &Instance, feature_set: &FeatureSet) -> f64 {
        let mut sum = 0.0;

        for (feature_index, is_included) in feature_set.0.iter().enumerate() {
            if !*is_included {
                continue;
            }

            let diff = self.feature_values[feature_index] - other.feature_values[feature_index];
            sum += diff * diff;
        }

        sum
    }
}
