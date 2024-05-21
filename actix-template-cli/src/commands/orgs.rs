use std::fmt::Display;

use actix_web_starter_client::{
    apis::{
        configuration::Configuration,
        orgs_api::{
            CreateOrgParams, CreateOrgSuccess, DeleteOrgSuccess, GetMyOrgsParams, GetMyOrgsSuccess,
        },
    },
    models::CreateOrgReqPayload,
};
use inquire::Select;

use super::configure::ActixTemplateConfiguration;

pub async fn create_org(settings: ActixTemplateConfiguration, name: Option<String>) {
    let name = if name.is_none() {
        inquire::Text::new("Enter a name for the organization:")
            .prompt()
            .expect("Prompt configured correctly")
    } else {
        name.unwrap()
    };

    let config = Configuration {
        base_path: settings.api_url.clone(),
        api_key: Some(actix_web_starter_client::apis::configuration::ApiKey {
            prefix: None,
            key: settings.api_key.clone(),
        }),
        ..Default::default()
    };

    let payload = CreateOrgReqPayload { name };

    let created = actix_web_starter_client::apis::orgs_api::create_org(
        &config,
        CreateOrgParams {
            create_org_req_payload: payload,
        },
    )
    .await
    .map_err(|e| {
        eprintln!("Error creating organization: {:?}", e);
        std::process::exit(1);
    })
    .unwrap()
    .entity
    .unwrap();

    match created {
        CreateOrgSuccess::Status201(org) => {
            println!("\nOrganization created successfully!\n");
            println!("Name: {}", org.name);
        }
        CreateOrgSuccess::UnknownValue(_) => {
            eprintln!("Error creating organization.");
            std::process::exit(1);
        }
    };
}

// TODO: better error type
enum OrgSelectError {
    NoOrgs,
    OrgFetchFailure,
}

struct OrgSelectOption {
    name: String,
    id: uuid::Uuid,
}

impl Display for OrgSelectOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

async fn select_from_my_orgs(
    config: &Configuration,
    prompt: &str,
) -> Result<uuid::Uuid, OrgSelectError> {
    let orgs = actix_web_starter_client::apis::orgs_api::get_my_orgs(
        &config,
        GetMyOrgsParams {
            limit: None,
            offset: None,
        },
    )
    .await
    .unwrap()
    .entity
    .unwrap();

    let org_list = match orgs {
        GetMyOrgsSuccess::Status200(org_list) => org_list,
        GetMyOrgsSuccess::UnknownValue(_) => {
            return Err(OrgSelectError::OrgFetchFailure);
        }
    };

    if org_list.is_empty() {
        return Err(OrgSelectError::NoOrgs);
    }

    let options: Vec<OrgSelectOption> = org_list
        .iter()
        .map(|org_result| OrgSelectOption {
            id: org_result.id,
            name: org_result.name.clone(),
        })
        .collect();

    let ans = Select::new(prompt, options)
        .prompt()
        .expect("Prompt is configured correctly");

    Ok(ans.id)
}

pub async fn delete_org(settings: ActixTemplateConfiguration) {
    // Fetch the list of orgs
    let config = Configuration {
        base_path: settings.api_url.clone(),
        api_key: Some(actix_web_starter_client::apis::configuration::ApiKey {
            prefix: None,
            key: settings.api_key.clone(),
        }),
        ..Default::default()
    };

    let selected = match select_from_my_orgs(&config, "Select an organization to delete:").await {
        Ok(ans) => ans,
        _ => {
            eprintln!("Error fetching organizations.");
            std::process::exit(1);
        }
    };

    let deleted = actix_web_starter_client::apis::orgs_api::delete_org(
        &config,
        actix_web_starter_client::apis::orgs_api::DeleteOrgParams {
            org_id: selected.to_string(),
        },
    )
    .await
    .map_err(|e| {
        eprintln!("Error deleting organization: {:?}", e);
    })
    .unwrap()
    .entity;

    if deleted.is_none() {
        eprintln!("Error deleting organization.");
        std::process::exit(1);
    }

    // TODO: FIX!!!
    match deleted.unwrap() {
        DeleteOrgSuccess::Status200() => {
            println!("\nOrganization deleted successfully!\n");
        }

        DeleteOrgSuccess::UnknownValue(_) => {
            eprintln!("Error deleting organization.");
            std::process::exit(1);
        }
    };
}
