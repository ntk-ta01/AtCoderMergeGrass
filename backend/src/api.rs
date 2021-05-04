use actix_http::{encoding::Decoder, Payload};
use actix_web::client::{Client, ClientResponse};
use actix_web::http::header;
use anyhow::{bail, Result};
use chrono::{Datelike, Local, NaiveDate, NaiveDateTime};
use serde::Deserialize;
use serde::Serialize;
use std::{
    collections::{HashMap, HashSet},
    time::Duration,
};

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

pub async fn get_atcoder_graph_data(user_id: &str, show_mode: ShowMode) -> Result<Vec<usize>> {
    if user_id.is_empty() {
        bail!("no input");
    }
    const ATCODER_API_URL: &str = "https://kenkoooo.com/atcoder/atcoder-api/results?user=";
    let client = Client::default();
    let response = client
        .get(format!("{}{}", ATCODER_API_URL, user_id))
        .header(header::ACCEPT_ENCODING, "gzip")
        .timeout(Duration::from_secs(20))
        .send()
        .await;

    let body = match response {
        Ok(mut response) => response.body().limit(2048 * 2048 * 126).await,
        Err(_) => {
            bail!("Probably an invalid username");
        }
    };

    // let submissions = response.unwrap().json::<Vec<Submission>>().await; // 一生Paylaod(overflow) 直せん
    let submissions: Vec<Submission> = serde_json::from_slice(&body.unwrap().to_vec())?;

    let (first_day, last_day, dates, date_to_idx) = create_dates_data().await;

    let mut counts = vec![0; dates.len()];
    const NINE_HOUR: i64 = 32400;

    match show_mode {
        ShowMode::Submissions => {
            for sub in submissions {
                let date = NaiveDateTime::from_timestamp(sub.epoch_second + NINE_HOUR, 0).date();
                if date < first_day || last_day < date {
                    continue;
                }
                let idx = date_to_idx[&date];
                counts[idx] += 1;
            }
        }
        ShowMode::AC => {
            for sub in submissions {
                let date = NaiveDateTime::from_timestamp(sub.epoch_second + NINE_HOUR, 0).date();
                if date < first_day || last_day < date || sub.result != "AC" {
                    continue;
                }
                let idx = date_to_idx[&date];
                counts[idx] += 1;
            }
        }
        ShowMode::UniqueAC => {
            // AC済みのsubだけに絞る
            // epoch_secondでソート
            // problem_idを覚えるHashSetを作る
            // AC時間の早い提出からcountしていく
            // countするときはcountするprobelm_idがまだHashSetにないことを確認
            // countしたらHashSetにproblem_idをいれる
            let mut submissions = submissions
                .into_iter()
                .filter(|sub| sub.result == "AC")
                .collect::<Vec<Submission>>();
            submissions.sort_by_key(|sub| sub.epoch_second);
            let mut counted_problems = HashSet::new();
            for sub in submissions {
                if !counted_problems.contains(&sub.problem_id) {
                    counted_problems.insert(sub.problem_id);
                    let date =
                        NaiveDateTime::from_timestamp(sub.epoch_second + NINE_HOUR, 0).date();
                    if date < first_day {
                        continue;
                    }
                    let idx = date_to_idx[&date];
                    counts[idx] += 1;
                }
            }
        }
    }
    Ok(counts)
}

async fn create_dates_data() -> (
    NaiveDate,
    NaiveDate,
    Vec<NaiveDate>,
    HashMap<NaiveDate, usize>,
) {
    const WEEKS: i64 = 53;
    const WEEKDAY: i64 = 7;

    let last_day = Local::today().naive_local();
    let mut next_sunday = last_day.succ();
    while next_sunday.weekday() != chrono::Weekday::Sun {
        next_sunday = next_sunday.succ();
    }
    let first_day = next_sunday - chrono::Duration::days(WEEKS * WEEKDAY);
    let mut day = first_day;
    let mut dates = vec![];
    let mut date_to_idx = HashMap::new();

    for i in 0..WEEKS * WEEKDAY {
        date_to_idx.insert(day, i as usize);
        dates.push(day);
        if day == last_day {
            break;
        }
        day = day.succ();
    }

    (first_day, last_day, dates, date_to_idx)
}
#[allow(clippy::upper_case_acronyms)]
#[derive(Deserialize, Debug)]
pub enum ShowMode {
    Submissions,
    AC,
    UniqueAC,
}

#[derive(Deserialize, Debug)]
pub struct AtCoderData {
    pub submissions: Vec<Submission>,
}

#[derive(Deserialize, Debug)]
pub struct Submission {
    id: i64,
    pub epoch_second: i64,
    pub problem_id: String,
    contest_id: String,
    user_id: String,
    language: String,
    point: f64,
    length: usize,
    pub result: String,
    execution_time: Option<i64>,
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
