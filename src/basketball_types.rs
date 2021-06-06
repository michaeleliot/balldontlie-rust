use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
  id: u32,
  first_name: String,
  last_name: String,
  position: String,
  height_feet: Option<u32>,
  height_inches: Option<u32>,
  weight_pounds: Option<u32>,
  team: Team
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Team {
  abbreviation: String,
  city: String,
  conference: String,
  division: String,
  full_name: String,
  id: u32,
  name: String
}

// TODO Make a string enum of status, period, and pattern of time period
#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
  id: u32,
  date: String,
  home_team_score: u32,
  visitor_team_score: u32,
  season: u32,
  period: u32,
  status: String,
  time: String,
  postseason: bool,
  home_team: Team,
  visitor_team: Team
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameStat {
  id: u32,
  date: String,
  home_team_id: u32,
  home_team_score: u32,
  visitor_team_id: u32,
  visitor_team_score: u32,
  season: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerStat {
  id: u32,
  first_name: String,
  last_name: String,
  position: String,
  height_feet: Option<u32>,
  height_inches: Option<u32>,
  weight_pounds: Option<u32>,
  team_id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Stat {
  id: Option<u32>,
  ast: Option<u32>,
  blk: Option<u32>,
  dreb: Option<u32>,
  fg3_pct: Option<f32>,
  fg3a: Option<u32>,
  fg3m: Option<u32>,
  fg_pct: Option<f32>,
  fga: Option<u32>,
  fgm: Option<u32>,
  ft_pct: Option<f32>,
  fta: Option<u32>,
  ftm: Option<u32>,
  min: Option<String>,
  oreb: Option<u32>,
  pf: Option<u32>,
  pts: Option<u32>,
  reb: Option<u32>,
  stl: Option<u32>,
  turnover: Option<u32>,
  game: GameStat,
  player: PlayerStat,
  team: Team,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {
  total_pages: u32,
  current_page: u32,
  next_page: Option<u32>,
  per_page: u32,
  total_count: u32
}

#[derive(Deserialize, Debug)]
pub struct ListReturnValue<T> {
    pub data: Vec<T>,
    meta: Meta,
}