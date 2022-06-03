use super::handlers;
use warp::Filter;

pub fn api() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("api" / "misc" / "ref")
        .and(warp::post())
        .and(handlers::mail_request())
        .and_then(handlers::send_mail)
}
