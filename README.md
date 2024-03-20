# rust_web_01 / learn_actix_web

My Rust and [actix-web](https://docs.rs/actix-web/latest/actix_web/) web development learning application.

The database used in this application is the [Oracle Corporation MySQL test database](https://github.com/datacharmer/test_db).

For each stage of the learning process, I document in a post which gets listed in the [Related post(s)](#related-posts) section below.

## Related post(s)

1. [Rust web application: MySQL server, sqlx, actix-web and tera.](https://behainguyen.wordpress.com/2023/10/18/rust-web-application-mysql-server-sqlx-actix-web-and-tera/)

The code version for the above post has been tagged with **v0.1.0**. It can be cloned with:
  
```
git clone -b v0.1.0 https://github.com/behai-nguyen/rust_web_01.git
```

We write a Rust web application using a MySQL database. We use the already familiar crate <a href="https://docs.rs/sqlx/latest/sqlx" title="Crate sqlx" target="_blank">sqlx</a> for database works. The web framework we're using is crate <a href="https://actix.rs/docs/" title="actix-web" target="_blank">actix-web</a>. For Cross-Origin Resource Sharing (CORS) controls, we use crate <a href="https://docs.rs/actix-cors/latest/actix_cors/" title="actix-cors" target="_blank">actix-cors</a>. For HTML template processing, we use crate <a href="https://docs.rs/tera/latest/tera/" title="tera" target="_blank">tera</a>, which implements <a href="http://jinja.pocoo.org/" title="Jinja2" target="_blank">Jinja2</a> template syntax.

2. [Rust: learning actix-web middleware 01.](https://behainguyen.wordpress.com/2023/11/26/rust-learning-actix-web-middleware-01/)

The code version for the above post has been tagged with **v0.2.0**. It can be cloned with:

```
git clone -b v0.2.0 https://github.com/behai-nguyen/rust_web_01.git
```

We add request path extractor and MySQL database query calls to the official <code>SayHi</code> middleware example. The middleware creates a text message, attaches it to the request via extension. This text message contains the detail of the first matched record, if any found, otherwise a no record found message. A resource endpoint service handler then extracts this message, and returns it as HTML to the client.

3. [Rust: retrofit integration tests to an existing actix-web application.](https://behainguyen.wordpress.com/2023/12/31/rust-retrofit-integration-tests-to-an-existing-actix-web-application/)

The code version for the above post has been tagged with **v0.3.0**. It can be cloned with:

```
git clone -b v0.3.0 https://github.com/behai-nguyen/rust_web_01.git
```

We've previously built an <a href="https://docs.rs/actix-web/latest/actix_web/" title="actix-web" target="_blank">actix-web</a> “application”, which has five (5) public <code>POST</code> and <code>GET</code> routes. We didn't implement any test at all. We're now retrofitting proper integration tests for these existing 5 (five) public routes.

4. [Rust: adding actix-session and actix-identity to an existing actix-web application.](https://behainguyen.wordpress.com/2024/01/03/rust-adding-actix-session-and-actix-identity-to-an-existing-actix-web-application/)

The code version for the above post has been tagged with **v0.4.0**. It can be cloned with:

```
git clone -b v0.4.0 https://github.com/behai-nguyen/rust_web_01.git
```

I've been studying user authentication with the <a href="https://docs.rs/actix-web/latest/actix_web/" title="actix-web" target="_blank">actix-web</a> framework. It seems that a popular choice is to use crate <a href="https://docs.rs/actix-identity/latest/actix_identity/" title="Crate actix_identity" target="_blank">actix-identity</a>, which requires crate <a href="https://docs.rs/actix-session/latest/actix_session/" title="Crate actix_session" target="_blank">actix-session</a>. To add these two (2) crates, the code of the existing application must be refactored a little. We first look at code refactoring and integration. Then we briefly discuss the official examples given in the documentation of the 2 (two) mentioned crates.

5. [Rust: actix-web endpoints which accept both ``application/x-www-form-urlencoded`` and ``application/json`` content types.](https://behainguyen.wordpress.com/2024/01/14/rust-actix-web-endpoints-which-accept-both-application-x-www-form-urlencoded-and-application-json-content-types/)

```
git clone -b v0.5.0 https://github.com/behai-nguyen/rust_web_01.git
```

We're implementing a login process for our actix-web learning application. We undertake some general updates to get ready to support login. We then implement a new /api/login route, which supports both application/x-www-form-urlencoded and application/json content types. In this post, we only implement deserialising the submitted request data, then echo some response. We also add a login page via route /ui/login.

6. [Rust: simple actix-web email-password login and request authentication using middleware.](https://behainguyen.wordpress.com/2024/01/28/rust-simple-actix-web-email-password-login-and-request-authentication-using-middleware/)

```
git clone -b v0.6.0 https://github.com/behai-nguyen/rust_web_01.git
```

For our learning <a href="https://docs.rs/actix-web/latest/actix_web/" title="actix-web" target="_blank">actix-web</a> application, we are now adding two new features. ⓵ A simple email-password login with no session expiry. 
⓶ A new middleware that manages 
<a href="https://behainguyen.wordpress.com/2024/01/28/rust-simple-actix-web-email-password-login-and-request-authentication-using-middleware/#definition-request-auth"><code>request authentication</code></a> 
using an 
<a href="https://behainguyen.wordpress.com/2024/01/28/rust-simple-actix-web-email-password-login-and-request-authentication-using-middleware/#definition-access-token"><code>access token</code></a> 
“generated” by the login process. All 
<a href="https://behainguyen.wordpress.com/2024/01/28/rust-simple-actix-web-email-password-login-and-request-authentication-using-middleware/#issues-covered-existing-routes">five existing routes</a> 
are now protected by this middleware: they can only be accessed if the 
request has a valid 
<a href="https://behainguyen.wordpress.com/2024/01/28/rust-simple-actix-web-email-password-login-and-request-authentication-using-middleware/#definition-access-token"><code>access token</code></a>. 
With these two new features added, this application acts as both an 
<a href="https://behainguyen.wordpress.com/2024/01/28/rust-simple-actix-web-email-password-login-and-request-authentication-using-middleware/#definition-app-server"><code>application server</code></a> 
and an 
<a href="https://behainguyen.wordpress.com/2024/01/28/rust-simple-actix-web-email-password-login-and-request-authentication-using-middleware/#definition-api-server"><code>API-like server</code> 
or a <code>service</code></a>.

7. [Rust: actix-web get SSL/HTTPS for localhost.](https://behainguyen.wordpress.com/2024/02/10/rust-actix-web-get-ssl-https-for-localhost/)

```
git clone -b v0.7.0 https://github.com/behai-nguyen/rust_web_01.git
```

We are going to enable our <a href="https://docs.rs/actix-web/latest/actix_web/" title="actix-web" target="_blank">actix-web</a> learning application to run under <code>HTTPS</code>. As a result, we need to do some minor refactoring to existing integration tests. We also move and rename an existing module for better code organisation.

8. [Rust: actix-web CORS, Cookies and AJAX calls.](https://behainguyen.wordpress.com/2024/02/13/rust-actix-web-cors-cookies-and-ajax-calls/)

```
git clone -b v0.8.0 https://github.com/behai-nguyen/rust_web_01.git
```

Continuing with our <a href="https://docs.rs/actix-web/latest/actix_web/" title="actix-web" target="_blank">actix-web</a> learning application, we will discuss proper AJAX calls to ensure reliable functionality of CORS and session cookies. This also addresses <a href="https://behainguyen.wordpress.com/2024/01/28/rust-simple-actix-web-email-password-login-and-request-authentication-using-middleware/#some-current-issues" title="issue ❷ raised" target="_blank">issue ❷ raised</a> in a <a href="https://behainguyen.wordpress.com/2024/01/28/rust-simple-actix-web-email-password-login-and-request-authentication-using-middleware/" title="Rust: simple actix-web email-password login and request authentication using middleware." target="_blank">previous post</a>.

9. [Rust: actix-web global extractor error handlers.](https://behainguyen.wordpress.com/2024/02/16/rust-actix-web-global-extractor-error-handlers/)

```
git clone -b v0.9.0 https://github.com/behai-nguyen/rust_web_01.git
```

Continuing with our <a href="https://docs.rs/actix-web/latest/actix_web/" title="actix-web" target="_blank">actix-web</a> learning application, we implement global extractor error handlers for both <code>application/json</code> and <code>application/x-www-form-urlencoded</code> data. This enhances the robustness of the code. Subsequently, we refactor the login data extraction process to leverage the global extractor error handlers.

10. [Rust: actix-web JSON Web Token authentication.](https://behainguyen.wordpress.com/2024/02/26/rust-actix-web-json-web-token-authentication/)

```
git clone -b v0.10.0 https://github.com/behai-nguyen/rust_web_01.git
```

In the <a href="https://behainguyen.wordpress.com/2024/01/28/rust-simple-actix-web-email-password-login-and-request-authentication-using-middleware/" title="Rust: simple actix-web email-password login and request authentication using middleware." target="_blank">sixth</a> post of our <a href="https://docs.rs/actix-web/latest/actix_web/" title="actix-web" target="_blank">actix-web</a> learning application, we implemented a basic email-password login process with a placeholder for a <code>token</code>. In this post, we will implement a comprehensive JSON Web Token (JWT)-based authentication system. We will utilise the <a href="https://docs.rs/jsonwebtoken/latest/jsonwebtoken/index.html" title="jsonwebtoken" target="_blank">jsonwebtoken</a> crate, which we have <a href="https://behainguyen.wordpress.com/2023/11/20/rust-json-web-token-some-investigative-studies-on-crate-jsonwebtoken/" title="Rust: JSON Web Token -- some investigative studies on crate jsonwebtoken" target="_blank">previously studied</a>.

11. [Rust: Actix-web and Daily Logging](https://behainguyen.wordpress.com/2024/03/13/rust-actix-web-and-daily-logging/)

```
git clone -b v0.11.0 https://github.com/behai-nguyen/rust_web_01.git
```

Currently, our <a href="https://docs.rs/actix-web/latest/actix_web/" title="actix-web" target="_blank">actix-web</a> learning application simply prints debug information to the console using the <code>println!</code> macro. In this post, we will implement proper non-blocking daily logging to files. <code>Daily logging</code> entails rotating to a new log file each day. <code>Non-blocking</code> refers to having a dedicated thread for file writing operations. We will utilise the <a href="https://docs.rs/tracing/latest/tracing/index.html" title="tracing" target="_blank">tracing</a>, <a href="https://docs.rs/tracing-appender/latest/tracing_appender/index.html" title="tracing-appender" target="_blank">tracing-appender</a>, and <a href="https://docs.rs/tracing-subscriber/latest/tracing_subscriber/index.html" title="tracing-subscriber" target="_blank">tracing-subscriber</a> crates for our logging implementation.

12. [Rust: Actix-web Daily Logging -- Fix Local Offset, Apply Event Filtering](https://behainguyen.wordpress.com/2024/03/18/rust-actix-web-daily-logging-fix-local-offset-apply-event-filtering/)

```
git clone -b v0.12.0 https://github.com/behai-nguyen/rust_web_01.git
```

In the <a href="https://behainguyen.wordpress.com/2024/03/13/rust-actix-web-and-daily-logging/#project-layout" title="Rust: Actix-web and Daily Logging" target="_blank">last post</a> of our <a href="https://docs.rs/actix-web/latest/actix_web/" title="actix-web" target="_blank">actix-web</a> learning application, we identified two problems. First, there is an issue with calculating the UTC time offset on Ubuntu 22.10, as described in the section <a href="https://behainguyen.wordpress.com/2024/03/13/rust-actix-web-and-daily-logging/#utcoffset-linux-problem" title="💥 Issue with calculating UTC time offset on Ubuntu 22.10" target="_blank">💥 Issue with calculating UTC time offset on Ubuntu 22.10</a>. Secondly, loggings from other crates that match the logging configuration are also written onto log files, as mentioned in the <a href="https://behainguyen.wordpress.com/2024/03/13/rust-actix-web-and-daily-logging/#concluding-remarks" title="Concluding Remarks" target="_blank">Concluding Remarks</a> section. We should be able to configure what gets logged. We will address both of these issues in this post.

13. [Rust: Actix-web -- Async Functions as Middlewares](https://behainguyen.wordpress.com/2024/03/20/rust-actix-web-async-functions-as-middlewares/)

```
git clone -b v0.13.0 https://github.com/behai-nguyen/rust_web_01.git
```

In the <a href="https://behainguyen.wordpress.com/2024/02/26/rust-actix-web-json-web-token-authentication/" title="Rust: actix-web JSON Web Token authentication" target="_blank">tenth</a> post of our <a href="https://docs.rs/actix-web/latest/actix_web/" title="actix-web" target="_blank">actix-web</a> learning application, we added an ad hoc middleware. In this post, with the assistance of the <a href="https://docs.rs/actix-web-lab/latest/actix_web_lab/index.html" title="actix-web-lab" target="_blank">actix-web-lab</a> crate, we will refactor this ad hoc middleware into a standalone <code>async</code> function to enhance the overall code readability.

## On .env

I understand it should not be checked in. But this is only a development project, I checked it in for the shake of completeness.

## License
[ MIT license ](http://www.opensource.org/licenses/mit-license.php)
and the [ GPL license](http://www.gnu.org/licenses/gpl.html).
