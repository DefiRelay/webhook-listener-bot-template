use crate::app_state::AppState;
use crate::types::domains::json::DomainJson;
use actix_web::web::Data;
use chrono::{DateTime, Utc};
use degen_sql::db::postgres::{models::model::PostgresModelError, postgres_db::Database};
use serde::Serialize;
use tokio_postgres::Row;

use crate::types::selected_record::SelectedRecord;
use crate::util::built_from_row::BuiltFromDbRow;
//use super::webhook_urls_model::WebhookUrl;



/// Represents a webhook trigger record in the database.
#[derive(Serialize, Clone, Debug)]
pub struct WebhookTrigger {
    pub webhook_id: i32,
    pub status: String,
    pub event_type: Option<String>,
    pub event_data: Option<DomainJson>,
    pub attempts: i32,
    pub last_triggered_at: Option<DateTime<Utc>>,
    //pub created_at: DateTime<Utc>,
}

impl BuiltFromDbRow for WebhookTrigger {
    fn from_row(row: &Row) -> Option<Self> {
        Some(Self {
            webhook_id: row.get("webhook_id"),
            status: row.get("status"),
            event_type: row.get("event_type"),
            event_data: row.get("event_data"),
            attempts: row.get("attempts"),
            last_triggered_at: row.get("last_triggered_at"),
          //  created_at: row.get("created_at"),
        })
    }
}


impl WebhookTrigger {
    //  Creates a new `WebhookTrigger` instance.
    pub fn new(webhook_id: i32) -> Self {
        Self {
            webhook_id,
            status: "pending".to_string(),
            event_type: None,
            event_data: None,
            attempts: 0,
            last_triggered_at: None,
          //  created_at: chrono::Utc::now(),
        }
    }
}

// Struct that combines a webhook trigger with its associated webhook URL
/*#[derive(Serialize, Clone, Debug)]
pub struct WebhookTriggerJoined {
    pub webhook_url: WebhookUrl,
    pub webhook_trigger: WebhookTrigger,
    
}



impl BuiltFromDbRow for WebhookTriggerJoined {
    fn from_row(row: &Row) -> Option<Self> {
        
         
        // Return the joined object
        Some(Self {
            webhook_url : WebhookUrl::from_row(&row)?,
            webhook_trigger : WebhookTrigger::from_row(&row)?,
           
        })
    }
}
*/

pub struct WebhookTriggersModel {}

impl WebhookTriggersModel {
    /// Inserts a new webhook trigger into the database.
    pub async fn insert_one(
        webhook_trigger: WebhookTrigger,
        psql_db: &Database,
    ) -> Result<i32, PostgresModelError> {
        let insert_query = "INSERT INTO webhook_triggers (webhook_id, status, event_type, event_data, attempts, last_triggered_at)
                            VALUES ($1, $2, $3, $4, $5, $6)
                            RETURNING id;";
        let result = psql_db
            .query_one(
                insert_query,
                &[
                    &webhook_trigger.webhook_id,
                    &webhook_trigger.status,
                    &webhook_trigger.event_type,
                    &webhook_trigger.event_data,
                    &webhook_trigger.attempts,
                    &webhook_trigger.last_triggered_at,
                ],
            )
            .await;

        match result {
            Ok(row) => Ok(row.get::<_, i32>("id")),
            Err(e) => Err(e.into()),
        }
    }

    /// Updates the status and last_triggered_at of a webhook trigger
    pub async fn update_status(
        id: i32,
        status: String,
        psql_db: &Database,
    ) -> Result<bool, PostgresModelError> {
        let update_query = "UPDATE webhook_triggers 
                           SET status = $2, last_triggered_at = NOW(), attempts = attempts + 1
                           WHERE id = $1;";
        
        let result = psql_db.execute(update_query, &[&id, &status]).await;

        match result {
            Ok(rows_affected) => Ok(rows_affected > 0),
            Err(e) => Err(e.into()),
        }
    }

    /// Finds all pending webhook triggers
    pub async fn find_pending_triggers(
        psql_db: &Database,
    ) -> Result<Vec<SelectedRecord<WebhookTrigger>>, PostgresModelError> {
        let query = "SELECT * FROM webhook_triggers WHERE status = 'pending' ORDER BY created_at ASC;";
        let result = psql_db.query(query, &[]).await?;

        let triggers = result
            .iter()
            .filter_map(|row| SelectedRecord::<WebhookTrigger>::from_row(row))
            .collect();

        Ok(triggers)
    }
    
    //  Finds webhook triggers joined with their webhook URLs
    /*pub async fn find_webhook_triggers_joined(
        status: Option<String>,
        limit: i64,
        psql_db: &Database,
    ) -> Result<Vec<WebhookTriggerJoined>, PostgresModelError> {
        // Build the query based on whether status is provided
        let query = match status {
            Some(_) => "
                SELECT 
                    t.id as id,
                    t.webhook_id,
                    t.status,
                    t.event_type,
                    t.event_data,
                    t.attempts,
                    t.last_triggered_at,
                    t.created_at as trigger_created_at,
                    u.id as url_id,
                    u.owner_wallet_address,
                    u.webhook_url,
                    u.scopes,
                    u.created_at as url_created_at
                FROM webhook_triggers t
                JOIN webhook_urls u ON t.webhook_id = u.id
                WHERE t.status = $1
                ORDER BY t.created_at ASC
                LIMIT $2
            ",
            None => "
                SELECT 
                    t.id as id,
                    t.webhook_id,
                    t.status,
                    t.event_type,
                    t.event_data,
                    t.attempts,
                    t.last_triggered_at,
                    t.created_at as trigger_created_at,
                    u.id as url_id,
                    u.owner_wallet_address,
                    u.webhook_url,
                    u.scopes,
                    u.created_at as url_created_at
                FROM webhook_triggers t
                JOIN webhook_urls u ON t.webhook_id = u.id
                ORDER BY t.created_at ASC
                LIMIT $1
            ",
        };
        
        // Execute the query with appropriate parameters
        let rows = match status {
            Some(status_value) => psql_db.query(query, &[&status_value, &limit]).await?,
            None => psql_db.query(query, &[&limit]).await?,
        };
        
        // Use BuiltFromDbRow to convert rows to WebhookTriggerJoined objects
        let joined_records = rows
            .iter()
            .filter_map(|row| WebhookTriggerJoined::from_row(row))
            .collect();
        
        Ok(joined_records)
    }

    /// Finds all webhook triggers for a specific webhook
    pub async fn find_by_webhook_id(
        webhook_id: i32,
        psql_db: &Database,
    ) -> Result<Vec<SelectedRecord<WebhookTrigger>>, PostgresModelError> {
        let query = "SELECT * FROM webhook_triggers WHERE webhook_id = $1 ORDER BY created_at DESC;";
        let result = psql_db.query(query, &[&webhook_id]).await?;

        let triggers = result
            .iter()
            .filter_map(|row| SelectedRecord::<WebhookTrigger>::from_row(row))
            .collect();

        Ok(triggers)
    }
    
    /// Finds a specific webhook trigger joined with its webhook URL
    pub async fn find_webhook_trigger_joined_by_id(
        trigger_id: i32,
        psql_db: &Database,
    ) -> Result<Option<SelectedRecord<WebhookTriggerJoined>>, PostgresModelError> {
        let query = "
            SELECT 
                t.id as  id,
                t.webhook_id,
                t.status,
                t.last_triggered_at,
               
                u.id as url_id, 
                u.owner_wallet_address,
                u.webhook_url,
                u.scopes,
                u.created_at as created_at
            FROM webhook_triggers t
            JOIN webhook_urls u ON t.webhook_id = u.id
            WHERE t.id = $1
        ";
        
        let row_result = psql_db.query_one(query, &[&trigger_id]).await;
        
        match row_result {
            Ok(row) => {
                // Get trigger ID
            //   let trigger_id = row.get::<_, i32>("id");  //use for selected row 
           
                
                // Create a joined record
                let joined = SelectedRecord ::from_row(&row) ;
                
                Ok(joined)
            },
            Err(e) => {
                if e.to_string().contains("no rows") {
                    return Ok(None);
                }
                Err(e.into())
            }
        }
    }*/
}