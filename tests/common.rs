/* Date Created: 29/12/2023. */

//! Common functions in used in tests.

use std::net::TcpListener;
use learn_actix_web::run;

pub fn spawn_app() -> String {
    let listener = TcpListener::bind("0.0.0.0:0")
        .expect("Failed to bind random port");
    
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();

    let server = run(listener).expect("Failed to create server");
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}

pub fn make_full_url(root: &str, path: &str) -> String {
    format!("{}{}", root, path)
}

pub fn make_data_url(root: &str, path: &str) -> String {
    format!("{}/data{}", root, path)
}

pub fn make_ui_url(root: &str, path: &str) -> String {
    format!("{}/ui{}", root, path)
}