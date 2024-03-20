use sqlx::postgres::PgPool;
use sqlx::migrate::Migrator;

pub async fn run_migrations(pool: &PgPool, migration_dir: &str) -> Result<(), sqlx::Error> {
    let migrator = Migrator::new(std::path::Path::new(migration_dir)).await?;
    migrator.run(pool).await?;
    Ok(())
}
