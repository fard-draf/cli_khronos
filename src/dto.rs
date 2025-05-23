use serde::Deserialize;

use crate::{
    domain::{Task, TaskID, TaskTag, TaskTitle},
    error::AppError,
    service::TimeService,
};

#[derive(Debug, Deserialize)]
pub struct TaskDTO {
    id: String,
    name: String,
    timedate: String,
    tags: String,
}

impl TaskDTO {
    pub fn into_domain(self) -> Result<Task, AppError> {
        let id = TaskID::from_dto(&self.id)?;
        let title = TaskTitle::from_dto(&self.name)?;
        let timeline = TimeService::timechart_from_raw(&self.timedate)?;
        let tag = TaskTag::from_dto(&self.tags)?;

        Ok(Task {
            id,
            title,
            timeline,
            tag,
        })
    }
}
