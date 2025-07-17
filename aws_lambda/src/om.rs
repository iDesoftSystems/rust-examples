use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCustomerParams {
    pub name: String,
    pub business_name: String,
    pub fiscal_number: String,
    pub is_natural_person: bool,
}
