use derive_builder::Builder;
use serde::{Serialize, Deserialize};

// By ID ---------------------------
#[derive(Builder, Default, Debug)]
pub struct PGetFollowersById {
    pub id: String,
    #[builder(setter(strip_option), default)]
    pub max_results: Option<i32>,
    #[builder(setter(strip_option), default)]
    pub pagination_token: Option<String>,
    #[builder(setter(strip_option), default)]
    pub expansions: Option<Vec<String>>,
    #[builder(setter(strip_option), default)]
    pub tweet_fields: Option<Vec<String>>,
    #[builder(setter(strip_option), default)]
    pub user_fields: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QPGetFollowersById {
    pub max_results: i32,
    pub expansions: String,
}
