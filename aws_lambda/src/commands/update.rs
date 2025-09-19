use eon::{command::ExecutableCommand, http::ApiError};
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, IntoActiveModel};
use validator::Validate;

use crate::queries;

#[derive(Validate)]
pub struct UpdateCustomerCommand {
    pub product_id: i32,

    #[validate(length(min = 1, max = 100))]
    pub name: String,

    #[validate(length(min = 1, max = 100))]
    pub business_name: String,

    #[validate(length(min = 1, max = 100))]
    pub fiscal_number: String,
    pub is_natural_person: bool,
}

impl ExecutableCommand<DatabaseConnection> for UpdateCustomerCommand {
    type Output = Result<(), ApiError>;

    async fn execute(self, client: &DatabaseConnection) -> Self::Output {
        let customer_model = queries::find_customer_by_id(client, self.product_id)
            .await?
            .ok_or(ApiError::EntityIdNotFound(self.product_id as i64))?;

        let mut customer_am = customer_model.into_active_model();
        customer_am.name = ActiveValue::Set(self.name);
        customer_am.business_name = ActiveValue::Set(self.business_name);
        customer_am.fiscal_number = ActiveValue::Set(self.fiscal_number);
        customer_am.is_natural_person = ActiveValue::Set(self.is_natural_person.into());

        customer_am.update(client).await?;

        Ok(())
    }
}
