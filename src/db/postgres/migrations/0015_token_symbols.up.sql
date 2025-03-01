CREATE TABLE token_symbols (
    id SERIAL PRIMARY KEY,

    token_address VARCHAR(255) NOT NULL,
     
    chain_id BIGINT NOT NULL  ,     

    token_decimals BIGINT NOT NULL  ,     

    token_symbol VARCHAR(255) NOT NULL, 

    

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(), 
    
    
    UNIQUE(token_address, chain_id)
);

 
  