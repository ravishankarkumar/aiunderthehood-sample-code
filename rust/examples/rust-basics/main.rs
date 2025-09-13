fn main() {
  let v1 = vec![1.0, 2.0, 3.0];
  let v2 = vec![4.0, 5.0, 6.0];
  let distance = euclidean_distance(&v1, &v2);
  println!("Euclidean Distance: {}", distance);
}

fn euclidean_distance(v1: &Vec<f64>, v2: &Vec<f64>) -> f64 {
  let mut sum = 0.0;
  for i in 0..v1.len() {
      sum += (v1[i] - v2[i]).powi(2);
  }
  sum.sqrt()
}