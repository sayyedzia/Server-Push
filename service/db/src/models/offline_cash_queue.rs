use chrono::prelude::*;
use diesel::prelude::*;

#[derive(thiserror::Error, Debug)]
pub enum DBError {
    #[error("DieselError: {:?}", _0)]
    Diesel(#[from] diesel::result::Error),
    #[error("PooledConnectionError: cannot get the connection from r2d2 pool")]
    PooledConnection,
}

pub fn insert_into(
    from_did: String,
    to_did: String,
    certificates: &serde_json::Value,
    transactions: &serde_json::Value,
    tx_hash: &str,
    signed_tx_hash: &str,
    status: bool,
) -> Result<i64, DBError> {
    use crate::schema::offline_cash_queue;
    let mut conn = crate::pg::get_conn().ok_or(DBError::PooledConnection)?;
    Ok(
        diesel::insert_into(offline_cash_queue::dsl::offline_cash_queue)
            .values((
                offline_cash_queue::dsl::from_did.eq(from_did),
                offline_cash_queue::dsl::to_did.eq(to_did),
                offline_cash_queue::dsl::certificates.eq(certificates),
                offline_cash_queue::dsl::transactions.eq(transactions),
                offline_cash_queue::dsl::tx_hash.eq(tx_hash),
                offline_cash_queue::dsl::signed_tx_hash.eq(signed_tx_hash),
                offline_cash_queue::dsl::status.eq(status),
                offline_cash_queue::dsl::created_on.eq(Utc::now()),
                offline_cash_queue::dsl::updated_on.eq(Utc::now()),
            ))
            .returning(offline_cash_queue::dsl::id)
            .get_result::<i64>(&mut conn)?,
    )
}

pub fn get_queued_offline_cash(
    version: i64,
    did: String,
) -> Result<Option<Vec<(i64, serde_json::Value, serde_json::Value)>>, DBError> {
    use crate::schema::offline_cash_queue::dsl::*;
    let mut conn = crate::pg::get_conn().ok_or(DBError::PooledConnection)?;
    let result = offline_cash_queue
        .filter(id.gt(version))
        .filter(status.eq(false))
        .filter(to_did.eq(did))
        .select((id, certificates, transactions))
        .load(&mut conn)
        .optional()?;
    Ok(result)
}

pub fn get_recv_status_queued_offline_cash(
    version: i64,
    did: String,
) -> Result<Option<Vec<i64>>, DBError> {
    use crate::schema::offline_cash_queue::dsl::*;
    let mut conn = crate::pg::get_conn().ok_or(DBError::PooledConnection)?;
    let result = offline_cash_queue
        .filter(id.le(version))
        .filter(to_did.eq(did))
        .select(id)
        .load(&mut conn)
        .optional()?;
    Ok(result)
}

pub fn set_recv_status(version: i64, did: String) -> Result<i64, DBError> {
    use crate::schema::offline_cash_queue::dsl::*;
    let mut conn = crate::pg::get_conn().ok_or(DBError::PooledConnection)?;
    let result = diesel::update(
        offline_cash_queue
            .filter(id.le(version))
            .filter(to_did.eq(did)),
    )
    .set(status.eq(true))
    .returning(id)
    .get_result::<i64>(&mut conn)?;

    Ok(result)
}
