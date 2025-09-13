use statrs::statistics::{Data, Distribution};
use statrs::distribution::{ContinuousCDF, StudentsT};

fn main() {
    // Synthetic datasets
    let data1 = vec![2.1, 2.3, 2.0, 2.2, 2.4]; // Group 1
    let data2 = vec![2.5, 2.7, 2.4, 2.6, 2.8]; // Group 2

    // Compute means
    let mean1 = Data::new(data1.clone()).mean().unwrap();
    let mean2 = Data::new(data2.clone()).mean().unwrap();
    println!("Mean1: {}, Mean2: {}", mean1, mean2);

    // Perform t-test (assuming equal variances)
    let t_stat = t_test(&data1, &data2);
    println!("T-statistic: {}", t_stat);

    // Check p-value (two-tailed, df=8)
    let t_dist = StudentsT::new(0.0, 1.0, 8.0).unwrap();
    let p_value = 2.0 * (1.0 - t_dist.cdf(t_stat.abs()));
    println!("P-value: {}", p_value);
}

fn t_test(data1: &[f64], data2: &[f64]) -> f64 {
    let n1 = data1.len() as f64;
    let n2 = data2.len() as f64;
    let mean1 = data1.iter().sum::<f64>() / n1;
    let mean2 = data2.iter().sum::<f64>() / n2;

    let var1 = data1.iter().map(|x| (x - mean1).powi(2)).sum::<f64>() / (n1 - 1.0);
    let var2 = data2.iter().map(|x| (x - mean2).powi(2)).sum::<f64>() / (n2 - 1.0);

    let se = ((var1 / n1) + (var2 / n2)).sqrt();
    (mean1 - mean2) / se
}