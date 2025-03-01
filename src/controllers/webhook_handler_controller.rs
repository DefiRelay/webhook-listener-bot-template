
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::sync::Arc;

use actix_web::HttpResponse;
use actix_web::Responder;
use chrono::Duration;
use chrono::Local;
use chrono::NaiveDate;
use chrono::NaiveDateTime;
 

use degen_sql::sql_builder::OrderingDirection;
use degen_sql::sql_builder::SqlBuilder;
use degen_sql::sql_builder::SqlStatementBase;
use ethers::types::U256;
use serde::{Deserialize, Serialize};

use actix_web::web::{self, Data, Json, ServiceConfig};
use teller_pools_bot_rs::db::postgres::models::teller_bids_model::SubmittedBid;
use teller_pools_bot_rs::db::postgres::models::teller_loans_model::TellerLoansModel;
use tokio_postgres::types::ToSql;


use super::app_state::AppState;
use super::WebController;


/*

curl -X POST http://localhost:8080/api/loan/get_bid \
     -H "Content-Type: application/json" \
     -d '{ }'




*/

pub struct LoanController {}

impl LoanController {}

impl WebController for LoanController {
    fn config(cfg: &mut ServiceConfig) {
        cfg.service(
            web::scope("/api/loan")
                // Add your routes here, e.g.,
                .route(
                    "/get_bid",
                    web::post().to(get_bid),
                ),
        );
    }
}

/*

  pub chain_id: u64,

    pub bid_id: U256,

    pub borrower: Address,
    pub lender: Address,
    pub market_id: U256,
    pub principal_token_address: Address,
    pub principal_amount: U256,

    pub bid_state: Option<TellerLoanBidState>, 



*/

#[derive(Serialize, Deserialize, Debug)]
pub struct GetBidInput {
    pub market_id: Option<u64>,
    pub chain_id: Option<u64>, 
    pub borrower: Option<String>, // address .. 
    pub principal_token_address: Option<String>,
    pub principal_amount: Option<U256>,
    pub bid_state: Option<String>,
} 
#[derive(Serialize, Deserialize, Debug)]
pub struct GetLoanOutput {
    
} 



// Route Handler
async fn get_bid(
      input: Json<GetBidInput>,
    app_state: Data<AppState>,
) -> impl Responder {
    //let current_timestamp = Local::now().naive_local();

    println!("get loan 1" );

    


          /*  let mut query = String::from("SELECT * FROM teller_bids");
            let mut conditions = Vec::new();
            


            let mut params_map  : HashMap<String, Box<dyn ToSql + Sync>> = HashMap::new();
            
            if let Some(market_id) = &input.market_id {
                params_map.insert("market_id".into() , Box::new(*market_id as i64 )) ;
            }
            if let Some(chain_id) = &input.chain_id {
                params_map.insert("chain_id".into() , Box::new(*chain_id as i64 )) ;
            }
            if let Some(borrower) = &input.borrower {
                params_map.insert("borrower".into() , Box::new( borrower.clone()  )) ;
            }
            if let Some(principal_token_address) = &input.principal_token_address {
                params_map.insert("principal_token_address".into() , Box::new( principal_token_address.clone() )) ;
            }
            if let Some(bid_state) = &input.bid_state {
                params_map.insert("bid_state".into() , Box::new( bid_state.clone() )) ;
            }


            let mut params: Vec<Box<dyn ToSql + Sync>> = Vec::new();

            for (key, param) in params_map.drain()  { 
                 params.push( param );
                 conditions.push(format!("{} = ${}", key, params.len() )); 
            }

           
                    // will add too many AND s ? 
            if !conditions.is_empty() {
                query.push_str(" WHERE ");
                query.push_str(&conditions.join(" AND "));
            }

            query.push_str(" ORDER BY created_at DESC LIMIT 1;");*/


             let mut params_map  : BTreeMap<String, Arc<dyn ToSql + Sync>> = BTreeMap::new();

             if let Some(market_id) = &input.market_id {
                params_map.insert("market_id".into() , Arc::new(*market_id as i64 )) ;
            }
            if let Some(chain_id) = &input.chain_id {
                params_map.insert("chain_id".into() , Arc::new(*chain_id as i64 )) ;
            }
            if let Some(borrower) = &input.borrower {
                params_map.insert("borrower".into() , Arc::new( borrower.clone()  )) ;
            }
            if let Some(principal_token_address) = &input.principal_token_address {
                params_map.insert("principal_token_address".into() , Arc::new( principal_token_address.clone() )) ;
            }
            if let Some(bid_state) = &input.bid_state {
                params_map.insert("bid_state".into() , Arc::new( bid_state.clone() )) ;
            }
  

            let sql_builder = SqlBuilder {
                statement_base:  SqlStatementBase::SelectAll,
                table_name: "teller_bids".into(),
                where_params: params_map,
                order:  Some( ( "created_at".into() ,  OrderingDirection::DESC ) ),
                limit: Some( 1 ) ,
            };

            let (query,params) = sql_builder.build() ;



            println!("{}", query);

            println!("{:?}", params);
 
        let mut  psql_db = app_state.database.lock().await ; 

        let row_result = psql_db
            .query_one_with_reconnect( &query,  &params.iter().map(|x| &**x).collect::<Vec<_>>()   )
            .await;

        drop(psql_db) ;


        println!( "{:?}" , row_result );

 



      match row_result {


            Ok(row) => {
 
                    let bid = SubmittedBid::from_row( &row ) ; 

                     HttpResponse::Ok().json( bid  )
            }

            Err(_err) => {


                return HttpResponse::InternalServerError().json("Database connection failed");
            }

        }


      

}

 

 