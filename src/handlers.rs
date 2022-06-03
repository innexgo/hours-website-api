use serde::{Serialize, Deserialize};
use warp::Filter;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MailRequest {
    pub subject: String,
    pub body: String,
}

pub async fn send_mail(req: MailRequest) -> Result<impl warp::Reply, warp::Rejection> {
    println!("Subject: {}", req.subject);
    println!("Body: {}", req.body);

    Ok(warp::reply::reply())
}

pub fn mail_request() -> impl Filter<Extract = (MailRequest,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
