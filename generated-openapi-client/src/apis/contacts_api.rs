/*
 * Trieve API
 *
 * Trieve OpenAPI Specification. This document describes all of the operations available through the Trieve API.
 *
 * The version of the OpenAPI document: 0.0.1
 * Contact: developers@trieve.ai
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::{apis::ResponseContent, models};
use super::{Error, configuration};

/// struct for passing parameters to the method [`create_contact`]
#[derive(Clone, Debug)]
pub struct CreateContactParams {
    /// The org id to use for the request
    pub organization: String,
    /// JSON request payload to create a new contact
    pub create_contact_req_payload: models::CreateContactReqPayload
}

/// struct for passing parameters to the method [`delete_contact`]
#[derive(Clone, Debug)]
pub struct DeleteContactParams {
    /// The contacts id to use for the request
    pub contact_id: String,
    /// The org id to use for the request
    pub organization: String
}

/// struct for passing parameters to the method [`get_contact`]
#[derive(Clone, Debug)]
pub struct GetContactParams {
    /// The contacts id to use for the request
    pub contact_id: String,
    /// The org id to use for the request
    pub organization: String
}

/// struct for passing parameters to the method [`update_contact`]
#[derive(Clone, Debug)]
pub struct UpdateContactParams {
    /// The contact id to use for the request
    pub contact_id: String,
    /// The org id to use for the request
    pub organization: String,
    /// JSON request payload to update the contact
    pub update_contact_req_payload: models::UpdateContactReqPayload
}


/// struct for typed successes of method [`create_contact`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateContactSuccess {
    Status201(models::Contact),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`delete_contact`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteContactSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_contact`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetContactSuccess {
    Status200(models::Contact),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`update_contact`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateContactSuccess {
    Status200(models::Contact),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_contact`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateContactError {
    Status401(models::ErrorRespPayload),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_contact`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteContactError {
    Status401(models::ErrorRespPayload),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_contact`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetContactError {
    Status401(models::ErrorRespPayload),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_contact`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateContactError {
    Status401(models::ErrorRespPayload),
    UnknownValue(serde_json::Value),
}


pub async fn create_contact(configuration: &configuration::Configuration, params: CreateContactParams) -> Result<ResponseContent<CreateContactSuccess>, Error<CreateContactError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let organization = params.organization;
    let create_contact_req_payload = params.create_contact_req_payload;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/contacts", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("Organization", organization.to_string());
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&create_contact_req_payload);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<CreateContactSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<CreateContactError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn delete_contact(configuration: &configuration::Configuration, params: DeleteContactParams) -> Result<ResponseContent<DeleteContactSuccess>, Error<DeleteContactError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let contact_id = params.contact_id;
    let organization = params.organization;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/contacts/{contact_id}", local_var_configuration.base_path, contact_id=crate::apis::urlencode(contact_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("Organization", organization.to_string());
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<DeleteContactSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<DeleteContactError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_contact(configuration: &configuration::Configuration, params: GetContactParams) -> Result<ResponseContent<GetContactSuccess>, Error<GetContactError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let contact_id = params.contact_id;
    let organization = params.organization;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/contacts/{contact_id}", local_var_configuration.base_path, contact_id=crate::apis::urlencode(contact_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("Organization", organization.to_string());
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<GetContactSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<GetContactError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn update_contact(configuration: &configuration::Configuration, params: UpdateContactParams) -> Result<ResponseContent<UpdateContactSuccess>, Error<UpdateContactError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let contact_id = params.contact_id;
    let organization = params.organization;
    let update_contact_req_payload = params.update_contact_req_payload;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/contacts/{contact_id}", local_var_configuration.base_path, contact_id=crate::apis::urlencode(contact_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("Organization", organization.to_string());
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&update_contact_req_payload);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<UpdateContactSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<UpdateContactError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

