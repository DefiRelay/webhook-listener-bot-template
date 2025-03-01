use crate::types::domains::bytes32::DomainBytes32;
use crate::types::domains::h256::DomainH256; 
use chrono::DateTime;
use chrono::Utc;
use crate::types::domains::pay_to_amounts::DomainPayToAmounts;
use crate::types::domains::eth_address::DomainEthAddress;
use crate::types::domains::uint256::DomainUint256;
use crate::types::domains::pay_to_array::DomainPayToArray;
use tokio_postgres::types::ToSql;
use ethers::{
    types::{Address, H256},
    utils::to_checksum,
};
use log::info;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;

use vibegraph::event::{self, ContractEvent};

use degen_sql::db::postgres::models::model::PostgresModelError;
use degen_sql::db::postgres::postgres_db::Database;

use ethers::types::{U256, U64};
use std::str::FromStr;

pub struct PaymentsModel {}

impl PaymentsModel {
    pub async fn insert_one(
     
         loan_summary: PaymentSummary, 

        psql_db: &Database,
    ) -> Result<i32, PostgresModelError> {
       // let nonce_decimal = Decimal::from_str(&loan_summary.nonce.to_string()).unwrap();
      //  let block_number_decimal = Decimal::from_str(&loan_summary. block_number.to_string()).unwrap();

        let status = "paid".to_string()  ;

        let insert_result = psql_db
            .query_one(
                "
                INSERT INTO payments 
                (
                contract_address,
                token_address,
                pay_to_array,
                pay_to_amounts,
                uuid,
                nonce,
                block_number,
                status,
                transaction_hash,
                chain_id
                ) 
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
                RETURNING id;
                ",
                &[
                    &loan_summary.payspec_contract_address,
                    &loan_summary.payment_token_address,
                    &loan_summary.pay_to_array,
                    &loan_summary.pay_to_amounts,
                    &loan_summary.uuid,
                    &loan_summary.nonce,
                    &loan_summary.payment_at_block,
                    &status,
                    &loan_summary.transaction_hash,
                    &loan_summary.chain_id,
                ],
            )
            .await;

        match insert_result {
            Ok(row) => Ok(row.get(0)), // Successfully inserted new row and got its ID.
            Err(e) => {
                eprintln!("Database error: Payment {:?}", e);
                Err(PostgresModelError::Postgres(e))
            }
        }
    }


       pub async fn insert_or_update_one(
        /*contract_address: &str,
        token_address: &str,
        pay_to_array: &str,
        pay_to_amounts: &str,
        uuid: &str,
        nonce: u64,
        block_number: u64,
        status: &str,
        transaction_hash: &str,
        chain_id: i64, */

        loan_summary: PaymentSummary, 
        psql_db: &Database,
    ) -> Result<i32, PostgresModelError> {
      //  let nonce_decimal = Decimal::from_str(&loan_summary.nonce.to_string()).unwrap();
     //   let block_number_decimal = Decimal::from_str(&loan_summary.block_number.to_string()).unwrap();


        let status = "paid".to_string()  ;

        // Using ON CONFLICT to handle the unique constraint (transaction_hash, chain_id)
        let upsert_result = psql_db
            .query_one(
                "
                INSERT INTO payments 
                (
                contract_address,
                token_address,
                pay_to_array,
                pay_to_amounts,
                uuid,
                nonce,
                block_number,
                status,
                transaction_hash,
                chain_id
                ) 
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
                ON CONFLICT (transaction_hash, chain_id) 
                DO UPDATE SET 
                    contract_address = EXCLUDED.contract_address,
                    token_address = EXCLUDED.token_address,
                    pay_to_array = EXCLUDED.pay_to_array,
                    pay_to_amounts = EXCLUDED.pay_to_amounts,
                    uuid = EXCLUDED.uuid,
                    nonce = EXCLUDED.nonce,
                    block_number = EXCLUDED.block_number,
                    status = EXCLUDED.status
                RETURNING id;
                ",
                &[
                    &loan_summary.payspec_contract_address,
                    &loan_summary.payment_token_address,
                    &loan_summary.pay_to_array,
                    &loan_summary.pay_to_amounts,
                    &loan_summary.uuid,
                    &loan_summary.nonce,
                    &loan_summary.payment_at_block,
                    &status,
                    &loan_summary.transaction_hash,
                    &loan_summary.chain_id,
                ],
            )
            .await;

        match upsert_result {
            Ok(row) => {
                let id: i32 = row.get(0);
                info!("Payment upserted with id: {}", id);
                Ok(id)
            },
            Err(e) => {
                eprintln!("Database error during upsert: Payment {:?}", e);
                Err(PostgresModelError::Postgres(e))
            }
        }
    }

    pub async fn find_by_transaction_hash(
        transaction_hash: &str,
        chain_id: i64,
        psql_db: &Database,
    ) -> Result<tokio_postgres::Row, PostgresModelError> {
        let row = psql_db
            .query_one(
                "
                SELECT * FROM payments
                WHERE transaction_hash = $1 AND chain_id = $2
                LIMIT 1;
                ",
                &[&transaction_hash, &chain_id],
            )
            .await;

        match row {
            Ok(row) => Ok(row),
            Err(e) => {
                eprintln!("Database error: Payment lookup {:?}", e);
                Err(PostgresModelError::Postgres(e))
            }
        }
    }

    pub async fn find_by_uuid(
        uuid: &str,
        psql_db: &Database,
    ) -> Result<tokio_postgres::Row, PostgresModelError> {
        let row = psql_db
            .query_one(
                "
                SELECT * FROM payments
                WHERE uuid = $1
                LIMIT 1;
                ",
                &[&uuid],
            )
            .await;

        match row {
            Ok(row) => Ok(row),
            Err(e) => {
                eprintln!("Database error: Payment lookup {:?}", e);
                Err(PostgresModelError::Postgres(e))
            }
        }
    }

    pub async fn update_status(
        id: i32,
        status: &str,
        psql_db: &Database,
    ) -> Result<(), PostgresModelError> {
        let result = psql_db
            .execute(
                "
                UPDATE payments
                SET status = $1
                WHERE id = $2;
                ",
                &[&status, &id],
            )
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("Database error: Payment status update {:?}", e);
                Err(PostgresModelError::Postgres(e))
            }
        }
    }

    pub async fn find_most_recent_payment(
        contract_address: &str,
        psql_db: &Database,
    ) -> Result<tokio_postgres::Row, PostgresModelError> {
        let row = psql_db
            .query_one(
                "
                SELECT *
                FROM payments
                WHERE contract_address = $1
                ORDER BY created_at DESC
                LIMIT 1;
                ",
                &[&contract_address],
            )
            .await;

        match row {
            Ok(row) => Ok(row),
            Err(e) => {
                eprintln!("Database error: Recent Payment {:?}", e);
                Err(PostgresModelError::Postgres(e))
            }
        }
    }

    pub async fn find_payments_by_status(
        status: &str,
        limit: i64,
        offset: i64,
        psql_db: &Database,
    ) -> Result<Vec<tokio_postgres::Row>, PostgresModelError> {
        let rows = psql_db
            .query(
                "
                SELECT *
                FROM payments
                WHERE status = $1
                ORDER BY created_at ASC
                LIMIT $2 OFFSET $3;
                ",
                &[&status, &limit, &offset],
            )
            .await;

        match rows {
            Ok(rows) => Ok(rows),
            Err(e) => {
                eprintln!("Database error: Payments by status {:?}", e);
                Err(PostgresModelError::Postgres(e))
            }
        }
    }
}



/*

#[derive(Clone,Debug)]
pub struct PaymentSummary {

    uuid: String,

    chain_id: i64,


    payment_token_address: DomainEthAddress,


    totalAmount: DomainUint256, 
    recipients: DomainPayToArray,
    amounts: DomainPayToAmounts,


    payment_at_block: Option< U64 >,
    payment_at_block_timestamp: Option< DateTime<Utc> > , 
    payment_at_unix_days_index:  Option< i64 >   

    
}
*/


#[derive(Clone,Debug)]
pub struct PaymentSummary {

    pub uuid: DomainBytes32,

    pub chain_id: i64,

    pub payspec_contract_address: DomainEthAddress, 

    pub payment_token_address: DomainEthAddress,


    pub nonce: DomainUint256, 


    pub totalAmount: DomainUint256, 
    pub pay_to_array: DomainPayToArray,
    pub pay_to_amounts: DomainPayToAmounts,


    pub transaction_hash: DomainH256, 


    pub payment_at_block: Option< i64  >,
    pub payment_at_block_timestamp: Option< DateTime<Utc> > , 
    pub payment_at_unix_days_index:  Option< i64 >   


}










