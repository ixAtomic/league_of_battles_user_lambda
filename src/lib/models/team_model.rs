use crate::models::match_model::MatchResponseModel;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TeamResponseModel {
    pub team: TeamModel,
    pub matches: Vec<MatchResponseModel>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TeamModel {
    pub id: String,
    pub current_streak: i64,
    pub highest_streak: i64,
    pub loss: i64,
    pub wins: i64,
    pub team_type: i64,
    pub team_name: String,
    pub position: i64,
    pub players: Vec<String>,
    pub matches: Vec<String>,
}
