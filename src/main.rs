mod teams; mod players; mod games; mod basketball_types; mod helpers;
use teams::{get_teams, get_team, TeamQueryParams};
use players::{get_players, get_player, PlayerQueryParams};
use games::{get_games};


fn main() -> Result<(), Box<dyn std::error::Error>> {
  let games = get_games(Default::default()).unwrap();
  println!("{:#?}", games);

  // let players_default = get_players(Default::default()).unwrap();
  // let players_query_params = PlayerQueryParams { page: 0, per_page: 3, search: "Davis".to_string() };
  // let players = get_players(players_query_params).unwrap();
  // let player = get_player(4).unwrap();
  // println!("{:#?}", players_default);
  // println!("{:#?}", players);
  // println!("{:#?}", player);

  // let teams_default = get_teams(Default::default()).unwrap();
  // let teams_query_params = TeamQueryParams { page: 0, per_page: 3 };
  // let teams = get_teams(teams_query_params).unwrap();
  // let team = get_team(4).unwrap();
  // println!("{:#?}", teams_default);
  // println!("{:#?}", teams);
  // println!("{:#?}", team);
  Ok(())
}