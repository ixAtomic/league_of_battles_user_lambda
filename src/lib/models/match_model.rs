use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct MatchResponseModel {
    pub id: String,
    pub match_date: String,
    pub winning_team: String,
    pub teams: Vec<String>,
}
