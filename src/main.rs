// main.rs

mod lib;

use std::error::Error;
use polars::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    // Preprocess data
    let df = lib::preprocess_data()?;

    // Group by 'Year' and calculate statistics
    let mut yearly_stats = df
        .groupby(["Year"])?
        .agg(&[
            // Aggregate 'Close' column to compute mean, median, and std
            ("Close", &[
                // The methods should be specified as strings
                // Polars expects method names as strings like "mean", "median", "std"
                // Make sure these methods are supported
                "mean",
                "median",
                "std",
            ])
        ])?
        .sort("Year", Default::default())?;

    // The aggregated columns will have names like "Close_mean", "Close_median", "Close_std"
    // Rename them to "mean", "median", "std" for simplicity
    yearly_stats.rename("Close_mean", "mean")?;
    yearly_stats.rename("Close_median", "median")?;
    yearly_stats.rename("Close_std", "std")?;

    // Generate plot
    lib::generate_plot(&yearly_stats)?;

    Ok(())
}
