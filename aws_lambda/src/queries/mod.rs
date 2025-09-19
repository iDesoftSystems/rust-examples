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

pub async fn find_customer_by_id(
    client: &impl ConnectionTrait,
    customer_id: i32,
) -> Result<Option<schemas::customer::Model>, DbErr> {
    let customer_opt = schemas::customer::Entity::find_by_id(customer_id)
        .one(client)
        .await?;

    Ok(customer_opt)
}
