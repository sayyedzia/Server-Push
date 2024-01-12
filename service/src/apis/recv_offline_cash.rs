#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("DBError: {}", _0)]
    DBError(#[from] db::models::offline_cash_queue::DBError),
}

#[derive(serde::Deserialize, Debug)]
pub struct Request {
    #[serde(rename = "version")]
    version: i64,
    #[serde(rename = "did")]
    did: String,
}

#[derive(serde::Serialize, Debug)]
pub struct Response {
    #[serde(rename = "version")]
    version: i64,
    // todo: fix it later, merge all the certificates into single array
    // and send only unique certificates
    #[serde(rename = "certificates")]
    certificates: Vec<serde_json::Value>,

    #[serde(rename = "transactions")]
    transactions: Vec<serde_json::Value>,
}

pub async fn handle_req(req: Request) -> Result<Response, Error> {
    tracing::info!("Receive Offline Cash request received: {:?}", req);

    let db_res =
        match db::models::offline_cash_queue::get_queued_offline_cash(req.version, req.did)? {
            Some(res) => res,
            None => vec![],
        };

    // tracing::info!("Queued Offline Cash from Database: {:#?}", db_res);

    let mut max_num_row = 0;
    let mut transactions = vec![];
    let mut certificates = vec![];
    for row in db_res.into_iter() {
        if max_num_row < row.0 {
            max_num_row = row.0;
        }
        if let serde_json::Value::Array(v) = row.1 {
            certificates.extend(v);
        }
        if let serde_json::Value::Array(v) = row.2 {
            transactions.extend(v);
        }
    }

    let rr = Response {
        version: max_num_row,
        certificates: certificates,
        transactions: transactions,
    };

    println!("response: {:?}", rr);

    Ok(rr)
}
