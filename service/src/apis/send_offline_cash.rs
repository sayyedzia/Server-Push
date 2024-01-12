#[derive(thiserror::Error, Debug)]
pub enum SendChainedCashError {
    #[error("DBError: {}", _0)]
    DBError(#[from] db::models::offline_cash_queue::DBError),
}

#[derive(serde::Deserialize, Debug)]
pub struct Request {
    #[serde(rename = "fromDid")]
    from_did: String,
    #[serde(rename = "toDid")]
    to_did: String,
    #[serde(rename = "certificates")]
    certificates: serde_json::Value,
    #[serde(rename = "transactions")]
    transactions: serde_json::Value,
    // hash: String,
    // sign: String,
}

pub async fn handle_req(req: Request) -> Result<(), SendChainedCashError> {
    tracing::info!("Send Offline Cash request received");
    tracing::info!("Save {:#?} in DB", req);
    let db_res = db::models::offline_cash_queue::insert_into(
        req.from_did,
        req.to_did,
        &req.certificates,
        &req.transactions,
        "",
        "",
        false,
    )?;
    tracing::info!("Database Response {:#?}", db_res);

    Ok(())
}
