use axum::{
    Json,
    extract::{Query, State},
};
use eon::{
    api::response::{ApiError, Created},
    types::pagination::{Paged, Pagination},
};

use crate::{
    commands::{CommandHandler, create::CreateCustomerCommand},
    http::om::{CreateCustomerParams, CustomerPage},
    queries,
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

pub async fn read_customers(
    State(ctx): State<AppContext>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<Paged<CustomerPage>>, ApiError> {
    let paginated = queries::find_all_customers_paginated(&ctx.conn, &pagination).await?;

    Ok(Json(paginated))
}
