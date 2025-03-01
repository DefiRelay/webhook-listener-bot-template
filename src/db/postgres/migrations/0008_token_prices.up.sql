CREATE TABLE token_prices (
    id SERIAL PRIMARY KEY,
    
    

    token_symbol VARCHAR(255) NOT NULL, 

    price_usd DECIMAL(16, 2) NOT NULL,      

    recorded_at TIMESTAMPTZ NOT NULL  , 
    recorded_at_unix_day_index BIGINT NOT NULL  , 

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(), 
      

        
    UNIQUE( token_symbol , recorded_at_unix_day_index) 

);

 
  