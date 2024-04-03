<!-- 01/04/2024. -->

# Summary

Attempt to group log entries for each request within the markers ``Request [no session Id] entry`` and ``Request [no session Id] exit``, or ``Request <Uuid V4> entry`` and ``Request <Uuid V4> exit``.

💥 However, this revision of the code has not quite been able to achieve that.

# Logging Issue in this Revision

## Correct Logging

Please note, the blank lines have been manually inserted for readability in this document. Actual logging files do not contain blank lines.

```
2024-04-02 23:55:28  INFO learn_actix_web: Request [no session Id] entry

2024-04-02 23:55:28 DEBUG learn_actix_web::auth_middleware: Auth -- requested path: /ui/login, method: GET; content type: 

2024-04-02 23:55:28  INFO learn_actix_web: Request [no session Id] exit
```

## Logging Issue

```
2024-04-02 23:55:33 DEBUG learn_actix_web::auth_middleware: Token extracted from identity Bearer.eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.
eyJlbWFpbCI6ImNoaXJzdGlhbi5rb2JsaWNrLjEwMDA0QGdtYWlsLmNvbSIsInNlc3Npb25faWQiOiIyN2RhMjBkZi1kMjFmLTRjMTMtOTc1NS05NWUzN2YxMWJlM2YiLCJpYXQiOjE3MTIwNjI1MzAsImV4cCI6MTcxMjA2NDMzMCwibGFzdF9hY3RpdmUiOjE3MTIwNjI1MzB9.YxFiUcni4SqxFztbCJTXrkJnAp7INK5QDO6QyGZ95Ic

2024-04-02 23:55:33  INFO learn_actix_web: Request 27da20df-d21f-4c13-9755-95e37f11be3f entry

2024-04-02 23:55:33 DEBUG learn_actix_web::auth_middleware: Auth -- Id "id=nmnEuYkwkLyQ0gMhtXKWcQGEv58WD6RKQldZS7rbh0Ep4/cFPD4nwTeXpN5xFw6bowvf8grxrbVPYZwACirP7u7pX+uRq6iuEcU2ui98+jFEE4IP7KAmdVwwIF4="

2024-04-02 23:55:33 DEBUG learn_actix_web::auth_middleware: Auth -- requested path: /data/employees, method: POST; content type: application/json

2024-04-02 23:55:33 DEBUG learn_actix_web::auth_middleware: Token extracted from identity Bearer.eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJlbWFpbCI6ImNoaXJzdGlhbi5rb2JsaWNrLjEwMDA0QGdtYWlsLmNvbSIsInNlc3Npb25faWQiOiIyN2RhMjBkZi1kMjFmLTRjMTMtOTc1NS05NWUzN2YxMWJlM2YiLCJpYXQiOjE3MTIwNjI1MzAsImV4cCI6MTcxMjA2NDMzMCwibGFzdF9hY3RpdmUiOjE3MTIwNjI1MzB9.YxFiUcni4SqxFztbCJTXrkJnAp7INK5QDO6QyGZ95Ic

2024-04-02 23:55:33 DEBUG learn_actix_web::auth_middleware: Token extracted from identity Bearer.eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJlbWFpbCI6ImNoaXJzdGlhbi5rb2JsaWNrLjEwMDA0QGdtYWlsLmNvbSIsInNlc3Npb25faWQiOiIyN2RhMjBkZi1kMjFmLTRjMTMtOTc1NS05NWUzN2YxMWJlM2YiLCJpYXQiOjE3MTIwNjI1MzAsImV4cCI6MTcxMjA2NDMzMywibGFzdF9hY3RpdmUiOjE3MTIwNjI1MzN9.Xej7L1ADZsuWDGJuRel057flK8DCuHaqFPJYV1VZFpE

2024-04-02 23:55:33 DEBUG learn_actix_web: Requested succeeded. Returning updated access token.

2024-04-02 23:55:33  INFO learn_actix_web: Request 27da20df-d21f-4c13-9755-95e37f11be3f exit
```

The first entry should be positioned below the second entry as follows:

```
2024-04-02 23:55:33  INFO learn_actix_web: Request 27da20df-d21f-4c13-9755-95e37f11be3f entry

2024-04-02 23:55:33 DEBUG learn_actix_web::auth_middleware: Token extracted from identity Bearer.eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.
eyJlbWFpbCI6ImNoaXJzdGlhbi5rb2JsaWNrLjEwMDA0QGdtYWlsLmNvbSIsInNlc3Npb25faWQiOiIyN2RhMjBkZi1kMjFmLTRjMTMtOTc1NS05NWUzN2YxMWJlM2YiLCJpYXQiOjE3MTIwNjI1MzAsImV4cCI6MTcxMjA2NDMzMCwibGFzdF9hY3RpdmUiOjE3MTIwNjI1MzB9.YxFiUcni4SqxFztbCJTXrkJnAp7INK5QDO6QyGZ95Ic
```

This discrepancy occurs due to the asynchronous nature of execution, where calls are not guaranteed to complete and return in chronological order.

# Code Changes

* ``Cargo.toml``: Added crate ``uuid = {version = "1.8", features = ["v4"]}``.

* ``src\helper\jwt_utils.rs`` -- Major refactoring:

    - Updated ``pub struct JWTPayload`` by adding ``session_id: String``. This is 
    ``Uuid v4``. Refer to [Crate uuid](https://docs.rs/uuid/latest/uuid/).

    - All relevant associated methods and tests were updated to accommodate the new ``session_id``
    field.

* ``src\auth_middleware.rs``: 

    - Added ``pub struct ResponseErrorStatus``.

    - Made the function ``pub fn extract_access_token`` public.

    - Rewrote the ``unauthorised_token`` closure to return a 401 error. It attaches an instance of [ApiStatus](https://github.com/behai-nguyen/rust_web_01/blob/d22804332a55c683dbc272d66fa829c478681ea7/src/bh_libs/api_status.rs#L18) to the request extension and forwards the request on.

* ``src\lib.rs``:

    - Removed ``update_return_jwt``. 
    
    - Added ``log_request_entry`` and ``finalise_request``.

* ``tests\common.rs`` and ``tests\test_jsonwebtoken.rs``: Called ``decode_token``
  with ``validate_exp: Option<bool>`` set to ``None``.

# References

Please note that, except for the first one, all of these reference materials are related to the section [Potential Solution to the Above Logging Issue](#potential-solution).

* [``users.rust-lang.org`` -- Actix-web / actix-web-lab, compiler gives error when in else block, please help](https://users.rust-lang.org/t/actix-web-actix-web-lab-compiler-gives-error-when-in-else-block-please-help/108925)

* [ensure actix logging in chronological order // pre-process](https://stackoverflow.com/questions/66065426/ensure-actix-logging-in-chronological-order-pre-process)

* [Production-Grade Logging in Rust Applications](https://betterprogramming.pub/production-grade-logging-in-rust-applications-2c7fffd108a6)

* [Tracing クレートを用いたカスタム JSON ログ](https://tech.emotion-tech.co.jp/entry/2022/12/10/073000)
  
  This implementation allows logging in JSON format without any additional crate. It enables stopping request fields from being logged.
  
* [tracing-actix-web closed issues](https://docs.rs/tracing-actix-web/0.7.10/tracing_actix_web/)

  These issues address the prevention of certain request fields from being logged, which unfortunately will not be supported.

  - [Is it possible to remove some of the default fields from the root span builder? #83](https://github.com/LukeMathWalker/tracing-actix-web/issues/83)

  - [Feature request: Option to disable the standard OTEL fields #101](https://github.com/LukeMathWalker/tracing-actix-web/issues/101)

<a id="potential-solution"></a>
# Potential Solution to the Above Logging Issue

By using the crates [tracing-log](https://docs.rs/tracing-log/latest/tracing_log/) and 
[tracing-actix-web](https://docs.rs/tracing-actix-web/0.7.6/tracing_actix_web/), and providing our own implementation for the trait [tracing_actix_web::RootSpanBuilder](https://docs.rs/tracing-actix-web/0.7.6/tracing_actix_web/trait.RootSpanBuilder.html), it becomes feasible to capture all log entries for each request within the markers.

However, another issue arises: all fields from the crate [tracing-actix-web](https://docs.rs/tracing-actix-web/0.7.6/tracing_actix_web/) are included in the JSON log entries, and there is no straightforward method to remove unwanted fields.

Although it is possible to remove unwanted fields, the process appears to be quite involved. Therefore, I have decided not to pursue it for the time being.
