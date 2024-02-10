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


## On .env

I understand it should not be checked in. But this is only a development project, I checked it in for the shake of completeness.

## License
[ MIT license ](http://www.opensource.org/licenses/mit-license.php)
and the [ GPL license](http://www.gnu.org/licenses/gpl.html).
