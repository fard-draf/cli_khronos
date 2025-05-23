use regex::Regex;
use serde_json::{self};
use std::{
    collections::HashMap,
    fs::{self},
};
use tracing::info;

use crate::{
    domain::{Task, TaskID, TimeRecords},
    dto::TaskDTO,
    error::AppError,
};
use chrono::{NaiveTime, TimeDelta, Timelike};

#[derive(Debug)]
pub struct FileService {
    pub instance: HashMap<TaskID, Task>,
    pub path: String,
}

impl FileService {
    pub fn new(path: &str) -> Result<Self, AppError> {
        Ok(Self {
            instance: HashMap::<TaskID, Task>::new(),
            path: path.to_string(),
        })
    }

    pub fn initializing(&mut self) -> Result<(), AppError> {
        let global_file = fs::read_to_string(&self.path)?;
        if global_file.is_empty() {
            return Err(AppError::EmptyFile);
        }

        self.extract_with_json(&global_file)?;

        Ok(())
    }

    pub fn extract_with_json(&mut self, raw_stce: &str) -> Result<(), AppError> {
        let dtos: Vec<TaskDTO> = serde_json::from_str(raw_stce)?;

        for dto in dtos {
            let task = dto.into_domain()?;
            self.instance.entry(task.id.clone()).or_insert(task);
        }

        Ok(())
    }
}

pub struct TimeService {}

impl TimeService {
    pub fn timechart_from_raw(raw_datetime: &str) -> Result<TimeRecords, AppError> {
        if raw_datetime.is_empty() || raw_datetime.chars().all(|c| !c.is_ascii_alphanumeric()) {
            return Err(AppError::UnvalidDateTimeFormat);
        }

        let re = Regex::new(
            r"(\d{2}[∶:]\d{2}[∶:]\d{2})\n([A-Za-z]{3}), (\d{2}/\d{2}) (\d{2}[∶:]\d{2}[∶:]\d{2}) – (\d{2}[∶:]\d{2}[∶:]\d{2})",
        )?;

        tracing::info!("Tentative de parsing: {:?}", raw_datetime);

        if let Some(captures) = re.captures(raw_datetime) {
            tracing::info!("Captures trouvées: {:?}", captures);

            let effective_duration = TimeService::parse_duration_from_hsm(&captures[1]);
            let day_week = &captures[2];
            let date = &captures[3];
            let starting_time =
                NaiveTime::parse_from_str(&captures[4].replace('∶', ":"), "%H:%M:%S")?;
            let ending_time =
                NaiveTime::parse_from_str(&captures[5].replace('∶', ":"), "%H:%M:%S")?;

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
                day_week: day_week.to_string(),
                date: date.to_string(),
                starting_time,
                ending_time,
                total_time,
                break_time,
            };

            Ok(instance)
        } else {
            info!("Parsing date error");
            Err(AppError::ErrorParsingDate)
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
