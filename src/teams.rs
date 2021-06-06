use crate::basketball_types::{Team, ListReturnValue};

#[tokio::main]
pub async fn get_teams() -> Result<Vec<Team>, Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://www.balldontlie.io/api/v1/teams")
        .await?
        .json::<ListReturnValue<Team>>()
        .await?;
    Ok(resp.data)
}

#[tokio::main]
pub async fn get_team(team_id: u32) -> Result<Team, Box<dyn std::error::Error>> {
    let request_url = format!("https://www.balldontlie.io/api/v1/teams/{team_id}", team_id = team_id,);
    let resp = reqwest::get(request_url)
        .await?
        .json::<Team>()
        .await?;
    Ok(resp)
}