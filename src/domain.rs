use crate::error::DomainError;
use serde::Deserialize;
use std::str::FromStr;

use chrono::{NaiveTime, TimeDelta, Timelike};
use uuid::Uuid;

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
pub struct TaskID(Uuid);

impl TaskID {
    pub fn from_dto(id: &str) -> Result<Self, DomainError> {
        Ok(Self(Uuid::from_str(id)?))
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
    pub fn from_dto(raw_title: &str) -> Result<Self, DomainError> {
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
    pub fn from_service(
        effective_duration: Option<TimeDelta>,
        day_week: String,
        date: String,
        starting_time: NaiveTime,
        ending_time: NaiveTime,
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
    pub fn from_dto(raw_tag: &str) -> Result<Self, DomainError> {
        let tag = raw_tag.trim().to_lowercase().to_string();

        Ok(Self(Some(tag)))
    }
}
