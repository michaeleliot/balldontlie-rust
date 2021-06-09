//! Retrieve aggregated stats for dates/players.
use crate::basketball_types::{ListReturnValue, Stat};
use crate::helpers::{format_numbers_query_param_array, format_strings_query_param_array};

#[derive(Debug)]
/// Query Params for stats api request.
pub struct StatsQueryParams {
    ///Page number for pagiantion.
    pub page: u32,
    ///Number of values per page. Max 100.
    pub per_page: u32,
    /// List of dates for filtering.
    pub dates: Vec<String>,
    /// List of seasons for filtering.
    pub seasons: Vec<u32>,
    /// List of player ids for filtering.
    pub player_ids: Vec<u32>,
    /// List of game ids for filtering.
    pub game_ids: Vec<u32>,
    /// Boolean value indicating if postseason games should be included.
    pub postseason: Option<bool>,
    /// Start date for range of games.
    pub start_date: Option<String>,
    /// End date for range of games.
    pub end_date: Option<String>,
}

impl Default for StatsQueryParams {
    fn default() -> Self {
        StatsQueryParams {
            page: 0,
            per_page: 30,
            player_ids: vec![],
            game_ids: vec![],
            dates: vec![],
            seasons: vec![],
            postseason: None,
            start_date: None,
            end_date: None,
        }
    }
}

/// Get averaged stats over a group of dates/players returned as a vector.
///
/// # Examples
///
/// ```
/// get_stats(Default::default())
/// ```
#[tokio::main]
pub async fn get_stats(
    query_params: StatsQueryParams,
) -> Result<Vec<Stat>, Box<dyn std::error::Error>> {
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
        "player_ids".to_string(),
        &query_params.player_ids,
    ));
    query_params_list.append(&mut format_numbers_query_param_array(
        "game_ids".to_string(),
        &query_params.game_ids,
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
    let request_url = "https://www.balldontlie.io/api/v1/stats";
    let resp = client
        .get(request_url)
        .query(&query_params_list)
        .send()
        .await?
        .json::<ListReturnValue<Stat>>()
        .await?;

    Ok(resp.data)
}
