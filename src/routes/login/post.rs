use actix_web::http::header::ContentType;
use actix_web::HttpResponse;

pub async fn login() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .finish()
}
