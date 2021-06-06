use crate::basketball_types::{Player, ListReturnValue};

#[tokio::main]
pub async fn get_players() -> Result<Vec<Player>, Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://www.balldontlie.io/api/v1/players")
        .await?
        .json::<ListReturnValue<Player>>()
        .await?;
    Ok(resp.data)
}

#[tokio::main]
pub async fn get_player(player_id: u32) -> Result<Player, Box<dyn std::error::Error>> {
    let request_url = format!("https://www.balldontlie.io/api/v1/players/{player_id}", player_id = player_id,);
    let resp = reqwest::get(request_url)
        .await?
        .json::<Player>()
        .await?;
    Ok(resp)
}