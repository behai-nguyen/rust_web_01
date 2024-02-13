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
//! # How This Middleware Works
//! 
//! * Notes On Routes
//! 
//!     - ``/ui/login``: the actual HTML login page.
//! 
//!     - ``/api/login``: the actual login / authentication process.
//! 
//!     - ``/ui/home``: the actual HTML home page.
//! 
//! * When Authenticated
//! 
//!     - Requests to routes ``/ui/login`` and ``/api/login`` are redirected to 
//!       ``/ui/home``. **WIP**: should the response be based on the original request 
//!       content type? I.e., if the original request is in ``application/x-www-form-urlencoded``, 
//!       then redirects to ``/ui/home``. Otherwise, if it is ``application/json``, then some 
//!       kind of JSON based on [`super::models::LoginSuccessResponse`].
//! 
//!     - Requests to any other routes should go through as is.
//! 
//! * When Not Authenticated
//! 
//!     - Requests to routes ``/ui/login`` and ``/api/login`` should go through.
//! 
//!     - Requests to any other route should get redirected to ``/ui/login``.
//!       See [`crate::auth_handlers::login_page`] for more detail on response.
//! 
use std::future::{ready, Ready};

use actix_web::{
    body::EitherBody,
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    http::header, Error, HttpResponse, HttpMessage,
};

use futures_util::future::LocalBoxFuture;
use actix_identity::IdentityExt;

use crate::helper::app_utils::{
    build_login_redirect_cookie,
    build_original_content_type_cookie
};

use crate::helper::messages::UNAUTHORISED_ACCESS_MSG;

/// Attempt to extracts access token from request header [`actix_web::http::header::AUTHORIZATION`], 
/// request cookie [`actix_web::http::header::AUTHORIZATION`], and request actix-identity 
/// extension using Redis.
/// 
/// **Work In Progress**: cookie extraction code works, but is commented out at present.
/// This is to assertain that we could always indeed rely on actix-identity to manage the 
/// token as documented.
/// 
/// # Notes on Request Identity, Header and Cookie
/// 
/// * Request actix-identity exension using Redis persists the identity, i.e. the access 
/// token, across requests when using a HTML client.
/// 
/// * When using clients such as Testfully, or AJAX calls, etc., after logged in, clients need
/// to remember this token locally, and set it to request [`actix_web::http::header::AUTHORIZATION`] 
/// header on subsequent requests to access protected resources.
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
        println!("Token extracted from header {}", value.to_str().unwrap());
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
        println!("Token extracted from identity {}", id.id().unwrap());
        return Some(String::from(id.id().unwrap()));
    }

    None
}

/// Verify that there is a valid access token for the current request.
/// 
/// **Work In Progress** definition of *a valid token*
/// 
/// In this implementation, *valid* is simply a non-blank string!
/// 
/// # Arguments
/// 
/// * `request` - from the calling middleware.
/// 
/// # Return
/// 
/// * ``true`` if the current request has an access token, and the token is valid.
/// ``false`` otherwise.
/// 
fn verify_valid_access_token(
    request: &ServiceRequest
) -> bool {
    // Attempts to extract the access token from the current request.
    let res = extract_access_token(request);
    // There is no token! Token is not valid.
    if res.is_none() {
        return false;
    }

    // Getting the actual access token as a string.
    let auth_token = res.unwrap();

    // TO_DO: token is a non-blank string. Token is valid!
    if auth_token.len() > 0 {
        return true;
    }

    // Token is not valid.
    false
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
        println!("Auth -- requested path: {}, method: {}; content type: {}", 
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
                .cookie(build_original_content_type_cookie(&request, request.content_type()));

            // If redirected to "/ui/login", then users must have attempted to access a 
            // protected resource while not logged in. Remembers the redirection, and the 
            // reason for the next anew redirected request.
            if route == "/ui/login" {
                builder.cookie(build_login_redirect_cookie(&request, UNAUTHORISED_ACCESS_MSG));
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

        // TO_DO: Windows IIS! This feels like a hack, I'm not sure how to handle this.
        // Or this is even correct. Please be careful.
        //
        // Without this, when is_logged_in is false, it would get redirect.
        //
        if request.path() == "/favicon.ico" {
             return call_request(request);
        }

        // TO_DO: Work in progress.
        // Check if access token exists? If exists, is it valid?
        let is_logged_in = verify_valid_access_token(&request);

        match is_logged_in {
            true => {
                match request.path().as_ref() {
                    "/ui/login" | "/api/login" => return redirect_to_home(request),
                    _ => return call_request(request)
                }
            }

            false => {
                match request.path().as_ref() {
                    "/ui/login" => return call_request(request),

                    "/api/login" => return call_request(request),

                    _ => return redirect_to_login(request)
                }
            }
        };
    }
}
