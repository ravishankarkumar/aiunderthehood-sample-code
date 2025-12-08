use linfa::prelude::*;
use linfa_linear::LinearRegression;
use ndarray::{array, Array2, Array1};

fn main() {
    // Synthetic dataset: features (size in sqft, age in years), target (price in $)
    let x: Array2<f64> = array![
        [1000.0, 5.0],
        [1500.0, 10.0],
        [2000.0, 3.0],
        [2500.0, 8.0],
        [3000.0, 2.0]
    ];
    let y: Array1<f64> = array![200000.0, 250000.0, 300000.0, 350000.0, 400000.0];

    // Create dataset
    let dataset = Dataset::new(x.clone(), y.clone());

    // Train linear regression model
    let model = LinearRegression::default().fit(&dataset).unwrap();
    println!("Intercept: {}, Weights: {:?}", model.intercept(), model.params());

    // Predict and evaluate
    let predictions = model.predict(&x);
    let mse = predictions
        .iter()
        .zip(y.iter())
        .map(|(p, t)| (p - t).powi(2))
        .sum::<f64>()
        / x.nrows() as f64;
    let y_mean = y.iter().sum::<f64>() / y.len() as f64;
    let ss_tot = y.iter().map(|y| (y - y_mean).powi(2)).sum::<f64>();
    let ss_res = predictions
        .iter()
        .zip(y.iter())
        .map(|(p, t)| (p - t).powi(2))
        .sum::<f64>();
    let r2 = 1.0 - ss_res / ss_tot;
    println!("MSE: {}, R^2: {}", mse, r2);

    // Test prediction
    let test_x = array![[2800.0, 4.0]];
    let test_pred = model.predict(&test_x);
    println!("Prediction for size=2800, age=4: {}", test_pred[0]);
}