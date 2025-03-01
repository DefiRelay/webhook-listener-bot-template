CREATE TABLE invoices (
    id SERIAL PRIMARY KEY,
    
    contract_address VARCHAR(255) NOT NULL, 
  
    token_address VARCHAR(255) NOT NULL, 
    pay_to_array TEXT NOT NULL,
    pay_to_amounts TEXT NOT NULL,  
    
    nonce NUMERIC(32)  NOT NULL ,
    
    uuid VARCHAR(255) NOT NULL, 

    
       
    
    chain_id BIGINT ,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    
     UNIQUE (transaction_hash, chain_id)
);

 
  