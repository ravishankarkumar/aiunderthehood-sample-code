use nalgebra::{Matrix2, Vector2};

fn main() {
    // Define vectors
    let v1 = Vector2::new(1.0, 2.0);
    let v2 = Vector2::new(3.0, 4.0);
    let dot_product = v1.dot(&v2);
    println!("Dot Product: {}", dot_product);

    // Define matrices
    let m1 = Matrix2::new(1.0, 2.0, 3.0, 4.0);
    let m2 = Matrix2::new(5.0, 6.0, 7.0, 8.0);
    let product = m1 * m2;
    println!("Matrix Product:\n{}", product);
}