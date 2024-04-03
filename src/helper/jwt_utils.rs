/* Date Created: 19/02/2024. */

//! JSON Web Token utilities using [jsonwebtoken](https://docs.rs/jsonwebtoken/latest/jsonwebtoken/index.html) crate.
//! Utilities in this module are implemented based on discussion in this [example 2](https://behai-nguyen.github.io/2023/11/19/rust-14-jsonwebtoken.html#the-second-example)
//! of the above post.
//! 
//! See also [Rust: JSON Web Token -- some investigative studies on crate jwt-simple](https://behai-nguyen.github.io/2023/11/17/rust-13-jwt-simple.html).
//! 
use serde::{Deserialize, Serialize};
use jsonwebtoken::{get_current_timestamp, encode, Algorithm, Header, 
    EncodingKey, decode, DecodingKey, Validation, errors::ErrorKind
};

use uuid::Uuid;

use actix_web::http::StatusCode;

use crate::bh_libs::api_status::ApiStatus;
use crate::helper::messages::{ 
    TOKEN_INVALID_MSG,
    TOKEN_EXPIRED_MSG,
    TOKEN_OTHER_ERR_MSG,
};

use crate::helper::constants::BEARER_TOKEN;

/// This implementation JSON Web Token payload.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JWTPayload {
    /// Custom field. The logged in user email. 
    email: String,
    /// Custom field. Uuid V4 unique Id for this authenticated session.
    session_id: String,
    /// Standard field. Its value stays fixed.
    iat: u64, 
    /// Standard field. A required field. For this implementation, its value gets 
    /// updated to a new expiry everytime a user makes a request.
    exp: u64, 
    /// Custom field. The last time this token was used for authentication. It's
    /// seconds since epoch.
    last_active: u64,
}

/// See [Rust: seconds since epoch -- “1970-01-01 00:00:00 UTC”](https://behai-nguyen.github.io/2023/11/12/rust-12-epoch-time.html) 
/// for discussion on seconds since epoch.
/// 
/// Without using [get_current_timestamp()](https://docs.rs/jsonwebtoken/latest/jsonwebtoken/fn.get_current_timestamp.html),
/// the equivalent implementation should be:
///
/// ```
/// use std::time::{SystemTime, UNIX_EPOCH};
/// use time::OffsetDateTime;
///
/// fn seconds_since_epoch() -> u64 {
///     let offset_dt = OffsetDateTime::now_utc();
/// 
///     // This call will never fail!
///     match SystemTime::from(offset_dt).duration_since(UNIX_EPOCH) {
///         Ok(n) => n.as_secs(),
///         Err(_) => unreachable!(), // ! Coerced to u64.
///     }
/// }
/// ```
fn seconds_since_epoch() -> u64 {
    get_current_timestamp()
}

impl JWTPayload {
    /// Creates and returns a [`JWTPayload`] instance.
    /// 
    /// # Arguments
    ///
    /// * `email` - email of the logged in user.
    /// 
    /// * `secs_valid_for` - the duration in seconds in which this token is 
    ///   valid for.
    ///
    /// # Return
    ///
    /// * [`JWTPayload`] instance.
    ///
    pub fn new(email: &str, secs_valid_for: u64) -> Self {
        let iat = seconds_since_epoch();

        Self {
            email: String::from(email),
            session_id: Uuid::new_v4().to_string(),
            iat,
            exp: iat + secs_valid_for,
            last_active: iat,
        }
    }

    /// Updates a [`JWTPayload`] instance expiry date and last active values.
    /// 
    /// # Arguments
    ///
    ///   * `seconds` - the number of seconds to move the expiry date to since
    ///      epoch.
    ///
    /// # Return 
    ///
    /// * [`JWTPayload`] instance with expiry date and last active values updated.
    ///
    pub fn update_expiry_secs(mut self, seconds: u64) -> Self {
        let sse = seconds_since_epoch();

        self.exp = sse + seconds;
        self.last_active = sse;
        self
    }

    /// Updates a [`JWTPayload`] instance expiry date and last active values.
    /// 
    /// # Arguments
    ///
    ///   * `mins` - the number of minutes to move the expiry date to since
    ///      epoch.
    ///
    /// # Return 
    ///
    /// * [`JWTPayload`] instance with expiry date and last active values updated.
    ///
    pub fn update_expiry_mins(mut self, mins: u64) -> Self {
        let sse = seconds_since_epoch();

        self.exp = sse + (mins * 60);
        self.last_active = sse;
        self
    }

    /// Updates a [`JWTPayload`] instance expiry date and last active values.
    /// 
    /// # Arguments
    ///
    ///   * `hours` - the number of hours to move the expiry date to since
    ///      epoch.
    ///
    /// # Return 
    ///
    /// * [`JWTPayload`] instance with expiry date and last active values updated.
    ///
    pub fn update_expiry_hours(mut self, hours: u64) -> Self {
        let sse = seconds_since_epoch();

        self.exp = sse + (hours * 60 * 60);
        self.last_active = sse;
        self
    }

    /// Gets a [`JWTPayload`] instance email.
    /// 
    /// # Return 
    ///
    /// * [`JWTPayload`] instance email.
    ///
    pub fn email(&self) -> String {
        self.email.clone()
    }

    /// Gets a [`JWTPayload`] instance session_id.
    /// 
    /// # Return 
    ///
    /// * [`JWTPayload`] instance session_id.
    ///
    pub fn session_id(&self) -> String {
        self.session_id.clone()
    }    

    /// Gets a [`JWTPayload`] instance issued at.
    /// 
    /// # Return 
    ///
    /// * [`JWTPayload`] instance issued at.
    ///
    pub fn issued_at(&self) -> u64 {
        self.iat
    }

    /// Gets a [`JWTPayload`] instance expiry.
    /// 
    /// # Return 
    ///
    /// * [`JWTPayload`] instance expiry.
    ///
    pub fn expiry(&self) -> u64 {
        self.exp
    }

    /// Gets a [`JWTPayload`] instance last active.
    /// 
    /// # Return 
    ///
    /// * [`JWTPayload`] instance last active.
    ///
    pub fn last_active(&self) -> u64 {
        self.last_active
    }
}

/// Create a [jsonwebtoken](https://docs.rs/jsonwebtoken/latest/jsonwebtoken/index.html) token.
/// For detail discussion, see [Rust: JSON Web Token -- some investigative studies on crate jsonwebtoken](https://behai-nguyen.github.io/2023/11/19/rust-14-jsonwebtoken.html).
///
/// # Arguments
/// 
/// * `email` - email of the logged in user.
///
/// * `secret_key` - [jsonwebtoken](https://docs.rs/jsonwebtoken/latest/jsonwebtoken/index.html) 
///    secret key used to encode the token.
///
/// * `secs_valid_for` - the duration in seconds in which this token is valid for.
/// 
/// # Return
/// 
/// * A JSON Web Token.
/// 
pub fn make_token(
    email: &str,
    secret_key: &[u8],
    secs_valid_for: u64
) -> String {
    let payload = JWTPayload::new(email, secs_valid_for);

    // This will create a JWT using HS256 as algorithm
    encode(&Header::default(), 
        &payload, 
        &EncodingKey::from_secret(secret_key),
    ).unwrap()
}

/// Create a [jsonwebtoken](https://docs.rs/jsonwebtoken/latest/jsonwebtoken/index.html) token.
/// For detail discussion, see [Rust: JSON Web Token -- some investigative studies on crate jsonwebtoken](https://behai-nguyen.github.io/2023/11/19/rust-14-jsonwebtoken.html).
///
/// # Arguments
/// 
/// * `payload` - a valid [`JWTPayload`] instance which is the token payload.
///
/// * `secret_key` - [jsonwebtoken](https://docs.rs/jsonwebtoken/latest/jsonwebtoken/index.html) 
///     secret key used to encode the token.
///
/// # Return
/// 
/// * A JSON Web Token.
/// 
pub fn make_token_from_payload(
    payload: &JWTPayload,
    secret_key: &[u8],
) -> String {
    // This will create a JWT using HS256 as algorithm
    encode(&Header::default(), 
        &payload, 
        &EncodingKey::from_secret(secret_key),
    ).unwrap()
}

/// Prepends [BEARER_TOKEN](`crate::helper::constants::BEARER_TOKEN`), i.e., ``Bearer.``,
/// to an existing JSON Web Token.
/// 
/// # Reference
///
/// * <https://datatracker.ietf.org/doc/html/rfc6750>, section 
///   [2.1.  Authorization Request Header Field](https://datatracker.ietf.org/doc/html/rfc6750#page-5).
///
/// # Arguments
///
/// * `token` - a (valid) JSON Web Token.
///
/// # Return
///
///    * ``Bearer.`` + token
/// 
pub fn make_bearer_token(token: &str) -> String {
	BEARER_TOKEN.to_owned() + token
}

/// Decodes a [jsonwebtoken](https://docs.rs/jsonwebtoken/latest/jsonwebtoken/index.html) token.
/// On successful, returns token payload [`JWTPayload`]. On failure, returns an instance of
/// [ApiStatus](`crate::bh_libs::api_status::ApiStatus`).
///
/// # Arguments
/// 
/// * `token` - a JSON Web Token.
///
/// * `secret_key` - [jsonwebtoken](https://docs.rs/jsonwebtoken/latest/jsonwebtoken/index.html) 
///     secret key used to encode the token.
///
/// # Return
/// 
/// * On successful, token payload [`JWTPayload`].
///
/// * On failure, an instance of [ApiStatus](`crate::bh_libs::api_status::ApiStatus`), field 
///   ``code`` set to [UNAUTHORIZED](https://docs.rs/actix-web/latest/actix_web/http/struct.StatusCode.html#associatedconstant.UNAUTHORIZED).
///   Detects two specific [jsonwebtoken](https://docs.rs/jsonwebtoken/latest/jsonwebtoken/index.html) errors
///   [InvalidToken](https://docs.rs/jsonwebtoken/latest/jsonwebtoken/errors/enum.ErrorKind.html#variant.InvalidToken)
///   and [ExpiredSignature](https://docs.rs/jsonwebtoken/latest/jsonwebtoken/errors/enum.ErrorKind.html#variant.ExpiredSignature).
///   All other errors are handled generically.
/// 
pub fn decode_token(
    token: &str,
    secret_key: &[u8],
    validate_exp: Option<bool>,
) -> Result<JWTPayload, ApiStatus> {
    let mut validation = Validation::new(Algorithm::HS256);
    // For the shake of simplicity, 0 would make leeway not having any effect
    // on expiration calculations.
    validation.leeway = 0;
    validation.validate_exp = validate_exp.unwrap_or(true);

    match decode::<JWTPayload>(token, 
        &DecodingKey::from_secret(secret_key), &validation) {
            Ok(x) => Ok(x.claims),

            Err(err) => match *err.kind() {
                ErrorKind::InvalidToken => Err(ApiStatus::new(StatusCode::UNAUTHORIZED.as_u16())
                    .set_message(TOKEN_INVALID_MSG)
                ),
                ErrorKind::ExpiredSignature => Err(ApiStatus::new(StatusCode::UNAUTHORIZED.as_u16())
                    .set_message(TOKEN_EXPIRED_MSG)
                ),
                _ => Err(ApiStatus::new(StatusCode::UNAUTHORIZED.as_u16()).set_message(TOKEN_OTHER_ERR_MSG)),
            },
    }
}

/// Decode a token with a prepended [BEARER_TOKEN](`crate::helper::constants::BEARER_TOKEN`), 
/// i.e., ``Bearer.``, prefix. For example,
/// ``Bearer.eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.e ... 0.5t1Gayvk74defCHrHkjJP-mFsNgFdY6yka9j6NZOm3g``.
/// 
/// # Reference
///
/// * <https://datatracker.ietf.org/doc/html/rfc6750>, section 
///   [2.1.  Authorization Request Header Field](https://datatracker.ietf.org/doc/html/rfc6750#page-5).
///
/// # Arguments
///
/// * `token` - a (valid) JSON Web Token with prefix ``Bearer.``.
///
/// * `secret_key` - [jsonwebtoken](https://docs.rs/jsonwebtoken/latest/jsonwebtoken/index.html) 
///    secret key used to encode the token.
///
/// # How It Works and Return Value
///
/// * Remove prefix ``Bearer.`` from token.
///
///  * Calls [decode_token()](`decode_token`) to do the work.
///
pub fn decode_bearer_token(
    token: &str,
    secret_key: &[u8],
    validate_exp: Option<bool>,
) -> Result<JWTPayload, ApiStatus> {
    decode_token(&token.split_at(7).1.to_string(), secret_key, validate_exp)
}

/// To run these tests below:
/// 
///    * cargo test helper::jwt_utils::tests
/// 
/// To run a specific test method: 
///
///    * cargo test helper::jwt_utils::tests::test_update_expiry_secs -- --exact 
///    * cargo test helper::jwt_utils::tests::test_update_expiry_mins -- --exact
///    * cargo test helper::jwt_utils::tests::test_update_expiry_hours -- --exact
///    * cargo test helper::jwt_utils::tests::test_make_token -- --exact
///    * cargo test helper::jwt_utils::tests::test_make_token_from_payload -- --exact
///    * cargo test helper::jwt_utils::tests::test_decode_token_valid -- --exact
///    * cargo test helper::jwt_utils::tests::test_decode_bearer_token_valid -- --exact
///    * cargo test helper::jwt_utils::tests::test_decode_token_expired -- --exact
///    * cargo test helper::jwt_utils::tests::test_decode_token_invalid -- --exact
#[cfg(test)]
mod tests {
    use std;
    use super::*;
    use dotenv::dotenv;
    use crate::config::Config;

    /// Verify valid syntax Uuid V4.
    /// 
    /// # Arguments
    /// 
    /// * `session_id` - Uuid V4 session Id.
    ///    It should look like: ``dde5c4a9-eed4-4273-a160-150204d5e521``,
    ///    ``67e55044-10b1-426f-9247-bb680e5fe0c8``.
    fn verify_session_id(session_id: String) {
        assert_eq!(session_id.len(), 36);

        let comps = session_id.split("-").collect::<Vec<&str>>();
        assert_eq!(comps.len(), 5);
        assert_eq!(comps[0].len(), 8);
        assert_eq!(comps[1].len(), 4);
        assert_eq!(comps[2].len(), 4);
        assert_eq!(comps[3].len(), 4);
        assert_eq!(comps[4].len(), 12);        
    }

    #[test]
    fn test_update_expiry_secs() {
        let mut jwt_payload = JWTPayload::new("behai_nguyen@hotmail.com", 45);

        // Wait for two seconds.
        let sleep_time = std::time::Duration::from_secs(2);
        std::thread::sleep(sleep_time);

        let sec_since_epoch = seconds_since_epoch();

        jwt_payload = jwt_payload.update_expiry_secs(30);

        assert_eq!((jwt_payload.expiry() - sec_since_epoch) <= 30, true);
        assert_eq!(jwt_payload.last_active() >= sec_since_epoch, true);
    }

    #[test]
    fn test_update_expiry_mins() {
        let mut jwt_payload = JWTPayload::new("behai_nguyen@hotmail.com", 45);

        // Wait for two seconds.
        let sleep_time = std::time::Duration::from_secs(2);
        std::thread::sleep(sleep_time);

        let sec_since_epoch = seconds_since_epoch();

        jwt_payload = jwt_payload.update_expiry_mins(1);

        assert_eq!((jwt_payload.expiry() - sec_since_epoch) <= 60, true);
        assert_eq!(jwt_payload.last_active() >= sec_since_epoch, true);
    }

    #[test]
    fn test_update_expiry_hours() {
        let mut jwt_payload = JWTPayload::new("behai_nguyen@hotmail.com", 45);

        // Wait for two seconds.
        let sleep_time = std::time::Duration::from_secs(2);
        std::thread::sleep(sleep_time);

        let sec_since_epoch = seconds_since_epoch();

        jwt_payload = jwt_payload.update_expiry_hours(1);

        assert_eq!((jwt_payload.expiry() - sec_since_epoch) <= 3600, true);
        assert_eq!(jwt_payload.last_active() >= sec_since_epoch, true);
    }

    #[test]
    fn test_make_token() {
        dotenv().ok();
        let config = Config::init();

        let email = "behai_nguyen@hotmail.com";

        let token = make_token(email, 
            config.jwt_secret_key.as_ref(), config.jwt_mins_valid_for * 60);
        assert_eq!(token.len() > 0, true);

        let jwt_payload = match decode::<JWTPayload>(&token, 
            &DecodingKey::from_secret(config.jwt_secret_key.as_ref()), 
            &Validation::new(Algorithm::HS256)) {
                Ok(x) => x.claims,
                Err(_) => panic!("Token decoded failed."),
        };

        assert_eq!(jwt_payload.email(), email);

        verify_session_id(jwt_payload.session_id().clone());
    }

    #[test]
    fn test_make_token_from_payload() {
        dotenv().ok();
        let config = Config::init();

        let email = "behai_nguyen@hotmail.com";

        let jwt_payload = JWTPayload::new(email, 45);

        let token = make_token_from_payload(&jwt_payload, config.jwt_secret_key.as_ref());
        assert_eq!(token.len() > 0, true);
        
        let jwt_pay_load1 = match decode::<JWTPayload>(&token, 
            &DecodingKey::from_secret(config.jwt_secret_key.as_ref()), 
            &Validation::new(Algorithm::HS256)) {
                Ok(x) => x.claims,
                Err(_) => panic!("Token decoded failed."),
        };

        assert_eq!(jwt_pay_load1.email(), email);

        verify_session_id(jwt_payload.session_id().clone());
    }

    #[test]
    fn test_decode_token_valid() {
        dotenv().ok();
        let config = Config::init();

        let email = "behai_nguyen@hotmail.com";

        let token = make_token(email, config.jwt_secret_key.as_ref(), 5);
        assert_eq!(token.len() > 0, true);

        let res = decode_token(&token, config.jwt_secret_key.as_ref(), None);
        // Token should be decoded successfully.
        assert_eq!(res.is_ok(), true);
        let jwt_payload = res.unwrap();

        assert_eq!(jwt_payload.email(), email);

        verify_session_id(jwt_payload.session_id().clone());
    }

    #[test]
    fn test_decode_bearer_token_valid() {
        dotenv().ok();
        let config = Config::init();

        let email = "behai_nguyen@hotmail.com";

        let token = make_bearer_token( &make_token(email, config.jwt_secret_key.as_ref(), 5) );
        assert_eq!(token.len() > 0, true);

        let res = decode_bearer_token(&token, config.jwt_secret_key.as_ref(), None);
        // Token should be decoded successfully.
        assert_eq!(res.is_ok(), true);
        let jwt_payload = res.unwrap();

        assert_eq!(jwt_payload.email(), email);
        
        verify_session_id(jwt_payload.session_id().clone());
    }

    #[test]
    fn test_decode_token_expired() {
        dotenv().ok();
        let config = Config::init();

        let email = "behai_nguyen@hotmail.com";

        let token = make_token(email, config.jwt_secret_key.as_ref(), 5);
        assert_eq!(token.len() > 0, true);

        // Wait until the token expired.
        let sleep_time = std::time::Duration::from_secs(7);
        std::thread::sleep(sleep_time);

        let res = decode_token(&token, config.jwt_secret_key.as_ref(), None);

        // Token decoded results in error.
        assert_eq!(res.is_err(), true);

        let api_status = res.err().unwrap();
        assert_eq!(api_status.get_code(), StatusCode::UNAUTHORIZED.as_u16());
        assert_eq!(api_status.get_message().unwrap(), TOKEN_EXPIRED_MSG);
    }    

    #[test]
    fn test_decode_token_invalid() {
        dotenv().ok();
        let config = Config::init();

        let token = "behai_nguyen@hotmail.com";

        let res = decode_token(&token, config.jwt_secret_key.as_ref(), None);

        // Token decoded results in error.
        assert_eq!(res.is_err(), true);

        let api_status = res.err().unwrap();
        assert_eq!(api_status.get_code(), StatusCode::UNAUTHORIZED.as_u16());
        assert_eq!(api_status.get_message().unwrap(), TOKEN_INVALID_MSG);
    }
}
