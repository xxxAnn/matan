use matan::utils;

fn main() -> Result<(), String> {
    utils::LayoutBuilder::two_dimension()
        .real_real((800, 600))
        .add_linear_function(|s| {
            // [from, to) (ms)
            s.add_snapshot_from_slope_form(0, 300, 0., 0., 3.)
            .add_snapshot_from_slope_form(299, 600, 0.005, 0., 3.)
            .add_snapshot_from_slope_form(599, 900, 0.01, 0., 3.)
            .add_snapshot_from_slope_form(899, 1200, 0.015, 0., 3.)
            .add_snapshot_from_slope_form(1199, 1500, 0.02, 0., 3.)
            .add_snapshot_from_slope_form(1499, 1800, 0.025, 0., 3.)
            .add_snapshot_from_slope_form(1799, 2100, 0.03, 0., 3.)
            .add_snapshot_from_slope_form(2099, 2400, 0.035, 0., 3.)
        })
        .run("Test")?;
    Ok(())
}