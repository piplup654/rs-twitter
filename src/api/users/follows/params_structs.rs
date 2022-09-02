use derive_builder::Builder;
use serde::{Serialize, Deserialize};

// By ID ---------------------------
#[derive(Builder, Default, Debug)]
pub struct PGetFollowersById {
    pub id: String,
}
