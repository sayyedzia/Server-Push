async fn from_body<T: serde::de::DeserializeOwned>(
    b: hyper::Body,
) -> Result<T, service::errors::RouteError> {
    let b = hyper::body::to_bytes(b)
        .await
        .map_err(|e| service::errors::RouteError::ReadBody(format!("{e}")))?;
    Ok(serde_json::from_slice(b.as_ref())?)
}

fn success(
    data: impl serde::Serialize,
) -> Result<hyper::Response<hyper::Body>, service::errors::RouteError> {
    #[derive(serde::Serialize)]
    struct ApiSuccess<T: serde::Serialize> {
        data: T,
        success: bool,
    }

    let resp = serde_json::to_vec(&ApiSuccess {
        data,
        success: true,
    })?;

    let mut response = hyper::Response::new(hyper::Body::from(resp));
    *response.status_mut() = hyper::StatusCode::OK;
    Ok(response)
}

#[tracing::instrument(skip_all)]
pub fn error(
    message: String,
    status: hyper::StatusCode,
) -> Result<hyper::Response<hyper::Body>, service::errors::RouteError> {
    #[derive(serde::Serialize)]
    struct ApiError {
        message: String,
        success: bool,
    }
    tracing::error!(message);
    let resp = serde_json::to_vec(&ApiError {
        success: false,
        message,
    })?;
    let mut response = hyper::Response::new(hyper::Body::from(resp));
    *response.status_mut() = status;
    Ok(response)
}

pub fn error_without_result(
    message: String,
    status: hyper::StatusCode,
) -> hyper::Response<hyper::Body> {
    #[derive(serde::Serialize)]
    struct ApiError {
        message: String,
        success: bool,
    }
    tracing::error!(message);
    let resp = serde_json::to_vec(&ApiError {
        success: false,
        message,
    })
    .unwrap();
    let mut response = hyper::Response::new(hyper::Body::from(resp));
    *response.status_mut() = status;
    response
}

#[tracing::instrument(skip_all)]
pub async fn handler(
    req: hyper::Request<hyper::Body>,
) -> Result<hyper::Response<hyper::Body>, service::errors::RouteError> {
    let (p, b) = req.into_parts();
    let start = std::time::Instant::now();
    tracing::info!("req: method: {}, path: {}", &p.method, p.uri.path());
    match (&p.method, p.uri.path()) {
        (&hyper::Method::GET, "/health/") => success(service::apis::health::health().await),
        (&hyper::Method::POST, "/v1/api/send/cash/offline/") => {
            match service::apis::send_offline_cash::handle_req(from_body(b).await?).await {
                Ok(send_offline_cash) => success(send_offline_cash),
                Err(e) => {
                    let msg = match e {
                        service::apis::send_offline_cash::SendChainedCashError::DBError(_) => {
                            String::from("Failed to fetch data from Database")
                        }
                    };
                    error(msg, hyper::StatusCode::BAD_REQUEST)
                }
            }
        }
        (&hyper::Method::POST, "/v1/api/recv/cash/offline/") => {
            match service::apis::recv_offline_cash::handle_req(from_body(b).await?).await {
                Ok(recv_offline_cash) => {
                    tracing::info!(
                        "SUCCESS: method: {}, path: {}, time: {:?}",
                        &p.method,
                        p.uri.path(),
                        start.elapsed()
                    );
                    success(recv_offline_cash)
                }
                Err(_e) => {
                    let msg = String::new();
                    error(msg, hyper::StatusCode::BAD_REQUEST)
                }
            }
        }
        (&hyper::Method::POST, "/v1/api/recv/status/") => {
            match service::apis::set_recv_status::handle_req(from_body(b).await?).await {
                Ok(set_recv_status) => success(set_recv_status),
                Err(e) => {
                    let msg = match e {
                        service::apis::set_recv_status::RecvStatusError::DBError(_) => {
                            String::from("Failed to fetch data from Database")
                        }
                    };
                    error(msg, hyper::StatusCode::BAD_REQUEST)
                }
            }
        }

        _ => error(
            format!("NOT_FOUND method: {}, path: {}", p.method, p.uri.path()),
            hyper::StatusCode::NOT_FOUND,
        ),
    }
}
