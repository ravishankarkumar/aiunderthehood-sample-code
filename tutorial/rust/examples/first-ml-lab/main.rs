use linfa::prelude::*;
use linfa_linear::LinearRegression;
use ndarray::array;

fn main() {
    // Synthetic dataset: feature (x) and target (y)
    let x = array![[1.0], [2.0], [3.0], [4.0], [5.0]];
    let y = array![2.1, 4.2, 6.1, 8.3, 10.0];

    // Create dataset
    let dataset = Dataset::new(x, y);

    // Train linear regression model
    let model = LinearRegression::default().fit(&dataset).unwrap();

    // Predict on new data
    let new_x = array![[6.0]];
    let prediction = model.predict(&new_x);
    println!("Prediction for x=6: {}", prediction[0]);

    // Print model parameters
    let intercept = model.intercept();
    let weights = model.params();
    println!("Intercept: {}, Weights: {:?}", intercept, weights);
}