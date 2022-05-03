use warp::Filter;

#[tokio::main]
async fn main() {
    let hello = warp::any().map(|| format!("OK"));
    warp::serve(hello)
        .tls()
        .cert_path("localhost.pem")
        .key_path("localhost-key.pem")
        .run(([127, 0, 0, 1], 3030))
        .await;
}
