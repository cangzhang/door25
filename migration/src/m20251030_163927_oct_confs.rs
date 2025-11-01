use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(m, "oct_confs",
            &[
            
            ("id", ColType::PkAuto),
            
            ("token", ColType::TextNull),
            ("user_pid", ColType::TextNull),
            ("openid", ColType::TextNull),
            ],
            &[
            ]
        ).await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "oct_confs").await
    }
}
