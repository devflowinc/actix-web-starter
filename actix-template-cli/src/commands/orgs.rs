use super::configure::ActixTemplateConfiguration;
use crate::errors::DefaultError;
use actix_web_starter_client::{
    apis::{
        configuration::Configuration,
        invitation_api,
        orgs_api::{
            CreateOrgParams, CreateOrgSuccess, GetOrgsForAuthedUserParams,
            GetOrgsForAuthedUserSuccess,
        },
    },
    models::{CreateOrgReqPayload, InvitationData, Org},
};
use clap::{Args, Subcommand};
use inquire::{Confirm, Select};
use std::fmt::Display;

#[derive(Subcommand)]
pub enum OrgCommands {
    Create(CreateOrg),
    Delete,
    Rename,
    Invite(InviteToOrg),
    Leave(LeaveOrg),
}

#[derive(Args)]
pub struct CreateOrg {
    /// The name of the organization you want to create
    pub name: Option<String>,
}

#[derive(Args)]
pub struct LeaveOrg {
    /// The name of the organization you want to create
    pub id: Option<String>,
}

#[derive(Args)]
pub struct InviteToOrg {
    /// The user's email
    #[arg(short, long)]
    pub email: Option<String>,
}

pub async fn create_org(
    settings: ActixTemplateConfiguration,
    name: Option<String>,
) -> Result<Org, DefaultError> {
    let name = if name.is_none() {
        inquire::Text::new("Enter a name for the organization:").prompt()?
    } else {
        name.unwrap()
    };

    let payload = CreateOrgReqPayload { name };

    let created = actix_web_starter_client::apis::orgs_api::create_org(
        &settings.into(),
        CreateOrgParams {
            create_org_req_payload: payload,
        },
    )
    .await?
    .entity
    .unwrap();

    match created {
        CreateOrgSuccess::Status201(org) => {
            println!("\nOrganization created successfully!\n");
            println!("Name: {}", org.name);
            return Ok(org);
        }
        CreateOrgSuccess::UnknownValue(_) => {
            return Err(DefaultError::new(
                "Could not parse response body creating org",
            ));
        }
    };
}

// TODO: better error type
#[derive(Debug)]
pub enum OrgSelectError {
    NoOrgs,
    OrgFetchFailure,
    CancelInput,
}

#[derive(Debug)]
pub struct OrgSelectOption {
    org: Org,
}

impl Display for OrgSelectOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.org.name)
    }
}

pub async fn select_from_my_orgs(
    config: &Configuration,
    prompt: &str,
) -> Result<Org, OrgSelectError> {
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
            org: org_result.to_owned(),
        })
        .collect();

    let ans = Select::new(prompt, options)
        .prompt()
        .map_err(|_| OrgSelectError::CancelInput)?;

    Ok(ans.org)
}

pub async fn delete_org(settings: ActixTemplateConfiguration) -> Result<(), DefaultError> {
    // Fetch the list of orgs
    let selected = match select_from_my_orgs(
        &settings.clone().into(),
        "Select an organization to delete:",
    )
    .await
    {
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
        &settings.into(),
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
        false => Err(DefaultError::new("Error deleting organization.")),
    }
}

pub async fn rename_org(settings: ActixTemplateConfiguration) -> Result<(), DefaultError> {
    let selected = select_from_my_orgs(
        &settings.clone().into(),
        "Select an organization to rename:",
    )
    .await
    .map_err(|e| match e {
        OrgSelectError::NoOrgs => DefaultError::new("No organizations found."),
        _ => DefaultError::new("Error fetching organizations."),
    })?;

    // Prompt for new name
    let new_name = inquire::Text::new("Enter the new name for the organization:")
        .prompt()
        .map_err(|e| {
            DefaultError::new(format!("Error prompting for new name: {:?}", e).as_str())
        })?;

    // Send the rename request
    let rename_payload = actix_web_starter_client::models::UpdateOrgReqPayload { name: new_name };

    let renamed = actix_web_starter_client::apis::orgs_api::update_org(
        &settings.into(),
        actix_web_starter_client::apis::orgs_api::UpdateOrgParams {
            organization: selected.id.clone(),
            org_id: selected.id,
            update_org_req_payload: rename_payload,
        },
    )
    .await
    .map_err(|e| DefaultError::new(format!("Error renaming organization: {:?}", e).as_str()))?
    .entity;

    if renamed.is_none() {
        println!("Organization renamed successfully.");
        return Ok(());
    }

    match renamed.unwrap() {
        actix_web_starter_client::apis::orgs_api::UpdateOrgSuccess::Status200(org) => {
            println!("Organization renamed successfully.");
            println!("Name: {}", org.name);
            Ok(())
        }
        _ => Err(DefaultError::new("Error renaming organization.")),
    }
}

pub async fn invite_user(
    email: Option<String>,
    settings: ActixTemplateConfiguration,
) -> Result<(), DefaultError> {
    let email = if email.is_none() {
        inquire::Text::new("Enter the email address of the user to invite:")
            .prompt()
            .map_err(|e| {
                DefaultError::new(format!("Error prompting for email: {:?}", e).as_str())
            })?
    } else {
        email.unwrap()
    };

    let invitation = invitation_api::post_invitation(
        &settings.clone().into(),
        invitation_api::PostInvitationParams {
            organization: settings.clone().org_id.to_string(),
            invitation_data: InvitationData {
                user_role: 0,
                organization_id: settings.clone().org_id,
                email,
                app_url: "http://localhost:8090/api".to_owned(),
                redirect_uri: "http://localhost:8090/api/auth/whoami".to_owned(),
            },
        },
    )
    .await
    .map_err(|e| DefaultError::new(format!("Error sending invitation: {:?}", e).as_str()))?
    .entity;

    match invitation {
        Some(invitation_api::PostInvitationSuccess::Status204()) => {
            println!("Invitation sent successfully.");
            Ok(())
        }
        _ => Err(DefaultError::new("Error sending invitation.")),
    }
}

pub async fn leave_org(
    org_id: Option<String>,
    settings: ActixTemplateConfiguration,
) -> Result<(), DefaultError> {
    let org_id = if org_id.is_none() {
        select_from_my_orgs(&settings.clone().into(), "Select an organization to leave:")
            .await
            .map_err(|e| match e {
                OrgSelectError::NoOrgs => DefaultError::new("No organizations found."),
                _ => DefaultError::new("Error fetching organizations."),
            })?
            .id
    } else {
        org_id.unwrap()
    };

    let left = actix_web_starter_client::apis::orgs_api::leave_org(
        &settings.into(),
        actix_web_starter_client::apis::orgs_api::LeaveOrgParams {
            org_id: org_id.clone(),
            organization: org_id.to_string(),
        },
    )
    .await
    .map_err(|e| DefaultError::new(format!("Error leaving organization: {:?}", e).as_str()))
    .unwrap()
    .status
    .is_success();

    if left {
        println!("Left organization successfully.");
        Ok(())
    } else {
        Err(DefaultError::new("Error leaving organization."))
    }
}
