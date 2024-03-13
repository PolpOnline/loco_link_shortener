use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Links::Table)
                    .col(
                        ColumnDef::new(Links::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Links::Original)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(Links::Shortened)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(Links::Clicks)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Links::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp))
                    )
                    .index(
                        Index::create()
                            .unique()
                            .name("idx_shortened")
                            .col(Links::Shortened),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Clicks::Table)
                    .col(
                        ColumnDef::new(Clicks::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Clicks::LinkId).integer().not_null())
                    .col(ColumnDef::new(Clicks::ClickedAt).timestamp().not_null())
                    .col(ColumnDef::new(Clicks::Address).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-clicks-link-id")
                            .from(Clicks::Table, Clicks::LinkId)
                            .to(Links::Table, Links::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Links::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Links {
    Table,
    Id,
    Original,
    Shortened,
    Clicks,
    CreatedAt,
}

#[derive(DeriveIden)]
pub enum Clicks {
    Table,
    Id,
    LinkId,
    ClickedAt,
    Address,
}
