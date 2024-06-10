use super::configure::ActixTemplateConfiguration;
use crate::{errors::DefaultError, ui::get_cancelable_render_config};
use actix_web_starter_client::{
    apis::deals_api::{self, CreateDealParams, CreateDealSuccess, GetDealParams, UpdateDealParams},
    models::{CreateDealReqPayload, Deal, UpdateDealReqPayload},
};
use clap::{Args, Subcommand};

#[derive(Subcommand)]
pub enum DealCommands {
    Create,
    Delete(DeleteDeal),
    Edit(EditDeal),
    View(ViewDeal),
}

#[derive(Args)]
pub struct DeleteDeal {
    /// The id of the deal you want to delete
    pub id: Option<String>,
}

#[derive(Args)]
pub struct EditDeal {
    /// The id of the deal you want to edit
    pub id: String,
}

#[derive(Args)]
pub struct ViewDeal {
    /// The id of the deal you want to delete
    pub id: String,
}

pub async fn create_deal_cmd(config: ActixTemplateConfiguration) -> Result<Deal, DefaultError> {
    let name = inquire::Text::new("Enter deal name:")
        .with_render_config(get_cancelable_render_config("No Description"))
        .prompt()?;

    let active = inquire::Confirm::new("Is this deal active?")
        .with_default(true)
        .prompt()?;

    let size = inquire::CustomType::<f32>::new("Enter deal size:")
        .with_default(0.0)
        .prompt()?;

    let result = deals_api::create_deal(
        &config.clone().into(),
        CreateDealParams {
            organization: config.org_id,
            create_deal_req_payload: CreateDealReqPayload {
                active: Some(Some(active)),
                name: Some(Some(name)),
                size: Some(Some(size)),
            },
        },
    )
    .await?
    .entity
    .ok_or_else(|| DefaultError::new("No entity returned from API for create_deal"))?;

    match result {
        CreateDealSuccess::Status201(deal) => {
            println!("Deal created successfully!");
            Ok(deal)
        }
        CreateDealSuccess::UnknownValue(_) => Err(DefaultError::new(
            "Unknown response from API for create_deal",
        )),
    }
}

pub async fn get_deal(
    config: ActixTemplateConfiguration,
    deal_id: String,
) -> Result<Deal, DefaultError> {
    let response = deals_api::get_deal(
        &config.clone().into(),
        GetDealParams {
            organization: config.org_id,
            deal_id,
        },
    )
    .await?
    .entity
    .ok_or_else(|| DefaultError::new("No entity returned from API for get_deal"))?;

    match response {
        deals_api::GetDealSuccess::Status200(deal) => Ok(deal),
        deals_api::GetDealSuccess::UnknownValue(_) => {
            Err(DefaultError::new("Unknown response from API for get_deal"))
        }
    }
}

pub async fn edit_deal_cmd(
    config: ActixTemplateConfiguration,
    deal_id: String,
) -> Result<Deal, DefaultError> {
    let deal = get_deal(config.clone(), deal_id.clone()).await?;
    let prev_deal_name = deal
        .name
        .unwrap_or(Some("".to_string()))
        .unwrap_or("".to_string());
    let prev_deal_size = deal.size.unwrap_or(Some(0.0)).unwrap_or(0.0);

    let name = inquire::Text::new("Enter deal name:")
        .with_default(&prev_deal_name)
        .prompt()
        .unwrap_or(prev_deal_name);

    let active = inquire::Confirm::new("Is this deal active?")
        .with_default(deal.active)
        .prompt()
        .unwrap_or(deal.active);

    let size = inquire::CustomType::<f32>::new("Enter deal size:")
        .with_default(prev_deal_size)
        .prompt()
        .unwrap_or(prev_deal_size);

    let result = deals_api::update_deal(
        &config.clone().into(),
        UpdateDealParams {
            organization: config.org_id,
            deal_id,
            update_deal_req_payload: UpdateDealReqPayload {
                active: Some(Some(active)),
                name: Some(Some(name)),
                size: Some(Some(size)),
            },
        },
    )
    .await
    .unwrap_or_else(|e| {
        eprintln!("Error updating deal: {:?}", e);
        std::process::exit(1);
    })
    .entity
    .ok_or_else(|| DefaultError::new("No entity returned from API for update_deal"))?;

    match result {
        deals_api::UpdateDealSuccess::Status200(deal) => {
            println!("Deal edited successfully!");
            Ok(deal)
        }
        deals_api::UpdateDealSuccess::UnknownValue(_) => Err(DefaultError::new(
            "Unknown response from API for update_deal",
        )),
    }
}

pub async fn view_deal_cmd(
    config: ActixTemplateConfiguration,
    deal_id: String,
) -> Result<Deal, DefaultError> {
    let deal = get_deal(config.clone(), deal_id.clone()).await?;
    let name = deal
        .name
        .clone()
        .unwrap_or(Some("".to_string()))
        .unwrap_or("".to_string());
    let size = deal.size.unwrap_or(Some(0.0)).unwrap_or(0.0);
    let active = deal.active;

    println!("Deal ID: {}", deal.id);
    println!("Name: {}", name);
    println!("Size: {}", size);
    println!("Active: {}", active);

    Ok(deal)
}

pub async fn delete_deal_cmd(
    config: ActixTemplateConfiguration,
    deal_id: Option<String>,
) -> Result<(), DefaultError> {
    let deal_id = if deal_id.is_none() {
        inquire::Text::new("Enter deal ID to delete:").prompt()?
    } else {
        deal_id.unwrap()
    };

    let delete_response = deals_api::delete_deal(
        &config.clone().into(),
        deals_api::DeleteDealParams {
            deal_id,
            organization: config.org_id,
        },
    )
    .await?
    .status
    .is_success();

    match delete_response {
        true => {
            println!("Deal deleted successfully");
            Ok(())
        }
        false => Err(DefaultError::new("Error deleting deal")),
    }
}
