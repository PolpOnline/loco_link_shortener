use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // User table
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .col(
                        ColumnDef::new(Users::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Users::Pid).uuid().not_null().unique_key())
                    .col(
                        ColumnDef::new(Users::Email)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Users::Name).string().not_null())
                    .col(ColumnDef::new(Users::EmailVerifiedAt).timestamp())
                    .col(ColumnDef::new(Users::Password).string().not_null())
                    .index(Index::create().unique().name("idx_email").col(Users::Email))
                    .to_owned(),
            )
            .await?;

        // Links table
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
                    .col(ColumnDef::new(Links::Name).string().not_null())
                    .col(ColumnDef::new(Links::Image).string())
                    .col(ColumnDef::new(Links::Original).string().not_null())
                    .col(
                        ColumnDef::new(Links::Shortened)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(Links::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .col(ColumnDef::new(Links::UserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-links-user-id")
                            .from(Links::Table, Links::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
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

        // Clicks table
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
                    .col(ColumnDef::new(Clicks::UserAgent).string())
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
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Links::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Clicks::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Links {
    Table,
    Id,
    Name,
    Image,
    Original,
    Shortened,
    CreatedAt,
    UserId,
}

#[derive(DeriveIden)]
pub enum Clicks {
    Table,
    Id,
    LinkId,
    ClickedAt,
    Address,
    UserAgent,
}

#[derive(DeriveIden)]
pub enum Users {
    Table,
    Id,
    Pid,
    Email,
    Name,
    EmailVerifiedAt,
    Password,
}
