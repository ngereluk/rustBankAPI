use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Account::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Account::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Account::AccountBalance).float().not_null())
                    .col(ColumnDef::new(Account::UpdatedAt).string().not_null())
                    .col(ColumnDef::new(Account::CreatedAt).string().not_null())
                    .col(ColumnDef::new(Account::Deleted).boolean().not_null())
                    .col(ColumnDef::new(Account::DeletedAt).string().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Account::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Account {
    Table,
    Id,
    AccountBalance,
    UpdatedAt,
    CreatedAt,
    Deleted,
    DeletedAt,
}
