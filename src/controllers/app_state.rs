
use std::sync::Arc;

use tokio::sync::Mutex;

use degen_sql::db::postgres::postgres_db::Database; 

 
pub struct AppState {
  
      pub database: Arc<Mutex<Database>>,
}