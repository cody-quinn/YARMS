use actix_web::HttpResponse;

#[inline]
pub fn build_redirect(location: &str) -> HttpResponse {
    HttpResponse::Found()
        .append_header(("Location", location))
        .finish()
}
