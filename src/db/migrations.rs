use sqlx::postgres::PgPool;
use sqlx::Error;

pub async fn run_migrations_if_needed(pool: &PgPool) -> Result<(), Error> {
    // Vérifier si la table de migration existe
    let table_exists: (bool,) = sqlx::query_as(
        "SELECT EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name = '_sqlx_migrations')"
    )
    .fetch_one(pool)
    .await?;

    if !table_exists.0 {
        println!("Running migrations...");
        sqlx::migrate!().run(pool).await?;
    } else {
        println!("Migrations already applied, skipping...");
    }

    Ok(())
}
