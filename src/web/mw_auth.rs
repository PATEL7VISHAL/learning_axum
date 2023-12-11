use axum::body::Body;
use axum::extract::{FromRequestParts, State};
use axum::http::request::Parts;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use axum::{async_trait, RequestPartsExt};
use lazy_regex::regex_captures;
use tower_cookies::{Cookie, Cookies};

use crate::ctx::Ctx;
use crate::error::{Error, Result};
use crate::model::ModelController;
use crate::web::AUTH_TOKEN;

pub async fn mw_require_auth(
    // ctx: Ctx,  //NOTE: Buy using only Ctx it not call the mw_require_auth but it create the ctx
    ctx: Result<Ctx>,
    req: Request<Body>,
    next: Next,
) -> Result<Response> {
    println!("->> {:<25} - mw_require_auth", "MIDDLEWARE");
    ctx?;
    Ok(next.run(req).await)
}

/// this middileware  compute the ctx and which can be later use in Ctx::from_request_parts
/// function to void duplicate operations because every the route and middleware takes ctx as args
/// then the `Ctx` get from the `from_request_parts` so here we kind of using catching mean create
/// ctx by this function and and cach it and use `Ctx::from_request_parts` use catch
pub async fn mw_ctx_resolver(
    _mc: State<ModelController>,
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response> {
    println!("->> {:<25} - mw_ctx_resolver", "MIDDLEWARE");

    // User the cookes extractor
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());
    // Compute Result<Ctx>
    let result_ctx = match auth_token
        .ok_or(Error::AuthFailNoAuthTokenCookie)
        .and_then(parse_token)
    {
        Ok((user_id, _exp, _sign)) => {
            //TODO: Token components validations.
            Ok(Ctx::new(user_id))
        }
        Err(e) => Err(e),
    };

    // Remove the cookies if something went rwont other then NoAuthTokenCookie
    if result_ctx.is_err() && !matches!(result_ctx, Err(Error::AuthFailNoAuthTokenCookie)) {
        cookies.remove(Cookie::from(AUTH_TOKEN));
    }

    // Store the ctx_result in the request extenstion.
    req.extensions_mut().insert(result_ctx); //NOTE: it's may be kind of the mapping but here
                                             //the key is value type inplictly is it reuqire to be
                                             //unique by type

    Ok(next.run(req).await)
}

// region:      Ctx Extractor
#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        println!("->> {:<25} - Ctx", "EXTRACTOR");
        parts
            .extensions
            .get::<Result<Ctx>>()
            .ok_or(Error::AuthFailCtxNotInRequestExt)?
            .clone()
    }
}
// endregion:      Ctx Extractor

/// Parse a token of format `[user]-[expiration]-[signature]`
/// Returns (user_id, expiration, signature)
fn parse_token(token: String) -> Result<(u64, String, String)> {
    let (_whole, user_id, exp, sign) = regex_captures!(r#"^user-(\d+)\.(.+)\.(.+)"#, &token)
        .ok_or(Error::AuthFailTokenWrongFormat)?;
    let user_id: u64 = user_id
        .parse()
        .map_err(|err| Error::AuthFailTokenWrongFormat)?;

    Ok((user_id, exp.to_string(), sign.to_string()))
}
