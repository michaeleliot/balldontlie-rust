pub fn format_numbers_query_param_array(query_param: String, arr: &Vec<u32>) -> Vec<(String, std::string::String)> {
  let query_param_format = format!("{}[]", &query_param);
  return arr
    .iter()
    .map(|x| {
      return (query_param_format.clone(), x.to_string())
    })
    .collect();
}

pub fn format_strings_query_param_array(query_param: String, arr: &Vec<String>) -> Vec<(String, std::string::String)> {
  let query_param_format = format!("{}[]", &query_param);
  return arr
    .iter()
    .map(|x| {
      return (query_param_format.clone(), x.to_string())
    })
    .collect();
}