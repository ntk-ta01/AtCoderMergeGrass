use actix_http::{encoding::Decoder, Payload};
use actix_web::client::{Client, ClientResponse};
use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;

pub async fn get_graph_data(access_token: &str) -> ClientResponse<Decoder<Payload>> {
    let query = r#"{"query" : "query { viewer { contributionsCollection { contributionCalendar { weeks { firstDay contributionDays { contributionCount } } } } } }"}"#;
    let client = Client::default();
    let response = client
        .post("https://api.github.com/graphql")
        .header("Content-type", "application/json")
        .header("User-Agent", "actix-web/3.0")
        .bearer_auth(access_token)
        .send_body(query)
        .await
        .unwrap();
    response
}

pub async fn parse_graph_response(mut res: ClientResponse<Decoder<Payload>>) -> Result<Vec<Week>> {
    let data_github: Data =
        serde_json::from_str(&String::from_utf8(res.body().await.unwrap().to_vec()).unwrap())?;
    Ok(data_github
        .data
        .viewer
        .contributionsCollection
        .contributionCalendar
        .weeks)
}

pub async fn get_user_id(access_token: &str) -> Result<String> {
    let query = r#"{"query": "query { viewer { login }}""#;
    let client = Client::new();
    let mut response = client
        .post("https://api.github.com/graphql")
        .header("Content-type", "application/json")
        .header("User-Agent", "actix-web/3.0")
        .bearer_auth(access_token)
        .send_body(query)
        .await
        .unwrap();
    let user_id: UserID =
        serde_json::from_str(&String::from_utf8(response.body().await.unwrap().to_vec()).unwrap())?;
    Ok(user_id.data.viewer.login)
}

#[derive(Deserialize, Debug)]
#[allow(clippy::upper_case_acronyms)]
pub struct UserID {
    data: UserViewer,
}

#[derive(Deserialize, Debug)]
struct UserViewer {
    viewer: Login,
}

#[derive(Deserialize, Debug)]
struct Login {
    login: String,
}

#[derive(Deserialize, Debug)]
pub struct Data {
    data: Viewer,
}
#[derive(Deserialize, Debug)]
struct Viewer {
    viewer: ContributionCollection,
}
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct ContributionCollection {
    contributionsCollection: ContributionCalendar,
}
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct ContributionCalendar {
    contributionCalendar: Weeks,
}
#[derive(Deserialize, Debug, Serialize)]
struct Weeks {
    weeks: Vec<Week>,
}
#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Serialize)]
pub struct Week {
    pub firstDay: String,
    pub contributionDays: Vec<ContributionCount>,
}
#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Serialize)]
pub struct ContributionCount {
    pub contributionCount: i32,
}
