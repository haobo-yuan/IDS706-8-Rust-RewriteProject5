mod lib;
use rusqlite::Connection;

fn main() -> rusqlite::Result<()> {
    let db_path = "/data/stock_AAPL.db";
    let csv_path = "/data/stock_AAPL.csv";

    // 初始化数据库
    let conn = lib::init_db(db_path)?;

    // 加载 CSV 数据到数据库
    lib::load_csv_to_db(&conn, csv_path)?;

    // 计算每年的统计数据
    let stats = lib::calculate_stats(&conn)?;

    // 输出表格
    println!("{:<6} {:<10} {:<10} {:<10}", "Year", "Mean", "Median", "Std");
    for (year, mean, median, std) in stats {
        println!("{:<6} {:<10.2} {:<10.2} {:<10.2}", year, mean, median, std);
    }

    Ok(())
}
