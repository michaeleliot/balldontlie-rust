use crate::basketball_types::{NoMetaListReturnValue, SeasonAverages};
use crate::helpers::format_numbers_query_param_array;

#[derive(Debug)]
pub struct SeasonAverageQueryParams {
    pub season: u32,
    pub player_ids: Vec<u32>,
}

impl Default for SeasonAverageQueryParams {
    fn default() -> Self {
        SeasonAverageQueryParams {
            season: 2019,
            player_ids: vec![],
        }
    }
}

#[tokio::main]
pub async fn get_season_averages(
    query_params: SeasonAverageQueryParams,
) -> Result<Vec<SeasonAverages>, Box<dyn std::error::Error>> {
    let mut query_params_list = vec![("season".to_string(), query_params.season.to_string())];

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
