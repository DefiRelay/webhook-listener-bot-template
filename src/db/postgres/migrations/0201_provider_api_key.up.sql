CREATE TABLE api_keys (
    id SERIAL PRIMARY KEY,
 
    

    owner_wallet_address VARCHAR(255) NOT NULL , 
        
    apikey TEXT NOT NULL , 


    name TEXT, 

    scopes TEXT , 

  
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW() 

    
);

 