use eulers_method::eulers_method;
use eulers_method::vector::Vector;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Define dy/dt here
    fn dy_dt(_t: f64, y: Vector) -> Result<Vector, Box<dyn Error>> {
        if y.dimension() != 2 {
            panic!("This dy_dt function only works on vectors with dimension 2!");
        }

        let mut dy_dt_components = Vec::with_capacity(y.dimension());

        dy_dt_components.push(y.components()[1]);
        dy_dt_components.push((-1.0 / 5.0 * y.components()[1]) - y.components()[0]);

        let dy_dt_vector = Vector::new(y.dimension(), dy_dt_components)?;
        Ok(dy_dt_vector)
    }

    // Set initial conditions here
    let y_0 = Vector::new(2, vec![0.0, 1.0])?;
    let t_interval = (0.0, 10.0);
    let delta_t = 0.1;
    // Set steps to display here. Set to an empty array to display all steps.
    let output_steps = [];

    eulers_method(
        &dy_dt,
        y_0,
        &t_interval,
        delta_t,
        &output_steps,
        false,
        true,
        false,
    )
}
