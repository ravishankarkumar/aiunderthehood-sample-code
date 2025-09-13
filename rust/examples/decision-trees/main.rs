use linfa::prelude::*;
use linfa_trees::DecisionTree;
use ndarray::{array, Array2, Array1};

fn main() {
    // Synthetic dataset: features (x1, x2), binary target (0 or 1)
    let x: Array2<f64> = array![
        [1.0, 2.0], [2.0, 1.0], [3.0, 3.0], [4.0, 5.0], [5.0, 4.0],
        [6.0, 1.0], [7.0, 2.0], [8.0, 3.0], [9.0, 4.0], [10.0, 5.0]
    ];
    let y: Array1<usize> = array![0, 0, 0, 0, 0, 1, 1, 1, 1, 1];

    // Create dataset
    let dataset = Dataset::new(x.clone(), y.clone());

    // Train decision tree classifier
    let model = DecisionTree::params()
        .max_depth(Some(3))
        .min_weight_split(2.0)
        .fit(&dataset)
        .unwrap();
    
    // Predict classes
    let predictions = model.predict(&x);
    println!("Predictions: {:?}", predictions);

    // Compute accuracy
    let accuracy = predictions
        .iter()
        .zip(y.iter())
        .filter(|(p, t)| p == t)
        .count() as f64
        / y.len() as f64;
    println!("Accuracy: {}", accuracy);
}