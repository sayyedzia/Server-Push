#[derive(thiserror::Error, Debug)]
pub enum RecvStatusError {
    #[error("DBError: {}", _0)]
    DBError(#[from] db::models::offline_cash_queue::DBError),
}

#[derive(serde::Deserialize, Debug)]
pub struct RecvStatusRequest {
    version: i64,
    did: String,
}

pub async fn handle_req(req: RecvStatusRequest) -> Result<(), RecvStatusError> {
    tracing::info!("RecvChainedCashStatus request received");

    let db_res = match db::models::offline_cash_queue::get_recv_status_queued_offline_cash(
        req.version,
        req.did.clone(),
    )? {
        Some(res) => res,
        None => vec![],
    };

    tracing::info!("Queued Offline Cash from Database: {:#?}", db_res);

    if db_res.is_empty() {
        tracing::info!("No rows to update");
    } else {
        tracing::info!("Update {:#?} in DB", req);
        let db_res = db::models::offline_cash_queue::set_recv_status(req.version, req.did)?;
        tracing::info!("Database Response {:#?}", db_res);
    }

    Ok(())
}
