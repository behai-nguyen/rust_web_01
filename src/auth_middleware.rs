/* Date Created: 02/12/2023. */

//! A middleware which provide authentication checking: apart from the login page,
//! which can be accessed in a non-authenticated state. All other JSON and HTML routes
//! can only be accesssed when authenticated.
//! 
//! This is middleware is based on the following official example 
//! [actix GitHub example middleware various redirect](https://github.com/actix/examples/blob/master/middleware/various/src/redirect.rs),
//! almost as is.
//! 
//! Most of the works are in the ``call(...)`` method.
//! 
//! # Notes On Routes
//! 
//! * ``/ui/login``: the actual HTML login page.
//! 
//! * ``/api/login``: the actual login / authentication process.
//! 
//! * ``/ui/home``: the actual HTML home page.
//!
//! # How This Middleware Works
//!
//! * If request to ``/favicon.ico`` should just go through.
//!
//! * Determine the status of the token.
//!
//! * If the token is invalid, return [Unauthorized()](https://docs.rs/actix-web/latest/actix_web/struct.HttpResponse.html#method.Unauthorized)
//!   whose body is JSON serialisation of [ApiStatus](`crate::bh_libs::api_status::ApiStatus`),
//!   which contains the token invalid reason. **The request is completed.**
//! 
//! * When authenticated
//!
//!     - Update the current [JWTPayload](`crate::helper::jwt_utils::JWTPayload`) to new expiry 
//!       and last active. Make a new token from  this updated [JWTPayload](`crate::helper::jwt_utils::JWTPayload`). 
//!
//!     - Then replace [actix-identity](https://docs.rs/actix-identity/0.7.0/actix_identity/)
//!       [Identity](https://docs.rs/actix-identity/0.7.0/actix_identity/struct.Identity.html) 
//!       login with this updated token. 
//!
//!     - Finally, set updated token to request extension, so that the next middleware can pick 
//!       it up and send it to clients via both response header and response cookie ``authorization``. 
//! 
//!     - Requests to routes ``/ui/login`` and ``/api/login`` are redirected to 
//!       ``/ui/home``. **WIP**: should the response be based on the original request 
//!       content type? I.e., if the original request is in ``application/x-www-form-urlencoded``, 
//!       then redirects to ``/ui/home``. Otherwise, if it is ``application/json``, then some 
//!       kind of JSON based on [LoginSuccessResponse](`super::models::LoginSuccessResponse`).
//! 
//!     - Requests to any other routes should go through as is.
//! 
//! * When not authenticated
//! 
//!     - Requests to routes ``/ui/login`` and ``/api/login`` should go through.
//! 
//!     - Requests to any other route should get redirected to ``/ui/login``.
//!       See [login_page](`crate::auth_handlers::login_page`) for more detail on response.
//! 
use std::future::{ready, Ready};

use actix_web::{
    body::EitherBody, dev::{self, Service, ServiceRequest, ServiceResponse, Transform}, 
    http::header, web::Data, Error, HttpMessage, HttpResponse, 
};

use futures_util::future::LocalBoxFuture;

use actix_identity::{IdentityExt, Identity};

use crate::{bh_libs::api_status::ApiStatus, helper::app_utils::{
    build_login_redirect_cookie,
    build_original_content_type_cookie,
    remove_login_redirect_cookie,
    remove_original_content_type_cookie
}};

use crate::helper::messages::UNAUTHORISED_ACCESS_MSG;

use super::AppState;
use crate::helper::jwt_utils::{
    JWTPayload, decode_bearer_token, 
    make_token_from_payload, make_bearer_token
};

/// The "status" of the token. Might be use another name?
/// Both ``payload`` and ``api_status`` can be None. But otherwise they are
/// mutually exclusive.
///
/// * When ``is_logged_in`` is ``false``, ``api_status`` might or might not be set:
///
///     * If ``api_status`` is set, then the token is in error.
///
///     * If ``api_status`` is not set, that means there is no token / login yet. 
///       Token is not in error.
///
/// * When ``is_logged_in`` is ``true``, ``payload`` is set. This an authenticated
///   web session.
///
struct TokenStatus {
    is_logged_in: bool,
    payload: Option<JWTPayload>,
    api_status: Option<ApiStatus>
}

/// Attempt to extracts access token from request header 
/// [AUTHORIZATION](`actix_web::http::header::AUTHORIZATION`), 
/// request cookie [AUTHORIZATION](`actix_web::http::header::AUTHORIZATION`), and request 
/// [actix-identity](https://docs.rs/actix-identity/0.7.0/actix_identity/) 
/// extension using Redis.
/// 
/// **Work In Progress**: cookie extraction code works, but is commented out at present.
/// This is to assertain that we could always indeed rely on 
/// [actix-identity](https://docs.rs/actix-identity/0.7.0/actix_identity/)
/// to manage the token as documented.
/// 
/// # Notes on Request Identity, Header and Cookie
/// 
/// * Request [actix-identity](https://docs.rs/actix-identity/0.7.0/actix_identity/)
/// exension using Redis persists the identity, i.e. the access 
/// token, across requests when using a HTML client. That is, the application acts as
/// an application server. 
/// 
/// * When using clients such as Testfully, or AJAX calls, etc., after logged in, clients need
/// to remember this token locally, and set it to request 
/// [AUTHORIZATION](`actix_web::http::header::AUTHORIZATION`)
/// header on subsequent requests to access protected resources. That is, the application acts
/// as an API-like server or a service.
/// 
/// # Arguments
/// 
/// * `request` - from the calling middleware.
/// 
/// # Return
/// 
/// * Optionally the access token as string if found.
/// 
fn extract_access_token(
    request: &ServiceRequest
) -> Option<String> {
    // If we use a client, such as Testfully, after logged in, we must remember the 
    // access token, on subsequent requests, we must include this token in the header
    // header::AUTHORIZATION. Then the access token will be extracted from this block 
    // of code. 
    if let Some(value) = request.headers().get(header::AUTHORIZATION) {
        tracing::debug!("Token extracted from header {}", value.to_str().unwrap());
        return Some(String::from(value.to_str().unwrap()));
    }

    // Cookie works also. I commented this block out, to assertain that we could
    // always indeed rely on actix-identity to manage the token as documented.
    /*
    if let Some(value) = request.cookie(header::AUTHORIZATION.as_str()) {
        println!("Token extracted from cookie {}", value.to_string());
        return Some(String::from(value.to_string()));
    }
    */

    // If we use the HTML client, then the token would be extracted from actix-identity. 
    // I.e., the access token will be extracted from this block of code.
    if let Some(id) = request.get_identity().ok() {
        tracing::debug!("Token extracted from identity {}", id.id().unwrap());
        return Some(String::from(id.id().unwrap()));
    }

    None
}

/// Verify that there is a valid JSON Web Token access token for the current request.
/// 
/// # Arguments
/// 
/// * `request` - contains [AppState](`super::AppState`).
/// 
/// # Return
/// 
/// * [`TokenStatus`].
///
fn verify_valid_access_token(
    request: &ServiceRequest
) -> TokenStatus {
    // Attempts to extract the access token from the current request.
    let res = extract_access_token(request);

    // There is no token! 
    // Not a logged in web session. Not an error.
    if res.is_none() {
        return TokenStatus{is_logged_in: false, payload: None, api_status: None};
    }

    // Retrieve the application state, where the Config object is.
    let app_state = request.app_data::<Data<AppState>>().cloned().unwrap();

    // Decode the access token to verify validity.
    // res.unwrap() -- the actual access token as a string.
    let res = decode_bearer_token(&res.unwrap(), app_state.cfg.jwt_secret_key.as_ref());

    if res.is_ok() {
        // Token is valid and not expired.
        TokenStatus{is_logged_in: true, payload: Some(res.unwrap()), api_status: None}
    }
    else {
        // Token is not valid. Set api_status to the invalid reason.
        TokenStatus{is_logged_in: false, payload: None, api_status: Some(res.err().unwrap())}
    }
}

/// The access token is valid. Update the current [JWTPayload](`crate::helper::jwt_utils::JWTPayload`) 
/// to new expiry and last active. Make a new token from this updated 
/// [JWTPayload](`crate::helper::jwt_utils::JWTPayload`). Then replace 
/// [actix-identity](https://docs.rs/actix-identity/0.7.0/actix_identity/)
/// [Identity](https://docs.rs/actix-identity/0.7.0/actix_identity/struct.Identity.html) login with 
/// this updated token. Finally, set updated token to request extension, so that the next middleware 
/// can pick it up and send it to clients via both response header and response cookie ``authorization``.
/// 
/// # Arguments
/// 
/// * `request` - contains [AppState](`super::AppState`).
///
/// * ``token_status`` -- contains the current [JWTPayload](`crate::helper::jwt_utils::JWTPayload`).
///
fn update_and_set_updated_token(request: &ServiceRequest, token_status: TokenStatus) {
    // Retrieve the application state, where the config (.env) object is.
    let app_state = request.app_data::<Data<AppState>>().cloned().unwrap();

    // Access the current JWTPayload.
    let current_payload = token_status.payload.unwrap().clone();

    // Update current JWTPayload's expiry, last active. And make a new token from this 
    // updated JWTPayload.
    let updated_token = make_token_from_payload(
        &current_payload.update_expiry_secs(app_state.cfg.jwt_mins_valid_for * 60), 
        app_state.cfg.jwt_secret_key.as_ref());

    // Replace actix-identity Identity login with updated token.
    Identity::login(&request.extensions(), String::from( make_bearer_token(&updated_token) )).unwrap();

    // Attach the updated token to request extension, so that the next middleware can pick it 
    // up and send it to clients via both response header and response cookie ``authorization``.
    request.extensions_mut().insert(updated_token);
}

/// The middleware factory. Naming and declaration remain as per 
/// [official example](https://github.com/actix/examples/blob/master/middleware/various/src/redirect.rs).
/// 
pub struct CheckLogin;

/// Copied from [official example](https://github.com/actix/examples/blob/master/middleware/various/src/redirect.rs).
/// 
impl<S, B> Transform<S, ServiceRequest> for CheckLogin
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = CheckLoginMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(CheckLoginMiddleware { service }))
    }
}

/// As per [official example](https://github.com/actix/examples/blob/master/middleware/various/src/redirect.rs).
/// 
pub struct CheckLoginMiddleware<S> {
    service: S,
}

/// As per [official example](https://github.com/actix/examples/blob/master/middleware/various/src/redirect.rs).
/// 
/// The ``call`` method has been modified to suite the working requirement of this middleware.
/// 
impl<S, B> Service<ServiceRequest> for CheckLoginMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future {
        if let Some(value) = request.cookie("id") {
            tracing::debug!("Auth -- Id {:#?}", String::from(value.to_string()));
        }

        tracing::debug!("Auth -- requested path: {}, method: {}; content type: {}", 
            request.path(), request.method(), request.content_type());

        let call_request = |req: ServiceRequest| -> Self::Future {
            let res = self.service.call(req);

            Box::pin(async move {
                // forwarded responses map to "left" body
                res.await.map(ServiceResponse::map_into_left_body)
            })
        };

        let redirect_to_route = |req: ServiceRequest, route: &str| -> Self::Future {
            let (request, _pl) = req.into_parts();
            
            let mut builder = HttpResponse::SeeOther();

            // Remembers the content type for the next anew redirected request.
            builder.insert_header((header::LOCATION, route))
                .cookie(build_original_content_type_cookie(request.content_type()));

            // If redirected to "/ui/login", then users must have attempted to access a 
            // protected resource while not logged in. Remembers the redirection, and the 
            // reason for the next anew redirected request.
            if route == "/ui/login" {
                builder.cookie(build_login_redirect_cookie(UNAUTHORISED_ACCESS_MSG));
            }

            let response = builder.finish().map_into_right_body();

            Box::pin(async { Ok(ServiceResponse::new(request, response)) })
        };
        
        let redirect_to_login = |req: ServiceRequest| -> Self::Future {
            redirect_to_route(req, "/ui/login")
        };

        let redirect_to_home = |req: ServiceRequest| -> Self::Future {
            redirect_to_route(req, "/ui/home")
        };

        // This closure just return a 401 response to the clients. The body of the response
        // is the JSON serialisation of api_status: ApiStatus.
        let unauthorised_token = |req: ServiceRequest, api_status: ApiStatus| -> Self::Future {
            let (request, _pl) = req.into_parts();

            let response = HttpResponse::Unauthorized()
                .insert_header((header::CONTENT_TYPE, header::ContentType::json()))
                .cookie(remove_login_redirect_cookie())
                .cookie(remove_original_content_type_cookie())            
                .body(serde_json::to_string(&api_status).unwrap())
                .map_into_right_body();

            Box::pin(async { Ok(ServiceResponse::new(request, response)) })
        };        

        // TO_DO: Windows IIS! This feels like a hack, I'm not sure how to handle this.
        // Or this is even correct. Please be careful.
        //
        // Without this, when is_logged_in is false, it would get redirect.
        //
        if request.path() == "/favicon.ico" {
             return call_request(request);
        }

        // Determine the status of the token.
        let token_status = verify_valid_access_token(&request);

        // The token is invalid.
        if !token_status.is_logged_in && token_status.api_status.is_some() {
            return unauthorised_token(request, token_status.api_status.unwrap());
        }

        match token_status.is_logged_in {
            true => {
                // Update token new expiry, last active.
                // Replace actix-identity Identity login with updated token.
                // Set updated token to request extension, so that the next middleware can pick it up
                // and send it to clients via both response header and response cookie ``authorization``.
                update_and_set_updated_token(&request, token_status);

                match request.path().as_ref() {
                    "/ui/login" | "/api/login" => redirect_to_home(request),
                    _ => call_request(request)
                }
            }

            false => {
                match request.path().as_ref() {
                    "/ui/login" => call_request(request),

                    "/api/login" => call_request(request),

                    _ => redirect_to_login(request)
                }
            }
        }
    }
}
