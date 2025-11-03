#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;
mod m20220101_000001_users;

mod m20251030_163927_oct_confs;
mod m20251031_155226_door_confs;
mod m20251101_033703_user_doors;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20251030_163927_oct_confs::Migration),
            Box::new(m20251031_155226_door_confs::Migration),
            Box::new(m20251101_033703_user_doors::Migration),
            // inject-above (do not remove this comment)
        ]
    }
}
