use cli_khronos::{domain::FileContent, error::AppError, service::FileService};

use chrono::{NaiveDate, NaiveTime, Utc};
use std::{fs, io::Error};
use thiserror::Error;

fn main() -> Result<(), AppError> {
    let mut setup = FileService::new("saved_logged_tasks.json")?;

    setup.initializing()?;
    println!("{:#?}", setup.instance.repo);

    // println!("{:?}", setup.initializing());
    Ok(())
}
