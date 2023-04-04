use crate::config;
use aws_sdk_dynamodb::{
    model::{AttributeValue, KeysAndAttributes},
    Client, Error,
};
use chrono::prelude::*;
use dotenv::dotenv;
use reqwest::{self};
use std::collections::HashMap;
use url::Url;

async fn connect() -> Result<Client, Error> {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);
    Ok(client)
}

#[derive(Debug)]
struct UserStatisticsQuery<'a> {
    partition_key: &'a str,
    begin_date: String,
    end_date: String,
}

impl<'a> UserStatisticsQuery<'a> {
    fn new(partition_key: &'a str, begin_date: Option<&'a str>, end_date: Option<&'a str>) -> Self {
        let begin_date = if let Some(date) = begin_date {
            String::from(date)
        } else {
            "00000000000000".to_string() //getting all rows as far back as possible. this is in the date format YYYYMMddHHmmss
        };
        let end_date = if let Some(date) = end_date {
            String::from(date)
        } else {
            Utc::now().naive_local().format("%Y%m%d%H%M%S").to_string()
        };
        Self {
            partition_key,
            begin_date,
            end_date,
        }
    }
}

pub async fn users() -> Result<(), Error> {
    let client = connect().await?;
    let req = client.list_tables().limit(10);
    let resp = req.send().await?;
    println!("Current DynamoDB tables: {:?}", resp.table_names);
    Ok(())
}

pub async fn get_user(user_id: &str) -> Result<HashMap<String, AttributeValue>, Error> {
    let client = connect().await?;
    let result = client
        .get_item()
        .table_name("Users")
        .key("id", AttributeValue::S(user_id.to_owned()))
        .send()
        .await?;
    let result = result.item.unwrap();
    Ok(result)
}

pub async fn get_user_teams(
    team_ids: &Vec<String>,
) -> Result<HashMap<String, Vec<HashMap<String, AttributeValue>>>, Error> {
    let client = connect().await?;
    //its kind of weird to make this mutable
    let teams = get_teams_and_attributes(team_ids);
    println!("teams returned from map function: {:?}", teams);
    let mut result = client
        .batch_get_item()
        .request_items("Teams", teams)
        .send()
        .await?;

    Ok(result.responses.take().unwrap())
}

fn get_teams_and_attributes(team_ids: &Vec<String>) -> KeysAndAttributes {
    let keys: Vec<HashMap<String, AttributeValue>> = team_ids
        .iter()
        .map(|id| {
            std::iter::once((String::from("id"), AttributeValue::S(id.to_string()))).collect()
        })
        .collect();

    //.map(|id| ("id", AttributeValue::S(id)));
    KeysAndAttributes::builder()
        .set_keys(if keys.is_empty() { None } else { Some(keys) })
        .build()
}

pub async fn get_user_statistics(
    puuid: &str,
    begin_date: Option<&str>,
    end_date: Option<&str>,
) -> Result<Vec<HashMap<String, AttributeValue>>, Error> {
    let client = connect().await?;
    let query = UserStatisticsQuery::new(puuid, begin_date, end_date);
    let result = client
        .query()
        .table_name("Stats")
        .key_condition_expression(
            "id = :partitionKey and match_date between :beginDate and :endDate",
        )
        .expression_attribute_values(
            ":partitionKey",
            AttributeValue::S(query.partition_key.to_string()),
        )
        .expression_attribute_values(":beginDate", AttributeValue::S(query.begin_date))
        .expression_attribute_values(":endDate", AttributeValue::S(query.end_date))
        .send()
        .await?;

    Ok(result.items.unwrap())
}

pub async fn get_matches(
    puuid: &str,
    start: &str,
    count: &str,
) -> Result<Vec<String>, serde_json::Error> {
    // Load environment variables from .env file
    dotenv().ok();

    let base = Url::parse(&config::RIOT_BASE).unwrap();
    let matches_url = base
        .join(&format!(
            "/lol/match/v5/matches/by-puuid/{puuid}/ids?start={start}&count={count}"
        ))
        .unwrap();

    // Send a GET request to the specified URL and await the response
    let response = reqwest::Client::new()
        .get(matches_url)
        .header("X-Riot-Token", &config::RIOT_API_KEY.to_string())
        .send()
        .await
        .unwrap();

    // Return the response body as a string
    let body = response.text().await.unwrap();

    let matches: Vec<String> = serde_json::from_str(&body).unwrap();

    Ok(matches)
}
