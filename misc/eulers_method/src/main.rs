fn eulers_method(
    dy_dt: &dyn Fn(f64, f64) -> f64,
    y_0: f64,
    t_interval: &(f64, f64),
    delta_t: f64,
    output_steps: &[i64],
) {
    let mut y = y_0;
    let mut t = t_interval.0;
    let mut points = Vec::new();

    let steps = ((t_interval.1 - t_interval.0) / 0.1 as f64).floor() as i64;
    // Go one step further so last output_step works
    for i in 0..(steps + 1) {
        let slope = dy_dt(t, y);

        // If there are no steps passed, display all
        if output_steps.len() == 0 || output_steps.contains(&i) {
            println!("*** Step {} ***", i);
            println!("  t_{} = {:.8}", i, t);
            println!("  y_{} = {:.8}", i, y);
            println!("  dy/dt = {:.8}", slope);
            println!();
        }
        points.push((t, y));

        y += delta_t * slope;
        t += delta_t;
    }

    // Print all points to copy into Desmos
    println!("*** Points ***");
    // Print first point separately since it doesn't require a leading comma
    print!("({:.8}, {:.8})", points[0].0, points[0].1);
    for point in points[1..].iter() {
        print!(",({:.8}, {:.8})", point.0, point.1);
    }
    println!();
}

/**
 * Main entry point into the Euler's Method program.
 */
fn main() {
    // Define dy/dt here
    fn dy_dt(t: f64, y: f64) -> f64 {
        return y.powi(2) - y.powi(3);
    }

    // Set initial conditions here
    let y_0 = 0.2;
    let t_interval = (0.0, 10.0);
    let delta_t = 0.1;
    // Set steps to display here. Set to an empty array to display all steps.
    let output_steps = [0, 10, 20, 30, 40, 50, 60, 70, 80, 90, 100];

    eulers_method(&dy_dt, y_0, &t_interval, delta_t, &output_steps);
}
