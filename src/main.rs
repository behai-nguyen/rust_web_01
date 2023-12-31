/* Date Created: 11/10/2023. */

//! Web application entry function.

use std::net::TcpListener;
use learn_actix_web::run;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("0.0.0.0:5000").expect("Failed to bind port 5000");
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    println!("Server is listening on port {}", port);

    let server = run(listener).await.unwrap();
    server.await
}