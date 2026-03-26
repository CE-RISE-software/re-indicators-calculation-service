#[tokio::main]
async fn main() {
    let config = re_indicators_calculation_service::config::RuntimeConfig::from_env();
    let app = re_indicators_calculation_service::app();
    let listener = tokio::net::TcpListener::bind(config.socket_addr())
        .await
        .expect("bind listener");

    axum::serve(listener, app).await.expect("serve application");
}
