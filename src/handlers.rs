use super::api;
use mail_service_api::client::MailService;

pub async fn send_contact_email(
    req: api::ContactRequest,
    mail_service: MailService,
) -> Result<impl warp::Reply, warp::Rejection> {
    let _ = mail_service
        .mail_new(mail_service_api::request::MailNewProps {
            request_id: 0,
            destination: "innexgo@gmail.com".to_owned(),
            topic: "Innexgo Sales: New Contact".to_owned(),
            title: "New contact from form:<br/>".to_owned(),
            content: format!(
                "name: <code>{}</code><br/>
                email: <code>{}</code><br/>
                school: <code>{}</code><br/>
                position: <code>{}</code><br/> +
                message: <code>{}</code><br/>",
                req.name, req.email, req.school, req.position, req.message
            ),
        })
        .await;

    Ok(warp::reply::reply())
}

pub async fn send_refer_email(
    req: api::ReferRequest,
    mail_service: MailService,
) -> Result<impl warp::Reply, warp::Rejection> {
    let _ = mail_service
        .mail_new(mail_service_api::request::MailNewProps {
            request_id: 0,
            destination: "innexgo@gmail.com".to_owned(),
            topic: "Innexgo Sales: New Refer".to_owned(),
            title: "New site visit using:<br/>".to_owned(),
            content: format!("refer code: <code>{}</code><br/>", req.ref_code),
        })
        .await;

    Ok(warp::reply::reply())
}
