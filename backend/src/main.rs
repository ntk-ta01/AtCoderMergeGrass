use actix_cors::Cors;
use actix_web::{
    client::{Client, Connector},
    cookie::{Cookie, SameSite},
    get,
    http::header,
    web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use openssl::ssl::{SslConnector, SslMethod};
use serde_derive::Deserialize;
use std::env;

mod api;
mod utils;

#[get("/")]
async fn welcome() -> String {
    "Welcome!".to_string()
}

#[get("/hello")]
async fn hello() -> String {
    "Hello!".to_string()
}

async fn hello_api() -> impl Responder {
    HttpResponse::Ok().body("Hello API!")
}

async fn get_token(web::Query(info): web::Query<Code>) -> HttpResponse {
    // 2. Users are redirected back to your site by GitHub
    let client_secret = &utils::get_env("CLIENT_SECRET");
    let client_id = &utils::get_env("CLIENT_ID");
    let builder = SslConnector::builder(SslMethod::tls()).unwrap();
    let client = Client::builder()
        .connector(Connector::new().ssl(builder.build()).finish())
        .finish();
    let res = client
        .post(format!(
            "https://github.com/login/oauth/access_token?client_id={}&client_secret={}&code={}",
            client_id, client_secret, info.code
        ))
        .send()
        .await;
    let bytes = res.unwrap().body().await.unwrap();
    let query = String::from_utf8(bytes.to_vec()).unwrap();
    let access_token = serde_qs::from_str::<AccessToken>(&query).unwrap();
    let cookie = Cookie::build("token", access_token.access_token)
        .path("/") // 必要
        .secure(true)
        .http_only(true)
        .same_site(SameSite::None)
        .finish();

    // 参考：https://github.com/kenkoooo/AtCoderProblems/blob/bb115ccebdad20afb3079197540a1ec3b48f9322/atcoder-problems-backend/src/server/auth.rs
    let redirect_url = "https://atcoder-merge-grass.herokuapp.com";
    HttpResponse::TemporaryRedirect()
        // .cookie(cookie)
        .header(header::SET_COOKIE, cookie.to_string())
        .header(header::LOCATION, redirect_url)
        .header(header::ACCESS_CONTROL_ALLOW_CREDENTIALS, "true")
        .finish()
}

#[get("/user")]
async fn get_user(req: HttpRequest) -> impl Responder {
    let cookie_string = utils::get_cookie_string_from_header(req);
    if let Some(s) = cookie_string {
        if let Some(token) = utils::get_cookie_value("token", s) {
            let user_id = api::get_user_id(&token).await;
            if let Ok(user_id) = user_id {
                return HttpResponse::Ok()
                    .header(header::ACCESS_CONTROL_ALLOW_CREDENTIALS, "true")
                    .body(format!("{{\"user_id\" : \"{}\"}}", user_id));
            }
        }
    }
    HttpResponse::InternalServerError()
        .header(header::ACCESS_CONTROL_ALLOW_CREDENTIALS, "true")
        .finish()
}

#[get("/data/github")]
async fn get_data_github(req: HttpRequest) -> impl Responder {
    let cookie_string = utils::get_cookie_string_from_header(req);
    if let Some(s) = cookie_string {
        if let Some(token) = utils::get_cookie_value("token", s) {
            let api_res = api::get_graph_data(&token).await;
            let weeks = api::parse_graph_response(api_res).await;
            if let Ok(weeks) = weeks {
                let response = HttpResponse::Ok()
                    .header(header::ACCESS_CONTROL_ALLOW_CREDENTIALS, "true")
                    .json(weeks);
                return response;
            }
        }
    }
    HttpResponse::InternalServerError()
        .header(header::ACCESS_CONTROL_ALLOW_CREDENTIALS, "true")
        .finish()
}

#[get("/data/atcoderproblems")]
async fn get_data_atcoderproblems(web::Query(info): web::Query<QueryAtCoder>) -> impl Responder {
    let values = api::get_atcoder_graph_data(&info.uid, info.show_mode).await;
    if let Ok(values) = values {
        let response = HttpResponse::Ok()
            .header(header::ACCESS_CONTROL_ALLOW_CREDENTIALS, "true")
            .json(values);
        return response;
    }
    HttpResponse::InternalServerError()
        .header(header::ACCESS_CONTROL_ALLOW_CREDENTIALS, "true")
        .finish()
}

#[derive(Debug, Deserialize)]
struct Code {
    code: String,
}
#[derive(Debug, Deserialize)]
struct AccessToken {
    access_token: String,
    scope: String,
    token_type: String,
}

#[derive(Debug, Deserialize)]
struct QueryAtCoder {
    uid: String,
    show_mode: api::ShowMode,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = env::var("HOST").expect("Host not set");
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");
    HttpServer::new(|| {
        let cors = Cors::default().allowed_origin("https://atcoder-merge-grass.herokuapp.com");
        App::new()
            .wrap(cors)
            .service(welcome)
            .service(hello)
            .service(get_data_github)
            .service(get_data_atcoderproblems)
            .service(get_user)
            .route("/api", web::get().to(hello_api))
            .route("/internal-api/authorize", web::get().to(get_token))
    })
    .bind((host, port))?
    .run()
    .await
}
