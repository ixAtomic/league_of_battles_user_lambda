use crate::dal::users;
use crate::models::match_model::MatchResponseModel;
use crate::models::team_model::{TeamModel, TeamResponseModel};
use chrono::{Duration, Utc};
// use crate::home::home::{HomePageResponse, StatisticsAggregateResponse};
use crate::models::user_model::{
    StatisticsWithAggregateModel, Stats, UserModel, UserResponseModel,
};
use lambda_http::Error;
use serde_dynamo::aws_sdk_dynamodb_0_26::{from_item, from_items};

pub async fn get_user_data(user_id: &str) -> Result<UserResponseModel, Error> {
    let user = get_user(user_id).await;
    // let matches = users::get_matches(&user.riot_puuid, "0", "20")
    //     .await
    //     .unwrap();
    let user_stats = construct_statistics_model(&user.riot_puuid).await;
    let teams = construct_team_model(&user.teams).await;

    Ok(UserResponseModel {
        user: user,
        stats: aggregate_stats(&user_stats, None),
        teams: teams,
        one_day_stats: aggregate_stats(&user_stats, Some(get_date_days_ago(1))),
        seven_day_stats: aggregate_stats(&user_stats, Some(get_date_days_ago(7))),
        thirty_day_stats: aggregate_stats(&user_stats, Some(get_date_days_ago(30))),
    })
}

fn aggregate_stats(stats: &Vec<Stats>, begin_date: Option<String>) -> StatisticsWithAggregateModel {
    let init = StatisticsWithAggregateModel {
        total_damage: 0,
        total_earnings: 0,
        total_kills: 0,
        total_wins: 0,
        total_losses: 0,
    };

    let f = |acc: StatisticsWithAggregateModel, stat: &Stats| StatisticsWithAggregateModel {
        total_damage: acc.total_damage + stat.damage,
        total_earnings: acc.total_earnings + stat.earnings,
        total_kills: acc.total_kills + stat.kills,
        total_wins: acc.total_wins + if stat.match_result { 1 } else { 0 },
        total_losses: acc.total_losses + if stat.match_result { 0 } else { 1 },
    };

    if let Some(date) = begin_date {
        stats
            .iter()
            .filter(|stat| stat.match_date > date)
            .fold(init, f)
    } else {
        stats.iter().fold(init, f)
    }
}

async fn construct_team_model(team_ids: &Vec<String>) -> Vec<TeamResponseModel> {
    //let mut teams: Vec<TeamModel> = Vec::new();
    let results = users::get_user_teams(team_ids).await.unwrap_or_default();
    let items = results.get("Teams").take().unwrap();
    let mut teams: Vec<TeamResponseModel> = Vec::new();
    for item in items {
        let team_model: TeamModel =
            from_item(item.to_owned()).expect("Should parse into Team Response model");
        let matches: Vec<MatchResponseModel> = construct_match_model(&team_model.matches).await;
        teams.push(TeamResponseModel {
            team: team_model,
            matches,
        })
    }
    teams
}

async fn construct_match_model(matches: &Vec<String>) -> Vec<MatchResponseModel> {
    let mut team_matches: Vec<MatchResponseModel> = Vec::new();
    let results = users::get_team_matches(matches).await.unwrap_or_default();
    let items = results.get("Matches").take().unwrap();
    for item in items {
        let matches_model: MatchResponseModel =
            from_item(item.to_owned()).expect("Should parse into Match Response model");
        team_matches.push(matches_model)
    }
    team_matches
}

async fn construct_statistics_model(puuid: &str) -> Vec<Stats> {
    let result = users::get_user_statistics(puuid, None, None)
        .await
        .unwrap_or_default();
    println!("result {:?}", result);
    let results: Vec<Stats> = from_items(result).expect("results should parse to Stats model");
    results
}

pub async fn get_user(username: &str) -> UserModel {
    let result = users::get_user(username).await.unwrap();
    let result: UserModel =
        from_item(result).expect("the result should parse to the Response Model");
    return result;
}

fn get_date_days_ago(days: i64) -> String {
    (Utc::now() - Duration::days(days))
        .format("%Y%m%d%H%M%S")
        .to_string()
}
