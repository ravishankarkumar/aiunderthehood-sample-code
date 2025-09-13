use linfa::prelude::*;
use linfa_svm::Svm;
use ndarray::{array, Array1, Array2};

fn main() {
  boolean_svm();
  regression_svm();
}

fn boolean_svm() {
    // Sample features: 4 samples, each with 2 features
    let x: Array2<f64> = array![
        [1.0, 2.0],
        [1.5, 1.8],
        [5.0, 8.0],
        [6.0, 9.0],
    ];

    // Labels: true = class 1, false = class 0
    let y: Array1<bool> = array![false, false, true, true];

    // Build the dataset
    let dataset = Dataset::new(x, y);

    // Specify output type: bool (for classification)
    let model = Svm::<f64, bool>::params()
        .linear_kernel()
        .pos_neg_weights(10.0, 10.0)
        .fit(&dataset)
        .expect("SVM training failed");

    // Predict on new samples
    let test_data: Array2<f64> = array![
        [1.2, 2.0],
        [6.0, 8.5]
    ];

    let predictions = model.predict(&test_data);

    println!("Predictions: {:?}", predictions.as_targets());
}

fn regression_svm() {
    // Sample features: 4 samples, each with 2 features
    let x: Array2<f64> = array![
        [1.0, 2.0],
        [1.5, 1.8],
        [5.0, 8.0],
        [6.0, 9.0],
    ];

    // Labels: continuous values
    let y: Array1<f64> = array![1.0, 2.0, 3.0, 4.0];

    // Build the dataset
    let dataset = Dataset::new(x, y);

    // Specify output type: f64 (for regression)
    let model = Svm::<f64, f64>::params()
        .linear_kernel()
        .fit(&dataset)
        .expect("SVM training failed");

    // Predict on new samples
    let test_data: Array2<f64> = array![
        [1.2, 2.0],
        [6.0, 8.5]
    ];

    let predictions = model.predict(&test_data);

    // Calculate Mean Squared Error (MSE) for accuracy
    let actual_values = array![1.1, 3.9]; // Replace with actual test labels
    let mse = predictions
        .iter()
        .zip(actual_values.iter())
        .map(|(pred, actual)| (pred - actual).powi(2))
        .sum::<f64>()
        / actual_values.len() as f64;

    println!("Predictions: {:?}", predictions);
    println!("Mean Squared Error (MSE): {:.4}", mse);
}