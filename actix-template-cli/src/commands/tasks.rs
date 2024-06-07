use actix_web_starter_client::{
    apis::tasks_api::{self, CreateTaskSuccess},
    models::{CreateTaskReqPayload, Task},
};

use crate::errors::DefaultError;

use super::configure::ActixTemplateConfiguration;

fn transform_option<T>(opt: Option<T>) -> Option<Option<T>> {
    opt.map(Some)
}

pub async fn create_task_cmd(config: ActixTemplateConfiguration) -> Result<Task, DefaultError> {
    // Get description
    let description = inquire::Text::new("Enter description:").prompt_skippable()?;
    // Get due date? (optional)
    let due_date = inquire::DateSelect::new("Enter a due date")
        .prompt_skippable()?
        .map(|d| {
            // Convert to 2021-01-01T00:00:00 style string
            d.format("%Y-%m-%dT00:00:00").to_string()
        });

    // Send to API
    let result = tasks_api::create_task(
        &config.clone().into(),
        tasks_api::CreateTaskParams {
            organization: config.org_id,
            create_task_req_payload: CreateTaskReqPayload {
                description: transform_option(description),
                contact_id: None,
                deadline: transform_option(due_date),
            },
        },
    )
    .await?
    .entity
    .unwrap();
    // Return task
    match result {
        CreateTaskSuccess::Status201(task) => Ok(task),
        CreateTaskSuccess::UnknownValue(_) => Err(DefaultError::new(
            "Could not parse response body creating task",
        )),
    }
}
