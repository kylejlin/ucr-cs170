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
/// that the classes are counted starting from 1, not 0.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct ClassStartingFrom1(pub usize);

/// We create a new type to help us remember
/// that the features are counted starting from 1, not 0.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct FeatureStartingFrom1(pub usize);

#[derive(Debug, Clone)]
pub struct FeatureSet(pub Vec<FeatureStartingFrom1>);

#[derive(Debug)]
pub struct Instance {
    /// Starts from 1.
    pub class: ClassStartingFrom1,
    pub feature_values: Vec<f64>,
}

pub fn forward_selection(dataset: &Dataset) -> FeatureSet {
    println!("Beginning search.");

    let mut current_set = FeatureSet(vec![]);
    let mut best_set = FeatureSet(vec![]);
    let mut best_accuracy = dataset.default_rate();

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
    println!("Beginning search.");

    let mut current_set = dataset.complete_feature_set();
    let mut best_set = dataset.complete_feature_set();
    let mut best_accuracy = leave_out_one_cross_validation(dataset, &current_set);

    for _ in 0..dataset.feature_count {
        let mut best_feature: Option<FeatureStartingFrom1> = None;
        let mut best_accuracy_for_current_outer_iteration = -1.0;

        for candidate_feature in dataset.features() {
            // Don't remove the feature if it was already removed.
            if !current_set.contains(candidate_feature) {
                continue;
            }

            let reduced_set = current_set.removing(candidate_feature);

            let accuracy = leave_out_one_cross_validation(&dataset, &reduced_set);

            println!(
                "    Using feature(s) {} accuracy is {:.1}%",
                reduced_set.pretty(),
                accuracy * 100.0
            );

            if accuracy > best_accuracy_for_current_outer_iteration {
                best_accuracy_for_current_outer_iteration = accuracy;
                best_feature = Some(candidate_feature);
            }
        }

        let best_feature = best_feature.unwrap();
        current_set.remove(best_feature);
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

/// Performs leave-one-out cross validation on the dataset
/// and returns the accuracy rate.
/// Only the features in `feature_set` are used.
pub fn leave_out_one_cross_validation(dataset: &Dataset, feature_set: &FeatureSet) -> f64 {
    if feature_set.is_empty() {
        return dataset.default_rate();
    }

    let mut correct_count = 0;

    for (index_of_instance_of_to_classify, instance_to_classify) in
        dataset.instances.iter().enumerate()
    {
        let mut closest_square_distance = f64::INFINITY;
        let mut closest_class = ClassStartingFrom1(0);

        for (neighbor_index, neighbor) in dataset.instances.iter().enumerate() {
            // Don't compare an instance to itself, since the distance is always 0.
            if index_of_instance_of_to_classify == neighbor_index {
                continue;
            }

            let square_distance = instance_to_classify.square_distance_to(neighbor, feature_set);

            // Comparing square distances is equivalent to comparing distances
            // (i.e., for any `a` and `b`, `a < b` if and only if `a*a < b*b`).
            // This optimization avoids an unnecessary square root operation.
            if square_distance < closest_square_distance {
                closest_square_distance = square_distance;
                closest_class = neighbor.class;
            }
        }

        if closest_class == instance_to_classify.class {
            correct_count += 1;
        }
    }

    (correct_count as f64) / (dataset.instances.len() as f64)
}

impl Dataset {
    pub fn features(&self) -> impl Iterator<Item = FeatureStartingFrom1> + Clone {
        (1..=self.feature_count).map(FeatureStartingFrom1)
    }

    pub fn complete_feature_set(&self) -> FeatureSet {
        FeatureSet(self.features().collect())
    }

    pub fn default_rate(&self) -> f64 {
        self.default_class_and_rate().1
    }

    pub fn most_common_class(&self) -> ClassStartingFrom1 {
        self.default_class_and_rate().0
    }

    fn default_class_and_rate(&self) -> (ClassStartingFrom1, f64) {
        use std::collections::HashMap;

        if self.instances.is_empty() {
            panic!("cannot compute default rate of an empty dataset");
        }

        let mut class_counts: HashMap<ClassStartingFrom1, usize> = HashMap::new();

        for instance in &self.instances {
            *class_counts.entry(instance.class).or_insert(0) += 1;
        }

        let mut max_count = 0;
        let mut class_with_max_count = ClassStartingFrom1(0);
        for (class, count) in class_counts {
            if count > max_count {
                max_count = count;
                class_with_max_count = class;
            }
        }

        let default_class = class_with_max_count;
        let default_rate = (max_count as f64) / (self.instances.len() as f64);

        (default_class, default_rate)
    }
}

impl FeatureSet {
    pub fn contains(&self, feature: FeatureStartingFrom1) -> bool {
        self.0.contains(&feature)
    }

    pub fn add(&mut self, feature: FeatureStartingFrom1) {
        self.0.push(feature);
    }

    pub fn adding(&self, feature: FeatureStartingFrom1) -> FeatureSet {
        let mut out = self.clone();
        out.add(feature);
        out
    }

    pub fn remove(&mut self, feature: FeatureStartingFrom1) {
        self.0.retain(|&f| f != feature);
    }

    pub fn removing(&self, feature: FeatureStartingFrom1) -> FeatureSet {
        let mut out = self.clone();
        out.remove(feature);
        out
    }

    pub fn iter(&self) -> impl Iterator<Item = FeatureStartingFrom1> + '_ {
        self.0.iter().copied()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Instance {
    pub fn square_distance_to(&self, other: &Instance, feature_set: &FeatureSet) -> f64 {
        let mut sum = 0.0;

        for feature in feature_set.iter() {
            let feature_index = feature.0 - 1;
            let diff = self.feature_values[feature_index] - other.feature_values[feature_index];
            sum += diff * diff;
        }

        sum
    }
}
