use crate::basketball_types::{Player, ListReturnValue};

pub struct PlayerQueryParams {
  pub page: u32,
  pub per_page: u32,
  pub search: String
}

impl Default for PlayerQueryParams {
  fn default() -> Self {
    PlayerQueryParams { page: 0, per_page: 30, search: "".to_string() }
  }
}

#[tokio::main]
pub async fn get_players(query_params: PlayerQueryParams) -> Result<Vec<Player>, Box<dyn std::error::Error>> {
  let query_params_list = vec![
    ("page", query_params.page.to_string()),
    ("per_page", query_params.per_page.to_string()),
    ("search", query_params.search)
  ];

  let client = reqwest::Client::new();  
  let request_url = "https://www.balldontlie.io/api/v1/players";
  let resp = client.get(request_url)
    .query(&query_params_list)
    .send()
    .await?
    .json::<ListReturnValue<Player>>()
    .await?;
  Ok(resp.data)
}

#[tokio::main]
pub async fn get_player(player_id: u32) -> Result<Player, Box<dyn std::error::Error>> {
    let request_url = format!("https://www.balldontlie.io/api/v1/players/{}", player_id);
    let resp = reqwest::get(request_url)
        .await?
        .json::<Player>()
        .await?;
    Ok(resp)
}