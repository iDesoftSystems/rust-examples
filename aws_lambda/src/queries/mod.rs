use eon::{
    api::response::ApiError,
    orm::Paginate,
    types::pagination::{Paged, Pagination},
};
use sea_orm::{ConnectionTrait, EntityTrait, QueryOrder};

use crate::{http::om::CustomerPage, schemas};

pub async fn find_all_customers_paginated(
    client: &impl ConnectionTrait,
    pagination: &Pagination,
) -> Result<Paged<CustomerPage>, ApiError> {
    let paged = schemas::customer::Entity::find()
        .order_by_desc(schemas::customer::Column::CreatedAt)
        .into_model::<CustomerPage>()
        .paginate(client, pagination)
        .await?;

    Ok(paged)
}
