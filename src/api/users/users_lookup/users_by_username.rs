use crate::authentication::Client;
use serde_json::value::Value;
use super::params_structs::{PGetUsersByUsernames, QPGetUsersByUsernames, PGetUsersByUsernamesBuilder};
use std::collections::HashMap;

impl Client {
    pub async fn get_users_by_usernames(&self, params: &PGetUsersByUsernames) -> Result<Value, reqwest::Error> {
        let request_url = String::from("https://api.twitter.com/2/users/by");
        let bearer_header = format!("Bearer {}", &self.bearer_token[..]);
        let reqwest_client = reqwest::Client::new();
        // empt value for tweet_fields and user_fields
        let empty_vec: &Vec<String> = &vec!["".to_string()];
        //-------------------------------------------
        let expansions = &params.expansions.as_ref().unwrap_or_else(|| &empty_vec);
        let tweet_fields = &params.tweet_fields.as_ref().unwrap_or_else(|| &empty_vec);
        let user_fields = &params.user_fields.as_ref().unwrap_or_else(|| &empty_vec);
        let query_params = QPGetUsersByUsernames {
            usernames: params.usernames.join(","),
            expansions: expansions.join(",")
        };
        let mut query_params_2 = HashMap::new();
        query_params_2.insert("user.fields", user_fields.join(","));
        query_params_2.insert("tweet.fields", tweet_fields.join(","));
        let user_request = match reqwest_client.get(request_url)
            .header("Authorization", bearer_header)
            .query(&query_params)
            .send().await {
                Ok(resp) => resp,
                Err(e) => return Err(e),
        };
        let resp_textified = user_request.text().await.expect("Error while trying to textify response");
        let resp_jsonified: Value = serde_json::from_str(&resp_textified).expect("Error while trying to jsonify response");
        Ok(resp_jsonified)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{authentication::get_api_credentials, api::users};
    #[tokio::test]
    async fn check_users_by_usernames() -> Result<(), Box<dyn std::error::Error>> {
        let (api_key, api_secret) = get_api_credentials()?;
        let mut client = Client {api_key, api_secret, bearer_token: String::from("")};
        let usernames: Vec<String> = vec!["elonmusk".to_string()];
        client.authenticate().await;
        let params = PGetUsersByUsernamesBuilder::default()
            .usernames(usernames.clone())
            .build()?;
        let resp = client.get_users_by_usernames(&params).await;
        assert!(resp.is_ok());
        Ok(())
    }
    #[tokio::test]
    async fn check_users_by_usernames_with_tweet_fields() -> Result<(), Box<dyn std::error::Error>> {
        let (api_key, api_secret) = get_api_credentials()?;
        let mut client = Client {api_key, api_secret, bearer_token: String::from("")};
        let usernames: Vec<String> = vec!["Spotify".to_string()];
        let tweet_fields = vec!["created_at".to_string()];
        client.authenticate().await;
        let params = PGetUsersByUsernamesBuilder::default()
            .usernames(usernames.clone())
            .tweet_fields(tweet_fields)
            .build()?;
        let resp = client.get_users_by_usernames(&params).await;
        assert!(resp.is_ok());
        Ok(())
    }
    #[tokio::test]
    async fn check_users_by_usernames_all() -> Result<(), Box<dyn std::error::Error>> {
        let (api_key, api_secret) = get_api_credentials()?;
        let mut client = Client {api_key, api_secret, bearer_token: String::from("")};
        let usernames: Vec<String> = vec!["0xpiplup".to_string()];
        let expansions = vec!["pinned_tweet_id".to_string()];
        let tweet_fields = vec!["created_at".to_string()];
        let user_fields = vec!["created_at".to_string(), "profile_image_url".to_string()];
        client.authenticate().await;
        let params = PGetUsersByUsernamesBuilder::default()
            .usernames(usernames.clone())
            .tweet_fields(tweet_fields)
            .expansions(expansions)
            .user_fields(user_fields)
            .build()?;
        let resp = client.get_users_by_usernames(&params).await;
        assert!(resp.is_ok());
        Ok(())
    }
}
