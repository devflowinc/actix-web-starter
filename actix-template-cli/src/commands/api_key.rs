use actix_web_starter_client::{
    apis::{api_key_api::CreateApiKeyParams, configuration::Configuration},
    models::CreateApiKeyReqPayload,
};

use super::configure::ActixTemplateConfiguration;
use crate::{errors::DefaultError, ApiKeyData};

pub async fn generate_api_key(
    settings: ActixTemplateConfiguration,
    api_key_data: ApiKeyData,
) -> Result<(), DefaultError> {
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
        .map_err(|_| DefaultError::new("Error generating API Key."))
        .unwrap()
        .entity
        .unwrap();

    match user {
        actix_web_starter_client::apis::api_key_api::CreateApiKeySuccess::Status201(api_key) => {
            println!("\nAPI Key generated successfully!\n");
            println!("Name: {}", name);
            println!("API Key: {}", api_key.api_key);
            return Ok(());
        }
        actix_web_starter_client::apis::api_key_api::CreateApiKeySuccess::UnknownValue(_) => {
            DefaultError::new("Unknown error generating API Key.")
        }
    };

    Ok(())
}
