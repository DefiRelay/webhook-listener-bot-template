


pub mod webhook_handler_controller;
 

 
use actix_web::web::ServiceConfig;

pub trait WebController {
    fn config(cfg: &mut ServiceConfig);
}




