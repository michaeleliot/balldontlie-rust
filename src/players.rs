//! Retrieve players.
use crate::basketball_types::{ListReturnValue, Player};

/// Query Params for players api request.
pub struct PlayerQueryParams {
    ///Page number for pagiantion.
    pub page: u32,
    ///Number of values per page. Max 100.
    pub per_page: u32,
    ///Optional search string to search players
    pub search: Option<String>,
}

impl Default for PlayerQueryParams {
    fn default() -> Self {
        PlayerQueryParams {
            page: 0,
            per_page: 30,
            search: None,
        }
    }
}

/// Get a vector of basketball players.
///
/// # Examples
///
/// ```
/// get_players(Default::default())
/// ```
#[tokio::main]
pub async fn get_players(
    query_params: PlayerQueryParams,
) -> Result<Vec<Player>, Box<dyn std::error::Error>> {
    let mut query_params_list = vec![
        ("page", query_params.page.to_string()),
        ("per_page", query_params.per_page.to_string())
    ];

    if query_params.search.is_some() {
        query_params_list.push((
            "search",
            query_params.search.unwrap().to_string(),
        ))
    }

    let client = reqwest::Client::new();
    let request_url = "https://www.balldontlie.io/api/v1/players";
    let resp = client
        .get(request_url)
        .query(&query_params_list)
        .send()
        .await?
        .json::<ListReturnValue<Player>>()
        .await?;
    Ok(resp.data)
}

/// Retrieves a basketball player.
///
/// # Examples
///
/// ```
/// get_player(1)
/// ```
#[tokio::main]
pub async fn get_player(player_id: u32) -> Result<Player, Box<dyn std::error::Error>> {
    let request_url = format!("https://www.balldontlie.io/api/v1/players/{}", player_id);
    let resp = reqwest::get(request_url).await?.json::<Player>().await?;
    Ok(resp)
}
