use matan::utils;

fn main() -> Result<(), String> {
    utils::LayoutBuilder::two_dimension()
        .real_real((800, 600))
        .add_linear_function(|s| {
            // [from, to) (ms)
            s.add_snapshot_from_slope_form(0, 3000, 3./4., 0., 3.)
        })
        .run("Test")?;
    Ok(())
}