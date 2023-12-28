use sea_orm::{Database, DatabaseConnection, DbErr};
use sea_orm_migration::MigrationTrait;

use std::env;

mod entities;
mod migrator;
mod crud;

use migrator::Migrator;

pub use crud::mutation::Mutation;
pub use crud::query::Query;
pub use entities::post;

pub async fn database_connection() -> Result<DatabaseConnection, DbErr> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let db = Database::connect(db_url).await?;
    Ok(db)
}

pub async fn migration(db: &DatabaseConnection) -> Result<(), DbErr> {
    Migrator::up(db, None).await?;
    Ok(())
}