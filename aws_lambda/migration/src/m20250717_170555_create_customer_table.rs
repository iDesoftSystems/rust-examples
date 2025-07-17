use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Customer::Table)
                    .if_not_exists()
                    .col(pk_auto(Customer::Id))
                    .col(string_len_uniq(Customer::Name, 100))
                    .col(string_len(Customer::BusinessName, 100))
                    .col(string_len(Customer::FiscalNumber, 100))
                    .col(boolean(Customer::IsNaturalPerson))
                    .col(date_time(Customer::CreatedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Customer::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Customer {
    Table,
    Id,
    /// Name by which the customer is known. For internal use.
    Name,
    /// Official name of the customer, for invoices and other documents
    BusinessName,
    FiscalNumber,
    IsNaturalPerson,
    CreatedAt,
}
