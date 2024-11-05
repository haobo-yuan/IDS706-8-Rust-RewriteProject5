use polars_core::prelude::*;
use polars_io::prelude::*;

use std::fs::File;
use std::error::Error;

use plotters::prelude::*;
// ChatGPT and Co-Pilot are used for reference


/// Function to preprocess data
pub fn preprocess_data() -> PolarsResult<DataFrame> {
    // Read the data from the CSV file with tab separator, https://docs.rs/polars/latest/polars/prelude/struct.CsvReader.html
    let mut df = CsvReader::CsvReadOptions::default()
            .with_has_header(true)
            .try_into_reader_with_file_path(Some("data/NASDAQ_100_Data_From_2010.csv".into()))?
            .with_delimiter(b'\t')
            .finish();

    // Filter for AAPL stock data
    df = df.filter(&df.column("Name")?.equal("AAPL")?)?;

    // Convert 'Date' column to Date type
    let date_series = df
        .column("Date")?
        .utf8()?
        .as_date(Some("%Y-%m-%d"))?
        .into_series()
        .rename("Date");

    // Replace 'Date' column with the converted date_series
    df.replace("Date", date_series)?;

    // Extract 'Year' from 'Date' and add as a new column
    let year_series = df
        .column("Date")?
        .date()?
        .year()
        .into_series()
        .rename("Year");

    df.with_column(year_series)?;

    Ok(df)
}

/// Function to generate plot
pub fn generate_plot(yearly_stats: &DataFrame) -> Result<(), Box<dyn Error>> {
    // Create a drawing area
    let root = BitMapBackend::new("pictures/plot.png", (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    // Extract data from DataFrame
    let years = yearly_stats
        .column("Year")?
        .i32()?
        .into_no_null_iter()
        .collect::<Vec<_>>();

    let means = yearly_stats
        .column("mean")?
        .f64()?
        .into_no_null_iter()
        .collect::<Vec<_>>();

    let medians = yearly_stats
        .column("median")?
        .f64()?
        .into_no_null_iter()
        .collect::<Vec<_>>();

    let stds = yearly_stats
        .column("std")?
        .f64()?
        .into_no_null_iter()
        .collect::<Vec<_>>();

    // Determine min and max values for axes
    let min_year = *years.first().unwrap();
    let max_year = *years.last().unwrap();
    let max_value = means
        .iter()
        .chain(medians.iter())
        .chain(stds.iter())
        .cloned()
        .fold(0.0_f64, f64::max);

    // Build chart
    let mut chart = ChartBuilder::on(&root)
        .caption("AAPL Close Price Statistics (2010-2021)", ("sans-serif", 40))
        .margin(5)
        .set_all_label_area_size(50)
        .build_cartesian_2d(min_year..max_year, 0.0..max_value)?;

    chart.configure_mesh().draw()?;

    // Plot mean
    chart.draw_series(LineSeries::new(
        years.iter().zip(means.iter()).map(|(&x, &y)| (x, y)),
        &RED,
    ))?
    .label("Mean")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 1, y)], &RED));

    // Plot median
    chart.draw_series(LineSeries::new(
        years.iter().zip(medians.iter()).map(|(&x, &y)| (x, y)),
        &BLUE,
    ))?
    .label("Median")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 1, y)], &BLUE));

    // Plot standard deviation
    chart.draw_series(LineSeries::new(
        years.iter().zip(stds.iter()).map(|(&x, &y)| (x, y)),
        &GREEN,
    ))?
    .label("Standard Deviation")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 1, y)], &GREEN));

    // Configure the legend
    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}
