use matan;

fn main() -> Result<(), String> {
    matan::utils::LayoutBuilder::two_dimension()
        .real_real((800, 600))
        .add_linear_function(|s| {
            // [from, to) (ms)
            s.add_transition_from_slope_form(0, 1000, (1., 0., 1.5), (-1., 0., 1.5), 500)
             .add_transition_from_slope_form(1000, 2000, (0.5, 0., 1.5), (-1.5, 0., 1.5), 500)
        })
        .run("Test")?;
    Ok(())
}