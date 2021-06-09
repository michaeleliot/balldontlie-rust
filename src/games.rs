//! Retrieve basketball games.
use crate::basketball_types::{Game, ListReturnValue};
use crate::helpers::{format_numbers_query_param_array, format_strings_query_param_array};

/// Query Params for games api request.
pub struct GamesQueryParams {
    ///Page number for pagiantion.
    pub page: u32,
    ///Number of values per page. Max 100.
    pub per_page: u32,
    /// List of team ids for filtering games.
    pub team_ids: Vec<u32>,
    /// List of dates for filtering games.
    pub dates: Vec<String>,
    /// List of seasons for filtering games.
    pub seasons: Vec<u32>,
    /// Boolean value indicating if postseason games should be included.
    pub postseason: Option<bool>,
    /// Start date for range of games.
    pub start_date: Option<String>,
    /// End date for range of games.
    pub end_date: Option<String>,
}

impl Default for GamesQueryParams {
    fn default() -> Self {
        GamesQueryParams {
            page: 0,
            per_page: 30,
            team_ids: vec![],
            dates: vec![],
            seasons: vec![],
            postseason: None,
            start_date: None,
            end_date: None,
        }
    }
}

/// Get a vector of basketball games.
///
/// # Examples
///
/// ```
/// let example_params = GamesQueryParams {
///   page: 2,
///   postseason: Some(true),
///   ..Default::default()
/// };
/// get_games(example_params)
/// ```
#[tokio::main]
pub async fn get_games(
    query_params: GamesQueryParams,
) -> Result<Vec<Game>, Box<dyn std::error::Error>> {
    let mut query_params_list = vec![
        ("page".to_string(), query_params.page.to_string()),
        ("per_page".to_string(), query_params.per_page.to_string()),
    ];

    if query_params.postseason.is_some() {
        query_params_list.push((
            "postseason".to_string(),
            query_params.postseason.unwrap().to_string(),
        ))
    }
    if query_params.start_date.is_some() {
        query_params_list.push((
            "start_date".to_string(),
            query_params.start_date.unwrap().to_string(),
        ))
    }
    if query_params.end_date.is_some() {
        query_params_list.push((
            "end_date".to_string(),
            query_params.end_date.unwrap().to_string(),
        ))
    }

    query_params_list.append(&mut format_numbers_query_param_array(
        "team_ids".to_string(),
        &query_params.team_ids,
    ));
    query_params_list.append(&mut format_numbers_query_param_array(
        "seasons".to_string(),
        &query_params.seasons,
    ));
    query_params_list.append(&mut format_strings_query_param_array(
        "dates".to_string(),
        &query_params.dates,
    ));

    let client = reqwest::Client::new();
    let request_url = "https://www.balldontlie.io/api/v1/games";
    let resp = client
        .get(request_url)
        .query(&query_params_list)
        .send()
        .await?
        .json::<ListReturnValue<Game>>()
        .await?;
    Ok(resp.data)
}

/// Retrieves a basketball game given the game_id.
///
/// # Examples
///
/// ```
/// get_game(1533)
/// ```
#[tokio::main]
pub async fn get_game(game_id: u32) -> Result<Game, Box<dyn std::error::Error>> {
    let request_url = format!(
        "https://www.balldontlie.io/api/v1/games/{game_id}",
        game_id = game_id
    );
    let resp = reqwest::get(request_url).await?.json::<Game>().await?;
    Ok(resp)
}
