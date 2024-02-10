/* Date Created: 29/12/2023. */

//! Light common functions in used in tests.

use std::{fs::File, io::Read as _,};
use std::net::TcpListener;

use actix_web::http::{StatusCode, header};
use reqwest::tls::Certificate;

use learn_actix_web::bh_libs::api_status::ApiStatus;
use learn_actix_web::run;
use learn_actix_web::models::LoginSuccessResponse;
use learn_actix_web::helper::endpoint::http_status_code;
// use learn_actix_web::helper::messages::LOGIN_FAILURE_MSG;

pub struct TestApp {
    pub app_url: String,
}

impl TestApp {
    pub fn mock_access_token(&self) -> String {
        String::from("chirstian.koblick.10004@gmail.com")
    }    
}

pub async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("0.0.0.0:0")
        .expect("Failed to bind random port");
    
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();

    let server = run(listener).await.unwrap();
    let _ = tokio::spawn(server);

    TestApp {
        app_url: format!("https://127.0.0.1:{}", port)
    }
}

/// See https://github.com/actix/examples/tree/master/https-tls/openssl
/// https://docs.rs/reqwest/latest/reqwest/tls/struct.Certificate.html
///  
/// This certificate has been generated on Ubuntu 22.10. 
///
/// It seems that ``.add_root_certificate(load_certificate())`` only 
/// required on the first run? Without it, certicate fails with:
///
/// ```
/// Failed to execute request.: reqwest::Error { kind: Request, url: Url { scheme: "https", 
///     cannot_be_a_base: false, username: "", password: None, host: Some(Ipv4(127.0.0.1)), 
///     port: Some(36159), path: "/api/login", query: None, fragment: None }, 
///     source: hyper::Error(Connect, Ssl(Error { code: ErrorCode(1), cause: 
///     Some(Ssl(ErrorStack([Error { code: 167772294, library: "SSL routines", 
///     function: "tls_post_process_server_certificate", reason: "certificate verify failed", 
///     file: "../ssl/statem/statem_clnt.c", line: 1889 }]))) }, X509VerifyResult { code: 18, 
///     error: "self-signed certificate" })) }
/// ```
///
/// After the first run (?), ``.add_root_certificate(load_certificate())`` can be removed?
///
pub fn load_certificate() -> Certificate {
    let mut buf = Vec::new();
    File::open("./cert/cert-pass.pem")
        .unwrap()
        .read_to_end(&mut buf).unwrap();

    Certificate::from_pem(&buf).unwrap()
}

/// Creates a reqwest::Client to work with the HTTPS server.
/// There is no need to load the certificate that the server is using.
/// 
/// # References
/// 
///    https://stackoverflow.com/questions/76724036/how-to-resolve-a-rust-reqwest-error-invalid-certificate
///    Answer https://stackoverflow.com/questions/76724036/how-to-resolve-a-rust-reqwest-error-invalid-certificate#comment135281495_76724036
/// 
/// mentions using ``.danger_accept_invalid_cert(true)``.
/// 
/// Without calling ``.danger_accept_invalid_certs(true)`` -- note the s -- it would
/// result in the following error:
/// 
/// ```
/// Failed to execute request.: reqwest::Error { kind: Request, url: Url { scheme: "https", cannot_be_a_base: false, username: "", password: None, host: Some(Ipv4(127.0.0.1)), port: Some(62730), path: "/ui/home", query: None, fragment: None }, source: hyper::Error(Connect, Os { code: -2146762487, kind: Uncategorized, message: "A certificate chain processed, but terminated in a root certificate which is not trusted by the trust provider." }) }
/// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
/// ```
///
/// or
/// 
/// ```
/// Failed to execute request.: reqwest::Error { kind: Request, url: Url { scheme: "http", cannot_be_a_base: false, username: "", password: None, host: Some(Ipv4(127.0.0.1)), port: Some(65030), path: "/api/login", query: None, fragment: None }, source: hyper::Error(Io, Os { code: 10054, kind: ConnectionReset, message: "An existing connection was forcibly closed by the remote host." }) }
/// ```
/// 
pub fn reqwest_client() -> reqwest::Client {
    reqwest::Client::builder()
        .add_root_certificate(load_certificate())
        .danger_accept_invalid_certs(true) 
        .cookie_store(true)
        .build()
        .unwrap()
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

pub fn make_api_url(root: &str, path: &str) -> String {
    format!("{}/api{}", root, path)
}

pub fn write_to_file(file_name: &str, content: &str) {
    use std::fs::File;
    use std::io::Write;
    
    let mut file = File::create(file_name).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}

pub async fn assert_html_home_page(response: reqwest::Response) {
    assert_eq!(response.status(), StatusCode::OK);

    let res = response.text().await;
    assert!(res.is_ok(), "Should have a HTML response.");

    // This should now always succeed.
    if let Ok(html) = res {
        assert!(html.contains("<title>Rust Web 1 | Home</title>"), "HTML: title.");
        assert!(html.contains("<button type=\"submit\">Logout</button>"), "HTML: logout button.");
    }
}

pub async fn assert_html_login_page(response: reqwest::Response) {
    assert_eq!(response.status(), StatusCode::OK);

    let res = response.text().await;
    assert!(res.is_ok(), "Should have a HTML response.");

    // This should now always succeed.
    if let Ok(html) = res {
        assert!(html.contains("<title>Rust Web 1 | Login</title>"), "HTML: title.");
        assert!(html.contains("<button type=\"submit\">Login</button>"), "HTML: Login button.");
    }    
}

pub fn assert_access_token_in_header(response: &reqwest::Response, access_token: &str) {
    let header = response.headers().get(header::AUTHORIZATION);
    assert_eq!(header.is_some(), true);
    assert_eq!(header.unwrap().to_str().unwrap(), access_token);
}

/// TO_DO: this works, but feels clunky. Need reworks!
pub fn assert_access_token_in_cookie(response: &reqwest::Response, access_token: &str) {
    // Assertain that cookie header::AUTHORIZATION is present.
    let mut found: bool = false;
    for c in response.cookies() {
        if c.name() == header::AUTHORIZATION.as_str() {
            assert_eq!(c.value(), access_token);
            found = true;
            break;
        }
    }
    assert_eq!(found, true);
}

pub async fn assert_json_successful_login(
    response: reqwest::Response,
    email: &str,
    access_token: &str) {
    assert_eq!(response.status(), StatusCode::OK);

    let res = response.json::<LoginSuccessResponse>().await;
    assert!(res.is_ok(), "Should have a JSON response.");

    // This should now always succeed.
    if let Ok(json_obj) = res {
        assert_eq!(json_obj.api_status.get_code(), http_status_code(StatusCode::OK));
        assert_eq!(json_obj.data.email, email);
        assert_eq!(json_obj.data.access_token, access_token);
    }        
}

pub fn assert_access_token_not_in_header(response: &reqwest::Response) {
    let header = response.headers().get(header::AUTHORIZATION);
    assert_eq!(header.is_none(), true);
}

/// TO_DO: this works, but feels clunky. Need reworks!
pub fn assert_access_token_not_in_cookie(response: &reqwest::Response) {
    // Assertain that cookie header::AUTHORIZATION is present.
    let mut found: bool = false;
    for c in response.cookies() {
        if c.name() == header::AUTHORIZATION.as_str() {
            found = true;
            break;
        }
    }
    assert_eq!(found, false);
}

pub async fn assert_redirected_html_login_page(
    response: reqwest::Response,
    status_code: StatusCode,
    message: &str) {
    assert_eq!(response.status(), status_code);

    let res = response.text().await;
    assert!(res.is_ok(), "Should have a HTML response.");

    // This should now always succeed.
    if let Ok(html) = res {
        assert!(html.contains("<title>Rust Web 1 | Login</title>"), "HTML: title.");
        assert!(html.contains(&format!("<h2>{}</h2>", message)), "HTML: redirect message.");
        assert!(html.contains("<button type=\"submit\">Login</button>"), "HTML: Login button.");
    }    
}

pub async fn assert_json_failure(
    response: reqwest::Response,
    status_code: StatusCode,
    reason: &str,
    reason_is_sub_text: bool) {
    assert_eq!(response.status(), status_code);

    let res = response.json::<ApiStatus>().await;
    assert!(res.is_ok(), "Should have a JSON response.");

    // This should now always succeed.
    if let Ok(json_obj) = res {
        assert_eq!(json_obj.get_code(), http_status_code(status_code));

        match reason_is_sub_text {
            true => assert!(json_obj.get_message().unwrap().contains(reason)),
            false => assert_eq!(json_obj.get_message().unwrap(), reason)
        };

        assert_eq!(json_obj.get_session_id(), None);
    }
}
