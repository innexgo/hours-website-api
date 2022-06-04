use std::convert::Infallible;

use super::handlers;
use mail_service_api::client::MailService;
use serde::{Deserialize, Serialize};
use warp::Filter;

fn with<T: Clone + Send>(t: T) -> impl Filter<Extract = (T,), Error = Infallible> + Clone {
    warp::any().map(move || t.clone())
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContactRequest {
    pub name: String,
    pub email: String,
    pub school: String,
    pub position: String,
    pub message: String,
}

pub fn contact_request() -> impl Filter<Extract = (ContactRequest,), Error = warp::Rejection> + Clone
{
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReferRequest {
    pub ref_code: String,
}

pub fn refer_request() -> impl Filter<Extract = (ReferRequest,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub fn api(
    mail_service: MailService,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let contact_endpoint = warp::path!("api" / "misc" / "contact")
        .and(warp::post())
        .and(contact_request())
        .and(with(mail_service.clone()))
        .and_then(handlers::send_contact_email);

    let refer_endpoint = warp::path!("api" / "misc" / "ref")
        .and(warp::post())
        .and(refer_request())
        .and(with(mail_service.clone()))
        .and_then(handlers::send_refer_email);

    contact_endpoint.or(refer_endpoint)
}
