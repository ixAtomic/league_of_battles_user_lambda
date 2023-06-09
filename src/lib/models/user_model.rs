use crate::models::team_model::TeamResponseModel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct UserResponseModel {
    pub user: UserModel,
    pub stats: StatisticsWithAggregateModel,
    pub teams: Vec<TeamResponseModel>,
    pub one_day_stats: StatisticsWithAggregateModel,
    pub seven_day_stats: StatisticsWithAggregateModel,
    pub thirty_day_stats: StatisticsWithAggregateModel,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserModel {
    pub user_name: String,
    pub teams: Vec<String>,
    pub last_name: String,
    pub email: String,
    pub first_name: String,
    pub riot_puuid: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Stats {
    pub id: String,
    pub match_date: String,
    pub damage: i64,
    pub earnings: i64,
    pub kills: i64,
    pub match_id: String,
    pub match_result: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StatisticsWithAggregateModel {
    pub total_damage: i64,
    pub total_earnings: i64,
    pub total_kills: i64,
    pub total_wins: i64,
    pub total_losses: i64,
}
