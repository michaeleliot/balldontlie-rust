//! Types for balldontlie data.
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
/// A Basketball Player.
pub struct Player {
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
    pub position: String,
    pub height_feet: Option<u32>,
    pub height_inches: Option<u32>,
    pub weight_pounds: Option<u32>,
    pub team: Team,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
/// A Basketball Team.
pub struct Team {
  pub abbreviation: String,
  pub city: String,
  pub conference: String,
  pub division: String,
  pub full_name: String,
  pub id: u32,
  pub name: String,
}

// TODO Make a string enum of status, period, and pattern of time period
#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
/// A Basketball Game.
pub struct Game {
  pub id: u32,
  pub date: String,
  pub home_team_score: u32,
  pub visitor_team_score: u32,
  pub season: u32,
  pub period: u32,
  pub status: String,
  pub time: String,
  pub postseason: bool,
  pub home_team: Team,
  pub visitor_team: Team,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
/// The stats for a basketball game.
pub struct GameStat {
  pub id: u32,
  pub date: String,
  pub home_team_id: u32,
  pub home_team_score: u32,
  pub visitor_team_id: u32,
  pub visitor_team_score: u32,
  pub season: u32,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
/// The stats for a basketball player.
pub struct PlayerStat {
  pub id: u32,
  pub first_name: String,
  pub last_name: String,
  pub position: String,
  pub height_feet: Option<u32>,
  pub height_inches: Option<u32>,
  pub weight_pounds: Option<u32>,
  pub team_id: u32,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
/// The detailed stats for an individual game and individual player. I.e. player x in game y
pub struct Stat {
  pub id: u32,
  pub ast: Option<u32>,
  pub blk: Option<u32>,
  pub dreb: Option<u32>,
  pub fg3_pct: Option<f32>,
  pub fg3a: Option<u32>,
  pub fg3m: Option<u32>,
  pub fg_pct: Option<f32>,
  pub fga: Option<u32>,
  pub fgm: Option<u32>,
  pub ft_pct: Option<f32>,
  pub fta: Option<u32>,
  pub ftm: Option<u32>,
  pub min: Option<String>,
  pub oreb: Option<u32>,
  pub pf: Option<u32>,
  pub pts: Option<u32>,
  pub reb: Option<u32>,
  pub stl: Option<u32>,
  pub turnover: Option<u32>,
  pub game: GameStat,
  pub player: PlayerStat,
  pub team: Team,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
/// The season averages for a particular player.
pub struct SeasonAverages {
  pub player_id: u32,
  pub season: u32,
  pub games_played: Option<u32>,
  pub ast: Option<f32>,
  pub blk: Option<f32>,
  pub dreb: Option<f32>,
  pub fg3_pct: Option<f32>,
  pub fg3a: Option<f32>,
  pub fg3m: Option<f32>,
  pub fg_pct: Option<f32>,
  pub fga: Option<f32>,
  pub fgm: Option<f32>,
  pub ft_pct: Option<f32>,
  pub fta: Option<f32>,
  pub ftm: Option<f32>,
  pub min: Option<String>,
  pub oreb: Option<f32>,
  pub pf: Option<f32>,
  pub pts: Option<f32>,
  pub reb: Option<f32>,
  pub stl: Option<f32>,
  pub turnover: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
/// Meta information about the api call, mainly for pagination.
pub struct Meta {
  pub total_pages: u32,
  pub current_page: u32,
  pub next_page: Option<u32>,
  pub per_page: u32,
  pub total_count: u32,
}

#[derive(Deserialize, Debug, Default, PartialEq)]
/// Return Value from balldontlie.
pub struct ListReturnValue<T> {
    /// The return data.
    pub data: Vec<T>,
    pub meta: Meta,
}

#[derive(Deserialize, Debug, Default, PartialEq)]
/// Return Value from balldontlie without meta.
pub struct NoMetaListReturnValue<T> {
    /// The return data.
    pub data: Vec<T>,
}
