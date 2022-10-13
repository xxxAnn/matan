use matan;

fn main() -> Result<(), String> {
    matan::utils::LayoutBuilder::two_dimension()
        .real_real((800, 600))
        .add_linear_function(|s| {
            // [from, to) (ms)
            s.add_transition_from_slope_form(0, 10000, (1., 0., 1.5), (-1., 0., 1.5), 5000)
             .add_transition_from_slope_form(12000, 22000, (0.5, 0., 1.5), (-1.5, 0., 1.5), 5000)
        })
        .run("Test")?;
    Ok(())
}