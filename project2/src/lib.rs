pub mod io;

#[derive(Debug)]
pub struct Dataset {
    pub class_count: usize,
    pub feature_count: usize,
    pub instances: Vec<Instance>,
}

#[derive(Debug)]
pub struct Instance {
    /// Starts from 1.
    pub class: usize,
    pub feature_values: Vec<f64>,
}
