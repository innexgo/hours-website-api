use serde::{Serialize, Deserialize};
use warp::Filter;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContactRequest {
    pub subject: String,
    pub body: String,
}

pub fn contact_request() -> impl Filter<Extract = (ContactRequest,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub async fn send_contact_email(req: ContactRequest) -> Result<impl warp::Reply, warp::Rejection> {
    println!("Subject: {}", req.subject);
    println!("Body: {}", req.body);

    Ok(warp::reply::reply())
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReferRequest {
    pub ref_code: String,
}

pub fn refer_request() -> impl Filter<Extract = (ReferRequest,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub async fn send_refer_email(req: ReferRequest) -> Result<impl warp::Reply, warp::Rejection> {
    println!("Refer code: {}", req.ref_code);

    Ok(warp::reply::reply())
}
