use polars::prelude::*;
use polars::lazy::prelude::*;
use std::error::Error;

pub fn example() -> PolarsResult<DataFrame> {
    CsvReadOptions::default()
            .with_has_header(true)
            .try_into_reader_with_file_path(Some(path.into()))?
            .finish()
}

pub fn compute_statistics(df: DataFrame) -> Result<DataFrame, Box<dyn Error>> {
    // 将 DataFrame 转换为 LazyFrame
    let lazy_df = df.lazy();

    // 确保 'Close' 列是浮点型，'Year' 列是整型
    let lazy_df = lazy_df
        .with_columns([
            col("Close").cast(DataType::Float64),
            col("Year").cast(DataType::Int32),
        ]);

    let result_df = lazy_df
        .group_by([col("Year")])
        .agg([
            col("Close").mean().alias("mean"),
            col("Close").median().alias("median"),
            col("Close").std(0).alias("std"),
        ])
        .sort(
            ["Year"], // 使用切片传递列名
            Default::default(),
        )
        .collect()?;

    Ok(result_df)
}
