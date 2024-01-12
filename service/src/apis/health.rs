#[derive(serde::Serialize)]
pub struct Health {
    env: String,
}

pub async fn health() -> Health {
    let env = service::utils::env()
        .unwrap_or_else(|| format!("ENV env variable not found in system env"));
    Health { env }
}
