use sea_orm::{FromQueryResult, prelude::DateTime};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCustomerParams {
    pub name: String,
    pub business_name: String,
    pub fiscal_number: String,
    pub is_natural_person: bool,
}

#[derive(Serialize, FromQueryResult)]
#[serde(rename_all = "camelCase")]
pub struct CustomerPage {
    pub id: i32,
    pub name: String,
    pub business_name: String,
    pub fiscal_number: String,
    pub is_natural_person: bool,
    pub created_at: DateTime,
}
