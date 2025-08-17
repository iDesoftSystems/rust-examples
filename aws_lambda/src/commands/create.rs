use crate::{commands::CommandHandler, schemas, state::AppContext};
use eon::api::response::ApiError;
use sea_orm::{ActiveModelTrait, ActiveValue, sqlx::types::chrono::Utc};
use validator::Validate;

#[derive(Validate)]
pub struct CreateCustomerCommand {
    #[validate(length(min = 1, max = 100))]
    pub name: String,

    #[validate(length(min = 1, max = 100))]
    pub business_name: String,

    #[validate(length(min = 1, max = 100))]
    pub fiscal_number: String,
    pub is_natural_person: bool,
}

impl CommandHandler for CreateCustomerCommand {
    type Output = i32;
    type Error = ApiError;

    async fn handle(self, ctx: &AppContext) -> Result<Self::Output, Self::Error> {
        self.validate()?;

        let customer_am = schemas::customer::ActiveModel {
            id: ActiveValue::NotSet,
            name: ActiveValue::Set(self.name),
            business_name: ActiveValue::Set(self.business_name),
            fiscal_number: ActiveValue::Set(self.fiscal_number),
            is_natural_person: ActiveValue::Set(self.is_natural_person.into()),
            created_at: ActiveValue::Set(Utc::now().naive_utc()),
        }
        .save(&ctx.conn)
        .await?;

        Ok(customer_am.id.unwrap())
    }
}
