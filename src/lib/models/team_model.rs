use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct TeamResponseModel {
    pub id: String,
    pub current_streak: i64,
    pub highest_streak: i64,
    pub loss: i64,
    pub wins: i64,
    pub team_type: i64,
    pub team_name: String,
    pub position: i64,
    pub players: Vec<String>,
}
