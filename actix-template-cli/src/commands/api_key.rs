use actix_web_starter_client::{
    apis::{api_key_api::CreateApiKeyParams, configuration::Configuration},
    models::{CreateApiKeyReqPayload, CreateApiKeyRespPayload},
};

use super::configure::ActixTemplateConfiguration;
use crate::ApiKeyData;

pub async fn generate_api_key(
    settings: ActixTemplateConfiguration,
    api_key_data: ApiKeyData,
) -> Result<(), Box<dyn std::error::Error>> {
    let name = if api_key_data.name.is_none() {
        inquire::Text::new("Enter a name for the API Key:")
            .with_help_message("This name will help you identify the API Key in the future.")
            .prompt()
            .unwrap()
    } else {
        api_key_data.name.unwrap()
    };

    let config = Configuration {
        base_path: settings.api_url.clone(),
        api_key: Some(actix_web_starter_client::apis::configuration::ApiKey {
            prefix: None,
            key: settings.api_key.clone(),
        }),
        ..Default::default()
    };

    let payload = CreateApiKeyReqPayload::new(name.clone());
    let data = CreateApiKeyParams {
        create_api_key_req_payload: payload,
    };

    let user = actix_web_starter_client::apis::api_key_api::create_api_key(&config, data)
        .await
        .map_err(|e| {
            eprintln!("Error generating API Key: {:?}", e);
            std::process::exit(1);
        })
        .unwrap()
        .entity
        .unwrap();

    match user {
        actix_web_starter_client::apis::api_key_api::CreateApiKeySuccess::Status200(api_key) => {
            println!("\nAPI Key generated successfully!\n");
            println!("Name: {}", name);
            println!("API Key: {}", api_key.api_key);
        }
        actix_web_starter_client::apis::api_key_api::CreateApiKeySuccess::UnknownValue(_) => {
            eprintln!("Error generating API Key.");
            std::process::exit(1);
        }
    }

    Ok(())
}
