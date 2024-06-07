use actix_web_starter_client::{
    apis::tasks_api::{self, CreateTaskSuccess, GetTaskParams},
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

pub async fn _get_task(
    config: ActixTemplateConfiguration,
    task_id: String,
) -> Result<Task, DefaultError> {
    let response = tasks_api::get_task(
        &config.clone().into(),
        GetTaskParams {
            task_id,
            organization: config.org_id,
        },
    )
    .await?
    .entity
    .unwrap();

    match response {
        tasks_api::GetTaskSuccess::Status200(task) => Ok(task),
        tasks_api::GetTaskSuccess::UnknownValue(_) => Err(DefaultError::new(
            "Could not parse response body getting task by id",
        )),
    }
}

pub async fn delete_task_cmd(
    config: ActixTemplateConfiguration,
    task_id: Option<String>,
) -> Result<(), DefaultError> {
    let task_id = if task_id.is_none() {
        inquire::Text::new("Enter task ID to delete:").prompt()?
    } else {
        task_id.unwrap()
    };

    let delete_response = tasks_api::delete_task(
        &config.clone().into(),
        tasks_api::DeleteTaskParams {
            task_id,
            organization: config.org_id,
        },
    )
    .await?
    .status
    .is_success();

    match delete_response {
        true => {
            println!("Task deleted successfully");
            Ok(())
        }
        false => {
            println!("Task could not be deleted");
            Err(DefaultError::new("Task could not be deleted"))
        }
    }
}
