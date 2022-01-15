use eulers_method::eulers_method;

fn main() {
    // Define dy/dt here
    fn dy_dt(t: f64, y: f64) -> f64 {
        9.8 - (0.18 * y.powi(2) / 54.0)
    }

    // Set initial conditions here
    let y_0 = 0.0;
    let t_interval = (0.0, 11.0);
    let delta_t = 0.001;
    // Set steps to display here. Set to an empty array to display all steps.
    let output_steps = [0, 10132, 10133, 10134, 10135, 11000];

    eulers_method(
        &dy_dt,
        y_0,
        &t_interval,
        delta_t,
        &output_steps,
        false,
        true,
    );
}
