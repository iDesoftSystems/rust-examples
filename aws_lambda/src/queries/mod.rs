use eon::pagination::{Paged, Paginate, Pagination};
use sea_orm::{ConnectionTrait, DbErr, EntityTrait, QueryOrder};

use crate::{http::om::CustomerPage, schemas};

pub async fn find_all_customers_paginated(
    client: &impl ConnectionTrait,
    pagination: &Pagination,
) -> Result<Paged<CustomerPage>, DbErr> {
    let paged = schemas::customer::Entity::find()
        .order_by_desc(schemas::customer::Column::CreatedAt)
        .into_model::<CustomerPage>()
        .paginate(client, pagination)
        .await?;

    Ok(paged)
}
