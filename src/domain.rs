use crate::error::{AppError, DomainError};
use serde::Deserialize;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use chrono::{NaiveTime, TimeDelta, Timelike};
use uuid::Uuid;

//===================================================================================================================================FileContent
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileContent {
    pub repo: HashMap<TaskTitle, Task>,
}

impl FileContent {
    pub fn new() -> Result<Self, DomainError> {
        Ok(Self {
            repo: HashMap::<TaskTitle, Task>::new(),
        })
    }
}

//===================================================================================================================================TASK
#[derive(Debug, Hash, Clone, PartialEq, Eq, Deserialize)]
pub struct Task {
    pub id: TaskID,
    pub title: TaskTitle,
    pub timeline: TimeRecords,
    pub tag: TaskTag,
}

//============================================================================================================TaskID
#[derive(Debug, Hash, Clone, PartialEq, Eq, Deserialize)]
pub struct TaskID(String);

impl TaskID {
    pub fn from_service(id: &str) -> Result<Self, DomainError> {
        Ok(TaskID(id.to_string()))
    }

    pub fn as_str(&self) -> String {
        self.0.to_string()
    }
}

//============================================================================================================TaskTitle
#[derive(Debug, Hash, Clone, PartialEq, Eq, Deserialize)]
pub struct TaskTitle {
    title: String,
}

impl TaskTitle {
    pub fn new(raw_title: &str) -> Result<Self, DomainError> {
        if raw_title.is_empty() || raw_title.chars().all(|c| !c.is_alphabetic()) {
            return Err(DomainError::UnvalidTaskTitleFormat);
        }
        let title = raw_title.trim().to_lowercase().to_string();

        if title.len() < 3 || title.len() > 15 {
            return Err(DomainError::UnvalidTaskTitleFormat);
        }

        Ok(Self { title })
    }

    pub fn as_str(&self) -> String {
        self.title.to_string()
    }
}

//============================================================================================================TimeData
#[derive(Debug, Hash, Clone, PartialEq, Eq, Deserialize)]
pub struct TimeRecords {
    pub effective_duration: Option<TimeDelta>,
    pub day_week: String,
    pub date: String,
    pub starting_time: NaiveTime,
    pub ending_time: NaiveTime,
    pub total_time: TimeDelta,
    pub break_time: TimeDelta,
}

impl TimeRecords {
    pub fn as_srt(&self) -> String {}

    pub fn new(
        effective_duration: Option<TimeDelta>,
        day_week: String,
        date: String,
        starting_time: NaiveTime,
        ending_time: NaiveTime,
        total_time: TimeDelta,
        break_time: TimeDelta,
    ) -> Result<Self, DomainError> {
        if ending_time <= starting_time {
            return Err(DomainError::UnvalidTimeRange);
        }

        let total_time = TimeRecords::calculate_total_time(starting_time, ending_time);
        let break_time = TimeRecords::calculate_break_time(effective_duration, total_time);

        Ok(TimeRecords {
            effective_duration,
            day_week,
            date,
            starting_time,
            ending_time,
            total_time,
            break_time,
        })
    }

    pub fn calculate_total_time(start: NaiveTime, end: NaiveTime) -> TimeDelta {
        if end >= start {
            TimeDelta::seconds(
                (end.num_seconds_from_midnight() - start.num_seconds_from_midnight()) as i64,
            )
        } else {
            TimeDelta::seconds(
                (86400 - start.num_seconds_from_midnight() + end.num_seconds_from_midnight())
                    as i64,
            )
        }
    }

    pub fn calculate_break_time(effective: Option<TimeDelta>, total: TimeDelta) -> TimeDelta {
        match effective {
            Some(duration) => total - duration,
            None => total,
        }
    }
}

//============================================================================================================TaskID
#[derive(Debug, Hash, Clone, PartialEq, Eq, Deserialize)]
pub struct TaskTag(Option<String>);

impl TaskTag {
    pub fn from_raw(raw_tag: &str) -> Result<Self, DomainError> {
        if raw_tag.is_empty()
            || raw_tag.chars().all(|c| !c.is_alphabetic())
            || raw_tag.chars().all(|c| c.is_ascii_punctuation())
        {
            return Err(DomainError::UnvalidTaskTagFormat);
        }

        let tag = raw_tag.trim().to_lowercase().to_string();

        Ok(Self(Some(tag)))
    }
}
