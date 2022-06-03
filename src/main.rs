mod api;
mod handlers;

#[tokio::main]
async fn main() {
    let api = api::api();

    warp::serve(api)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
