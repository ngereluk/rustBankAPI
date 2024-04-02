use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::FirstName).string().not_null())
                    .col(ColumnDef::new(User::LastName).string().not_null())
                    .col(ColumnDef::new(User::AddressLine1).string().null())
                    .col(ColumnDef::new(User::AddressLine2).string().null())
                    .col(ColumnDef::new(User::PostalCode).string().null())
                    .col(ColumnDef::new(User::City).string().null())
                    .col(ColumnDef::new(User::Province).string().null())
                    .col(ColumnDef::new(User::Dob).string().null())
                    .col(ColumnDef::new(User::CreatedAt).string().not_null())
                    .col(ColumnDef::new(User::UpdatedAt).string().not_null())
                    .col(ColumnDef::new(User::Deleted).string().not_null())
                    .col(ColumnDef::new(User::DeletedAt).string().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum User {
    Table,
    Id,
    FirstName,
    LastName,
    AddressLine1,
    AddressLine2,
    PostalCode,
    City,
    Province,
    Dob,
    CreatedAt,
    UpdatedAt,
    Deleted,
    DeletedAt
}
