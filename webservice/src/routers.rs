use super::handlers::*;
use actix_web::web;
use actix_web::web::ServiceConfig;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

pub fn course_routes(cfg: &mut web::ServiceConfig) {
    cfg
        //.service(web::scope("/courses"))
        .route("/courses/", web::post().to(new_course));
}