use std::collections::HashMap;
use std::convert::From;

pub struct QueryString{
          data: HashMap<String, String>,
}

impl Form<&str> for QueryString {
          fn from(s: &str) -> Self{
                    let mut data = HashMap::new();

                    //give name=bon
          }
}