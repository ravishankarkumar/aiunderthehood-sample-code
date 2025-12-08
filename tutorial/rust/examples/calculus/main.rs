use nalgebra::Vector2;

fn main() {
    // Define point (x, y)
    let point = Vector2::new(2.0, 3.0);

    // Compute gradient of f(x, y) = x^2 + y^2
    let gradient = Vector2::new(2.0 * point.x, 2.0 * point.y);
    println!("Gradient at ({}, {}): {:?}", point.x, point.y, gradient);
}