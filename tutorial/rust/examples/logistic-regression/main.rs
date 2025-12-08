use linfa::prelude::*;
use linfa_logistic::LogisticRegression;
use ndarray::{array, Array2, Array1};

fn main() {
    // Synthetic dataset: features (x1, x2), binary target (0 or 1)
    let x: Array2<f64> = array![
        [1.0, 2.0], [2.0, 1.0], [3.0, 3.0], [4.0, 5.0], [5.0, 4.0],
        [6.0, 1.0], [7.0, 2.0], [8.0, 3.0], [9.0, 4.0], [10.0, 5.0]
    ];
    let y: Array1<i32> = array![0, 0, 0, 0, 0, 1, 1, 1, 1, 1];

    // Create dataset
    let dataset = Dataset::new(x.clone(), y.clone());

    // Train logistic regression
    let model = LogisticRegression::default()
        .max_iterations(100)
        .fit(&dataset)
        .unwrap();
    println!("Intercept: {}, Weights: {:?}", model.intercept(), model.params());

    // Predict probabilities
    let probs = model.predict(&x);
    println!("Predicted Probabilities: {:?}", probs);

    // Compute accuracy
    let predictions = probs.mapv(|p| if (p as f64) >= 0.5 { 1 } else { 0 });
    let accuracy = predictions
        .iter()
        .zip(y.iter())
        .filter(|(p, t)| p == t)
        .count() as f64
        / y.len() as f64;
    println!("Accuracy: {}", accuracy);

    // Compute log-loss with clipping to avoid ln(0)
    let log_loss = -y
        .iter()
        .zip(probs.iter())
        .map(|(t, p)| {
            let t_float = *t as f64;
            let p_clipped = (*p as f64).max(1e-10).min(1.0 - 1e-10); // Clip to avoid ln(0)
            t_float * p_clipped.ln() + (1.0 - t_float) * (1.0 - p_clipped).ln()
        })
        .sum::<f64>()
        / y.len() as f64;
    println!("Log-Loss: {}", log_loss);
}