# rust_web_01 / learn-actix-web

We write a Rust web application using a MySQL database. We use the already familiar crate <a href="https://docs.rs/sqlx/latest/sqlx" title="Crate sqlx" target="_blank">sqlx</a> for database works. The web framework we're using is crate <a href="https://actix.rs/docs/" title="actix-web" target="_blank">actix-web</a>. For Cross-Origin Resource Sharing (CORS) controls, we use crate <a href="https://docs.rs/actix-cors/latest/actix_cors/" title="actix-cors" target="_blank">actix-cors</a>. For HTML template processing, we use crate <a href="https://docs.rs/tera/latest/tera/" title="tera" target="_blank">tera</a>, which implements <a href="http://jinja.pocoo.org/" title="Jinja2" target="_blank">Jinja2</a> template syntax.

## Related post(s)

* [Rust web application: MySQL server, sqlx, actix-web and tera.](https://behainguyen.wordpress.com/2023/10/18/rust-web-application-mysql-server-sqlx-actix-web-and-tera/)

The code version for the above post has been tagged with **v0.1.0**. It can be cloned with:
  
```
git clone -b v0.1.0 https://github.com/behai-nguyen/rust_web_01.git
```

* [Rust: learning actix-web middleware 01.](https://behainguyen.wordpress.com/2023/11/26/rust-learning-actix-web-middleware-01/)

The code version for the above post has been tagged with **v0.2.0**. It can be cloned with:

```
git clone -b v0.2.0 https://github.com/behai-nguyen/rust_web_01.git
```

* [Rust: retrofit integration tests to an existing actix-web application.](https://behainguyen.wordpress.com/2023/12/31/rust-retrofit-integration-tests-to-an-existing-actix-web-application/)

The code version for the above post has been tagged with **v0.3.0**. It can be cloned with:

```
git clone -b v0.3.0 https://github.com/behai-nguyen/rust_web_01.git
```

## On .env

I understand it should not be checked in. But this is only a development project, I checked it in for the shake of completeness.

## License
[ MIT license ](http://www.opensource.org/licenses/mit-license.php)
and the [ GPL license](http://www.gnu.org/licenses/gpl.html).
