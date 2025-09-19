use crate::schemas;
use eon::{command::ExecutableCommand, http::ApiError};
use sea_orm::{
    ActiveModelTrait, ActiveValue, DatabaseConnection, TryIntoModel, sqlx::types::chrono::Utc,
};
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

impl ExecutableCommand<DatabaseConnection> for CreateCustomerCommand {
    type Output = Result<i32, ApiError>;

    async fn execute(self, client: &DatabaseConnection) -> Self::Output {
        self.validate()?;

        let customer_am = schemas::customer::ActiveModel {
            id: ActiveValue::NotSet,
            name: ActiveValue::Set(self.name),
            business_name: ActiveValue::Set(self.business_name),
            fiscal_number: ActiveValue::Set(self.fiscal_number),
            is_natural_person: ActiveValue::Set(self.is_natural_person.into()),
            created_at: ActiveValue::Set(Utc::now().naive_utc()),
        }
        .save(client)
        .await?
        .try_into_model()?;

        Ok(customer_am.id)
    }
}
