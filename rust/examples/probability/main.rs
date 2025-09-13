use rand::rng;
use rand_distr::{Distribution, Normal};

fn main() {
    // Define normal distribution (mean=0, std=1)
    let normal = Normal::new(0.0, 1.0).unwrap();
    let mut rng = rng();

    // Generate 5 samples
    let samples: Vec<f64> = (0..5).map(|_| normal.sample(&mut rng)).collect();
    println!("Samples from N(0,1): {:?}", samples);

    // Compute sample mean
    let mean = samples.iter().sum::<f64>() / samples.len() as f64;
    println!("Sample Mean: {}", mean);
}