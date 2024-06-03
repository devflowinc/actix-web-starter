use std::fmt::Display;

use actix_web_starter_client::{
    apis::{
        configuration::Configuration,
        invitation_api,
        orgs_api::{
            CreateOrgError, CreateOrgParams, CreateOrgSuccess, GetOrgsForAuthedUserParams,
            GetOrgsForAuthedUserSuccess,
        },
    },
    models::{CreateOrgReqPayload, InvitationData, Org},
};
use inquire::{Confirm, Select};

use super::configure::ActixTemplateConfiguration;

pub async fn create_org(
    settings: ActixTemplateConfiguration,
    name: Option<String>,
) -> Result<Org, CreateOrgError> {
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
            return Ok(org);
        }
        CreateOrgSuccess::UnknownValue(_) => {
            eprintln!("Error creating organization.");
            std::process::exit(1);
        }
    };
}

// TODO: better error type
#[derive(Debug)]
pub enum OrgSelectError {
    NoOrgs,
    OrgFetchFailure,
}

#[derive(Debug)]
pub struct OrgSelectOption {
    pub name: String,
    pub id: String,
}

impl Display for OrgSelectOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub async fn select_from_my_orgs(
    config: &Configuration,
    prompt: &str,
) -> Result<OrgSelectOption, OrgSelectError> {
    let orgs = actix_web_starter_client::apis::orgs_api::get_orgs_for_authed_user(
        &config,
        GetOrgsForAuthedUserParams {
            limit: None,
            offset: None,
        },
    )
    .await
    .unwrap()
    .entity
    .unwrap();

    let org_list = match orgs {
        GetOrgsForAuthedUserSuccess::Status200(org_list) => org_list,
        GetOrgsForAuthedUserSuccess::UnknownValue(_) => {
            return Err(OrgSelectError::OrgFetchFailure);
        }
    };

    if org_list.is_empty() {
        return Err(OrgSelectError::NoOrgs);
    }

    let options: Vec<OrgSelectOption> = org_list
        .iter()
        .map(|org_result| OrgSelectOption {
            id: org_result.id.clone(),
            name: org_result.name.clone(),
        })
        .collect();

    let ans = Select::new(prompt, options)
        .prompt()
        .expect("Prompt is configured correctly");

    Ok(ans)
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
        Err(OrgSelectError::NoOrgs) => {
            println!("No organizations found.");
            std::process::exit(0);
        }
        _ => {
            eprintln!("Error fetching organizations.");
            std::process::exit(1);
        }
    };

    let ans = Confirm::new(format!("Are you sure you want to delete {}?", selected.name).as_str())
        .with_default(false)
        .prompt()
        .expect("Prompt is configured correctly");

    if ans == false {
        println!("Deletion cancelled.");
        std::process::exit(0);
    }

    match actix_web_starter_client::apis::orgs_api::delete_org(
        &config,
        actix_web_starter_client::apis::orgs_api::DeleteOrgParams {
            org_id: selected.id.clone(),
            organization: selected.id.to_string(),
        },
    )
    .await
    .map_err(|e| {
        eprintln!("Error deleting organization: {:?}", e);
    })
    .unwrap()
    .status
    .is_success()
    {
        true => {
            println!("Organization deleted successfully.");
            std::process::exit(0);
        }
        false => {
            eprintln!("Error deleting organization.");
            std::process::exit(1);
        }
    }
}

pub async fn rename_org(settings: ActixTemplateConfiguration) {
    // Fetch the list of orgs
    let config = Configuration {
        base_path: settings.api_url.clone(),
        api_key: Some(actix_web_starter_client::apis::configuration::ApiKey {
            prefix: None,
            key: settings.api_key.clone(),
        }),
        ..Default::default()
    };

    let selected = match select_from_my_orgs(&config, "Select an organization to rename:").await {
        Ok(ans) => ans,
        Err(OrgSelectError::NoOrgs) => {
            println!("No organizations found.");
            std::process::exit(0);
        }
        _ => {
            eprintln!("Error fetching organizations.");
            std::process::exit(1);
        }
    };

    // Prompt for new name
    let new_name = inquire::Text::new("Enter the new name for the organization:")
        .prompt()
        .expect("Prompt configured correctly");

    // Send the rename request
    let rename_payload = actix_web_starter_client::models::UpdateOrgReqPayload { name: new_name };

    let renamed = actix_web_starter_client::apis::orgs_api::update_org(
        &config,
        actix_web_starter_client::apis::orgs_api::UpdateOrgParams {
            organization: selected.id.clone(),
            org_id: selected.id,
            update_org_req_payload: rename_payload,
        },
    )
    .await
    .map_err(|e| {
        eprintln!("Error renaming organization: {:?}", e);
    })
    .unwrap()
    .entity;

    if renamed.is_none() {
        println!("Organization renamed successfully.");
        std::process::exit(0);
    }

    match renamed.unwrap() {
        actix_web_starter_client::apis::orgs_api::UpdateOrgSuccess::Status200(org) => {
            println!("Organization renamed successfully.");
            println!("Name: {}", org.name);
        }
        _ => {
            eprintln!("Error renaming organization.");
            std::process::exit(1);
        }
    };
}

pub async fn invite_user(
    org_id: Option<String>,
    email: Option<String>,
    settings: ActixTemplateConfiguration,
) {
    let org_id = if org_id.is_none() {
        let config = Configuration {
            base_path: settings.api_url.clone(),
            api_key: Some(actix_web_starter_client::apis::configuration::ApiKey {
                prefix: None,
                key: settings.api_key.clone(),
            }),
            ..Default::default()
        };

        match select_from_my_orgs(&config, "Select an organization to invite the user to:").await {
            Ok(ans) => ans.id,
            Err(OrgSelectError::NoOrgs) => {
                println!("No organizations found.");
                std::process::exit(0);
            }
            _ => {
                eprintln!("Error fetching organizations.");
                std::process::exit(1);
            }
        }
    } else {
        org_id.unwrap()
    };
    let email = if email.is_none() {
        inquire::Text::new("Enter the email address of the user to invite:")
            .prompt()
            .expect("Prompt configured correctly")
    } else {
        email.unwrap()
    };

    let config = Configuration {
        base_path: settings.api_url.clone(),
        api_key: Some(actix_web_starter_client::apis::configuration::ApiKey {
            prefix: None,
            key: settings.api_key.clone(),
        }),
        ..Default::default()
    };

    let invitation = invitation_api::post_invitation(
        &config,
        invitation_api::PostInvitationParams {
            organization: org_id.to_string(),
            invitation_data: InvitationData {
                user_role: 0,
                organization_id: org_id,
                email: email,
                app_url: "http://localhost:8090/api".to_owned(),
                redirect_uri: "http://localhost:8090/api/auth/whoami".to_owned(),
            },
        },
    )
    .await
    .map_err(|e| {
        eprintln!("Error renaming organization: {:?}", e);
    })
    .unwrap()
    .entity;

    match invitation {
        Some(invitation_api::PostInvitationSuccess::Status204()) => {
            println!("Invitation sent successfully.");
        }
        _ => {
            eprintln!("Error sending invitation.");
            std::process::exit(1);
        }
    }
}

pub async fn leave_org(org_id: Option<String>, settings: ActixTemplateConfiguration) {
    let org_id = if org_id.is_none() {
        let config = Configuration {
            base_path: settings.api_url.clone(),
            api_key: Some(actix_web_starter_client::apis::configuration::ApiKey {
                prefix: None,
                key: settings.api_key.clone(),
            }),
            ..Default::default()
        };

        match select_from_my_orgs(&config, "Select an organization to leave:").await {
            Ok(ans) => ans.id,
            Err(OrgSelectError::NoOrgs) => {
                println!("No organizations found.");
                std::process::exit(0);
            }
            _ => {
                eprintln!("Error fetching organizations.");
                std::process::exit(1);
            }
        }
    } else {
        org_id.unwrap()
    };

    let config = Configuration {
        base_path: settings.api_url.clone(),
        api_key: Some(actix_web_starter_client::apis::configuration::ApiKey {
            prefix: None,
            key: settings.api_key.clone(),
        }),
        ..Default::default()
    };

    let left = actix_web_starter_client::apis::orgs_api::leave_org(
        &config,
        actix_web_starter_client::apis::orgs_api::LeaveOrgParams {
            org_id: org_id.clone(),
            organization: org_id.to_string(),
        },
    )
    .await
    .map_err(|e| {
        eprintln!("Error leaving organization: {:?}", e);
    })
    .unwrap()
    .status
    .is_success();

    if left {
        println!("Left organization successfully.");
        std::process::exit(0);
    } else {
        eprintln!("Error leaving organization.");
        std::process::exit(1);
    }
}
