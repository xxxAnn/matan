use matan;

fn main() -> Result<(), String> {
    matan::utils::LayoutBuilder::two_dimension()
        .real_real((800, 600))
        .add_linear_function(|s| {
            // [from, to) (ms)
            s.add_transition_from_slope_form(0, 3000, (3./4., 0., 3.), (1., 0., 3.), 100)
        })
        .run("Test")?;
    Ok(())
}