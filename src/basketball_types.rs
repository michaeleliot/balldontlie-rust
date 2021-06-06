use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
  id: u32,
  first_name: String,
  last_name: String,
  position: String,
  height_feet: Option<u32>,
  height_inches: Option<u32>,
  weight_pounds: Option<u32>,
  team: Team
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Team {
  abbreviation: String,
  city: String,
  conference: String,
  division: String,
  full_name: String,
  id: u32,
  name: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {
  total_pages: u32,
  current_page: u32,
  next_page: Option<u32>,
  per_page: u32,
  total_count: u32
}

#[derive(Deserialize, Debug)]
pub struct ListReturnValue<T> {
    pub data: Vec<T>,
    meta: Meta,
}