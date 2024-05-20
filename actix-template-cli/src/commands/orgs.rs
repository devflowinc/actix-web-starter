use actix_web_starter_client::{
    apis::{
        configuration::Configuration,
        orgs_api::{CreateOrgParams, CreateOrgSuccess},
    },
    models::CreateOrgReqPayload,
};

use super::configure::ActixTemplateConfiguration;

pub async fn create_org(settings: ActixTemplateConfiguration, name: Option<String>) {
    let name = if name.is_none() {
        inquire::Text::new("Enter a name for the organization:")
            .prompt()
            .unwrap()
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
        eprintln!("Error generating API Key: {:?}", e);
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
