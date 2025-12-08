use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("visualizations/linear_regression.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Linear Regression", ("sans-serif", 50))
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0.0f64..6.0, 0.0f64..12.0)?;

    chart.configure_mesh().draw()?;

    // Data points (from your lab)
    let data = vec![(1.0, 2.1), (2.0, 4.2), (3.0, 6.1), (4.0, 8.3), (5.0, 10.0)];
    chart.draw_series(PointSeries::of_element(
        data.clone(),
        5,
        &RED,
        &|c, s, st| Circle::new(c, s, st.filled()),
    ))?;

    // Best-fit line (approximated from your lab: y ≈ 1.99x + 0.14)
    chart.draw_series(LineSeries::new(
      vec![(0.0, 0.14), (6.0, 0.14 + 1.99 * 6.0)],
      ShapeStyle::from(&RGBColor(200, 180, 0)).stroke_width(3),
    ))?;

    // Residuals as individual line segments
    chart.draw_series(data.iter().map(|&(x, y)| {
        PathElement::new(
            vec![(x, y), (x, 1.99 * x + 0.14)],
            ShapeStyle::from(&GREEN).stroke_width(2),
        )
    }))?;

    Ok(())
}