/* Date Created: 22/11/2023. */

//! A middleware which searches a database for employees based partial last name 
//! and a partial firstname, if found some, attach a string message with ***the first*** 
//! matched employee details to the request via ``extensions_mut``. If none matched, 
//! the message is ``"No employee found"``.
//!
//! This middleware is based largely on: ⓵ the ``SayHi`` middleware example found in the official
//! document at [Module actix_web::middleware](https://docs.rs/actix-web/latest/actix_web/middleware/index.html);
//! and ⓶ [actix GitHub middleware request-extensions](https://github.com/actix/examples/tree/master/middleware/request-extensions)
//! example.
//!
//! [fn call(&self, req: ServiceRequest) -> Self::Future](`SayHiMiddleware<S>::call`) is where all the works
//! take place.

use std::{future::{ready, Ready, Future}, pin::Pin};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    web::Data, Error, HttpMessage, 
};

use async_std::task;

use super::AppState;
use crate::models::get_employees;

#[derive(Debug, Clone)]
pub struct Msg(pub String);

pub struct SayHi;

// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for SayHi
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = SayHiMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(SayHiMiddleware { service }))
    }
}
pub struct SayHiMiddleware<S> {
    /// The next service to call
    service: S,
}

// This future doesn't have the requirement of being `Send`.
// See: futures_util::future::LocalBoxFuture
type LocalBoxFuture<T> = Pin<Box<dyn Future<Output = T> + 'static>>;

// `S`: type of the wrapped service
// `B`: type of the body - try to be generic over the body where possible
impl<S, B> Service<ServiceRequest> for SayHiMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<Result<Self::Response, Self::Error>>;

    // This service is ready when its next service is ready
    forward_ready!(service);

    /// The request path is ``/helloemployee/{last_name}/{first_name}``, where ``{last_name}``
    /// is partial last name, ``{first_name}`` is partial first name.
    /// 
    /// The request [app_data](https://docs.rs/actix-web/latest/actix_web/dev/struct.ServiceRequest.html#method.app_data) 
    /// also has [AppState](`super::AppState`) where the read to use database connection pool is.
    /// 
    /// Attempts to retrieve employee records based on partial last name and partial 
    /// first name. Calls to [`get_employees`] method to do database work. 
    /// 
    /// If some employees matched, then only attach a string message with ***the first*** 
    /// matched employee details to the request via ``extensions_mut``. If none matched, 
    /// the message is ``"No employee found"``.
    /// 
    fn call(&self, req: ServiceRequest) -> Self::Future {
        println!("Hi from start. You requested: {}", req.path());

        let last_name: String = req.match_info().get("last_name").unwrap().parse::<String>().unwrap();
        let first_name: String = req.match_info().get("first_name").unwrap().parse::<String>().unwrap();
        
        println!("Middleware. last name: {}, first name: {}.", last_name, first_name);

        // Retrieve the application state, where the database connection object is.
        let app_state = req.app_data::<Data<AppState>>().cloned().unwrap();
        // Query the database using the partial last name and partial first name.
        let query_result = task::block_on(get_employees(&app_state.db, &last_name, &first_name));

        // Attached message.
        let hello_msg: String;

        // Is there any data retrieved?
        match query_result.len() {
            // No matched employee.
            0 => {
                println!("Hi from response -- no employees found.");

                hello_msg = String::from("No employee found");
            },

            // There is/are some matched employee(s).
            _ => {
                println!("Hi from response -- some employees found.");

                let emp = &query_result[0];

                hello_msg = format!("Hi first employee found 🦀 no: {}, dob: {}, 
                    first name: {}, last name: {}, gender: {}, hire date: {}", 
                    emp.emp_no, emp.birth_date, emp.first_name, emp.last_name, emp.gender, emp.hire_date);
            }
        };

        // Attached message to request.
        req.extensions_mut().insert(Msg(hello_msg.to_owned()));

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}
