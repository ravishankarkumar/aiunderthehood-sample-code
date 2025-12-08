use plotters::prelude::*;
use ndarray::array;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("visualizations/gradient_descent.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Gradient Descent", ("sans-serif", 50))
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-3.0f64..3.0, -1.0f64..9.0)?;

    chart.configure_mesh().draw()?;

    // Plot cost function J(theta) = theta^2
    chart.draw_series(LineSeries::new(
        (-300..=300).map(|x| x as f64 / 100.0).map(|x| (x, x * x)),
        ShapeStyle::from(&RGBColor(200, 180, 0)).stroke_width(3),
    ))?;

    // Simulate gradient descent
    let mut theta = 2.0;
    let learning_rate = 0.1;
    let mut points = Vec::new();
    for _ in 0..10 {
        points.push((theta, theta * theta));
        theta -= learning_rate * 2.0 * theta; // Gradient: 2*theta
    }

    // Plot points
    chart.draw_series(PointSeries::of_element(
        points,
        5,
        &RED,
        &|c, s, st| Circle::new(c, s, st.filled()),
    ))?;

    Ok(())
}