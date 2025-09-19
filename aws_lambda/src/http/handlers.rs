use axum::{
    Json,
    extract::{Path, Query, State},
    response::NoContent,
};
use eon::{
    command::ExecutableCommand,
    http::{ApiError, Created},
    pagination::{Paged, Pagination},
};

use crate::{
    commands::{CreateCustomerCommand, UpdateCustomerCommand},
    http::om::{CreateCustomerParams, CustomerPage, UpdateCustomerParams},
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
    .execute(&ctx.conn)
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

pub async fn update_customer(
    State(ctx): State<AppContext>,
    Path(product_id): Path<i32>,
    Json(payload): Json<UpdateCustomerParams>,
) -> Result<NoContent, ApiError> {
    UpdateCustomerCommand {
        product_id,
        name: payload.name,
        business_name: payload.business_name,
        fiscal_number: payload.fiscal_number,
        is_natural_person: payload.is_natural_person,
    }
    .execute(&ctx.conn)
    .await?;

    Ok(NoContent)
}
