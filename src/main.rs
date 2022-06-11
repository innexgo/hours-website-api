use std::net::Ipv4Addr;

use actix_web::http::StatusCode;
use actix_web::{error, App, HttpResponse, HttpServer};
use clap::Parser;
use serde::{Deserialize, Serialize};

mod handlers;
use derive_more::Display;
use handlers::send_contact_email;

#[derive(Clone, Debug, Serialize, Deserialize, Display)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AppError {
    DecodeError,
    InternalServerError,
    Unauthorized,
    BadRequest,
    NotFound,
    Network,
    Unknown,
}

impl error::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(Err::<(), _>(self))
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            AppError::DecodeError => StatusCode::BAD_GATEWAY,
            AppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::BadRequest => StatusCode::BAD_REQUEST,
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::Network => StatusCode::BAD_GATEWAY,
            AppError::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[derive(Parser, Debug, Clone)]
#[clap(about, version, author)]
struct Opts {
    #[clap(short, long)]
    port: u16,
    #[clap(short, long)]
    mail_service_url: String,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let Opts {
        port,
        mail_service_url,
    } = Opts::parse();

    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(mail_service_url.clone()))
            .service(send_contact_email)
    })
    .bind((Ipv4Addr::LOCALHOST, port))?
    .run()
    .await
}
