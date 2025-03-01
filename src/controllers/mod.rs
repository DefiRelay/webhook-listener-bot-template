


pub mod webhook_handler_controller;
 


pub mod app_state; 

use actix_web::web::ServiceConfig;

pub trait WebController {
    fn config(cfg: &mut ServiceConfig);
}




