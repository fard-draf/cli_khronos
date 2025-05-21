use std::path::PathBuf;

use crate::error::AppError;
use chrono::{NaiveDate, NaiveTime};

#[derive(Debug)]
struct FileContent {
    content: Vec<Task>,
}

impl FileContent {
    fn new(path: &str) -> Result<Self, AppError> {
        Self
    }
}

#[derive(Debug)]
pub struct Path {
    path: PathBuf,
}

#[derive(Debug)]
struct Task {
    title: String,
    date: NaiveDate,
    duration: NaiveTime,
    tag: Option<String>,
}
