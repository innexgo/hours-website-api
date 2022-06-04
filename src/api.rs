use super::handlers;
use warp::Filter;

pub fn api() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let contact_endpoint = warp::path!("api" / "misc" / "contact")
        .and(warp::post())
        .and(handlers::contact_request())
        .and_then(handlers::send_contact_email);

    let refer_endpoint = warp::path!("api" / "misc" / "ref")
        .and(warp::post())
        .and(handlers::refer_request())
        .and_then(handlers::send_refer_email);

    contact_endpoint
        .or(refer_endpoint)
}
