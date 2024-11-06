// ChatGPT is used for reference
use std::error::Error;
use rusqlite::{Connection, Result};
use csv::ReaderBuilder;
use serde::Deserialize;

// 定义 `StockRecord` 并派生 `Deserialize`
#[derive(Debug, Deserialize)]
pub struct StockRecord {
    date: String,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    adj_close: f64,
    volume: i64,
    name: String,
    year: i32,
}

// 创建数据库连接并初始化表
pub fn init_db(db_path: &str) -> Result<Connection> {
    let conn = Connection::open(db_path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS stock_data (
            date TEXT,
            open REAL,
            high REAL,
            low REAL,
            close REAL,
            adj_close REAL,
            volume INTEGER,
            name TEXT,
            year INTEGER
        )",
        [],
    )?;
    Ok(conn)
}

// 从 CSV 文件加载数据并插入到数据库
pub fn load_csv_to_db(conn: &Connection, csv_path: &str) -> Result<(), Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().from_path(csv_path)?;
    for result in rdr.deserialize() {
        let record: StockRecord = result?;
        conn.execute(
            "INSERT INTO stock_data (date, open, high, low, close, adj_close, volume, name, year)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            rusqlite::params![
                record.date,
                record.open,
                record.high,
                record.low,
                record.close,
                record.adj_close,
                record.volume,
                record.name,
                record.year,
            ],
        )?;
    }
    Ok(())
}

// 按照年份分组，计算均值、中位数和标准差
pub fn calculate_stats(conn: &Connection) -> Result<Vec<(i32, f64, f64, f64)>> {
    let mut stmt = conn.prepare(
        "SELECT year,
                AVG(close) AS mean,
                MEDIAN(close) AS median,  -- SQLite 默认不支持 median，需要扩展支持
                STDDEV(close) AS std      -- SQLite 默认不支持 std，需要扩展支持
         FROM stock_data
         GROUP BY year"
    )?;
    let results = stmt
        .query_map([], |row| {
            Ok((
                row.get(0)?,  // year
                row.get(1)?,  // mean
                row.get(2)?,  // median
                row.get(3)?,  // std
            ))
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(results)
}


fn main() -> Result<(), Box<dyn Error>> {
    let db_path = "data/stock_AAPL.db";
    let csv_path = "data/stock_AAPL.csv";

    // 初始化数据库
    let conn = init_db(db_path)?;

    // 加载 CSV 数据到数据库
    load_csv_to_db(&conn, csv_path)?;

    // 计算每年的统计数据
    let stats = calculate_stats(&conn)?;

    // 输出表格
    println!("{:<6} {:<10} {:<10} {:<10}", "Year", "Mean", "Median", "Std");
    for (year, mean, median, std) in stats {
        println!("{:<6} {:<10.2} {:<10.2} {:<10.2}", year, mean, median, std);
    }

    Ok(())
}
