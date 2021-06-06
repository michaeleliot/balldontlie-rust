mod teams; mod basketball_types;
use teams::{get_teams, get_team};

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let players = get_teams().unwrap();
  let player = get_team(4).unwrap();
  println!("{:#?}", players);
  println!("{:#?}", player);
  Ok(())
}