use crate::basketball_types::{ListReturnValue, Team};
use std::default::Default;

pub struct TeamQueryParams {
    pub page: u32,
    pub per_page: u32,
}

impl Default for TeamQueryParams {
    fn default() -> Self {
        TeamQueryParams {
            page: 0,
            per_page: 30,
        }
    }
}

#[tokio::main]
pub async fn get_teams(
    query_params: TeamQueryParams,
) -> Result<Vec<Team>, Box<dyn std::error::Error>> {
    let mut query_params_list = vec![];
    query_params_list.push(("page", query_params.page));
    query_params_list.push(("per_page", query_params.per_page));

    let client = reqwest::Client::new();
    let request_url = "https://www.balldontlie.io/api/v1/teams";
    let resp = client
        .get(request_url)
        .query(&query_params_list)
        .send()
        .await?
        .json::<ListReturnValue<Team>>()
        .await?;
    Ok(resp.data)
}

#[tokio::main]
pub async fn get_team(team_id: u32) -> Result<Team, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let request_url = format!("https://www.balldontlie.io/api/v1/teams/{}", team_id);
    let resp = client.get(request_url).send().await?.json::<Team>().await?;
    Ok(resp)
}
