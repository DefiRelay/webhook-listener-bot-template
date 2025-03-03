

use actix_web::HttpResponse;
use actix_web::Responder;
 

use defi_relay_webhook_listener_bot::types::defi_relay_webhook_payload::DefiRelayWebhookPayload;
use defi_relay_webhook_listener_bot::types::defi_relay_webhook_payload::PaymentSummary;
use log::warn;
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
                    "/handle",
                    web::post().to(handle_webhook),
                ),
        );
    }
}
 
/*
 #[derive(Serialize,Deserialize)]
pub struct HandleWebhookInput {

    payload: DefiRelayWebhookPayload, 

}*/

// Route Handler
async fn handle_webhook(
     input: Json<DefiRelayWebhookPayload>,
    app_state: Data<AppState>,
) -> impl Responder {
    println!("got webhook !" );



    let input_payload = &input ; 


    let mut  parsed_payment_summary:Option<PaymentSummary> = None ; 

    if let Some(event_data) = &input_payload.event_data {
        if let Some(event_type) = &input_payload.event_type  {
            match   event_type.as_str()  {

                 "payment_summary" => { 

                    parsed_payment_summary = serde_json::from_value( event_data.0 .clone() ).ok();

                } ,

                _ => {


                    warn! (   "unknown payload event type  "   );
                }


            } 

        }
    }

    println!( "parsed_payment_summary {:?}", parsed_payment_summary );
    
    // Return a success response for testing connectivity
    return HttpResponse::Ok().json("Webhook received successfully");
}

 

 