
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::sync::Arc;

use actix_web::HttpResponse;
use actix_web::Responder;
 

use degen_sql::sql_builder::OrderingDirection;
use degen_sql::sql_builder::SqlBuilder;
use degen_sql::sql_builder::SqlStatementBase;
use ethers::types::U256;
use serde::{Deserialize, Serialize};

use actix_web::web::{self, Data, Json, ServiceConfig};
 
use tokio_postgres::types::ToSql;


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

     

 

      
      return HttpResponse::InternalServerError().json("Database connection failed");
}

 

 