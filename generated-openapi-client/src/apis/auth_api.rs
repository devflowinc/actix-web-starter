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

/// struct for passing parameters to the method [`callback`]
#[derive(Clone, Debug)]
pub struct CallbackParams {
    pub state: String,
    pub session_state: String,
    pub code: String
}

/// struct for passing parameters to the method [`login`]
#[derive(Clone, Debug)]
pub struct LoginParams {
    /// URL to redirect to after successful login
    pub redirect_uri: Option<String>,
    /// Code sent via email as a result of successful call to send_invitation
    pub inv_code: Option<String>
}

/// struct for passing parameters to the method [`logout`]
#[derive(Clone, Debug)]
pub struct LogoutParams {
    pub redirect_uri: Option<String>
}


/// struct for typed successes of method [`callback`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CallbackSuccess {
    Status303(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`login`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LoginSuccess {
    Status303(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`logout`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LogoutSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`whoami`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WhoamiSuccess {
    Status200(models::User),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`callback`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CallbackError {
    Status400(models::ErrorRespPayload),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`login`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LoginError {
    Status400(models::ErrorRespPayload),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`logout`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LogoutError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`whoami`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WhoamiError {
    Status400(models::ErrorRespPayload),
    UnknownValue(serde_json::Value),
}


/// OpenID Connect callback  This is the callback route for the OAuth provider, it should not be called directly. Redirects to browser with set-cookie header.
pub async fn callback(configuration: &configuration::Configuration, params: CallbackParams) -> Result<ResponseContent<CallbackSuccess>, Error<CallbackError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let state = params.state;
    let session_state = params.session_state;
    let code = params.code;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/auth/callback", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("state", &state.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("session_state", &session_state.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("code", &code.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<CallbackSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<CallbackError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Login  This will redirect you to the OAuth provider for authentication with email/pass, SSO, Google, Github, etc.
pub async fn login(configuration: &configuration::Configuration, params: LoginParams) -> Result<ResponseContent<LoginSuccess>, Error<LoginError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let redirect_uri = params.redirect_uri;
    let inv_code = params.inv_code;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/auth", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = redirect_uri {
        local_var_req_builder = local_var_req_builder.query(&[("redirect_uri", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = inv_code {
        local_var_req_builder = local_var_req_builder.query(&[("inv_code", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<LoginSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<LoginError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Logout  Invalidate your current auth credential stored typically stored in a cookie. This does not invalidate your API key.
pub async fn logout(configuration: &configuration::Configuration, params: LogoutParams) -> Result<ResponseContent<LogoutSuccess>, Error<LogoutError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let redirect_uri = params.redirect_uri;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/auth", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = redirect_uri {
        local_var_req_builder = local_var_req_builder.query(&[("redirect_uri", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<LogoutSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<LogoutError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get Currently Auth'ed User  Get the currently auth'ed user. This will return the user object for the currently auth'ed user.
pub async fn whoami(configuration: &configuration::Configuration) -> Result<ResponseContent<WhoamiSuccess>, Error<WhoamiError>> {
    let local_var_configuration = configuration;

    // unbox the parameters


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/auth/whoami", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
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
        let local_var_entity: Option<WhoamiSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<WhoamiError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

