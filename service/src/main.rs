pub struct HttpService;

impl hyper::service::Service<hyper::Request<hyper::Body>> for HttpService {
    type Response = hyper::Response<hyper::Body>;
    type Error = hyper::Error;
    type Future = std::pin::Pin<
        Box<dyn futures::Future<Output = Result<Self::Response, Self::Error>> + Send>,
    >;

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: hyper::Request<hyper::Body>) -> Self::Future {
        Box::pin(async {
            match service::router::handler(req).await {
                Ok(r) => Ok(r),
                Err(e) => {
                    tracing::error!(message = format!("handler-error: {}", e));
                    Ok(service::router::error_without_result(
                        serde_json::to_string(&serde_json::json!({
                            "success": false,
                            "message": "INTERNAL_SERVER_ERROR"
                        }))
                        .unwrap(),
                        hyper::StatusCode::INTERNAL_SERVER_ERROR,
                    ))
                }
            }
        })
    }
}

async fn http_main() -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
    // Initialize the database connection setup
    let env = service::utils::env().expect("ENV variable expected");
    dotenv::from_filename(format!("{env}.env"))
        .expect(format!("{env}.env file not found").as_str());

    // if no port is provided, take 8000 as default
    let port: u16 = match service::utils::port() {
        Some(port) => match port.parse() {
            Ok(port) => port,
            Err(_) => 8000,
        },
        None => 8000,
    };

    let socket_address: std::net::SocketAddr = ([0, 0, 0, 0], port).into();
    let listener = tokio::net::TcpListener::bind(socket_address).await?;
    tracing::info!(
        "#### Started Server at: {}:{} ####",
        socket_address.ip(),
        socket_address.port()
    );

    loop {
        tokio::select! {
            // Handle incoming HTTP connections
            result = listener.accept() => {
                match result {
                    Ok((tcp_stream, _)) => {
                        tokio::task::spawn(async move {
                            if let Err(http_err) = hyper::server::conn::Http::new()
                                .serve_connection(tcp_stream, HttpService {})
                                .with_upgrades()
                                .await
                            {
                                eprintln!("Error while serving HTTP connection: {}", http_err);
                            }
                        });
                    }
                    Err(err) => {
                        eprintln!("Error accepting connection: {}", err);
                    }
                }
            }
        }
    }
}

async fn traced_main() {
    use tracing_subscriber::layer::SubscriberExt;
    let level = std::env::var("TRACING")
        .unwrap_or_else(|_| "info".to_owned())
        .parse::<tracing_forest::util::LevelFilter>()
        .unwrap_or(tracing_forest::util::LevelFilter::INFO);
    if service::utils::is_traced() {
        tracing_forest::worker_task()
            .set_global(true)
            .build_with(|_layer: tracing_forest::ForestLayer<_, _>| {
                tracing_subscriber::Registry::default()
                    .with(tracing_forest::ForestLayer::default())
                    .with(level)
            })
            .on(http_main())
            .await
            .expect("service error")
    } else {
        tracing_forest::worker_task()
            .set_global(true)
            .build_with(|_layer: tracing_forest::ForestLayer<_, _>| {
                tracing_subscriber::FmtSubscriber::default().with(level)
                // tracing_subscriber::Registry::default()
                //     .with()
                //     .with(level)
            })
            .on(http_main())
            .await
            .expect("service error")
    }
}

fn main() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(traced_main())
}
