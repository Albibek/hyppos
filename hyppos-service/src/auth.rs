use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures::future::{ok, Ready};

use actix_web::{web, HttpResponse, Responder};

use actix_service::{Service, Transform};
use actix_session::{Session, UserSession};
use actix_web::{
    dev::ServiceRequest, dev::ServiceResponse, http::header, http::StatusCode,
    Error as ActixWebError,
};
use log::info;
use oauth2::basic::BasicClient;
use oauth2::reqwest::http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl,
    Scope, TokenResponse, TokenUrl,
};

use serde::Deserialize;

use crate::github_types;
use crate::State;

pub static GITHUB_CLIENT_ID: &'static str = "e024d6957a492c88efdb";
pub static GITHUB_CLIENT_SECRET: &'static str = "2cbfe08ef31906c50b94e3cbe63847c95d64d06b";
pub static GITHUB_AUTH_URL: &'static str = "https://github.com/login/oauth/authorize";
pub static GITHUB_TOKEN_URL: &'static str = "https://github.com/login/oauth/access_token";
pub static AUTH_CALLBACK_URL: &'static str = "http://127.0.0.1:8000/auth/callback";

#[derive(Clone)]
pub(crate) struct AuthState {
    client: BasicClient,
    //api_base_url: String,
}

// https://github.com/login/oauth/authorize
pub(crate) fn configure() -> AuthState {
    let client = BasicClient::new(
        ClientId::new(GITHUB_CLIENT_ID.to_string()),
        Some(ClientSecret::new(GITHUB_CLIENT_SECRET.to_string())),
        AuthUrl::new(GITHUB_AUTH_URL.to_string()).expect("good auth url"),
        Some(TokenUrl::new(GITHUB_TOKEN_URL.to_string()).expect("good token URL")),
    )
    .set_redirect_url(
        RedirectUrl::new(AUTH_CALLBACK_URL.to_string()).expect("correct redirect URL"),
    );
    AuthState { client }
}

pub(crate) async fn index(session: Session) -> impl Responder {
    let user = session.get::<github_types::User>("user").unwrap();
    let link = if user.is_some() { "logout" } else { "login" };

    let html = format!(
        r#"<html>
        <head><title>authenticate with github</title></head>
        <body>
            {} <a href="/{}">{}</a>
        </body>
    </html>"#,
        user.unwrap_or(github_types::User {
            login: "anonymous".into(),
            id: 0
        })
        .login,
        link,
        link
    );

    HttpResponse::Ok().body(html)
}

pub(crate) fn login(data: web::Data<State>) -> HttpResponse {
    // Create a PKCE code verifier and SHA-256 encode it as a code challenge.
    let (pkce_code_challenge, _pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();
    // Generate the authorization URL to which we'll redirect the user.
    let (auth_url, _csrf_token) = &data
        .auth
        .client
        .authorize_url(CsrfToken::new_random)
        // Set the desired scopes.
        .add_scope(Scope::new("read_user".to_string()))
        // Set the PKCE code challenge.
        .set_pkce_challenge(pkce_code_challenge)
        .url();

    HttpResponse::Found()
        .header(header::LOCATION, auth_url.to_string())
        .finish()
}

pub(crate) fn logout(session: Session) -> HttpResponse {
    session.remove("token");
    HttpResponse::Found()
        .header(header::LOCATION, "/".to_string())
        .finish()
}

#[derive(Deserialize)]
pub(crate) struct AuthRequest {
    code: String,
    state: String,
}

pub(crate) async fn callback(
    session: Session,
    state: web::Data<State>,
    params: web::Query<AuthRequest>,
) -> HttpResponse {
    let code = AuthorizationCode::new(params.code.clone());
    let _state = CsrfToken::new(params.state.clone());

    // Exchange the code with a token.
    let token = &state.auth.client.exchange_code(code).request(http_client);
    let token = match token {
        Ok(token) => token,
        Err(e) => {
            return HttpResponse::BadRequest().body(format!("{:?}", e));
        }
    };

    let token = token.access_token().secret();
    info!("access token: {:?}", token);
    session
        .set(
            "token",
            serde_json::to_string(token).expect("serializing token"),
        )
        .expect("setting session field");

    let user: github_types::User = state.github.for_token(token).get_user().await.unwrap();
    session.set("user", user).expect("setting user data");

    let html = format!(
        r#"<html>
        <head><title>OAuth2 Test</title></head>
        <body>
            Github user info:
            <pre>{:?}</pre>
            <a href="/">Home</a>
        </body>
    </html>"#,
        serde_json::to_string(token)
    );
    HttpResponse::Ok().body(html)
}

pub(crate) struct AuthCheck;

impl<S, B> Transform<S> for AuthCheck
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = ActixWebError>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = ActixWebError;
    type InitError = ();
    type Transform = AuthCheckMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthCheckMiddleware { service })
    }
}

pub(crate) struct AuthCheckMiddleware<S> {
    service: S,
}

impl<S, B> Service for AuthCheckMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = ActixWebError>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = ActixWebError;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let session = req.get_session();
        let has_token = session.get::<String>("token").unwrap_or(None).is_some();
        if has_token
            || req.uri().path().starts_with("/auth")
            || req.uri().path().starts_with("/public")
            || req.uri().path() == "/"
            || true
        {
            info!("user authorized");
            let fut = self.service.call(req);

            Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            })
        } else {
            info!("redirecting unauthorized");
            let fut = self.service.call(req);
            Box::pin(async move {
                let mut res = fut.await?;

                let resp = res.response_mut();
                resp.take_body();
                *resp.status_mut() = StatusCode::FORBIDDEN;

                Ok(res)
            })
        }
    }
}
