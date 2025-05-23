use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{self, from_str};
use std::fs::{self, File};

use crate::{
    domain::{FileContent, Task, TaskID, TaskTag, TaskTitle, TimeRecords},
    error::AppError,
};
use chrono::{NaiveTime, TimeDelta, Timelike};

#[derive(Debug)]
pub struct FileService {
    pub instance: FileContent,
    pub path: String,
}

impl FileService {
    pub fn new(path: &str) -> Result<Self, AppError> {
        Ok(Self {
            instance: FileContent::new()?,
            path: path.to_string(),
        })
    }

    pub fn initializing(&mut self) -> Result<(), AppError> {
        let global_file = fs::read_to_string(&self.path)?;
        if global_file.is_empty() {
            return Err(AppError::EmptyFile);
        }

        let global_file = global_file[2..(global_file.len() - 2)].to_string();
        let cuted = global_file.split("},{").collect::<Vec<&str>>();

        for element in cuted {
            self.extract_with_rgx(element)?;
        }

        Ok(())
    }

    // pub fn extract_with_rgx(&mut self, raw_stce: &str) -> Result<(), AppError> {
    //     // let stce_fmt = format!("r#") + raw_stce;
    //     let pattern =
    //         r#"\{"id":"([^"]+)","name":"([^"]+)","timedate":"([^"]+)","tags":"([^"]+)"\}"#;

    //     let re = Regex::new(pattern)?;

    //     println!("{:#?}", re.captures(raw_stce));
    //     if let Some(captures) = re.captures(raw_stce) {
    //         let id = IDService::from_raw(&captures[1]);
    //         let title = TaskTitle::new(&captures[2])?;
    //         let timeline = TimeService::timechart_from_raw(&captures[3])?;
    //         let tag = TaskTag::from_raw(&captures[4])?;

    //         println!(
    //             "id : {:?}, title: {:?}, timeline:{:#?}, tag: {:?}",
    //             id, title, timeline, tag
    //         );

    //         let task = Task {
    //             id: TaskID::from_service(id)?,
    //             title,
    //             timeline,
    //             tag,
    //         };

    //         println!("{:#?}", task);
    //         let mut hashmap = FileContent::new()?;
    //         hashmap.repo.entry(task.title.clone()).or_insert(task);
    //     }

    //     Ok(())
    // }

    pub fn extract_with_json(&mut self, raw_stce: &str) -> Result<(), AppError> {
        let tasks: Vec<Task> = serde_json::from_str(raw_stce)?;
        for task in tasks {
            let id = IDService::from_raw(&task.id.as_str());
            let title = TaskTitle::new(&task.title.as_str())?;
            let timeline = TimeService::timechart_from_raw(&task.timeline)?;
            let tag = TaskTag::from_raw(&task.tags)?;
        }
        println!(
            "id : {:?}, title: {:?}, timeline:{:#?}, tag: {:?}",
            id, title, timeline, tag
        );

        let task = Task {
            id: TaskID::from_service(id)?,
            title,
            timeline,
            tag,
        };

        println!("{:#?}", task);
        let mut hashmap = FileContent::new()?;
        hashmap.repo.entry(task.title.clone()).or_insert(task);

        Ok(())
    }
}

struct IDService {
    id: String,
}
impl IDService {
    pub fn from_raw(raw_id: &str) -> Self {
        Self {
            id: raw_id.to_string(),
        }
    }
}

struct TimeService {}

impl TimeService {
    fn timechart_from_raw(raw_datetime: &str) -> Result<TimeRecords, AppError> {
        if raw_datetime.is_empty() || raw_datetime.chars().all(|c| !c.is_ascii_alphanumeric()) {
            return Err(AppError::UnvalidDateTimeFormat);
        }

        let re = Regex::new(
            r"(\d{2}[∶:]\d{2}[∶:]\d{2})\\n[A-Za-z]{3}, (\d{2}/\d{2}) (\d{2}[∶:]\d{2}[∶:]\d{2}) – (\d{2}[∶:]\d{2}[∶:]\d{2})",
        )?;

        if let Some(captures) = re.captures(&raw_datetime.replace('∶', ":")) {
            let effective_duration = TimeService::parse_duration_from_hsm(&captures[1]);
            let day = &captures[2];
            let date = &captures[3];
            let starting_time = NaiveTime::parse_from_str(&captures[4], "%H:%M:%S")?;
            let ending_time = NaiveTime::parse_from_str(&captures[5], "%H:%M:%S")?;

            let total_time = if ending_time >= starting_time {
                TimeDelta::seconds(
                    (ending_time.num_seconds_from_midnight()
                        - starting_time.num_seconds_from_midnight()) as i64,
                )
            } else {
                TimeDelta::seconds(
                    (86400 - starting_time.num_seconds_from_midnight()
                        + ending_time.num_seconds_from_midnight()) as i64,
                )
            };

            let break_time = if let Some(value) = effective_duration {
                total_time - value
            } else {
                total_time
            };

            let instance = TimeRecords {
                effective_duration,
                day_week: day.to_string(),
                date: date.to_string(),
                starting_time,
                ending_time,
                total_time,
                break_time,
            };

            Ok(instance)
        } else {
            return Err(AppError::UnvalidFormat);
        }
    }

    pub fn parse_duration_from_hsm(input: &str) -> Option<TimeDelta> {
        let normalized = TimeService::normalize_time_chars(input);
        let parts = normalized.split(':').collect::<Vec<&str>>();

        if parts.len() != 3 {
            return None;
        }

        let hours: i64 = parts[0].parse().ok()?;
        let minutes: i64 = parts[1].parse().ok()?;
        let seconds: i64 = parts[2].parse().ok()?;

        Some(TimeDelta::hours(hours) + TimeDelta::minutes(minutes) + TimeDelta::seconds(seconds))
    }

    pub fn normalize_time_chars(input: &str) -> String {
        input.replace('∶', ":")
    }
}
