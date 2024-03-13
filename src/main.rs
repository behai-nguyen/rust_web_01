/* Date Created: 11/10/2023. */

//! Web application entry function.

use dotenv::dotenv;
use std::net::TcpListener;
use time::UtcOffset;
use learn_actix_web::helper::app_logger::init_app_logger;
use learn_actix_web::run;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    // Call this to load RUST_LOG.
    dotenv().ok(); 

    // Calling UtcOffset::current_local_offset().unwrap() here works in Ubuntu 22.10, i.e.,
    // it does not raise the IndeterminateOffset error.
    //
    // TO_DO. But this does not guarantee that it will always work! 
    //
    let _guards = init_app_logger(UtcOffset::current_local_offset().unwrap());

    let listener = TcpListener::bind("0.0.0.0:5000").expect("Failed to bind port 5000");
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();

    tracing::debug!("Server is listening on port {}", port);

    let server = run(listener).await.unwrap();
    server.await
}
