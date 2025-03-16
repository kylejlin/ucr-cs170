use super::*;

pub struct BadDatasetSyntaxError {
    pub message: String,
}

impl BadDatasetSyntaxError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

/// This function parses a string and returns a dataset.
/// It enforces the following constraints:
/// - The class of each instance must be at least `1.0`. Classes are floored to integers.
/// - The number of features in each instance must be consistent.
pub fn parse_dataset(s: &str) -> Result<Dataset, BadDatasetSyntaxError> {
    let mut instances = vec![];
    let mut class_count = 0;
    let mut data_set_feature_count = 0;

    for (line_index, line) in s.lines().enumerate() {
        let line_number = line_index + 1;

        let mut parts = line.split_whitespace();

        let class = parts.next().ok_or(BadDatasetSyntaxError::new(&format!(
            "missing class on line {line_number}"
        )))?;
        let class: f64 = class
            .parse::<f64>()
            .map_err(|_| {
                BadDatasetSyntaxError::new(&format!("invalid class on line {line_number}: {class}"))
            })?
            .floor();
        if class < 1.0 {
            return Err(BadDatasetSyntaxError::new(&format!(
                "invalid class on {line_number}: {class}. class must be no less than 1"
            )));
        }
        let class = class as usize;

        class_count = class_count.max(class);

        let instance_feature_count = parts.clone().count();

        if data_set_feature_count == 0 {
            data_set_feature_count = instance_feature_count;
        }

        if instance_feature_count != data_set_feature_count {
            return Err(BadDatasetSyntaxError::new(&format!(
                "inconsistent feature count on line {line_number}. expected {data_set_feature_count} features, but got {instance_feature_count}"
            )));
        }

        let feature_values = parts
            .map(|part| {
                part.parse::<f64>().map_err(|_| {
                    BadDatasetSyntaxError::new(&format!(
                        "invalid feature value on line {line_number}: {part}"
                    ))
                })
            })
            .collect::<Result<Vec<f64>, BadDatasetSyntaxError>>()?;

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
