use axum::{Json, extract::State};

use crate::{
    commands::{CommandHandler, create::CreateCustomerCommand},
    err::ApiError,
    om::CreateCustomerParams,
    responses::Created,
    state::AppContext,
};

pub async fn create_customer(
    State(ctx): State<AppContext>,
    Json(payload): Json<CreateCustomerParams>,
) -> Result<Created<i32>, ApiError> {
    let saved_id = CreateCustomerCommand {
        name: payload.name,
        business_name: payload.business_name,
        fiscal_number: payload.fiscal_number,
        is_natural_person: payload.is_natural_person,
    }
    .handle(&ctx)
    .await?;

    Ok(Created::new(saved_id))
}
