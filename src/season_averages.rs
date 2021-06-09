//! Retrieve aggregated season averages.
use crate::basketball_types::{NoMetaListReturnValue, SeasonAverages};
use crate::helpers::format_numbers_query_param_array;

#[derive(Debug)]
/// Query Params for season averages api request.
pub struct SeasonAverageQueryParams {
    /// Season for averages.
    pub season: Option<u32>,
    /// List of player ids for filtering.
    pub player_ids: Vec<u32>,
}

impl Default for SeasonAverageQueryParams {
    fn default() -> Self {
        SeasonAverageQueryParams {
            season: None,
            player_ids: vec![],
        }
    }
}

/// Retrieves the season averages for particular player(s) returned as a vector. Selects at random if not requested.
///
/// # Examples
///
/// ```
/// get_season_averages(Default::default())
/// ```
#[tokio::main]
pub async fn get_season_averages(
    query_params: SeasonAverageQueryParams,
) -> Result<Vec<SeasonAverages>, Box<dyn std::error::Error>> {
    let mut query_params_list = vec![];

    if query_params.season.is_some() {
      query_params_list.push((
          "season".to_string(),
          query_params.season.unwrap().to_string(),
      ))
    }
    query_params_list.append(&mut format_numbers_query_param_array(
        "player_ids".to_string(),
        &query_params.player_ids,
    ));

    let client = reqwest::Client::new();
    let request_url = "https://www.balldontlie.io/api/v1/season_averages";
    let resp = client
        .get(request_url)
        .query(&query_params_list)
        .send()
        .await?
        .json::<NoMetaListReturnValue<SeasonAverages>>()
        .await?;

    Ok(resp.data)
}
