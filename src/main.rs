use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = re_indicators_calculation_service::app();
    let addr = SocketAddr::from(([127, 0, 0, 1], 8081));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("bind listener");

    axum::serve(listener, app).await.expect("serve application");
}
