use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::error::AppError;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use uuid::Uuid;

#[derive(Debug)]
pub struct FileContent {
    pub content: HashMap<TaskTitle, Task>,
    pub path: PathBuf,
}

impl FileContent {
    pub fn new(raw_path: &str) -> Result<Self, AppError> {
        if raw_path.is_empty() {
            return Err(AppError::UnvalidPath);
        }
        let path = PathBuf::from(raw_path.trim());
        let content = HashMap::<TaskTitle, Task>::new();
        Ok(Self { path, content })
    }
}

#[derive(Debug)]
pub struct Task {
    pub id: TaskID,
    pub title: TaskTitle,
    pub timeline: TimeData,
    pub tag: TaskTag,
}

#[derive(Debug)]
pub struct TaskTitle {
    title: String,
}

impl TaskTitle {
    pub fn new(raw_title: &str) -> Result<Self, AppError> {
        if raw_title.is_empty() || raw_title.chars().all(|c| !c.is_alphabetic()) {
            return Err(AppError::UnvalidTaskTitleFormat);
        }
        let title = raw_title.trim().to_lowercase().to_string();

        if title.len() < 3 || title.len() > 15 {
            return Err(AppError::UnvalidTaskName);
        }

        Ok(Self { title })
    }
}

#[derive(Debug)]
struct TimeData {
    duration: String,
    day_week: String,
    date: String,
    starting_hour: NaiveTime,
    ending_HOUR: NaiveTime
}

impl TimeData {
    fn from_raw(raw_datetime: &str) -> Result<Self, AppError> {
        if raw_datetime.is_empty() || raw_datetime.chars().all(|c| !c.is_ascii_alphanumeric()) {
            return Err(AppError::UnvalidDateTimeFormat);
        }
        
        let pattern = r#""id":"([^"]+)","name":"([^"]+)","timedate":"([^"]+)","tags":"([^"]+)""#;
        let re = Regex::new(pattern)?;

        let cleaned_datetime = raw_datetime.chars().skip(11).fold(String::new(), |str, e| 
        {str.push(e);
        str}
    );
        
        let datetime = NaiveDateTime::parse_from_str(
            &cleaned_datetime,"(\d{2}[::]\d{2}[::]\d{2}[^\"]+)" )
    }

    fn normalize_time_chars(input: &str) -> String {
        input.replace('∶', ":")
    }
}

#[derive(Debug)]
struct TaskTag(Option<String>);

impl TaskTag {
    fn from_raw(raw_tag: &str) -> Result<Self, AppError> {
        if raw_tag.is_empty()
            || raw_tag.chars().all(|c| !c.is_alphabetic())
            || raw_tag.chars().all(|c| c.is_ascii_punctuation())
        {
            return Err(AppError::UnvalidTaskTagFormat);
        }

        let tag = raw_tag.trim().to_lowercase().to_string();

        Ok(Self(Some(tag)))
    }
}
