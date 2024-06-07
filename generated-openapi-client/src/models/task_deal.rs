/*
 * Trieve API
 *
 * Trieve OpenAPI Specification. This document describes all of the operations available through the Trieve API.
 *
 * The version of the OpenAPI document: 0.0.1
 * Contact: developers@trieve.ai
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskDeal {
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "deal_id")]
    pub deal_id: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "task_id")]
    pub task_id: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

impl TaskDeal {
    pub fn new(created_at: String, deal_id: String, id: String, task_id: String, updated_at: String) -> TaskDeal {
        TaskDeal {
            created_at,
            deal_id,
            id,
            task_id,
            updated_at,
        }
    }
}
