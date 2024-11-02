mod lib;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 请将 "data.csv" 替换为您的 CSV 文件路径
    let df = lib::read_csv("..data/stock_AAPL.csv")?;

    let result_df = lib::compute_statistics(df)?;

    println!("{}", result_df);

    Ok(())
}
