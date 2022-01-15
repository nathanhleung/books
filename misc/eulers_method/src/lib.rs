pub fn eulers_method(
  dy_dt: &dyn Fn(f64, f64) -> f64,
  y_0: f64,
  t_interval: &(f64, f64),
  delta_t: f64,
  output_steps: &[i64],
  show_points: bool,
  latex_format: bool,
) {
  let mut y = y_0;
  let mut t = t_interval.0;
  let mut points = Vec::new();

  let steps = ((t_interval.1 - t_interval.0) / delta_t as f64).floor() as i64;
  // Go one step further (to `=steps`) so last output_step works
  for i in 0..=steps {
    let slope = dy_dt(t, y);

    // If there are no steps passed, display all
    if output_steps.len() == 0 || output_steps.contains(&i) {
      // Print in format for LaTeX tables
      if latex_format {
        println!("{} & {:.3} & {:.8} & {:.8} \\\\", i, t, y, slope);
      } else {
        println!("*** Step {} ***", i);
        println!("  t_{} = {:.8}", i, t);
        println!("  y_{} = {:.8}", i, y);
        println!("  dy/dt = {:.8}", slope);
        println!();
      }
    }
    points.push((t, y));

    y += delta_t * slope;
    t += delta_t;
  }

  // Print all points to copy into Desmos
  if show_points {
    println!("*** Points ***");
    // Print first point separately since it doesn't require a leading comma
    print!("({:.8}, {:.8})", points[0].0, points[0].1);
    for point in points[1..].iter() {
      print!(",({:.8}, {:.8})", point.0, point.1);
    }
    println!();
  }
}
