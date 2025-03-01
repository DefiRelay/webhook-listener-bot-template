

use actix_web::HttpResponse;
use actix_web::Responder;
 

use serde::{Deserialize, Serialize};

use actix_web::web::{self, Data, Json, ServiceConfig};
 


use defi_relay_webhook_listener_bot::app_state::AppState;



use super::WebController;


/*

curl -X POST http://localhost:8080/api/webhook/ \
     -H "Content-Type: application/json" \
     -d '{ }'




*/

pub struct WebhookHandlerController {}

impl WebhookHandlerController {}

impl WebController for WebhookHandlerController {
    fn config(cfg: &mut ServiceConfig) {
        cfg.service(
            web::scope("/api/webhook")
                // Add your routes here, e.g.,
                .route(
                    "/",
                    web::post().to(handle_webhook),
                ),
        );
    }
}
 

 #[derive(Serialize,Deserialize)]
pub struct HandleWebhookInput {


}

// Route Handler
async fn handle_webhook(
     input: Json<HandleWebhookInput>,
    app_state: Data<AppState>,
) -> impl Responder {
    println!("got webhook !" );
    
    // Return a success response for testing connectivity
    return HttpResponse::Ok().json("Webhook received successfully");
}

 

 