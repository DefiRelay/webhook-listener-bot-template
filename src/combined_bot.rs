
mod bots;

 

 

use bots::vibegraph_bot::run_vibegraph_bot  ; 
 use bots::payment_summary_bot::run_payment_summary; 

#[tokio::main]
async fn main() {
    env_logger::init();
    dotenvy::dotenv().ok();

    //if any panics, the entire combined bot panics 
    let result = tokio::try_join!(
        tokio::spawn(run_vibegraph_bot()),
      
         tokio::spawn( run_payment_summary ()  )
    );

    match result {
        Ok(_) => println!("All workers completed successfully."),
        Err(err) => {
            eprintln!("A worker thread panicked or failed: {:?}", err);
            std::process::exit(1); // 🚨 Force the application to exit
        }
    }
}
