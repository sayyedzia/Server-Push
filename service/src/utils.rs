pub fn is_traced() -> bool {
    std::env::var("TRACING").is_ok() || std::env::args().any(|e| e == "--trace")
}
pub fn env() -> Option<String> {
    std::env::var("ENV").ok().map(|x| x.to_lowercase())
}

pub fn port() -> Option<String> {
    std::env::var("PORT").ok()
}
