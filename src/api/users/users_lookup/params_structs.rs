use derive_builder::Builder;
use serde::{Serialize, Deserialize};

// By IDs ---------------------------
#[derive(Builder, Default, Debug)]
pub struct PGetUsersByIds {
    pub ids: Vec<String>,
    #[builder(setter(strip_option), default)]
    pub expansions: Option<Vec<String>>,
    #[builder(setter(strip_option), default)]
    pub tweet_fields: Option<Vec<String>>,
    #[builder(setter(strip_option), default)]
    pub user_fields: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct QPGetUsersByIds {
    pub ids: String,
    pub expansions: String,
}
//-----------------------------------

// By Username-----------------------
#[derive(Builder, Default, Debug)]
pub struct PGetUsersByUsernames {
    pub usernames: Vec<String>,
    #[builder(setter(strip_option), default)]
    pub expansions: Option<Vec<String>>,
    #[builder(setter(strip_option), default)]
    pub tweet_fields: Option<Vec<String>>,
    #[builder(setter(strip_option), default)]
    pub user_fields: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct QPGetUsersByUsernames {
    pub usernames: String,
    pub expansions: String,
}
//-----------------------------------
