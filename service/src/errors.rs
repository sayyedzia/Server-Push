#[derive(thiserror::Error, Debug)]
pub enum RouteError {
    #[error("BodyReadError: {0}")]
    ReadBody(String),
    #[error("JsonParseError: {0}")]
    JsonParse(#[from] serde_json::Error),
    #[error("SendOfflineCashError: {0}")]
    SendOfflineCashError(#[from] service::apis::send_offline_cash::SendChainedCashError),
    #[error("RecvOfflineCashError: {0}")]
    RecvOfflineCashError(#[from] service::apis::recv_offline_cash::Error),
    #[error("RecvStatusError: {0}")]
    RecvStatusError(#[from] service::apis::set_recv_status::RecvStatusError),
}
