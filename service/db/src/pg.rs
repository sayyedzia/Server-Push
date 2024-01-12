fn get_connection_pool() -> diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>
{
    let db_url: String = std::env::var("DATABASE_URL").expect("DATABASE_URL env var not found");
    let connection_manager = diesel::r2d2::ConnectionManager::<diesel::PgConnection>::new(db_url);

    diesel::r2d2::Pool::builder()
        .max_size(10)
        .idle_timeout(Some(std::time::Duration::from_secs(600)))
        .connection_timeout(std::time::Duration::from_secs(30))
        .build(connection_manager)
        .expect("Error in building the connection pool for postgres")
}

static DB_POOL: once_cell::sync::Lazy<
    diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>,
> = once_cell::sync::Lazy::new(|| get_connection_pool());

pub fn get_conn(
) -> Option<diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>> {
    match DB_POOL.get() {
        Ok(p) => Some(p),
        Err(e) => {
            tracing::error!("DB_ERROR: Not able to get the connection from pool: {e}");
            None
        }
    }
}

// type Pool = r2d2::Pool<r2d2_postgres::PostgresConnectionManager<r2d2_postgres::postgres::NoTls>>;
// static DB_POOL: once_cell::sync::Lazy<Pool> = once_cell::sync::Lazy::new(|| {
//     let db_url: String = std::env::var("DATABASE_URL").expect("DATABASE_URL env var not found");
//     let connection_manager = r2d2_postgres::PostgresConnectionManager::new(
//         db_url.parse().expect("Error in parsing the DATABASE_URL"),
//         r2d2_postgres::postgres::NoTls,
//     );
//     Pool::builder()
//         .max_size(10)
//         .idle_timeout(Some(std::time::Duration::from_secs(600)))
//         .connection_timeout(std::time::Duration::from_secs(30))
//         .build(connection_manager)
//         .expect("Error in building the connection pool for postgres")
// });

// pub async fn get_conn() -> Option<
//     r2d2::PooledConnection<
//         r2d2_postgres::PostgresConnectionManager<r2d2_postgres::postgres::NoTls>,
//     >,
// > {
//     match tokio::task::spawn_blocking(move || {
//         DB_POOL.get().expect("Error in getting the pool from cell")
//     })
//     .await
//     {
//         Ok(c) => Some(c),
//         Err(e) => {
//             eprint!("{e}");
//             None
//         }
//     }
// }
//
// pub async fn test_query() {
//     let mut conn = get_conn().await.unwrap();
//     match conn.query("SELECT 1", &[]) {
//         Ok(_) => {
//             println!("db connection setup successful");
//         }
//         Err(e) => {
//             println!("Error Occurred while querying for test connection: {e}");
//         }
//     }
// }
//
//
