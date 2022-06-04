use clap::Parser;
use mail_service_api::client::MailService;

mod api;
mod handlers;

#[derive(Parser, Debug, Clone)]
#[clap(about, version, author)]
struct Opts {
    #[clap(short, long)]
    port: u16,
    #[clap(short, long)]
    mail_service_url: String,
}

#[tokio::main]
async fn main() {
    let Opts {
        port,
        mail_service_url,
    } = Opts::parse();

    let mail_service = MailService::new(&mail_service_url).await;

    let api = api::api();

    warp::serve(api)
        .run(([127, 0, 0, 1], port))
        .await;
}
