use chrono::{NaiveDate, NaiveTime, Utc};
use std::{fs, io::Error};
use thiserror::Error;

fn main() -> Result<(), Error> {
    let contents = fs::read_to_string("file.txt")?;
    let rust_count = contents.lines().filter(|c| c.contains("Rust")).count();
    let total_count = contents.lines().count();
    println!("rust time: {}, total: {}", rust_count, total_count);
    Ok(())
}
