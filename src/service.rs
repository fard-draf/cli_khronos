use regex::Regex;
use std::fs;

use crate::{
    domain::{FileContent, Task, TaskTitle},
    error::AppError,
};
use chrono::{NaiveDate, NaiveTime};

#[derive(Debug)]
pub struct FileService {
    pub instance: FileContent,
}

impl FileService {
    pub fn initializing(&mut self) -> Result<(), AppError> {
        let global_file = fs::read_to_string(&self.instance.path)?;
        if global_file.is_empty() {
            return Err(AppError::EmptyFile);
        }

        let cuted = global_file.split("},{").collect::<Vec<&str>>();
        println!("{:#?}", cuted);

        self.regex_sentance(cuted);

        Ok(())
    }

    pub fn regex_sentance(&mut self, raw_stce: &str) -> Result<(), AppError> {
        
        let stce_fmt = format!("r#") + raw_stce;

        let pattern = r#""id":"([^"]+)","name":"([^"]+)","timedate":"([^"]+)","tags":"([^"]+)""#;
        let re = Regex::new(pattern)?;
         
         if let Some(captures) = re.captures(&stce_fmt);
        Ok(())
    }
}
